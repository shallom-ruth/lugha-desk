"""Packaging boundaries: integrity, allowed files and fresh build provenance."""
import hashlib
import importlib.util
from pathlib import Path
import tempfile
import unittest
import zipfile

SCRIPT = Path(__file__).resolve().parents[1] / 'scripts' / 'stage_bundle.py'


class StageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        spec = importlib.util.spec_from_file_location('stage_bundle', SCRIPT)
        cls.stage = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.stage)

    def test_corrupt_runtime_is_rejected_before_extraction(self):
        with tempfile.TemporaryDirectory() as folder:
            archive = Path(folder) / 'runtime.zip'
            archive.write_bytes(b'corrupt')
            with self.assertRaisesRegex(ValueError, 'checksum'):
                self.stage.verified_runtime(archive, '0' * 64, ['llama-server.exe'])

    def test_zip_path_traversal_is_rejected(self):
        for name in ['../llama-server.exe', 'C:/llama-server.exe', '/llama-server.exe']:
            with self.subTest(name=name), tempfile.TemporaryDirectory() as folder:
                archive = Path(folder) / 'runtime.zip'
                with zipfile.ZipFile(archive, 'w') as out:
                    out.writestr('llama-server.exe', b'fixture')
                    out.writestr(name, b'foreign')
                digest = hashlib.sha256(archive.read_bytes()).hexdigest()
                with self.assertRaises(ValueError):
                    self.stage.verified_runtime(archive, digest, ['llama-server.exe'])

    def test_unselected_upstream_tools_are_not_staged(self):
        with tempfile.TemporaryDirectory() as folder:
            archive = Path(folder) / 'runtime.zip'
            with zipfile.ZipFile(archive, 'w') as out:
                out.writestr('llama-server.exe', b'fixture')
                out.writestr('llama-bench.exe', b'upstream-tool')
            digest = hashlib.sha256(archive.read_bytes()).hexdigest()
            self.assertEqual(self.stage.verified_runtime(archive, digest,
                ['llama-server.exe']), {'llama-server.exe': b'fixture'})

    def test_staging_uses_fresh_app_and_records_hash_without_model(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            archive = root / 'runtime.zip'
            with zipfile.ZipFile(archive, 'w') as out:
                out.writestr('llama-server.exe', b'runtime-fixture')
            app = root / 'app.exe'
            app.write_bytes(b'fresh-signed-app-fixture')
            notices = root / 'notices'
            notices.mkdir()
            (notices / 'LICENSE.txt').write_text('fixture license')
            digest = hashlib.sha256(archive.read_bytes()).hexdigest()
            manifest = self.stage.stage_bundle(app, archive, digest,
                ['llama-server.exe'], notices, root / 'staged')
            self.assertEqual((root / 'staged/lugha-desk.exe').read_bytes(), app.read_bytes())
            self.assertEqual(manifest['files']['lugha-desk.exe']['sha256'],
                hashlib.sha256(app.read_bytes()).hexdigest())
            self.assertFalse(any(name.startswith('model/') for name in manifest['files']))
            self.assertIn('notices/LICENSE.txt', manifest['files'])
            with self.assertRaisesRegex(ValueError, 'empty'):
                self.stage.stage_bundle(app, archive, digest,
                    ['llama-server.exe'], notices, root / 'staged')


if __name__ == '__main__':
    unittest.main()
