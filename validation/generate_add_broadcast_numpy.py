"""Generate independent NumPy broadcast-add output bits for the new exercise."""
from pathlib import Path
import json
import numpy as np


def main():
    rng = np.random.default_rng(16016)
    shapes = [
        ((), ()), ((), (3,)), ((3,), ()), ((3,), (2, 1)),
        ((2, 1), (3,)), ((1, 2, 1), (3,)), ((0, 1), (1,)),
        ((0,), (1,)), ((2, 3), (2, 3)), ((1, 3), (2, 1)),
        ((2, 1, 3), (1, 4, 1)), ((0, 2), (1, 2)),
    ]
    special = np.array([0, 0x8000000000000000, 0x7ff0000000000000,
                        0xfff0000000000000, 0x7ff8000000001234,
                        1, 0x8000000000000001, 0x3ff0000000000000], dtype=np.uint64)
    cases = []
    for left_shape, right_shape in shapes:
        for _ in range(8):
            left = rng.choice(special, int(np.prod(left_shape))).view(np.float64).reshape(left_shape)
            right = rng.choice(special, int(np.prod(right_shape))).view(np.float64).reshape(right_shape)
            left_bits = list(map(int, left.ravel().view(np.uint64)))
            right_bits = list(map(int, right.ravel().view(np.uint64)))
            with np.errstate(invalid="ignore"):
                output = np.add(left, right)
            values = output.ravel().view(np.uint64)
            cases.append({"left_shape": list(left_shape), "right_shape": list(right_shape),
                          "shape": list(output.shape), "left_bits": left_bits,
                          "right_bits": right_bits,
                          "expected": [None if np.isnan(value) else int(bits)
                                       for value, bits in zip(output.ravel(), values)]})
    result = {"operation": "numpy.add", "numpy_version": np.__version__,
              "scope": "right-aligned broadcasting into owned row-major f64 output; NaNs by class",
              "generator": "python3 validation/generate_add_broadcast_numpy.py", "seed": 16016,
              "cases": cases, "notes": "Huge empty shapes and incompatible shapes have Rust-only tests."}
    Path(__file__).with_name("add_broadcast-numpy.json").write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")
    print(f"NumPy {np.__version__}: {len(cases)} broadcast addition cases")


if __name__ == "__main__":
    main()
