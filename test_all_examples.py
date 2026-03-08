import subprocess
import sys
from pathlib import Path

examples_dir = Path("examples")

for file in sorted(examples_dir.glob("*.py")):
    print(f"\n=== Running {file.name} ===\n")
    result = subprocess.run([sys.executable, str(file)])
    if result.returncode != 0:
        print(f"\nStopped: {file.name} exited with code {result.returncode}")
        sys.exit(result.returncode)
