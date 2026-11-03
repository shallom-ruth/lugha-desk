"""Real native loader checks, including rejection of missing framework files."""
import argparse
from pathlib import Path
import subprocess
import tempfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--probe', type=Path, required=True)
parser.add_argument('--framework', type=Path, required=True)
parser.add_argument('--runtime', type=Path, required=True)
args = parser.parse_args()
with tempfile.TemporaryDirectory(prefix='lugha-empty-framework-') as empty:
    missing = subprocess.run([str(args.probe), empty, str(args.runtime)], capture_output=True, text=True)
    if missing.returncode == 0:
        raise RuntimeError('The loader probe accepted a missing framework')
    print('PASS: missing supplied framework was rejected; no global runtime fallback accepted.')
result = subprocess.run([str(args.probe), str(args.framework), str(args.runtime)], capture_output=True, text=True)
print(result.stdout)
if result.returncode:
    raise RuntimeError(f'Pinned runtime / Desktop framework compatibility failed (exit {result.returncode}). {result.stderr}')
