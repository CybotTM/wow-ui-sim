"""Behavioral CLI fixtures for the pinned, separate Wrath source cache."""
import json
from pathlib import Path
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch

import gen_blizzard_ui_manifest as generator


WRATH_REVISION = 'c4e0255fc574598428ee1c25f160ab949798bc98'
EXPECTED_ENTRIES = [
    'Blizzard_Test/Test.lua',
    'FrameXML/FrameXML.toc',
    'FrameXML/UIPanelTemplates.lua',
]


class WrathManifestTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.cache = self.root / 'cache'
        self.repo = self.root / 'repo'
        self.snapshot = self.cache / 'wrath'
        self.receipt = self.snapshot / '.wow-ui-sim-wrath-source.json'
        self.pin = self.repo / 'data/blizzard-ui-builds/wrath.json'
        self.manifest = self.repo / 'data/blizzard-ui-files/wrath.txt'
        self.write(self.snapshot / 'AddOns/Blizzard_Test/Test.lua', b'-- wrath addon\n')
        self.write(self.snapshot / 'FrameXML/FrameXML.toc', b'UIPanelTemplates.lua\n')
        self.write(self.snapshot / 'FrameXML/UIPanelTemplates.lua', b'-- wrath framexml\n')
        self.write_json(self.receipt, {'gethe_revision': WRATH_REVISION})
        self.write_json(self.pin, {'gethe_revision': WRATH_REVISION})
        self.original_manifest = b'ExistingWrath/Keep.lua\r\n'
        self.write(self.manifest, self.original_manifest)
        self.protected_roots = []
        for profile, branch in [('mists', 'classic'), ('wowforever', 'forever')]:
            snapshot = self.cache / branch
            self.write(snapshot / 'Interface/AddOns/Blizzard_Test/Test.lua',
                       f'-- {profile} snapshot sentinel\r\n'.encode())
            self.write(snapshot / 'version.txt', f'{profile}-sentinel\n'.encode())
            manifest = self.repo / f'data/blizzard-ui-files/{profile}.txt'
            self.write(manifest, f'{profile}/Keep.lua\r\n'.encode())
            self.protected_roots.extend([snapshot, manifest])
        self.protected_before = self.protected_bytes()
        for name, value in [('CACHE_ROOT', self.cache), ('REPO_ROOT', self.repo),
                            ('GIT_ROOT', self.root / 'git-cache')]:
            self.enterContext(patch.object(generator, name, value))

    def write(self, path, content):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(content)

    def write_json(self, path, content):
        self.write(path, (json.dumps(content) + '\n').encode())

    def protected_bytes(self):
        contents = {}
        for root in self.protected_roots:
            paths = [root] if root.is_file() else root.rglob('*')
            for path in paths:
                if path.is_file():
                    contents[str(path.relative_to(self.root))] = path.read_bytes()
        return contents

    def invoke_main(self):
        try:
            with patch.object(sys, 'argv',
                              ['gen_blizzard_ui_manifest.py', 'wrath', '--no-refresh']):
                generator.main()
        finally:
            self.assertEqual(self.protected_bytes(), self.protected_before)

    def assert_manifest_entries(self):
        self.assertEqual(self.manifest.read_text(encoding='utf-8').splitlines(),
                         EXPECTED_ENTRIES)

    def assert_rejected_without_replacement(self):
        with self.assertRaises(SystemExit) as failure:
            self.invoke_main()
        self.assertNotIn(failure.exception.code, (None, 0))
        self.assertEqual(self.manifest.read_bytes(), self.original_manifest)

    def test_wrath_no_refresh_writes_addons_and_framexml_root_relative_entries(self):
        self.invoke_main()
        self.assert_manifest_entries()

    def test_archive_digest_receipt_is_optional_metadata(self):
        self.write_json(self.receipt, {
            'gethe_revision': WRATH_REVISION,
            'archive_sha256': 'a' * 64,
        })
        self.invoke_main()
        self.assert_manifest_entries()

    def test_missing_framexml_preserves_existing_wrath_manifest(self):
        shutil.rmtree(self.snapshot / 'FrameXML')
        self.assert_rejected_without_replacement()

    def test_receipt_revision_mismatch_preserves_existing_wrath_manifest(self):
        self.write_json(self.receipt, {'gethe_revision': '0' * 40})
        self.assert_rejected_without_replacement()

    def test_pin_file_is_authoritative_over_matching_historical_receipt(self):
        self.write_json(self.pin, {'gethe_revision': '1' * 40})
        self.assert_rejected_without_replacement()


if __name__ == '__main__':
    unittest.main()
