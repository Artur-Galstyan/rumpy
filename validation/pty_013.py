"""Smoke-test Rustlings completion and next-exercise display in a disposable PTY."""
import os
from pathlib import Path
import pty
import select
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory(prefix="rumpy-013-pty-") as tmp:
    work = Path(tmp) / "repo"
    shutil.copytree(ROOT, work, ignore=shutil.ignore_patterns(".git", "target", "__pycache__", ".rustlings-state.txt"))
    master, slave = pty.openpty()
    process = subprocess.Popen(["rustlings", "run", "012_add"], cwd=work, stdin=slave, stdout=slave, stderr=slave)
    os.close(slave)
    data = b""
    entered = False
    deadline = time.monotonic() + 90
    try:
        while time.monotonic() < deadline:
            ready, _, _ = select.select([master], [], [], 0.2)
            if ready:
                try:
                    chunk = os.read(master, 65536)
                except OSError:
                    break
                if not chunk:
                    break
                data += chunk
                if b"Press ENTER" in data and not entered:
                    os.write(master, b"\n")
                    entered = True
            elif process.poll() is not None:
                break
        if process.poll() is None:
            process.kill()
            raise RuntimeError("Rustlings PTY timed out: " + data.decode(errors="replace"))
        output = data.decode(errors="replace")
        print(output)
        assert process.wait() == 0
        assert "solutions/04_elementwise/012_add.rs" in output
        assert "013_multiply" in output
        print("PASS Rustlings solution path and next exercise 013_multiply in disposable PTY")
    finally:
        if process.poll() is None:
            process.kill()
        process.wait()
        os.close(master)
