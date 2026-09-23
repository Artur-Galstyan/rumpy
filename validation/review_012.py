"""Extra debug/release and mutation checks for publication 012, in a disposable copy."""
from pathlib import Path
import json
import re
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def run(args, cwd, *, red=False):
    result = subprocess.run(args, cwd=cwd, capture_output=True, text=True, timeout=180)
    output = result.stdout + result.stderr
    if red:
        assert result.returncode != 0 and "test result: FAILED" in output, output
        assert "could not compile" not in output, output
    else:
        assert result.returncode == 0, output
    print("PASS", "negative control" if red else "", " ".join(args))
    print("\n".join(line for line in output.splitlines() if "test result:" in line))
    return output


def main():
    with tempfile.TemporaryDirectory(prefix="rumpy-012-review-") as tmp:
        work = Path(tmp) / "repo"
        shutil.copytree(ROOT, work, ignore=shutil.ignore_patterns(".git", "target", "__pycache__", ".rustlings-state.txt"))
        source = (work / "exercises/03_reductions/011_argmax.rs").read_text()
        tests = source.split("#[cfg(test)]", 1)[1]
        assert tests.count("use super::argmax;") == 1
        tests = tests.replace("use super::argmax;", "use rumpy::argmax;", 1)
        (work / "tests/review_public_argmax.rs").write_text("#[cfg(test)]" + tests)
        # The complete preserved test module, with only its import changed.
        for flags in [[], ["--release"]]:
            run(["cargo", "test", *flags, "--test", "review_public_argmax", "--bin", "012_add_sol"], work)

        fixture = json.loads((ROOT / "validation/argmax-numpy.json").read_text())
        blocks = []
        for case in fixture["cases"]:
            values = ",".join(f"f64::from_bits({x}u64)" for x in case["input_bits"])
            blocks.append("{" + f"let a = rumpy::Array::from_vec(vec![{values}], vec!{json.dumps(case['input_shape'])}).unwrap();"
                          + f"assert_eq!(rumpy::argmax(&a), {case['expected_index']}usize);" + "}")
        text = ""
        for i in range(0, len(blocks), 32):
            text += f"#[test] fn oracle_{i}() {{" + "\n".join(blocks[i:i + 32]) + "}\n"
        (work / "tests/review_argmax_oracle.rs").write_text(text)
        for flags in [[], ["--release"]]:
            run(["cargo", "test", *flags, "--test", "review_argmax_oracle"], work)
        print(f"PASS public argmax: {len(fixture['cases'])} saved NumPy index cases in both modes")

        exercise = work / "exercises/04_elementwise/012_add.rs"
        stub = exercise.read_text()
        reference = (work / "solutions/04_elementwise/012_add.rs").read_text()
        assert stub.split("#[cfg(test)]", 1)[1] == reference.split("#[cfg(test)]", 1)[1]
        output = run(["cargo", "test", "--bin", "012_add"], work, red=True)
        assert "not yet implemented: add" in output and "0 passed; 12 failed" in output
        for name, old, new in [
            ("count-only-validation", "a.shape() != b.shape()", "a.size() != b.size()"),
            ("zero-seed", "x + y", "0.0 + x + y"),
            ("subtraction", "x + y", "x - y"),
            ("flattened-output", "a.shape().to_vec()", "vec![a.size()]"),
            ("empty-before-validation", "    // Validate the complete shapes", "    if a.size() == 0 { return a.clone(); }\n    // Validate the complete shapes"),
        ]:
            assert reference.count(old) == 1, name
            exercise.write_text(reference.replace(old, new, 1))
            output = run(["cargo", "test", "--bin", "012_add"], work, red=True)
            failed = re.findall(r"^test (\S+) \.\.\. FAILED$", output, re.MULTILINE)
            print(f"PASS rejected {name}: {failed}")
        # The checker must reject a missing oracle instead of silently omitting it.
        (work / "validation/add-numpy.json").unlink()
        result = subprocess.run(["python3", "scripts/check_authoring.py"], cwd=work, capture_output=True, text=True, timeout=30)
        assert result.returncode != 0 and "Missing or empty NumPy fixture for add" in result.stderr
        print("PASS missing-fixture negative control")


if __name__ == "__main__":
    main()
