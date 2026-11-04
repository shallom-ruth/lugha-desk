"""Real native loader checks, including rejection of missing framework files."""
import argparse
from pathlib import Path
import subprocess
import tempfile
import shutil
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--probe', type=Path, required=True)
parser.add_argument('--framework', type=Path, required=True)
parser.add_argument('--runtime', type=Path, required=True)
parser.add_argument('--dumpbin', type=Path, required=True)
args = parser.parse_args()
with tempfile.TemporaryDirectory(prefix='lugha-empty-framework-') as empty:
    missing = subprocess.run([str(args.probe), empty, str(args.runtime)], capture_output=True, text=True)
    if missing.returncode == 0:
        raise RuntimeError('The loader probe accepted a missing framework')
    print('PASS: an empty supplied framework was rejected.')
with tempfile.TemporaryDirectory(prefix='lugha-partial-framework-') as partial:
    # Keep the old mandatory files; omit a real import of llama-server-impl.dll.
    for source in args.framework.glob('*.dll'):
        if source.name.lower() != 'vcruntime140_1.dll':
            shutil.copy2(source, Path(partial) / source.name)
    missing = subprocess.run([str(args.probe), partial, str(args.runtime)], capture_output=True, text=True)
    if missing.returncode == 0:
        raise RuntimeError('The loader probe accepted a partial framework missing vcruntime140_1.dll')
    print('PASS: partial framework missing the real vcruntime140_1 import was rejected.')
# Examine every distributed PE, including CPU-specific backends, not only the
# baseline DLLs loaded by the native probe. OS-provided UCRT/API sets are separate.
vc_imports = set()
for binary in sorted(args.runtime.iterdir()):
    if binary.suffix.lower() not in ('.dll', '.exe'):
        continue
    dependencies = subprocess.run([str(args.dumpbin), '/nologo', '/dependents', str(binary)],
                                  capture_output=True, text=True, check=True)
    vc_imports.update(name.lower() for name in re.findall(
        r'^\s*((?:vcruntime140|msvcp140|concrt140|vcomp140|vccorlib140|vcamp140)[\w.-]*\.dll)\s*$',
        dependencies.stdout, re.IGNORECASE | re.MULTILINE))
available = {file.name.lower() for file in args.framework.glob('*.dll')}
if not vc_imports or vc_imports - available:
    raise RuntimeError(f'Framework does not cover distributed PE VC imports: {sorted(vc_imports - available)}')
print('PASS: supplied framework covers VC imports of all distributed PE files: ' + ', '.join(sorted(vc_imports)))
result = subprocess.run([str(args.probe), str(args.framework), str(args.runtime)], capture_output=True, text=True)
print(result.stdout)
if result.returncode:
    raise RuntimeError(f'Pinned runtime / Desktop framework compatibility failed (exit {result.returncode}). {result.stderr}')
