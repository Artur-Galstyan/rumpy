"""Generate arange fixtures: uv run --with numpy==2.4.3 scripts/generate_arange_oracle.py."""
from pathlib import Path
import json
import numpy as np

ROOT = Path(__file__).resolve().parents[1]


def main():
    inputs = [(start, stop, step)
              for start in range(-8, 9)
              for stop in range(-8, 9)
              for step in (1, 2, 3, 7, 20)]
    inputs.extend([
        (2147483646, 2147483647, 2),
        (-2147483648, -2147483643, 2),
        (-2147483648, 2147483647, 2147483647),
        (2147483647, -2147483648, 1),
        (0, 97, 4),
    ])
    cases = []
    for start, stop, step in inputs:
        result = np.arange(start, stop, step, dtype=np.float64)
        cases.append({
            'start': start, 'stop': stop, 'step': step,
            'shape': list(result.shape), 'size': int(result.size),
            'data_bits': [int(x) for x in result.view(np.uint64)],
        })
    fixture = {
        'numpy_version': np.__version__,
        'operation': 'numpy.arange(start, stop, step, dtype=float64)',
        'scope': 'i32 endpoints, positive i32 step, at most 100000 output values',
        'encoding': 'unsigned binary64 bits',
        'cases': cases,
    }
    path = ROOT / 'validation/arange-numpy.json'
    path.write_text(json.dumps(fixture, indent=2, allow_nan=False) + '\n')
    print(f'Wrote {len(cases)} actual NumPy {np.__version__} cases to {path}')


if __name__ == '__main__':
    main()
