"""Check a newly authored publication without editing the learner's files.

Python 3.11+, Cargo, and Rustlings 6.5.0 are required. Saved NumPy fixtures
describe zeros/ones(shape) and full(shape, scalar fill_value). Extend oracle
calls explicitly when the course introduces other signatures.
"""
from pathlib import Path
import json
import shutil
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def run(args, cwd, *, unfinished=False):
    result = subprocess.run(
        args, cwd=cwd, text=True, capture_output=True, timeout=180
    )
    output = result.stdout + result.stderr
    if unfinished:
        valid = (
            result.returncode != 0
            and "not yet implemented" in output
            and "test result: FAILED" in output
            and "could not compile" not in output
        )
    else:
        valid = result.returncode == 0
    if not valid:
        raise RuntimeError(f"Unexpected result for {args}:\n{output}")
    print(f"PASS {'expected TODO failure: ' if unfinished else ''}{' '.join(args)}")
    return output


def main():
    progress = json.loads((ROOT / ".rumpy/progress.json").read_text())
    metadata = tomllib.loads((ROOT / "info.toml").read_text())
    exercises = {item["name"]: item for item in metadata["exercises"]}
    current = progress["current_exercise"]
    graduated = progress.get("graduated", {})
    if current in graduated:
        raise ValueError("The current publication needs an unfinished task")

    with tempfile.TemporaryDirectory(prefix="rumpy-author-check-") as temp:
        work = Path(temp) / "repo"
        shutil.copytree(
            ROOT, work,
            ignore=shutil.ignore_patterns(".git", "target", ".rustlings-state.txt", "__pycache__"),
        )
        run(["cargo", "fmt", "--check"], work)
        run(["cargo", "check", "--all-targets"], work)
        run(["cargo", "clippy", "--all-targets", "--", "-D", "warnings"], work)
        run(["cargo", "test", "--lib", "--test", "scaffold"], work)
        for name in graduated:
            run(["cargo", "test", "--bin", name], work)
        run(["cargo", "test", "--bin", current], work, unfinished=True)

        # Require the same contract tests in each independent reference file.
        for name, item in exercises.items():
            relative = Path(item.get("dir") or "") / f"{name}.rs"
            exercise = (work / "exercises" / relative).read_text()
            reference = (work / "solutions" / relative).read_text()
            marker = "#[cfg(test)]"
            if marker not in exercise or marker not in reference:
                raise ValueError(f"Missing contract tests for {name}")
            if exercise.split(marker, 1)[1].strip() != reference.split(marker, 1)[1].strip():
                raise ValueError(f"Reference and exercise tests differ for {name}")
        run(["rustlings", "dev", "check", "--require-solutions"], work)

        # Install references only in this disposable copy, never into src/.
        for name, item in exercises.items():
            if name not in graduated:
                relative = Path(item.get("dir") or "") / f"{name}.rs"
                shutil.copyfile(work / "solutions" / relative, work / "exercises" / relative)
        run(["cargo", "test", "--all-targets"], work)

        # Compare completed library functions and active references to real oracle data.
        declarations = []
        checks = ["#[test]", "fn matches_saved_numpy_fixtures() {"]
        case_count = 0
        for fixture_path in sorted((work / "validation").glob("*-numpy.json")):
            name = fixture_path.name.removesuffix("-numpy.json")
            fixture = json.loads(fixture_path.read_text())
            if name in graduated:
                callable_name = graduated[name]["public_api"]
            else:
                item = exercises[name]
                relative = Path(item.get("dir") or "") / f"{name}.rs"
                module = f"oracle_{name}"
                declarations.extend([
                    "#[allow(dead_code)]",
                    f'#[path = "../exercises/{relative.as_posix()}"]',
                    f"mod {module};",
                ])
                callable_name = f"{module}::{name}"
            for case in fixture["cases"]:
                shape = json.dumps(case["shape"])
                if name == "full":
                    arguments = f"expected_shape, f64::from_bits({case['fill_bits']}u64)"
                    values = ", ".join(f"f64::from_bits({bits}u64)" for bits in case["data_bits"])
                    data = f"[{values}]"
                elif name in {"zeros", "ones"}:
                    arguments = "expected_shape"
                    data = json.dumps(case["data"])
                else:
                    raise ValueError(f"Unsupported oracle signature: {name}")
                checks.extend([
                    f"    let expected_shape: &[usize] = &{shape};",
                    f"    let expected_data: &[f64] = &{data};",
                    f"    let a = {callable_name}({arguments});",
                    "    assert_eq!(a.shape(), expected_shape);",
                    f"    assert_eq!(a.size(), {case['size']});",
                    "    assert_eq!(a.as_slice().len(), expected_data.len());",
                    "    for (actual, expected) in a.as_slice().iter().zip(expected_data) {",
                    "        assert_eq!(actual.to_bits(), expected.to_bits());",
                    "    }",
                ])
                case_count += 1
        if case_count == 0:
            raise ValueError("No NumPy oracle cases found")
        checks.append("}")
        (work / "tests/author_numpy_oracle.rs").write_text(
            "\n".join(declarations + checks) + "\n"
        )
        run(["cargo", "test", "--test", "author_numpy_oracle"], work)
        print(f"PASS {case_count} saved NumPy cases, including shape and exact value bits")
    print("PASS publication checks. Learner files remain untouched.")


if __name__ == "__main__":
    main()
