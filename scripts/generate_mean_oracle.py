"""Reproduce with uv run --with numpy==2.4.3 scripts/generate_mean_oracle.py.

Use --check for a read-only fixture check, and --verify-rust for contract/oracle tests.
"""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import warnings

import numpy as np

ROOT = Path(__file__).resolve().parents[1]


def sequential(source):
    total = np.float64(0.0)
    for value in source.ravel(order="C"):
        total = np.add(total, value)
    return np.divide(total, np.float64(source.size))


def build_fixture():
    cases = []

    def add(shape, bits):
        # Never route signaling NaN inputs through Python float conversion.
        bits = np.asarray(bits, dtype=np.uint64)
        source = bits.view(np.float64).reshape(shape)
        before = bits.copy()
        with warnings.catch_warnings(), np.errstate(invalid="ignore", over="ignore", divide="ignore"):
            warnings.simplefilter("ignore", RuntimeWarning)
            expected = np.mean(source, dtype=np.float64)
            ordered = sequential(source)
        is_nan = bool(np.isnan(expected))
        if is_nan:
            assert np.isnan(ordered)
        else:
            assert expected.view(np.uint64) == ordered.view(np.uint64)
        assert np.array_equal(bits, before)
        cases.append({
            "input_shape": list(shape),
            "input_bits": [int(x) for x in bits],
            "expected_bits": None if is_nan else int(expected.view(np.uint64)),
            "expected_nan": is_nan,
        })

    # Small multiples of 1/8 keep all finite sums exact, whatever NumPy's order.
    shapes = [(), (0,), (1,), (2,), (7,), (16,), (0, 3), (3, 0),
              (2, 3), (3, 2), (1, 7), (2, 2, 2), (1, 2, 1, 3), (2, 0, 4)]
    for shape in shapes:
        size = int(np.prod(shape, dtype=np.int64))
        for offset in range(6):
            values = ((np.arange(size, dtype=np.int64) * 7 + offset * 5) % 33 - 16)
            values = values.astype(np.float64) / np.float64(8.0)
            add(shape, values.view(np.uint64))

    pos_inf, neg_inf = 0x7ff0000000000000, 0xfff0000000000000
    specials = [
        ((), [0x8000000000000000]),
        ((2,), [0x8000000000000000, 0x8000000000000000]),
        ((), [pos_inf]), ((), [neg_inf]),
        ((2,), [pos_inf, neg_inf]), ((2,), [neg_inf, pos_inf]),
        ((3,), [0x3ff0000000000000, pos_inf, 0xbff0000000000000]),
        ((2,), [neg_inf, 0x4000000000000000]),
    ]
    for nan in [0x7ff8000000000042, 0xfff8000000001234,
                0x7ff0000000000001, 0xfff0000000000042]:
        specials.extend([((), [nan]), ((3,), [0, nan, pos_inf])])
    for shape, bits in specials:
        add(shape, bits)

    # This is an ordered-Rust-contract check, not a general NumPy equivalence claim.
    assert sequential(np.array([1e16, 1.0, -1e16, 1.0], dtype=np.float64)) == 0.25
    assert len(cases) == 100
    return {
        "numpy_version": np.__version__,
        "operation": "numpy.mean(a, dtype=numpy.float64)",
        "encoding": "unsigned binary64 bits in row-major order; NaN result bits are null",
        "contract": {
            "rust_signature": "pub fn mean(a: &Array) -> f64",
            "order": "start at positive zero; add each flat value left-to-right in f64; divide once by element count as f64",
            "ranks": "all ranks; scalar participates in initial addition; empty returns NaN",
            "special_values": "IEEE infinities and overflow; NaN class only; input bits unchanged",
            "unsupported": ["axis", "keepdims", "dtype", "out", "where"],
            "numpy_difference": "NumPy may use pairwise summation and is not bit-identical for ill-conditioned inputs",
            "fixture_scope": "84 small exact dyadic cases and 16 signed-zero/nonfinite cases; both orders checked",
            "rust_only": "order-sensitive rounding, subnormals, finite overflow, usize::MAX empty dimensions",
        },
        "cases": cases,
    }


