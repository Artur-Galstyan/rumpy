"""Generate reproducible same-shape f64 subtraction cases from real NumPy."""
from pathlib import Path
import json
import math
import numpy as np


def bits(values):
    return np.asarray(values, dtype=np.float64).reshape(-1).view(np.uint64).tolist()


def main():
    cases = []

    def record(shape, left_bits, right_bits):
        # Integer views retain signaling-NaN input bits until the operation.
        left = np.asarray(left_bits, dtype=np.uint64).view(np.float64).reshape(shape)
        right = np.asarray(right_bits, dtype=np.uint64).view(np.float64).reshape(shape)
        before_left = left.reshape(-1).view(np.uint64).copy()
        before_right = right.reshape(-1).view(np.uint64).copy()
        with np.errstate(all="ignore"):
            result = np.subtract(left, right)
        assert list(result.shape) == list(shape)
        assert np.array_equal(left.reshape(-1).view(np.uint64), before_left)
        assert np.array_equal(right.reshape(-1).view(np.uint64), before_right)
        cases.append({
            "shape": list(shape), "left_bits": list(map(int, left_bits)),
            "right_bits": list(map(int, right_bits)),
            "expected": [None if np.isnan(value) else int(raw)
                         for value, raw in zip(result.reshape(-1), result.reshape(-1).view(np.uint64))],
        })

    rng = np.random.default_rng(14014)
    for shape in [(), (1,), (7,), (2, 3), (3, 2), (1, 2, 1, 3), (2, 2, 2), (0,), (2, 0, 3)]:
        count = math.prod(shape)
        for _ in range(12):
            left = rng.integers(-10000, 10001, count).astype(np.float64) / 8
            right = rng.integers(-10000, 10001, count).astype(np.float64) / 16
            record(shape, bits(left), bits(right))
    special = bits([0.0, -0.0, 1.0, -1.0, np.inf, -np.inf,
                    np.finfo(np.float64).max, -np.finfo(np.float64).max])
    special += [1, 0x8000000000000001, 0x7ff8000000001234, 0x7ff0000000000001]
    for left in special:
        record((len(special),), [left] * len(special), special.copy())
        record((len(special),), special.copy(), [left] * len(special))
    for _ in range(20):
        left = rng.bit_generator.random_raw(16) & np.uint64(0xffefffffffffffff)
        right = rng.bit_generator.random_raw(16) & np.uint64(0xffefffffffffffff)
        record((4, 4), left.tolist(), right.tolist())

    document = {
        "operation": "numpy.subtract", "numpy_version": np.__version__,
        "scope": "identical shapes only; null output means NaN class; other outputs are binary64 bits",
        "generator": "python3 validation/generate_subtract_numpy.py", "seed": 14014,
        "cases": cases,
        "notes": "Broadcastable shape mismatches are Rust-only rejection tests; huge empty axes are Rust-only tests.",
    }
    destination = Path(__file__).with_name("subtract-numpy.json")
    destination.write_text(json.dumps(document, indent=2, allow_nan=False) + "\n")
    print(f"NumPy {np.__version__}: {len(cases)} subtract cases, {sum(len(case['expected']) for case in cases)} results")


if __name__ == "__main__":
    main()
