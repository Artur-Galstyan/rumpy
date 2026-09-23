"""Generate real NumPy same-shape add fixtures; run with Python and NumPy installed."""
from pathlib import Path
import json
import math
import numpy as np


def bits(values):
    return np.asarray(values, dtype=np.float64).reshape(-1).view(np.uint64).tolist()


def main():
    cases = []

    def add_case(shape, left_bits, right_bits):
        # Integer views preserve signaling NaNs before NumPy arithmetic.
        left = np.asarray(left_bits, dtype=np.uint64).view(np.float64).reshape(shape)
        right = np.asarray(right_bits, dtype=np.uint64).view(np.float64).reshape(shape)
        with np.errstate(all="ignore"):
            result = np.add(left, right)
        assert list(result.shape) == list(shape)
        cases.append({
            "shape": list(shape), "left_bits": left_bits, "right_bits": right_bits,
            "expected": [None if np.isnan(x) else int(raw)
                         for x, raw in zip(result.reshape(-1), result.reshape(-1).view(np.uint64))],
        })

    rng = np.random.default_rng(12012)
    for shape in [(), (1,), (7,), (2, 3), (3, 2), (1, 2, 1, 3), (2, 2, 2), (0,), (2, 0, 3)]:
        count = math.prod(shape)
        for _ in range(12):
            left = rng.integers(-10000, 10001, count).astype(np.float64) / 8
            right = rng.integers(-10000, 10001, count).astype(np.float64) / 16
            add_case(shape, bits(left), bits(right))
    # All special-value pairs test operand order, signed zeros, infinities and NaNs.
    special = bits([0.0, -0.0, 1.0, -1.0, np.inf, -np.inf, np.finfo(np.float64).max,
                    -np.finfo(np.float64).max, 9007199254740992.0])
    special += [1, 0x8000000000000001, 0x7ff8000000001234, 0x7ff0000000000001]
    for left in special:
        add_case((len(special),), [left] * len(special), special.copy())
    # Arbitrary finite binary64 pairs exercise exponent gaps and rounding.
    for _ in range(20):
        left = rng.bit_generator.random_raw(16) & np.uint64(0xffefffffffffffff)
        right = rng.bit_generator.random_raw(16) & np.uint64(0xffefffffffffffff)
        add_case((4, 4), left.tolist(), right.tolist())

    document = {
        "operation": "numpy.add", "numpy_version": np.__version__,
        "scope": "identical shapes only; null expected entries mean NaN class; other entries are binary64 bits",
        "generator": "python3 validation/generate_add_numpy.py", "seed": 12012,
        "cases": cases,
        "notes": "Rust-only shape rejections include broadcastable pairs; these are not claimed as NumPy rejections. Huge empty dimensions are Rust-only contract tests.",
    }
    destination = Path(__file__).with_name("add-numpy.json")
    destination.write_text(json.dumps(document, indent=2, allow_nan=False) + "\n")
    print(f"NumPy {np.__version__}: {len(cases)} add cases, {sum(len(c['expected']) for c in cases)} result values -> {destination}")


if __name__ == "__main__":
    main()
