"""Validate historical audit scope without accepting unrecorded input drift."""
import hashlib
import json
import subprocess

# Map exact historical digests to exact merged replacements, by path.
# The original 8.2.5 closure ends at 127aa3724035d9e668143d9b28aaa4cf6e176fa1;
# its historical inputs are from 7ff3dc540^.
LATER_AUDIT_REPLACEMENTS = {
    # C_Map.GetMapDisplayInfo closure: 63f3a8a7373c8e52ffcc005bb443a9cf64b59fbf.
    'tests/data/patch_8_0_1_sweep_known_gaps.json': {
        'bad5e7e4e77494f4e506684fb63b97d5b0d8ff9dd8279f714fbf00ad32bac55a':
            '618f9a1c663f16ce1df4613ff953e69b322e2cd04fe2868ad099189ab669e1b1',
    },
    'data/patch-api/sources/9.2.5-page-coverage.json': {
        'd022e992c195f5756620fc4534430cb4f2bf372cbb7b9ee3d9075f17ad5524df':
            '64896487daebe0b4710259a861edd9fe17a2114a8822e00e4363f0aa4c284e97',
    },
    'tests/data/patch_9_2_5_sweep_known_gaps.json': {
        'a834106b494886a3ac4877e5a623d7552aefd27188fab04555257c533651f3e9':
            'f7370f8626dab454cb1815e064173a7ad12c36b53046c1db2ae44a31a32c524a',
    },
    # Later closures: 1285d9eeee11e51c0fc0d8d9c7b1b4a8919b7ea8 then
    # 593e543f53dc91222b652608ae884bd29a903f66.
    'tests/data/patch_9_1_0_sweep_known_gaps.json': {
        # Historical bytes: 95f82e8d0f3612285886810157430da357aec5d1.
        '211bdd9a825d4d16547b19e64357112779bf421718a5289bbab7fb3f080c141b':
            'ef32b149c4fc7e35d3a9369f819563a252df09cab2fc2e0d988cd02985a6b1fc',
        # Historical bytes: 1d2417266cc4164ac52cfee6525a4b74735638bd.
        '65e517e2e8c4d7351728246d77e194af7c683ab5c7c8afdae263119029659d5a':
            'ef32b149c4fc7e35d3a9369f819563a252df09cab2fc2e0d988cd02985a6b1fc',
    },
    # Later bytes: 1285d9eeee11e51c0fc0d8d9c7b1b4a8919b7ea8.
    'tests/data/patch_10_0_0_sweep_known_gaps.json': {
        # Historical bytes: 465c908ce6c88b70a94f4ee6fdb6c0928c69a42e.
        'e6e0214cd31bba35f46e777ea57907a7da75b197a803bc5f39c1e4d2fba23819':
            '048bea0113882fc2faa82b7bd3f6578e3e8e149de7193eba9a0829948fe9b602',
    },
    # Later bytes: 43158d03c149fe87304a8f98d2133a30fbdb77fe.
    'tests/data/patch_10_0_2_sweep_known_gaps.json': {
        # Historical bytes: 465c908ce6c88b70a94f4ee6fdb6c0928c69a42e.
        '6b4b5a740fee6214af8b69145ab6e041ee04208be751c91215d226815ddfe703':
            '590e9b7934f7d351fbb54558c29d83424187939562c917c235d16f27f6f20d52',
    },
    # Later closures: ccd526c6c9f783cf7d3fe15eda05c42c6cabab59 then
    # f04a8c49a351f1f7a9687305755ba493b85bad15.
    'tests/data/patch_10_2_5_sweep_known_gaps.json': {
        # Historical bytes: 465c908ce6c88b70a94f4ee6fdb6c0928c69a42e.
        '2df48e1d3d7e92d81cbeeff946070e444a8cb6560da5f66a1b01e33b6753fbe7':
            '3c45a34df6abb9433e12471c3578e610164ab5c3fc6743e0c203a5fb17be259c',
    },
    # Later bytes: 1285d9eeee11e51c0fc0d8d9c7b1b4a8919b7ea8.
    'tests/data/patch_11_1_5_sweep_known_gaps.json': {
        # Historical bytes: 465c908ce6c88b70a94f4ee6fdb6c0928c69a42e.
        'faff51bfc8c5353064e898599684424b20729a10878ce3a499598eb2397b48c3':
            '5a3823927462e6bf0b5f19086c43a9b072a174bf4ac33ab4f3106c30f7317659',
    },
    # Later closures: ccd526c6c9f783cf7d3fe15eda05c42c6cabab59 then
    # f04a8c49a351f1f7a9687305755ba493b85bad15.
    'tests/data/patch_11_1_7_sweep_known_gaps.json': {
        # Historical bytes: 465c908ce6c88b70a94f4ee6fdb6c0928c69a42e.
        'ca385ab6271755b60fe494629ebaf9ea3f9b27a4c004ca9127aa74403b9641df':
            '2583b5b920e816d42b6f5a3c694f51ef3f23382fd42b4e5e3fd16700037504b0',
    },
    # Privacy overlay: f9eeca84a63a2b556a3cd47c57466ffaa1dedb84.
    # Exact before/after hashes from privacy-corrections-2026-10-13/
    # retail-correction-manifest.json; never rewrite the redacted public log.
    'data/patch-api/evidence/10.0.0-session-2026-10-07/p1000-startup-stderr.txt': {
        # Historical bytes: 465c908ce6c88b70a94f4ee6fdb6c0928c69a42e.
        '4fb8e141198cdccbcab7f30dc480987627b0caeaea3921da42b12f27b1f490f0':
            'e2d33e8f7a49df8b32ab921b482c60c30c63ba8a92b819dae51333b3f32b41e7',
    },
    # Integration replaced the 53-gap result with 51 gaps but retained its old
    # sweep table. Each stage must read its own exact snapshot, not mix them.
    # Later bytes: 1d2417266cc4164ac52cfee6525a4b74735638bd.
    'data/patch-api/evidence/9.1.0-session-2026-10-07/p910_sweep_out-results.json': {
        # Historical bytes: 95f82e8d0f3612285886810157430da357aec5d1.
        '1e227828eda80c2dcb18c58ad4c3739b0f3304a60930cce6a796aec6173d4b5e':
            'd85b2de60a7bab39c8f50aaead29bfdd021abbe249eed22e92056509a46c97c9',
    },
}


