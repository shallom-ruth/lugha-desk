"""Store payload boundary regressions; fixtures exercise real files and ZIPs."""
import copy
import hashlib
import json
from pathlib import Path
import struct
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET
import zipfile
import zlib

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'scripts'))
try:
    import stage_store
except ModuleNotFoundError as error:
    if error.name != 'stage_store':
        raise
    stage_store = None
try:
    import verify_store_package
except ModuleNotFoundError as error:
    if error.name != 'verify_store_package':
        raise
    verify_store_package = None

F = 'http://schemas.microsoft.com/appx/manifest/foundation/windows10'
R = 'http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities'
PUBLISHER = 'CN=417167AF-8330-486F-8BD2-5C96DFDA0EE8'
MICROSOFT = 'CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US'


def png(path, width, height):
    def chunk(kind, data):
        return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data))
    path.write_bytes(b'\x89PNG\r\n\x1a\n' +
                     chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 2, 0, 0, 0)) +
                     chunk(b'IDAT', zlib.compress((b'\0' + b'\x12\x3b\x34' * width) * height)) + chunk(b'IEND', b''))


class StorePayloadTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='lugha-store-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / 'Lugha test É with spaces'
        self.root.mkdir()
        self.app = self.root / 'built-app.exe'
        self.app.write_bytes(b'MZ-test-app')
        self.runtime = self.root / 'runtime.zip'
        with zipfile.ZipFile(self.runtime, 'w') as archive:
            archive.writestr('llama-server.exe', b'MZ-test-runtime')
            archive.writestr('llama.dll', b'MZ-test-dll')
            archive.writestr('llama-cli.exe', b'not selected')
        self.model = self.root / 'source.gguf'
        self.model.write_bytes(b'abc')
        self.notices = self.root / 'notices'
        (self.notices / 'model-runtime').mkdir(parents=True)
        (self.notices / 'LICENSE').write_text('Fixture app license')
        (self.notices / 'model-runtime/LICENSE-APACHE-2.0.txt').write_text('Fixture model notice')
        self.assets = self.root / 'assets'
        self.assets.mkdir()
        for name, side in [('StoreLogo.png', 50), ('Square44x44Logo.png', 44), ('Square150x150Logo.png', 150)]:
            png(self.assets / name, side, side)
        self.output = self.root / 'payload'
        self.config = {
            'identity': {'name': 'RuthShallom.LughaDesk', 'publisher': PUBLISHER,
                         'publisher_display_name': 'Ruth Shallom', 'version': '1.0.1.0', 'architecture': 'x64'},
            'app_id': 'LughaDesk',
            'windows': {'min_version': '10.0.19041.0', 'max_tested': '10.0.26100.0'},
            'model': {'file': 'TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf', 'bytes': 3,
                      'sha256': 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad'},
            'runtime': {'sha256': hashlib.sha256(self.runtime.read_bytes()).hexdigest(),
                        'files': ['llama-server.exe', 'llama.dll']},
            'notices_files': ['LICENSE', 'model-runtime/LICENSE-APACHE-2.0.txt'],
        }
        self.framework = {'name': 'Microsoft.VCLibs.140.00.UWPDesktop', 'publisher': MICROSOFT,
                          'version': '14.0.33728.0', 'architecture': 'x64', 'probe_passed': True}

    def build(self, config=None, framework=None):
        self.assertIsNotNone(stage_store, 'Store payload staging has not been implemented')
        return stage_store.stage_store(self.app, self.runtime, self.model, self.notices,
                                       self.assets, self.output, config or self.config,
                                       self.framework if framework is None else framework)

    def test_complete_payload_binds_reserved_identity_and_direct_app(self):
        (self.root / 'private.pfx').write_bytes(b'never include')
        result = self.build()
        manifest = ET.parse(self.output / 'AppxManifest.xml').getroot()
        self.assertEqual(manifest.find(f'{{{F}}}Identity').attrib,
                         {'Name': 'RuthShallom.LughaDesk', 'Publisher': PUBLISHER,
                          'Version': '1.0.1.0', 'ProcessorArchitecture': 'x64'})
        app = manifest.find(f'{{{F}}}Applications/{{{F}}}Application')
        self.assertEqual(app.attrib['Executable'], 'lugha-desk.exe')
        self.assertEqual(app.attrib['EntryPoint'], 'Windows.FullTrustApplication')
        self.assertEqual(app.attrib['Id'], 'LughaDesk')
        dependency = manifest.find(f'{{{F}}}Dependencies/{{{F}}}PackageDependency')
        self.assertEqual(dependency.attrib, {'Name': 'Microsoft.VCLibs.140.00.UWPDesktop',
                                           'MinVersion': '14.0.33728.0', 'Publisher': MICROSOFT})
        self.assertEqual(manifest.find(f'{{{F}}}Capabilities/{{{R}}}Capability').attrib['Name'], 'runFullTrust')
        self.assertEqual((self.output / 'lugha-desk.exe').read_bytes(), b'MZ-test-app')
        self.assertEqual((self.output / 'model/TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf').read_bytes(), b'abc')
        self.assertEqual((self.output / 'runtime/llama.dll').read_bytes(), b'MZ-test-dll')
        self.assertFalse((self.output / 'runtime/llama-cli.exe').exists())
        files = {p.relative_to(self.output).as_posix() for p in self.output.rglob('*') if p.is_file()}
        self.assertEqual(files, {'AppxManifest.xml', 'store-payload.json', 'lugha-desk.exe',
            'model/TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf', 'runtime/llama-server.exe', 'runtime/llama.dll',
            'notices/LICENSE', 'notices/model-runtime/LICENSE-APACHE-2.0.txt',
            'Assets/StoreLogo.png', 'Assets/Square44x44Logo.png', 'Assets/Square150x150Logo.png'})
        for name, record in result['files'].items():
            self.assertEqual(record['sha256'], hashlib.sha256((self.output / name).read_bytes()).hexdigest())
        self.assertEqual(json.loads((self.output / 'store-payload.json').read_text()), result)

    def test_wrong_model_size_is_rejected_before_payload_creation(self):
        self.model.write_bytes(b'abcd')
        with self.assertRaises(ValueError): self.build()
        self.assertFalse(self.output.exists())

    def test_checked_in_config_builds_the_reserved_store_product(self):
        config_file = Path(__file__).resolve().parents[1] / 'packaging/store/store-config.json'
        self.assertTrue(config_file.is_file(), 'The reserved Store product configuration is missing')
        config = json.loads(config_file.read_text(encoding='utf-8'))
        self.assertEqual(config['model']['bytes'], 1_082_015_808)
        self.assertEqual(config['model']['sha256'], 'b5056635be0c9bb832f0c964dc063bd4b4462e253f9f9d94eebf0aa0960b7798')
        config.update(model=self.config['model'], runtime=self.config['runtime'], notices_files=self.config['notices_files'])
        self.build(config=config)
        identity = ET.parse(self.output / 'AppxManifest.xml').getroot().find(f'{{{F}}}Identity')
        self.assertEqual(identity.attrib, {'Name': 'RuthShallom.LughaDesk', 'Publisher': PUBLISHER,
                                          'Version': '1.0.1.0', 'ProcessorArchitecture': 'x64'})

    def test_same_size_corrupt_model_is_rejected_before_payload_creation(self):
        self.model.write_bytes(b'abd')
        with self.assertRaises(ValueError): self.build()
        self.assertFalse(self.output.exists())

    def test_missing_model_is_rejected(self):
        self.model.unlink()
        with self.assertRaises(ValueError): self.build()
        self.assertFalse(self.output.exists())

    def test_unlisted_notice_or_private_file_cannot_enter_payload(self):
        for name in ['credentials.dpapi', 'unreviewed-note.txt']:
            extra = self.notices / name
            extra.write_bytes(b'private or unreviewed')
            with self.assertRaises(ValueError): self.build()
            self.assertFalse(self.output.exists())
            extra.unlink()

    def test_missing_license_cannot_be_silently_omitted(self):
        (self.notices / 'LICENSE').unlink()
        with self.assertRaises(ValueError): self.build()
        self.assertFalse(self.output.exists())

    def test_wrong_framework_identity_or_unproven_dependency_is_rejected(self):
        for field, value in [('name', 'Other.Framework'), ('publisher', 'CN=Someone else'),
                             ('architecture', 'x86'), ('probe_passed', False), ('version', '')]:
            framework = dict(self.framework, **{field: value})
            with self.subTest(field=field), self.assertRaises(ValueError): self.build(framework=framework)
            self.assertFalse(self.output.exists())

    def test_bad_logo_size_and_extra_asset_are_rejected(self):
        png(self.assets / 'Square44x44Logo.png', 43, 44)
        with self.assertRaises(ValueError): self.build()
        png(self.assets / 'Square44x44Logo.png', 44, 44)
        (self.assets / 'private.key').write_bytes(b'not allowed')
        with self.assertRaises(ValueError): self.build()
        self.assertFalse(self.output.exists())

    def test_existing_destination_is_not_overwritten(self):
        self.output.mkdir()
        sentinel = self.output / 'user-file.txt'
        sentinel.write_bytes(b'preserve me')
        with self.assertRaises(ValueError): self.build()
        self.assertEqual(sentinel.read_bytes(), b'preserve me')

    def test_model_output_path_cannot_escape_payload(self):
        config = copy.deepcopy(self.config)
        config['model']['file'] = '../outside.gguf'
        with self.assertRaises(ValueError): self.build(config=config)
        self.assertFalse(self.output.exists())
        self.assertFalse((self.root / 'outside.gguf').exists())

    def test_runtime_integrity_remains_required(self):
        self.runtime.write_bytes(b'corrupt archive')
        with self.assertRaises(ValueError): self.build()
        self.assertFalse(self.output.exists())

    def test_linked_notice_is_rejected(self):
        notice = self.notices / 'LICENSE'
        notice.unlink()
        try:
            notice.symlink_to(self.app)
        except OSError:
            self.skipTest('Windows account cannot create a test symlink')
        with self.assertRaises(ValueError): self.build()
        self.assertFalse(self.output.exists())

    def package_fixture(self, changes=None):
        package = self.root / 'test.msix'
        contents = {p.relative_to(self.output).as_posix(): p.read_bytes()
                    for p in self.output.rglob('*') if p.is_file()}
        contents.update({'AppxBlockMap.xml': b'<BlockMap/>', '[Content_Types].xml': b'<Types/>'})
        contents.update(changes or {})
        with zipfile.ZipFile(package, 'w') as archive:
            for name, data in contents.items(): archive.writestr(name, data)
        return package

    def verify(self, package):
        self.assertIsNotNone(verify_store_package, 'Actual-package integrity verification is not implemented')
        return verify_store_package.verify_package(package, self.output)

    def test_actual_package_payload_verifies(self):
        self.build()
        result = self.verify(self.package_fixture())
        self.assertEqual(result['files_verified'], 10)

    def test_same_size_package_model_tampering_is_rejected(self):
        self.build()
        package = self.package_fixture({'model/TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf': b'abd'})
        with self.assertRaises(ValueError): self.verify(package)

    def test_extra_private_file_in_package_is_rejected(self):
        self.build()
        with self.assertRaises(ValueError): self.verify(self.package_fixture({'private.pfx': b'secret'}))

    def test_package_cannot_change_publisher_even_with_updated_local_hash_report(self):
        report = self.build()
        manifest = self.output / 'AppxManifest.xml'
        manifest.write_bytes(manifest.read_bytes().replace(PUBLISHER.encode(), b'CN=Wrong publisher'))
        report['files']['AppxManifest.xml'] = {'bytes': manifest.stat().st_size,
                                              'sha256': hashlib.sha256(manifest.read_bytes()).hexdigest()}
        (self.output / 'store-payload.json').write_text(json.dumps(report))
        with self.assertRaises(ValueError): self.verify(self.package_fixture())


if __name__ == '__main__':
    unittest.main()
