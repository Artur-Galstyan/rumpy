"""Reproduce: uv run --with numpy==2.4.3 scripts/generate_transpose_oracle.py.

Use --check to compare without writes. Use --verify-rust to compile both files
and test the reference against every NumPy case in disposable target storage.
"""
from pathlib import Path
import argparse
import json
import subprocess
import tempfile

import numpy as np

ROOT = Path(__file__).resolve().parents[1]


def build_fixture():
    # Integer views avoid Python float conversions, including signaling NaNs.
    patterns = np.array([
        0, 0x8000000000000000, 0x3ff0000000000000, 0xc004000000000000,
        0x7ff0000000000000, 0xfff0000000000000,
        0x7ff8000000000042, 0xfff8000000001234,
        0x7ff0000000000001, 0xfff0000000000042,
        1, 0x8000000000000001,
    ], dtype=np.uint64)
    shapes = sorted({(r, c) for r in range(9) for c in range(9)} |
                    {(0, 31), (31, 0), (1, 31), (31, 1), (7, 13), (13, 7)})
    cases = []
    for rows, columns in shapes:
        size = rows * columns
        # Unique finite values catch permutations hidden by repeated patterns.
        finite = np.arange(size, dtype=np.float64).view(np.uint64)
        for bits in (finite, np.resize(patterns, size)):
            source = bits.view(np.float64).reshape((rows, columns))
            result = np.transpose(source).copy(order='C')
            assert result.flags.c_contiguous
            assert not np.shares_memory(source, result)
            restored = np.transpose(result).copy(order='C')
            assert np.array_equal(restored.ravel().view(np.uint64), bits)
            cases.append({
                'input_shape': list(source.shape),
                'input_bits': [int(x) for x in bits],
                'shape': list(result.shape),
                'data_bits': [int(x) for x in result.ravel(order='C').view(np.uint64)],
                'size': int(result.size),
            })
    return {
        'numpy_version': np.__version__,
        'operation': "numpy.transpose(a).copy(order='C')",
        'encoding': 'unsigned binary64 bits in row-major order',
        'scope': 'two-axis small arrays; wrong ranks and usize::MAX empty axes are Rust-only tests',
        'cases': cases,
    }


def verify_rust(fixture):
    exercise = ROOT / 'exercises/02_shape/007_transpose.rs'
    solution = ROOT / 'solutions/02_shape/007_transpose.rs'
    assert exercise.read_text().split('#[cfg(test)]', 1)[1] == solution.read_text().split('#[cfg(test)]', 1)[1]
    subprocess.run(['cargo', 'build', '--lib'], cwd=ROOT, check=True)
    target = ROOT / 'target'
    with tempfile.TemporaryDirectory(prefix='transpose-check-', dir=target) as tmp:
        tmp = Path(tmp)
        common = ['rustc', '--edition', '2024', '--extern',
                  f'rumpy={target / "debug/librumpy.rlib"}',
                  '-L', f'dependency={target / "debug/deps"}']
        for name, path in [('exercise', exercise), ('solution', solution)]:
            binary = tmp / name
            subprocess.run(common + ['--test', str(path), '-o', str(binary)], check=True, cwd=ROOT)
            run = subprocess.run([str(binary)], capture_output=True, text=True, timeout=15)
            if name == 'exercise':
                assert run.returncode != 0 and 'not yet implemented: transpose' in run.stdout, run.stdout + run.stderr
                print('Unfinished exercise compiles and fails at its intended todo!')
            else:
                assert run.returncode == 0, run.stdout + run.stderr
                print(run.stdout)
        blocks = []
        for case in fixture['cases']:
            bits = ','.join(f'f64::from_bits({x}u64)' for x in case['input_bits'])
            shape = ','.join(str(x) for x in case['input_shape'])
            expected_shape = ','.join(str(x) for x in case['shape'])
            expected_bits = ','.join(f'{x}u64' for x in case['data_bits'])
            blocks.append('{' + f'let a = rumpy::Array::from_vec(vec![{bits}], vec![{shape}]).unwrap();'
                          'let b = reference::transpose(&a);'
                          f'let shape: &[usize] = &[{expected_shape}];'
                          f'let bits: &[u64] = &[{expected_bits}];'
                          'assert_eq!(b.shape(), shape);'
                          f'assert_eq!(b.size(), {case["size"]});'
                          'assert_eq!(b.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(), bits);' + '}')
        source = tmp / 'oracle.rs'
        text = f'#[allow(dead_code)] #[path = {json.dumps(str(solution))}] mod reference;\n'
        for start in range(0, len(blocks), 32):
            text += f'fn batch_{start}() {{' + '\n'.join(blocks[start:start + 32]) + '}\n'
        text += 'fn main() {' + ''.join(f'batch_{start}();' for start in range(0, len(blocks), 32)) + '}\n'
        source.write_text(text)
        binary = tmp / 'oracle'
        subprocess.run(common + [str(source), '-o', str(binary)], check=True, cwd=ROOT)
        subprocess.run([str(binary)], check=True, timeout=15)
        print(f'Rust reference matches all {len(blocks)} NumPy cases; test modules are identical')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--verify-rust', action='store_true')
    args = parser.parse_args()
    fixture = build_fixture()
    text = json.dumps(fixture, indent=2, allow_nan=False) + '\n'
    path = ROOT / 'validation/transpose-numpy.json'
    if args.check:
        assert path.read_text() == text, 'Fixture differs; use its recorded NumPy version'
    else:
        path.write_text(text)
    print(f'{"Verified" if args.check else "Wrote"} {len(fixture["cases"])} NumPy {np.__version__} cases')
    if args.verify_rust:
        verify_rust(fixture)


if __name__ == '__main__':
    main()