def verify_rust(fixture):
    exercise = ROOT / "exercises/03_reductions/009_mean.rs"
    solution = ROOT / "solutions/03_reductions/009_mean.rs"
    assert exercise.read_text().split("#[cfg(test)]", 1)[1] == solution.read_text().split("#[cfg(test)]", 1)[1]
    with tempfile.TemporaryDirectory(prefix="mean-check-") as tmp:
        tmp = Path(tmp)
        # Compile the actual library into disposable storage without changing Cargo metadata.
        library = tmp / "librumpy.rlib"
        subprocess.run(["rustc", "--edition", "2024", "--crate-name", "rumpy",
                        "--crate-type", "rlib", str(ROOT / "src/lib.rs"),
                        "-o", str(library)], check=True, cwd=ROOT)
        common = ["rustc", "--edition", "2024", "--extern", f"rumpy={library}"]
        for name, path in [("exercise", exercise), ("solution", solution)]:
            binary = tmp / name
            subprocess.run(common + ["--test", str(path), "-o", str(binary)], check=True, cwd=ROOT)
            run = subprocess.run([str(binary)], capture_output=True, text=True, timeout=15)
            if name == "exercise":
                assert run.returncode != 0 and "not yet implemented: mean" in run.stdout, run.stdout + run.stderr
                print("Unfinished exercise compiles and fails at its intended todo!")
            else:
                assert run.returncode == 0, run.stdout + run.stderr
                print(run.stdout)
        blocks = []
        for i, case in enumerate(fixture["cases"]):
            bits = ",".join(f"f64::from_bits({x}u64)" for x in case["input_bits"])
            shape = ",".join(str(x) for x in case["input_shape"])
            check = "assert!(actual.is_nan());" if case["expected_nan"] else (
                f'assert_eq!(actual.to_bits(), {case["expected_bits"]}u64, "case {i}");')
            blocks.append("{" + f"let a = rumpy::Array::from_vec(vec![{bits}], vec![{shape}]).unwrap();"
                          "let before = a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>();"
                          "let shape_before = a.shape().to_vec();"
                          "let strides_before = a.strides().to_vec();"
                          "let actual = reference::mean(&a);" + check +
                          "assert_eq!(a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(), before);"
                          "assert_eq!(a.shape(), shape_before); assert_eq!(a.strides(), strides_before);}")
        text = f'#[allow(dead_code)] #[path = {json.dumps(str(solution))}] mod reference;\n'
        for start in range(0, len(blocks), 32):
            text += f"fn batch_{start}() {{" + "\n".join(blocks[start:start + 32]) + "}\n"
        text += "fn main() {" + "".join(f"batch_{start}();" for start in range(0, len(blocks), 32)) + "}\n"
        source = tmp / "oracle.rs"
        source.write_text(text)
        binary = tmp / "oracle"
        subprocess.run(common + [str(source), "-o", str(binary)], check=True, cwd=ROOT)
        subprocess.run([str(binary)], check=True, timeout=15)
        print(f"Rust reference matches all {len(blocks)} NumPy cases; test modules are identical")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--verify-rust", action="store_true")
    args = parser.parse_args()
    fixture = build_fixture()
    text = json.dumps(fixture, indent=2, allow_nan=False) + "\n"
    path = ROOT / "validation/mean-numpy.json"
    if args.check:
        assert path.read_text() == text, "Fixture differs; use its recorded NumPy version"
    else:
        path.write_text(text)
    print(f'{"Verified" if args.check else "Wrote"} {len(fixture["cases"])} NumPy {np.__version__} cases')
    if args.verify_rust:
        verify_rust(fixture)


if __name__ == "__main__":
    main()
