"""Behavioral separation of frozen 8.1.0 proof and current producer closures."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = Path('data/patch-api/evidence/8.1.0-session-2026-10-08')
CURRENT_RESULTS = Path('data/patch-api/evidence/publication-533c-reconciliation-2026-10-12/current-publication-results.json')
CLOSED = {
    'wt-global-api-C_AchievementInfo.GetSupercedingAchievements-13',
    'wt-global-api-C_Calendar.GetEventIndexInfo-20',
    'wt-global-api-C_CurrencyInfo.DoesWarModeBonusApply-29',
    'wt-global-api-C_FriendList.GetFriendInfo-44',
    'wt-global-api-C_Map.GetBountySetMaps-77',
    'wt-global-api-C_MountJournal.GetMountFromItem-78',
}


def load(path):
    return json.loads(path.read_text())


class ValidationTests(unittest.TestCase):
    def setUp(self):
        spec = importlib.util.spec_from_file_location('p810_validation', ROOT / EVIDENCE / 'validate.py')
        self.validator = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.validator)
        self.directory = tempfile.TemporaryDirectory(prefix='p810-validation-')
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        (self.root / '.git').symlink_to(ROOT / '.git', target_is_directory=True)
        self.evidence = self.root / EVIDENCE
        self.evidence.mkdir(parents=True)
        for path in (ROOT / EVIDENCE).iterdir():
            if path.is_file():
                (self.evidence / path.name).symlink_to(path)
        for path in (ROOT / 'data/patch-api/evidence').iterdir():
            if path.is_dir() and path.name not in (EVIDENCE.name, CURRENT_RESULTS.parent.name):
                (self.root / 'data/patch-api/evidence' / path.name).symlink_to(path, target_is_directory=True)
        (self.root / 'data/patch-api/sources').symlink_to(ROOT / 'data/patch-api/sources', target_is_directory=True)
        (self.root / 'docs').symlink_to(ROOT / 'docs', target_is_directory=True)
        (self.root / 'tools').symlink_to(ROOT / 'tools', target_is_directory=True)
        (self.root / 'tests/data').mkdir(parents=True)
        for path in (ROOT / 'tests').glob('patch_*_publication_sweep.rs'):
            (self.root / 'tests' / path.name).symlink_to(path)
        for path in (ROOT / 'tests/data').glob('patch_*_sweep_known_gaps.json'):
            (self.root / 'tests/data' / path.name).symlink_to(path)
        self.frozen = load(ROOT / EVIDENCE / 'patch_8_1_0_publication_sweep-results.json')
        self.historical_gaps = {key for key, row in self.frozen.items() if not row['ok']}
        self.current_gaps = self.historical_gaps - CLOSED
        self.fixture = self.root / 'tests/data/patch_8_1_0_sweep_known_gaps.json'
        self.store(self.fixture, sorted(self.current_gaps))
        self.validator.ROOT = self.root
        self.validator.EVIDENCE = self.evidence
        self.validator.SOURCES = self.root / 'data/patch-api/sources'

    def store(self, path, value):
        if path.is_symlink():
            path.unlink()
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(value) + '\n')

    def publish_current(self):
        results = json.loads(json.dumps(self.frozen))
        for key in CLOSED:
            results[key]['ok'] = True
            results[key]['observed']['detail'] = 'raw=function; lookup=function'
        self.store(self.root / CURRENT_RESULTS, results)
        return results

    def accounting(self):
        register = load(ROOT / 'data/patch-api/sources/8.1.0-wikitext-register.json')
        lines = (ROOT / 'data/patch-api/sources/8.1.0-api-changes.wikitext').read_text().splitlines()
        text = (ROOT / 'data/patch-api/sources/8.1.0-api-changes.txt').read_text()
        return self.validator.check_accounting(register, self.validator.load_extractor(), lines, text)

    def test_frozen_accounting_remains_87_ok_58_gaps_with_52_current_gaps(self):
        self.assertEqual(len(self.current_gaps), 52)
        self.assertEqual(self.accounting()['gaps'], 58)
        self.assertEqual(sum(row['ok'] for row in self.frozen.values()), 87)
        self.assertEqual(self.validator.check_proof()['publication_sweeps'], 40)

    def test_frozen_result_cannot_be_rewritten_to_current(self):
        self.store(self.evidence / 'patch_8_1_0_publication_sweep-results.json', self.publish_current())
        with self.assertRaises(AssertionError):
            self.accounting()

    def test_frozen_gap_review_cannot_drop_a_producer_row(self):
        review = load(self.evidence / 'p810-gap-review.json')
        self.store(self.evidence / 'p810-gap-review.json', [row for row in review if row['source_id'] not in CLOSED])
        with self.assertRaises(AssertionError):
            self.accounting()

    def test_frozen_summary_cannot_be_rewritten_to_93_ok_52_gaps(self):
        summary = load(self.evidence / 'p810-sweep-summary.json')
        row = next(row for row in summary if row['patch'] == '8.1.0')
        row.update(ok=93, gaps=52)
        self.store(self.evidence / 'p810-sweep-summary.json', summary)
        with self.assertRaises(AssertionError):
            self.validator.check_proof()

    def test_original_two_removals_are_separate_from_six_producers(self):
        self.assertEqual(self.validator.check_supersessions()['integrated_gap_closures'], 2)
        self.publish_current()
        self.assertEqual(self.validator.check_current_producer_closures()['current_producer_closures'], 6)

    def test_fixture_writes_leave_existing_source_capture_unchanged(self):
        source_root = ROOT
        source_capture = source_root / CURRENT_RESULTS
        with tempfile.TemporaryDirectory(prefix='p810-reference-') as directory:
            reference = Path(directory)
            for name in ('.git', 'tools', 'docs', 'tests'):
                (reference / name).symlink_to(source_root / name, target_is_directory=True)
            (reference / 'data/patch-api').mkdir(parents=True)
            (reference / 'data/patch-api/sources').symlink_to(
                source_root / 'data/patch-api/sources', target_is_directory=True)
            evidence = reference / 'data/patch-api/evidence'
            evidence.mkdir()
            for path in (source_root / 'data/patch-api/evidence').iterdir():
                if path.is_dir() and path.name != CURRENT_RESULTS.parent.name:
                    (evidence / path.name).symlink_to(path, target_is_directory=True)
            capture = reference / CURRENT_RESULTS
            capture.parent.mkdir()
            original = source_capture.read_bytes()
            capture.write_bytes(original)
            case = ValidationTests('test_missing_current_packet_fails_without_historical_fallback')
            with patch.dict(globals(), ROOT=reference):
                try:
                    case.setUp()
                    case.publish_current()
                    self.assertEqual(capture.read_bytes(), original)
                finally:
                    case.doCleanups()

    def test_missing_current_packet_fails_without_historical_fallback(self):
        with self.assertRaisesRegex(FileNotFoundError, 'current-publication-results.json'):
            self.validator.check_current_producer_closures()

    def test_current_fixture_rejects_unreviewed_extra_closure_and_stale_gap(self):
        self.publish_current()
        extra = sorted(self.current_gaps)[0]
        for gaps in (self.current_gaps - {extra}, self.current_gaps | {sorted(CLOSED)[0]}, self.current_gaps | {'invented-id'}):
            with self.subTest(gaps=len(gaps)):
                self.store(self.fixture, sorted(gaps))
                with self.assertRaises(AssertionError):
                    self.validator.check_current_producer_closures()

    def test_current_results_reject_failed_closure_missing_row_and_new_gap(self):
        for mutation in ('failed-closure', 'missing-row', 'new-gap', 'wrong-symbol', 'fallback-only'):
            results = self.publish_current()
            key = sorted(CLOSED)[0]
            if mutation == 'failed-closure':
                results[key]['ok'] = False
            elif mutation == 'missing-row':
                del results[key]
            elif mutation == 'new-gap':
                results[next(k for k, row in results.items() if row['ok'] and k not in CLOSED)]['ok'] = False
            elif mutation == 'wrong-symbol':
                results[key]['expected']['symbol'] = 'C_Invented.Query'
            else:
                results[key]['observed']['detail'] = 'raw=nil; lookup=function'
            self.store(self.root / CURRENT_RESULTS, results)
            with self.subTest(mutation=mutation), self.assertRaises(AssertionError):
                self.validator.check_current_producer_closures()


if __name__ == '__main__':
    unittest.main()
