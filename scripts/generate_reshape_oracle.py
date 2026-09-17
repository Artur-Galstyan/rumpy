"""Reproduce fixtures: uv run --with numpy==2.4.3 scripts/generate_reshape_oracle.py."""
from pathlib import Path
import itertools
import json
import math
import numpy as np

ROOT = Path(__file__).resolve().parents[1]


def main():
    shapes: list[tuple[int, ...]] = [()]
    for rank in range(1, 4):
        shapes.extend(itertools.product(range(4), repeat=rank))
    shapes.extend([(24,), (2, 3, 4), (1, 2, 3, 1), (0, 7), (4, 0, 2)])
    shapes = sorted(set(shapes))
    # Direct bit views avoid NaN conversion and retain negative zero.
    patterns = np.array([
        0, 0x8000000000000000, 0x3ff0000000000000, 0xc004000000000000,
        0x7ff0000000000000, 0xfff0000000000000,
        0x7ff8000000000042, 0xfff8000000001234, 1,
    ], dtype=np.uint64)
    cases = []
    for source_shape in shapes:
        size = math.prod(source_shape)
        bits = np.resize(patterns, size)
        source = bits.view(np.float64).reshape(source_shape)
        for shape in shapes:
            if math.prod(shape) != size:
                continue
            result = np.reshape(source, shape, order='C', copy=True)
            assert not np.shares_memory(source, result)
            cases.append({
                'input_shape': list(source_shape),
                'input_bits': [int(x) for x in bits],
                'shape': list(result.shape),
                'size': int(result.size),
                'data_bits': [int(x) for x in result.ravel(order='C').view(np.uint64)],
            })
    rejected = []
    for source_shape, shape in [((6,), (2, 2)), ((0,), ()), ((), (0,)), ((2,), (3, 1))]:
        source = np.zeros(source_shape, dtype=np.float64)
        try:
            np.reshape(source, shape, order='C', copy=True)
        except ValueError:
            rejected.append({
                'input_shape': list(source_shape),
                'input_bits': [int(x) for x in source.ravel().view(np.uint64)],
                'shape': list(shape), 'expected': math.prod(shape),
                'actual': int(source.size), 'numpy_error': 'ValueError',
            })
        else:
            raise AssertionError('NumPy accepted incompatible shape')
    fixture = {
        'numpy_version': np.__version__,
        'operation': "numpy.reshape(a, shape, order='C', copy=True)",
        'encoding': 'unsigned binary64 bits in row-major order',
        'scope': 'explicit small shapes; scaffold overflow rules have separate Rust tests',
        'cases': cases, 'rejected': rejected,
    }
    path = ROOT / 'validation/reshape-numpy.json'
    path.write_text(json.dumps(fixture, indent=2, allow_nan=False) + '\n')
    print(f'Wrote {len(cases)} successful and {len(rejected)} rejected NumPy {np.__version__} cases')


if __name__ == '__main__':
    main()
