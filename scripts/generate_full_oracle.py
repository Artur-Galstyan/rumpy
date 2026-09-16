"""Generate actual NumPy full results. Run with: uv run --with numpy==2.4.3 scripts/generate_full_oracle.py

Bit-pattern JSON keeps negative zero, NaN payloads, and infinity valid in strict JSON.
No expected result is inferred from the Rust implementation.
"""
from pathlib import Path
import json
import numpy as np

ROOT = Path(__file__).resolve().parents[1]
SHAPES = [[], [0], [4], [2, 3], [3, 2], [1, 3, 1], [2, 0, 4], [2, 3, 0], [2, 2, 2, 2]]
FILL_BITS = [
    0x0000000000000000,  # positive zero
    0x8000000000000000,  # negative zero
    0x3ff0000000000000,  # one
    0x400c000000000000,  # 3.5
    0xc002000000000000,  # -2.25
    0x7ff0000000000000,  # positive infinity
    0xfff0000000000000,  # negative infinity
    0x7ff8000000000042,  # quiet NaN with a payload
    0xfff8000000000011,  # negative quiet NaN with a payload
    0x7fefffffffffffff,  # largest finite positive value
    0xffefffffffffffff,  # largest finite negative magnitude
    0x0010000000000000,  # smallest positive normal
    0x0000000000000001,  # smallest positive subnormal
]


def main():
    cases = []
    for shape in SHAPES:
        for bits in FILL_BITS:
            fill = np.array(bits, dtype=np.uint64).view(np.float64)[()]
            result = np.full(tuple(shape), fill, dtype=np.float64, order='C')
            cases.append({
                'shape': list(result.shape),
                'fill_bits': bits,
                'size': int(result.size),
                'data_bits': [int(x) for x in result.ravel(order='C').view(np.uint64)],
            })
    fixture = {
        'numpy_version': np.__version__,
        'operation': 'numpy.full(shape, fill_value, dtype=float64, order=C)',
        'encoding': 'unsigned binary64 bits; signaling NaNs are covered by Rust contract tests separately',
        'cases': cases,
    }
    path = ROOT / 'validation/full-numpy.json'
    path.write_text(json.dumps(fixture, indent=2, allow_nan=False) + '\n')
    print(f'Wrote {len(cases)} actual NumPy {np.__version__} cases to {path}')


if __name__ == '__main__':
    main()
