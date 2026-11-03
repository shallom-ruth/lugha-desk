"""Build the complete, read-only Store payload. No signing or publication."""
import argparse
import json
from pathlib import Path, PurePosixPath
import re
import shutil
import struct
import xml.etree.ElementTree as ET

from stage_bundle import sha256, stage_bundle, verified_runtime

FOUNDATION = 'http://schemas.microsoft.com/appx/manifest/foundation/windows10'
UAP = 'http://schemas.microsoft.com/appx/manifest/uap/windows10'
RESCAP = 'http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities'
MICROSOFT = 'CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US'
LOGOS = {'StoreLogo.png': (50, 50), 'Square44x44Logo.png': (44, 44), 'Square150x150Logo.png': (150, 150)}
for prefix, uri in [('', FOUNDATION), ('uap', UAP), ('rescap', RESCAP)]:
    ET.register_namespace(prefix, uri)


def plain_path(path):
    path = Path(path)
    for item in [path, *path.parents]:
        if item.is_symlink() or (hasattr(item, 'is_junction') and item.is_junction()):
            raise ValueError(f'Linked paths are not accepted: {item.name}')
    return path


def safe_relative(name):
    if not isinstance(name, str) or not name or not name.isascii():
        raise ValueError('Package paths must be nonempty ASCII names')
    path = PurePosixPath(name)
    if (path.is_absolute() or path.as_posix() != name or
            any(part in ('..', '.') for part in path.parts) or '\\' in name or ':' in name):
        raise ValueError(f'Unsafe package path: {name}')
    return path


def four_part_version(value):
    if not isinstance(value, str) or not re.fullmatch(r'\d+\.\d+\.\d+\.\d+', value):
        raise ValueError('A four-part version is required')
    parts = tuple(map(int, value.split('.')))
    if parts[0] == 0 or any(part > 65535 for part in parts):
        raise ValueError('Invalid package version')
    return parts


def approved_tree(root, expected):
    root = plain_path(root)
    if not root.is_dir() or not expected or len(set(name.lower() for name in expected)) != len(expected):
        raise ValueError('A complete, unique file allowlist is required')
    for name in expected:
        safe_relative(name)
    actual = set()
    for item in root.rglob('*'):
        plain_path(item)
        if item.is_file():
            actual.add(item.relative_to(root).as_posix())
    if actual != set(expected):
        raise ValueError('Missing or unapproved payload input files')


def write_manifest(destination, config, framework):
    def element(parent, name, text=None, namespace=FOUNDATION, **attributes):
        child = ET.SubElement(parent, f'{{{namespace}}}{name}', attributes)
        child.text = text
        return child
    identity = config['identity']
    package = ET.Element(f'{{{FOUNDATION}}}Package', {'IgnorableNamespaces': 'uap rescap'})
    element(package, 'Identity', Name=identity['name'], Publisher=identity['publisher'],
            Version=identity['version'], ProcessorArchitecture=identity['architecture'])
    properties = element(package, 'Properties')
    element(properties, 'DisplayName', 'Lugha Desk')
    element(properties, 'PublisherDisplayName', identity['publisher_display_name'])
    element(properties, 'Description', 'Experimental offline African-language translation with a local AI model.')
    element(properties, 'Logo', r'Assets\StoreLogo.png')
    dependencies = element(package, 'Dependencies')
    element(dependencies, 'TargetDeviceFamily', Name='Windows.Desktop',
            MinVersion=config['windows']['min_version'], MaxVersionTested=config['windows']['max_tested'])
    element(dependencies, 'PackageDependency', Name=framework['name'],
            MinVersion=framework['version'], Publisher=framework['publisher'])
    element(element(package, 'Resources'), 'Resource', Language='en-us')
    app = element(element(package, 'Applications'), 'Application', Id=config['app_id'],
                  Executable='lugha-desk.exe', EntryPoint='Windows.FullTrustApplication')
    element(app, 'VisualElements', namespace=UAP, DisplayName='Lugha Desk',
            Description='Experimental offline text translation', BackgroundColor='#123B34',
            Square150x150Logo=r'Assets\Square150x150Logo.png', Square44x44Logo=r'Assets\Square44x44Logo.png')
    element(element(package, 'Capabilities'), 'Capability', namespace=RESCAP, Name='runFullTrust')
    ET.indent(package)
    ET.ElementTree(package).write(destination, encoding='utf-8', xml_declaration=True)