def preserved_input_matches(root, path, recorded_digest):
    """Accept original bytes or a specific, merged later-audit replacement."""
    current_digest = hashlib.sha256((root / path).read_bytes()).hexdigest()
    return current_digest == recorded_digest or LATER_AUDIT_REPLACEMENTS.get(path, {}).get(
        recorded_digest
    ) == current_digest


def historical_bytes(root, path, revision):
    """Check current input provenance, then return the historical proof input."""
    original = subprocess.check_output(['git', 'show', f'{revision}:{path}'], cwd=root)
    digest = hashlib.sha256(original).hexdigest()
    assert preserved_input_matches(root, path, digest), f'unrecorded input change: {path}'
    return original


def historical_json(root, path, revision):
    """Decode a historical input after checking its current provenance."""
    return json.loads(historical_bytes(root, path, revision))


def read_audit_bytes(root, path, revision):
    """Use historical bytes only for explicitly recorded later replacements."""
    relative = path.relative_to(root).as_posix()
    if relative in LATER_AUDIT_REPLACEMENTS:
        return historical_bytes(root, relative, revision)
    return path.read_bytes()


def read_audit_json(root, path, revision):
    """Use historical accounting only for explicitly recorded later closures."""
    return json.loads(read_audit_bytes(root, path, revision))


def _historical_paths(root, revision, directory, pattern):
    names = subprocess.check_output(
        ['git', 'ls-tree', '-r', '--name-only', revision, directory],
        cwd=root, text=True,
    ).splitlines()
    paths = [root / name for name in names if (root / name).match(pattern)]
    assert paths, f'no {pattern} at {revision}'
    for path in paths:
        assert path.is_file(), f'missing historical input: {path.relative_to(root)}'
    return paths


def historical_registers(root, revision):
    """Keep the complete register set at the audit revision, not today's set."""
    return _historical_paths(root, revision, 'data/patch-api/sources', '*-wikitext-register.json')


def historical_sweep_tests(root, revision):
    """Keep the source-defined sweep set at the audit revision."""
    return _historical_paths(root, revision, 'tests', 'patch_*_publication_sweep.rs')
