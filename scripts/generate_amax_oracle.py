"""Generate with uv run --with numpy==2.4.3 scripts/generate_amax_oracle.py.

Use --check for reproducibility and --verify-rust for disposable Rust tests.
"""

import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import warnings

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
SEED = 10010


def build_fixture():
    cases = []

    def add(shape, raw_bits):
        # Integer views preserve signaling NaNs without Python float conversion.
        bits = np.asarray(raw_bits, dtype=np.uint64)
        assert not (np.any(bits == 0) and np.any(bits == 0x8000000000000000))
        source = bits.view(np.float64).reshape(shape)
        before = bits.copy()
        with warnings.catch_warnings(), np.errstate(invalid="ignore"):
            warnings.simplefilter("ignore", RuntimeWarning)
            expected = np.amax(source)
        is_nan = bool(np.isnan(expected))
        assert np.array_equal(bits, before)
        cases.append({
            "input_shape": list(shape),
            "input_bits": [int(x) for x in bits],
            "expected_bits": None if is_nan else int(expected.view(np.uint64)),
            "expected_nan": is_nan,
        })

    rng = np.random.default_rng(SEED)
    shapes = [(), (1,), (2,), (7,), (16,), (65,), (2, 3), (3, 2),
              (1, 7), (2, 2, 2), (1, 2, 1, 3), (1, 1, 6, 1, 1)]
    for shape in shapes:
        size = int(np.prod(shape, dtype=np.int64))
        for _ in range(8):
            # Finite values with varied exponents; no rounding in the reduction.
            values = np.ldexp(rng.uniform(-1.0, 1.0, size),
                              rng.integers(-1022, 1024, size))
            add(shape, values.view(np.uint64))

    pos_inf, neg_inf = 0x7ff0000000000000, 0xfff0000000000000
    neg_zero = 0x8000000000000000
    specials = [
        ((), [0]), ((), [neg_zero]), ((2,), [0, 0]),
        ((2,), [neg_zero, neg_zero]), ((), [pos_inf]), ((), [neg_inf]),
        ((2,), [pos_inf, neg_inf]), ((2,), [neg_inf, pos_inf]),
        ((2,), [neg_inf, neg_inf]), ((3,), [1, 2, 0]),
        ((2,), [0x8000000000000002, 0x8000000000000001]),
        ((2,), [0x7fefffffffffffff, 0x7fefffffffffffff]),
        ((3,), [0xc022000000000000, 0xc000000000000000, 0xc014000000000000]),
    ]
    for nan in [0x7ff8000000000042, 0xfff8000000001234,
                0x7ff0000000000001, 0xfff0000000000042]:
        specials.append(((), [nan]))
        for position in range(3):
            values = [pos_inf, neg_zero, neg_inf]
            values[position] = nan
            specials.append(((3,), values))
    for shape, bits in specials:
        add(shape, bits)

    empty_cases = []
    for shape in [(0,), (0, 0), (0, 7), (9, 0), (2, 0, 3, 1)]:
        source = np.empty(shape, dtype=np.float64)
        try:
            np.amax(source)
        except ValueError:
            empty_cases.append({"input_shape": list(shape)})
        else:
            raise AssertionError("NumPy accepted an empty maximum without initial")

    return {
        "numpy_version": np.__version__,
        "seed": SEED,
        "operation": "numpy.amax(a)",
        "encoding": "unsigned binary64 bits in row-major order; NaN result bits are null",
        "contract": {
            "rust_signature": "pub fn amax(a: &Array) -> f64",
            "selection": "select maximum without arithmetic; equal values retain FIRST stored value",
            "ranks": "all ranks; scalar copies its value; empty panics",
            "empty_panic": "amax requires at least one element",
            "special_values": "any NaN gives NaN class only; other results copy selected bits; input unchanged",
            "unsupported": ["axis", "out", "initial", "where", "keepdims", "dtype"],
            "numpy_difference": "NumPy does not specify signed-zero tie bits; mixed zero signs are excluded from fixtures",
            "rust_only": "first mixed-zero tie and usize::MAX empty shapes",
        },
        "cases": cases,
        "empty_cases": empty_cases,
    }


