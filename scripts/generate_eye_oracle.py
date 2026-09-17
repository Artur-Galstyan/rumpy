"""Generate eye fixtures: uv run --with numpy==2.4.3 scripts/generate_eye_oracle.py."""
from pathlib import Path
import json
import numpy as np

ROOT = Path(__file__).resolve().parents[1]


def main():
    inputs = [(rows, cols) for rows in range(13) for cols in range(13)]
    inputs.extend([(0, 1000), (1000, 0), (1, 1000), (1000, 1), (7, 31), (31, 7)])
    cases = []
    for rows, cols in inputs:
        result = np.eye(rows, cols, k=0, dtype=np.float64, order="C")
        cases.append({
            "rows": rows, "cols": cols,
            "shape": list(result.shape), "size": int(result.size),
            "data_bits": [int(x) for x in result.ravel(order="C").view(np.uint64)],
        })
    fixture = {
        "numpy_version": np.__version__,
        "operation": "numpy.eye(rows, cols, k=0, dtype=float64, order='C')",
        "scope": "rows and cols in 0..=1000; main diagonal only",
        "encoding": "unsigned binary64 bits in row-major order",
        "cases": cases,
    }
    path = ROOT / "validation/eye-numpy.json"
    path.write_text(json.dumps(fixture, indent=2, allow_nan=False) + "\n")
    print(f"Wrote {len(cases)} actual NumPy {np.__version__} cases to {path}")


if __name__ == "__main__":
    main()
