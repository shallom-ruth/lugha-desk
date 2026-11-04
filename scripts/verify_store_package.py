"""Check the actual unsigned MSIX bytes, identity and allowed payload."""
import argparse
import hashlib
import json
from pathlib import Path
import xml.etree.ElementTree as ET
import zipfile


def verify_package(package, payload):
    report_bytes = (Path(payload) / 'store-payload.json').read_bytes()
    report = json.loads(report_bytes)
    expected = set(report['files']) | {'store-payload.json'}
    metadata = {'AppxBlockMap.xml', '[Content_Types].xml', 'AppxMetadata/CodeIntegrity.cat'}
    with zipfile.ZipFile(package) as archive:
        names = [i.filename for i in archive.infolist() if not i.is_dir()]
        if len(names) != len(set(name.lower() for name in names)):
            raise ValueError('Duplicate or case-colliding MSIX files')
        if not expected.issubset(names) or set(names) - expected - metadata:
            raise ValueError('Missing or unexpected package files')
        if archive.read('store-payload.json') != report_bytes:
            raise ValueError('Package report does not match validated staging')
        for name, record in report['files'].items():
            if archive.getinfo(name).file_size != record['bytes']:
                raise ValueError(f'Package size mismatch: {name}')
            with archive.open(name) as stream:
                digest = hashlib.file_digest(stream, 'sha256').hexdigest()
            if digest != record['sha256']:
                raise ValueError(f'Package checksum mismatch: {name}')
        identity = ET.fromstring(archive.read('AppxManifest.xml')).find('{*}Identity')
        expected_identity = {'Name': 'RuthShallom.LughaDesk',
            'Publisher': 'CN=417167AF-8330-486F-8BD2-5C96DFDA0EE8',
            'Version': '1.0.1.0', 'ProcessorArchitecture': 'x64'}
        if identity is None or identity.attrib != expected_identity:
            raise ValueError('Package identity does not match the reserved Store product')
    return {'files_verified': len(report['files']),
            'payload_bytes': sum(record['bytes'] for record in report['files'].values()),
            'package_bytes': Path(package).stat().st_size}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--package', type=Path, required=True)
    parser.add_argument('--payload', type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(verify_package(args.package, args.payload), indent=2))