def verify_rust(fixture):
    exercise = ROOT / "exercises/03_reductions/010_amax.rs"
    solution = ROOT / "solutions/03_reductions/010_amax.rs"
    assert exercise.read_text().split("#[cfg(test)]", 1)[1] == solution.read_text().split("#[cfg(test)]", 1)[1]
    with tempfile.TemporaryDirectory(prefix="amax-check-") as tmp:
        tmp = Path(tmp)
        library = tmp / "librumpy.rlib"
        subprocess.run(["rustc", "--edition", "2024", "--crate-name", "rumpy",
                        "--crate-type", "rlib", str(ROOT / "src/lib.rs"),
                        "-o", str(library)], check=True, cwd=ROOT)
        common = ["rustc", "--edition", "2024", "--extern", f"rumpy={library}"]
        for name, path, flags in [("exercise", exercise, []),
                                  ("solution", solution, []),
                                  ("solution-release", solution, ["-O"])]:
            binary = tmp / name
            subprocess.run(common + flags + ["--test", str(path), "-o", str(binary)], check=True, cwd=ROOT)
            run = subprocess.run([str(binary)], capture_output=True, text=True, timeout=15)
            if name == "exercise":
                assert run.returncode != 0 and "not yet implemented: amax" in run.stdout, run.stdout + run.stderr
                print("Unfinished exercise compiles and fails at its intended todo!")
            else:
                assert run.returncode == 0, run.stdout + run.stderr
                print(name + ":\n" + run.stdout)
        blocks = []
        for i, case in enumerate(fixture["cases"]):
            bits = ",".join(f"f64::from_bits({x}u64)" for x in case["input_bits"])
            shape = ",".join(str(x) for x in case["input_shape"])
            check = "assert!(actual.is_nan());" if case["expected_nan"] else (
                f'assert_eq!(actual.to_bits(), {case["expected_bits"]}u64, "case {i}");')
            blocks.append("{" + f"let a = rumpy::Array::from_vec(vec![{bits}], vec![{shape}]).unwrap();"
                          "let before = a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>();"
                          "let shape_before = a.shape().to_vec();"
                          "let actual = reference::amax(&a);" + check +
                          "assert_eq!(a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(), before);"
                          "assert_eq!(a.shape(), shape_before);}")
        for case in fixture["empty_cases"]:
            shape = ",".join(str(x) for x in case["input_shape"])
            blocks.append("{" + f"let expected_shape: &[usize] = &[{shape}];"
                          "let a = rumpy::Array::from_vec(vec![], expected_shape.to_vec()).unwrap();"
                          "let panic = std::panic::catch_unwind(|| reference::amax(&a)).unwrap_err();"
                          "let message = panic.downcast_ref::<String>().map(String::as_str)"
                          ".or_else(|| panic.downcast_ref::<&str>().copied());"
                          'assert_eq!(message, Some("amax requires at least one element"));'
                          "assert_eq!(a.shape(), expected_shape); assert!(a.as_slice().is_empty());}")
        text = f'#[allow(dead_code)] #[path = {json.dumps(str(solution))}] mod reference;\n'
        for start in range(0, len(blocks), 32):
            text += f"fn batch_{start}() {{" + "\n".join(blocks[start:start + 32]) + "}\n"
        text += "fn main() {std::panic::set_hook(Box::new(|_| {}));" + "".join(
            f"batch_{start}();" for start in range(0, len(blocks), 32)) + "}\n"
        source = tmp / "oracle.rs"
        source.write_text(text)
        binary = tmp / "oracle"
        subprocess.run(common + [str(source), "-o", str(binary)], check=True, cwd=ROOT)
        subprocess.run([str(binary)], check=True, timeout=15)
        print(f"Rust reference matches {len(fixture['cases'])} scalar and {len(fixture['empty_cases'])} empty NumPy cases; test modules are identical")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--verify-rust", action="store_true")
    args = parser.parse_args()
    fixture = build_fixture()
    text = json.dumps(fixture, indent=2, allow_nan=False) + "\n"
    path = ROOT / "validation/amax-numpy.json"
    if args.check:
        assert path.read_text() == text, "Fixture differs; use its recorded NumPy version"
    else:
        path.write_text(text)
    print(f'{"Verified" if args.check else "Wrote"} {len(fixture["cases"])} scalar and {len(fixture["empty_cases"])} empty NumPy {np.__version__} cases; seed {SEED}')
    if args.verify_rust:
        verify_rust(fixture)


if __name__ == "__main__":
    main()