def stage_store(app, runtime_archive, model, notices, assets, output, config, framework):
    app, runtime_archive, model, notices, assets, output = map(
        plain_path, (app, runtime_archive, model, notices, assets, output))
    if output.exists():
        raise ValueError('Store staging requires a fresh destination')
    identity = config['identity']
    if (not re.fullmatch(r'[A-Za-z0-9.-]{3,50}', identity['name']) or
            not identity['publisher'].startswith('CN=') or not identity['publisher_display_name'] or
            identity['architecture'] != 'x64' or not re.fullmatch(r'[A-Za-z][A-Za-z0-9]*', config['app_id'])):
        raise ValueError('Invalid Store identity')
    if four_part_version(identity['version'])[3] != 0:
        raise ValueError('The Store package revision must be zero')
    if four_part_version(config['windows']['min_version']) > four_part_version(config['windows']['max_tested']):
        raise ValueError('Invalid Windows version range')
    if (framework.get('name') != 'Microsoft.VCLibs.140.00.UWPDesktop' or
            framework.get('publisher') != MICROSOFT or framework.get('architecture') != 'x64' or
            framework.get('probe_passed') is not True):
        raise ValueError('A verified, compatible Microsoft Desktop framework is required')
    four_part_version(framework.get('version'))
    model_info = config['model']
    model_name = safe_relative(model_info['file'])
    if len(model_name.parts) != 1 or model_name.suffix != '.gguf':
        raise ValueError('The model filename must be a GGUF basename')
    if (type(model_info['bytes']) is not int or model_info['bytes'] < 1 or
            not re.fullmatch(r'[0-9a-fA-F]{64}', model_info['sha256'])):
        raise ValueError('Invalid model integrity lock')
    if not model.is_file() or model.stat().st_size != model_info['bytes']:
        raise ValueError('The complete pinned model is required; model size mismatch')
    if sha256(model) != model_info['sha256'].lower():
        raise ValueError('Model checksum mismatch')
    if not app.is_file():
        raise ValueError('The freshly built app is required')
    approved_tree(notices, config['notices_files'])
    approved_tree(assets, list(LOGOS))
    for name, dimensions in LOGOS.items():
        data = (assets / name).read_bytes()
        if (len(data) < 33 or data[:8] != b'\x89PNG\r\n\x1a\n' or data[12:16] != b'IHDR' or
                struct.unpack('>II', data[16:24]) != dimensions):
            raise ValueError(f'Invalid Store PNG dimensions: {name}')
    # Verify before creating output; stage_bundle enforces the same archive allowlist on copy.
    runtime = config['runtime']
    verified_runtime(runtime_archive, runtime['sha256'], runtime['files'])
    stage_bundle(app, runtime_archive, runtime['sha256'], runtime['files'], notices, output)
    (output / 'bundle-manifest.json').unlink()
    (output / 'model').mkdir()
    shutil.copyfile(model, output / 'model' / model_name.name)
    shutil.copytree(assets, output / 'Assets')
    write_manifest(output / 'AppxManifest.xml', config, framework)
    files = {file.relative_to(output).as_posix(): {'bytes': file.stat().st_size, 'sha256': sha256(file)}
             for file in sorted(output.rglob('*')) if file.is_file()}
    report = {'schema': 1, 'identity': identity,
              'framework': {key: framework[key] for key in ('name', 'publisher', 'version', 'architecture', 'probe_passed')},
              'files': files}
    (output / 'store-payload.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('app', 'runtime', 'model', 'notices', 'assets', 'output', 'config', 'framework'):
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    config = json.loads(args.config.read_text(encoding='utf-8-sig'))
    config['runtime'] = json.loads(Path(__file__).with_name('runtime-lock.json').read_text(encoding='utf-8-sig'))
    framework = json.loads(args.framework.read_text(encoding='utf-8-sig'))
    report = stage_store(args.app, args.runtime, args.model, args.notices, args.assets, args.output, config, framework)
    print(f'Staged {len(report["files"])} Store payload files, including the verified offline model.')


if __name__ == '__main__':
    main()
