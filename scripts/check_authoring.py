"""Check a newly authored publication without editing the learner's files.

Python 3.11+, Cargo, and Rustlings 6.5.0 are required. Saved NumPy fixtures
describe zeros/ones(shape), full(shape, scalar), arange(start, stop, step),
eye(rows, cols), reshape(array, shape) -> Result<Array, ShapeError>, and
transpose(array) -> Array, and sum/mean/amax(array) -> f64. Solved active tasks and their public copies stay
distinct from graduation until the learner connects the original runner.
Known missing historical markers remain visible baseline errors. A disposable
project excludes those runners from a second Rustlings check. Their complete
regression tests and references still run, without edits to learner source.
"""
from pathlib import Path
import json
import re
import shutil
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def run(args, cwd, *, unfinished=False, known_failure=None):
    result = subprocess.run(
        args, cwd=cwd, text=True, capture_output=True, timeout=180
    )
    output = result.stdout + result.stderr
    if known_failure:
        failed = set(re.findall(r"^test (\S+) \.\.\. FAILED$", output, re.MULTILINE))
        valid = (result.returncode != 0 and failed == {known_failure}
                 and "12 passed; 1 failed" in output
                 and "attempt to multiply with overflow" in output
                 and "src/array.rs:" in output and "could not compile" not in output)
        if result.returncode == 0:
            print(f"BASELINE RESOLVED: {args}; update progress metadata")
            return output
        if valid:
            print(f"BASELINE ERROR: {args}: {known_failure}: stride overflow; remaining 12 tests pass")
            return output
    elif unfinished:
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
    solved_active = {
        name: record for name, record in progress.get("active", {}).items()
        if record.get("status") == "solved_pending_regression_connection"
    }
    connected_active = {
        name: record for name, record in progress.get("active", {}).items()
        if record.get("status") == "connected_regression_failure"
    }
    public_records = graduated | connected_active
    baseline_failures = progress.get("author_check_baseline", {}).get("test_failures", {})
    graduated_exercises = {item["exercise"] for item in graduated.values()}
    if current in graduated_exercises:
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
        for name in sorted(graduated_exercises):
            run(["cargo", "test", "--bin", name], work)
        for record in connected_active.values():
            name = record["exercise"]
            run(["cargo", "test", "--bin", name], work,
                known_failure=baseline_failures.get(name))
        for operation, record in solved_active.items():
            name = record["exercise"]
            run(["cargo", "test", "--bin", name], work)
            item = exercises[name]
            source = work / "exercises" / (item.get("dir") or "") / f"{name}.rs"
            tests = source.read_text().split("#[cfg(test)]", 1)[1]
            original_import = f"use super::{operation};"
            if tests.count(original_import) != 1:
                raise ValueError(f"Unexpected active test import for {name}")
            # Test the public copy without changing the learner's actual runner.
            public_tests = tests.replace(original_import, f"use {record['public_api']};", 1)
            test_name = f"author_{name}_public"
            (work / "tests" / f"{test_name}.rs").write_text("#[cfg(test)]" + public_tests)
            run(["cargo", "test", "--test", test_name], work)
        run(["cargo", "test", "--bin", current], work, unfinished=True)

        # Require the same contract tests in each independent reference file.
        for name, item in exercises.items():
            relative = Path(item.get("dir") or "") / f"{name}.rs"
            exercise = (work / "exercises" / relative).read_text()
            reference = (work / "solutions" / relative).read_text()
            marker = "#[cfg(test)]"
            if marker not in exercise or marker not in reference:
                raise ValueError(f"Missing contract tests for {name}")
            exercise_tests = exercise.split(marker, 1)[1].strip()
            reference_tests = reference.split(marker, 1)[1].strip()
            # Graduation can change only this test import, not the assertions.
            for operation, record in public_records.items():
                if record["exercise"] == name:
                    if "regression_test_import" in record:
                        original = record["regression_test_import"]
                        normalized = record["reference_test_import"]
                        if exercise_tests.count(original) != 1:
                            raise ValueError(f"Unexpected regression import for {name}")
                        exercise_tests = exercise_tests.replace(original, normalized, 1)
                    else:
                        exercise_tests = exercise_tests.replace(
                            f"use {record['public_api']};", f"use super::{operation};", 1
                        )
            if exercise_tests != reference_tests:
                raise ValueError(f"Reference and exercise tests differ for {name}")
            run(["rustfmt", "--edition", "2024", "--check",
                 str(work / "solutions" / relative)], work)
            run(["cargo", "test", "--bin", f"{name}_sol"], work,
                known_failure=baseline_failures.get(f"{name}_sol"))

        declared_missing = set(progress.get("author_check_baseline", {}).get("missing_todo", []))
        connected_exercises = {record["exercise"] for record in public_records.values()}
        if not declared_missing <= connected_exercises:
            raise ValueError("Only connected public-API runners may have a historical-marker baseline")
        missing = set()
        for name in declared_missing:
            item = exercises[name]
            relative = Path(item.get("dir") or "") / f"{name}.rs"
            if "// TODO" not in (work / "exercises" / relative).read_text():
                missing.add(name)
        if missing:
            result = subprocess.run(
                ["rustlings", "dev", "check", "--require-solutions"],
                cwd=work, text=True, capture_output=True, timeout=180,
            )
            output = result.stdout + result.stderr
            expected_errors = [
                f"Error: Didn't find any `// TODO` comment in the file `exercises/{exercises[name]['dir']}/{name}.rs`."
                for name in missing
            ]
            if result.returncode == 0 or not any(error in output for error in expected_errors):
                raise RuntimeError(f"Unexpected Rustlings baseline result:\n{output}")
            print(f"BASELINE ERROR: missing historical // TODO in {sorted(missing)}")
            # Move unchanged files outside Rustlings directories only in this copy.
            rustlings_work = Path(temp) / "rustlings-metadata-check"
            shutil.copytree(work, rustlings_work, ignore=shutil.ignore_patterns("target"))
            parts = (rustlings_work / "info.toml").read_text().split("[[exercises]]")
            kept = [parts[0]]
            for part in parts[1:]:
                if tomllib.loads(part)["name"] not in missing:
                    kept.append("[[exercises]]" + part)
            (rustlings_work / "info.toml").write_text("".join(kept))
            for name in missing:
                relative = Path(exercises[name].get("dir") or "") / f"{name}.rs"
                for folder in ("exercises", "solutions"):
                    source = rustlings_work / folder / relative
                    destination = rustlings_work / "baseline-preserved" / folder / relative
                    destination.parent.mkdir(parents=True, exist_ok=True)
                    shutil.move(source, destination)
            run(["rustlings", "dev", "update"], rustlings_work)
            run(["rustlings", "dev", "check", "--require-solutions"], rustlings_work)
            print("PASS isolated Rustlings metadata check; full-repository baseline error remains")
        else:
            run(["rustlings", "dev", "check", "--require-solutions"], work)

        # Install references only in this disposable copy, never into src/.
        for name, item in exercises.items():
            if name not in connected_exercises:
                relative = Path(item.get("dir") or "") / f"{name}.rs"
                shutil.copyfile(work / "solutions" / relative, work / "exercises" / relative)
        # Run every bin separately so the known learner regression cannot hide later tests.
        run(["cargo", "test", "--lib", "--test", "scaffold"], work)
        cargo = tomllib.loads((work / "Cargo.toml").read_text())
        for item in cargo["bin"]:
            name = item["name"]
            run(["cargo", "test", "--bin", name], work,
                known_failure=baseline_failures.get(name))
        if (work / "src/main.rs").exists():
            run(["cargo", "test", "--bin", cargo["package"]["name"]], work)

        # Compare completed library functions and active references to real oracle data.
        declarations = []
        checks = ["#[test]", "fn matches_saved_numpy_fixtures() {"]
        case_count = 0
        rejected_count = 0
        empty_reduction_count = 0
        for fixture_path in sorted((work / "validation").glob("*-numpy.json")):
            name = fixture_path.name.removesuffix("-numpy.json")
            fixture = json.loads(fixture_path.read_text())
            if name in public_records:
                callable_name = public_records[name]["public_api"]
            elif name in solved_active:
                callable_name = solved_active[name]["public_api"]
            else:
                exercise_id = progress["active"][name]["exercise"]
                item = exercises[exercise_id]
                relative = Path(item.get("dir") or "") / f"{exercise_id}.rs"
                module = f"oracle_{name}"
                declarations.extend([
                    "#[allow(dead_code)]",
                    f'#[path = "../exercises/{relative.as_posix()}"]',
                    f"mod {module};",
                ])
                callable_name = f"{module}::{name}"
            for case in fixture["cases"]:
                # Bound each function's IR/debug-info size as the fixture set grows.
                # Keep all cases, but avoid one enormous Rust test function.
                if case_count and case_count % 64 == 0:
                    checks.extend(["}", "#[test]", f"fn numpy_batch_{case_count // 64}() {{"])
                if name in {"sum", "mean", "amax"}:
                    # Scalar reductions have no output Array shape or buffer.
                    values = ", ".join(f"f64::from_bits({bits}u64)" for bits in case["input_bits"])
                    expected = (
                        "    assert!(actual.is_nan());" if case["expected_nan"] else
                        f"    assert_eq!(actual.to_bits(), {case['expected_bits']}u64);"
                    )
                    checks.extend([
                        "    {",
                        f"    let input = rumpy::Array::from_vec(vec![{values}], vec!{json.dumps(case['input_shape'])}).unwrap();",
                        "    let before: Vec<u64> = input.as_slice().iter().map(|v| v.to_bits()).collect();",
                        f"    let actual = {callable_name}(&input);",
                        expected,
                        "    assert_eq!(input.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(), before);",
                        "    }",
                    ])
                    case_count += 1
                    continue
                shape = json.dumps(case["shape"])
                setup = []
                result_suffix = ""
                if name == "full":
                    arguments = f"expected_shape, f64::from_bits({case['fill_bits']}u64)"
                    values = ", ".join(f"f64::from_bits({bits}u64)" for bits in case["data_bits"])
                    data = f"[{values}]"
                elif name in {"zeros", "ones"}:
                    arguments = "expected_shape"
                    data = json.dumps(case["data"])
                elif name == "arange":
                    arguments = f"{case['start']}i32, {case['stop']}i32, {case['step']}i32"
                    values = ", ".join(f"f64::from_bits({bits}u64)" for bits in case["data_bits"])
                    data = f"[{values}]"
                elif name == "eye":
                    arguments = f"{case['rows']}usize, {case['cols']}usize"
                    values = ", ".join(f"f64::from_bits({bits}u64)" for bits in case["data_bits"])
                    data = f"[{values}]"
                elif name in {"reshape", "transpose"}:
                    input_values = ", ".join(f"f64::from_bits({bits}u64)" for bits in case["input_bits"])
                    setup = [
                        f"    let input = rumpy::Array::from_vec(vec![{input_values}], vec!{json.dumps(case['input_shape'])}).unwrap();"
                    ]
                    arguments = "&input, expected_shape" if name == "reshape" else "&input"
                    values = ", ".join(f"f64::from_bits({bits}u64)" for bits in case["data_bits"])
                    data = f"[{values}]"
                    result_suffix = ".unwrap()" if name == "reshape" else ""
                else:
                    raise ValueError(f"Unsupported oracle signature: {name}")
                checks.extend([
                    "    {",  # Separate scopes avoid a deep debug-info shadow chain.
                    f"    let expected_shape: &[usize] = &{shape};",
                    f"    let expected_data: &[f64] = &{data};",
                    *setup,
                    f"    let a = {callable_name}({arguments}){result_suffix};",
                    "    assert_eq!(a.shape(), expected_shape);",
                    f"    assert_eq!(a.size(), {case['size']});",
                    "    assert_eq!(a.as_slice().len(), expected_data.len());",
                    "    for (actual, expected) in a.as_slice().iter().zip(expected_data) {",
                    "        assert_eq!(actual.to_bits(), expected.to_bits());",
                    "    }",
                    "    }",
                ])
                case_count += 1
            for case in fixture.get("empty_cases", []):
                if name != "amax":
                    raise ValueError(f"Unsupported empty reduction: {name}")
                checks.extend([
                    "    {",
                    f"    let expected_shape: &[usize] = &{json.dumps(case['input_shape'])};",
                    "    let input = rumpy::Array::from_vec(vec![], expected_shape.to_vec()).unwrap();",
                    f"    let panic = std::panic::catch_unwind(|| {callable_name}(&input)).unwrap_err();",
                    "    let message = panic.downcast_ref::<String>().map(String::as_str)",
                    "        .or_else(|| panic.downcast_ref::<&str>().copied());",
                    '    assert_eq!(message, Some("amax requires at least one element"));',
                    "    assert_eq!(input.shape(), expected_shape);",
                    "    assert!(input.as_slice().is_empty());",
                    "    }",
                ])
                empty_reduction_count += 1
            for case in fixture.get("rejected", []):
                if name != "reshape" or case["numpy_error"] != "ValueError":
                    raise ValueError(f"Unsupported rejected oracle case: {name}")
                values = ", ".join(f"f64::from_bits({bits}u64)" for bits in case["input_bits"])
                checks.extend([
                    "    {",
                    f"    let input = rumpy::Array::from_vec(vec![{values}], vec!{json.dumps(case['input_shape'])}).unwrap();",
                    f"    let target_shape: &[usize] = &{json.dumps(case['shape'])};",
                    f"    assert_eq!({callable_name}(&input, target_shape), Err(rumpy::ShapeError::LengthMismatch {{ expected: {case['expected']}, actual: {case['actual']} }}));",
                    "    }",
                ])
                rejected_count += 1
        if case_count == 0:
            raise ValueError("No NumPy oracle cases found")
        checks.append("}")
        (work / "tests/author_numpy_oracle.rs").write_text(
            "\n".join(declarations + checks) + "\n"
        )
        run(["cargo", "test", "--test", "author_numpy_oracle"], work)
        print(f"PASS {case_count} saved NumPy cases, including array shapes, exact non-NaN bits, and reduction NaN classes")
        print(f"PASS {rejected_count} saved NumPy shape rejections with exact scaffold errors")
        print(f"PASS {empty_reduction_count} saved NumPy empty reductions with exact Rust panic text")
    print("PASS publication checks with any BASELINE ERROR above reported separately. Learner files remain untouched.")


if __name__ == "__main__":
    main()
