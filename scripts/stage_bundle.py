"""Stage freshly built Lugha code and an integrity-pinned upstream runtime.

Developer tooling only. Python is never required by Lugha Desk users.
"""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import zipfile


def sha256(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def verified_runtime(archive, expected_sha, allowed_files):
    if sha256(archive) != expected_sha.lower():
        raise ValueError('Runtime archive checksum mismatch')
    if not allowed_files or len(set(allowed_files)) != len(allowed_files):
        raise ValueError('Invalid runtime file allowlist')
    result = {}
    with zipfile.ZipFile(archive) as bundle:
        seen = set()
        for info in bundle.infolist():
            name = info.filename
            path = PurePosixPath(name)
            if (path.is_absolute() or '..' in path.parts or '\\' in name or
                    ':' in name or (info.external_attr >> 16) & 0o170000 == 0o120000):
                raise ValueError(f'Unsafe runtime path: {name}')
            if info.is_dir():
                continue
            if name in seen:
                raise ValueError(f'Duplicate runtime path: {name}')
            seen.add(name)
            if name in allowed_files:
                if len(path.parts) != 1 or info.file_size > 150_000_000:
                    raise ValueError(f'Invalid runtime binary: {name}')
                result[name] = bundle.read(info)
    if set(result) != set(allowed_files):
        raise ValueError('Required runtime files are missing')
    return result


def stage_bundle(app, archive, expected_sha, allowed_files, notices, destination):
    app, notices, destination = Path(app), Path(notices), Path(destination)
    if destination.exists() and any(destination.iterdir()):
        raise ValueError('Staging destination must be empty')
    if not app.is_file() or not notices.is_dir() or not any(notices.rglob('*')):
        raise ValueError('Fresh app and license notices are required')
    runtime = verified_runtime(archive, expected_sha, allowed_files)
    destination.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(app, destination / 'lugha-desk.exe')
    (destination / 'runtime').mkdir()
    for name, data in runtime.items():
        (destination / 'runtime' / name).write_bytes(data)
    shutil.copytree(notices, destination / 'notices')
    files = {}
    for file in sorted(destination.rglob('*')):
        if file.is_file():
            name = file.relative_to(destination).as_posix()
            if file.is_symlink() or name.startswith('model/'):
                raise ValueError('Linked files and model weights cannot be embedded')
            files[name] = {'sha256': sha256(file), 'bytes': file.stat().st_size}
    manifest = {'files': files}
    (destination / 'bundle-manifest.json').write_text(
        json.dumps(manifest, indent=2) + '\n', encoding='utf-8')
    return manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--app', required=True, type=Path)
    parser.add_argument('--runtime', required=True, type=Path)
    parser.add_argument('--lock', required=True, type=Path)
    parser.add_argument('--notices', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args()
    lock = json.loads(args.lock.read_text(encoding='utf-8'))
    result = stage_bundle(args.app, args.runtime, lock['sha256'], lock['files'],
                          args.notices, args.output)
    print(f"Staged {len(result['files'])} verified files; no model embedded")


if __name__ == '__main__':
    main()
