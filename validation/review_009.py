"""Read-only review: uv run --with numpy==2.4.3 validation/review_009.py."""
import importlib.util
import itertools
import json
from pathlib import Path
import subprocess
import tempfile
from unittest.mock import patch

import numpy as np

ROOT = Path(__file__).resolve().parents[1]


def main():
    shapes = [list(s) for rank in range(6) for s in itertools.product(range(1, 4), repeat=rank)]
    cases = []
    blocks = []
    for shape in shapes:
        a = np.empty(shape, dtype=np.float64, order="C")
        strides = [s // a.itemsize for s in a.strides]
        cases.append({"shape": shape, "element_strides": strides})
        blocks.append(f'{{ let a = Array::from_vec(vec![0.0; {a.size}], vec!{shape}).unwrap(); let expected: &[usize] = &{strides}; assert_eq!(a.strides(), expected); assert_eq!(a.ndim(), {len(shape)}); assert_eq!(a.size(), {a.size}); assert_eq!(a.strides().len(), a.ndim()); }}')
    empty_shapes = [[0], [0, 3], [3, 0], [2, 0, 3]]
    source = 'use rumpy::{Array, ShapeError}; fn main() {\n' + '\n'.join(blocks)
    for shape in empty_shapes:
        source += f'println!("EMPTY {{:?}} {{:?}}", vec!{shape}, Array::from_vec(vec![], vec!{shape}).unwrap().strides());\n'
    source += '''
assert_eq!(Array::from_vec(vec![], vec![]), Err(ShapeError::LengthMismatch { expected: 1, actual: 0 }));
assert_eq!(Array::from_vec(vec![0.0], vec![2, 2]), Err(ShapeError::LengthMismatch { expected: 4, actual: 1 }));
assert_eq!(Array::from_vec(vec![], vec![usize::MAX, 2]), Err(ShapeError::SizeOverflow));
assert_eq!(Array::from_vec(vec![0.0], vec![0, usize::MAX, 2]), Err(ShapeError::LengthMismatch { expected: 0, actual: 1 }));
for shape in [vec![usize::MAX, 0], vec![0, usize::MAX], vec![usize::MAX, 2, 0]] { assert!(Array::from_vec(vec![], shape).is_ok()); }
std::panic::set_hook(Box::new(|_| {}));
let result = std::panic::catch_unwind(|| Array::from_vec(vec![], vec![0, usize::MAX, 2]));
match result { Ok(a) => println!("HUGE_EMPTY {:?}", a.unwrap().strides()), Err(_) => println!("HUGE_EMPTY PANIC") }
}
'''
    with tempfile.TemporaryDirectory(prefix="rumpy-independent-review-") as tmp:
        tmp = Path(tmp)
        (tmp / "review.rs").write_text(source)
        for mode, flags in [("debug", []), ("release", ["-O"])]:
            lib = tmp / f"lib{mode}.rlib"
            subprocess.run(["rustc", "--edition=2024", "--crate-name=rumpy", "--crate-type=rlib", *flags, str(ROOT / "src/lib.rs"), "-o", str(lib)], check=True)
            exe = tmp / mode
            subprocess.run(["rustc", "--edition=2024", *flags, str(tmp / "review.rs"), "--extern", f"rumpy={lib}", "-o", str(exe)], check=True)
            result = subprocess.run([str(exe)], check=True, capture_output=True, text=True)
            print(mode, result.stdout)
        spec = importlib.util.spec_from_file_location("checker", ROOT / "scripts/check_authoring.py")
        assert spec is not None and spec.loader is not None
        checker = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checker)
        expected = "tests::zero_axis_takes_precedence_over_overflow"
        for name in ["006_reshape", "006_reshape_sol"]:
            checker.run(["cargo", "test", "--bin", name, "--target-dir", str(tmp / "target")], ROOT, known_failure=expected)
        baseline = f"test {expected} ... FAILED\n12 passed; 1 failed\nattempt to multiply with overflow\nsrc/array.rs:53\n"
        variants = {
            "extra_failure": baseline + "test tests::other ... FAILED\n",
            "wrong_test": baseline.replace(expected, "tests::other"),
            "wrong_cause": baseline.replace("attempt to multiply with overflow", "assertion failed"),
            "compile_failure": baseline + "could not compile",
            "wrong_count": baseline.replace("12 passed", "11 passed"),
        }
        for label, output in variants.items():
            with patch.object(checker.subprocess, "run", return_value=subprocess.CompletedProcess([], 101, output, "")):
                try:
                    checker.run(["mock"], ROOT, known_failure=expected)
                except RuntimeError:
                    print("CHECKER REJECTED", label)
                else:
                    raise AssertionError(label)
    print(json.dumps({"numpy_version": np.__version__, "positive_shapes_passed_per_profile": len(cases), "ranks": [0, 1, 2, 3, 4, 5], "empty_numpy_element_strides": [{"shape": s, "strides": [v // 8 for v in np.empty(s, dtype=np.float64).strides]} for s in empty_shapes]}, indent=2))


if __name__ == "__main__":
    main()
