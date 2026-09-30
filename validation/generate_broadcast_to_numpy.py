"""Save real NumPy broadcast_to copies as binary64 bits for an owned Rust subset."""
from pathlib import Path
import json
import math
import numpy as np


def main():
    rng = np.random.default_rng(15015)
    cases = []

    def record(source_shape, target_shape, raw):
        source = np.asarray(raw, dtype=np.uint64).view(np.float64).reshape(source_shape)
        before = source.reshape(-1).view(np.uint64).copy()
        output = np.broadcast_to(source, target_shape).copy(order="C")
        assert np.array_equal(source.reshape(-1).view(np.uint64), before)
        cases.append({
            "input_shape": list(source_shape), "shape": list(target_shape),
            "input_bits": list(map(int, before)),
            "data_bits": list(map(int, output.reshape(-1).view(np.uint64))),
            "size": int(output.size),
        })

    pairs = [
        ((), ()), ((), (2, 3)), ((), (0, 3)),
        ((1,), (0,)), ((1,), (2, 3)), ((3,), (2, 3)),
        ((2, 1), (2, 3)), ((1, 3), (4, 3)), ((2, 3), (2, 3)),
        ((1, 2, 1), (3, 2, 4)), ((0, 1), (0, 5)), ((0, 2), (3, 0, 2)),
        ((1, 0, 1), (4, 0, 3)),
    ]
    special = [0, 0x8000000000000000, 0x7ff0000000000000,
               0xfff0000000000000, 0x7ff8000000001234,
               0x7ff0000000000001, 1, 0x8000000000000001]
    for source_shape, target_shape in pairs:
        count = math.prod(source_shape)
        for _ in range(10):
            raw = rng.choice(np.asarray(special, dtype=np.uint64), count).tolist()
            record(source_shape, target_shape, raw)
    document = {
        "operation": "numpy.broadcast_to", "numpy_version": np.__version__,
        "scope": "compatible trailing axes only; output copied to owned C-order f64 storage; bit-preserving view",
        "generator": "python3 validation/generate_broadcast_to_numpy.py", "seed": 15015,
        "cases": cases,
        "notes": "Incompatible ranks and huge empty axes are separate Rust-only tests.",
    }
    destination = Path(__file__).with_name("broadcast_to-numpy.json")
    destination.write_text(json.dumps(document, indent=2, allow_nan=False) + "\n")
    print(f"NumPy {np.__version__}: {len(cases)} cases, {sum(len(c['data_bits']) for c in cases)} output values")


if __name__ == "__main__":
    main()
