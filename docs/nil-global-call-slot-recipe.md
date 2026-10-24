# S25 named nil-call slot proof — parent recipe, NOT EXECUTED

Staged 2026-10-24. Static READY-FOR-SLOT is not runtime acceptance. No slot was selected, reserved or used. Future commands use Pyrun argv helpers only. No agents, Bash, vendor edits, decompiled-source reads, push or merge to master.

## 1. Immutable source custody

| Binding | Exact source | rilua revision | Purpose |
|---|---|---|---|
| OLD | `8098f4d75dde4b3599ab88ce7f10d8257e32d809` | `b2629b6329484945cb14e43b0e5590ee308c5df4` | Wrath S25 diagnostic RED only; no master/new-pin producers |
| MASTER | `571284aaa8bb29f17d27e90110e4f863921d9330` | `b2629b6329484945cb14e43b0e5590ee308c5df4` | Paired stock comparison, NOT historical S25 RED |
| GREEN integration | `9083bf83fa63bbfbe0242ee835e939a2824896db` | `101d7b0287e56efbded3c2d2b87936310aad4aa6` | Merge of lane `1920b10c9f7b4c908055497c6f01b6f88b4be51f` and MASTER |

ROOT = `/home/osso/.worktrees/wow-ui-sim-rilua-nilcall`, branch `rilua-nilcall`. Parent records full final branch SHA including this recipe commit as GREEN before execution; confirms its code/test/manifest/lock tree is identical to GREEN integration above. Do not use a moving `master` name. Record full tree and source hashes even for documentation-only descendants.

Merge was conflict-free; wiki index/log equal MASTER, with no unique lane wiki changes discarded. All five diagnostic test files remain byte-identical to OLD. The OLD manifest/lock are not replaced with the newer pin. Do not build new switch wiring against the OLD dependency.

Before each source transition, require clean tracked/untracked inputs and no owned live unit, child, executable or slot writer. Save final branch SHA, then `git switch --detach OLD` for RED; later `git switch --detach MASTER` for stock reference; restore with `git switch rilua-nilcall`. Never merge master into OLD, cherry-pick newer producers there, or mutate pins to manufacture RED. Source switching happens only in this lane, never the canonical checkout. Preserve source/executable receipts before any transition.

Bind each command to full SHA/tree, complete tracked/untracked input manifest and hashes, Cargo.toml/lock hashes, resolved rilua source, `.cargo/config.toml`, actual `cargo -V`/`rustc -Vv`, flags/environment, profile, slot ownership, selected cache and artifact SHA-256. Record runtime `data/blizzard-ui-files/P.txt`, cache completion/provenance and selected TOC/Lua/XML hashes. Wrath includes its selected FrameXML TOC. No cache refresh during comparison; mismatch is HOLD, not a diagnostic failure. B03 follows B01; this staging does not grant runtime admission.

## 2. One owned slot; durable asynchronous exit capture

Parent chooses ONE idle approved `/home/osso/.cache/wow-ui-sim-targets/slot-{0,1,2,3}` only after checking existing owner/lock/marker, all relevant units/process cgroups and open slot references. Use the shared slot coordinator's existing lock and ownership protocol; a new lane-private lock does not authorize reuse. Acquire its exclusive lock before marker creation/transfer, record owner/session, grant, source/profile, actual unit MainPID and history. Do not overwrite another marker, infer idle from MainPID=0, create a fifth target, use canonical target or prune caches. If coordination cannot establish exclusive ownership, HOLD.

Bind SLOT_LOCK to the coordinator's existing absolute lock path for the chosen slot and record it; do not invent an alternate filename. Each unit also holds that same kernel lock for its entire command lifetime using `['flock', '--exclusive', '--nonblock', '--conflict-exit-code=75', '--no-fork', str(SLOT_LOCK), *ARGV]` as its launched command. Lock contention/status75 is HOLD, never test RED. If the coordinator already holds the kernel lock, use its established transfer/execution protocol rather than self-deadlocking or bypassing it; no submission until ownership and lifetime locking agree.

Keep ownership through compilation, discovery, each exact execution, stock captures and immediate dependent diagnostics. Serialize source/profile transitions. Update the granted marker with each unit's actual MainPID, not systemd-run's launcher PID. Hold through errors, interruption, failed integrity or incomplete attribution; no unconditional `finally` release. Release only after parent acceptance, saved evidence and confirmed exit of all owned processes/references. A retained active/exited unit is not a live child but must retain its exit receipt until saved.

Use new private OUT attempt directory outside target caches (0700), logs/receipts 0600; never overwrite earlier attempts. Stable `CAPACITY_ID=rilua-nilcall-sim-proof`. Every launch needs fresh exit-0 admission containing literal `| ADD:` from:

```python
['env', 'CAPACITY_ID=rilua-nilcall-sim-proof', 'timeout', '35',
 '/home/osso/.worktrees/logs/capacity.sh']
```

No priority identity/FIFO bypass. On HOLD return to independent work and schedule next bounded attempt within 150 seconds; no synchronous build polling or unbounded admission loop. Submission returns immediately; inspect later. No timeout on compilation. Runtime timeout is 88s plus 2s kill grace, never over 90s.

Future launcher argv (U/log paths/ROOT/SLOT/ARGV are bound in saved receipt):

```python
def launch_argv(U, LOG_OUT, LOG_ERR, ROOT, SLOT, ARGV, CHILD_XDG=None):
    argv = ['systemd-run', '--user', '--remain-after-exit', '--unit=' + U,
        '--slice=agents.slice', '--property=Type=exec',
        '--property=MemoryMax=9G', '--property=MemorySwapMax=0',
        '--property=StandardOutput=append:' + str(LOG_OUT),
        '--property=StandardError=append:' + str(LOG_ERR),
        '--working-directory=' + str(ROOT),
        '--setenv=CARGO_INCREMENTAL=0', '--setenv=CARGO_BUILD_JOBS=1',
        '--setenv=CARGO_TARGET_DIR=' + str(SLOT),
        '--setenv=CAPACITY_ID=rilua-nilcall-sim-proof']
    if CHILD_XDG is not None:
        argv += ['--setenv=XDG_DATA_HOME=' + str(CHILD_XDG)]
    return argv + ARGV
```

Precreate fresh logs and save argv/binding BEFORE submission. `cli.command(*argv).capture().run()` returning 0 proves submission only. Save submission stdout/stderr including exceptions; query the unique unit rather than resubmitting if launch completion is uncertain. Later inspect:

```python
['systemctl', '--user', 'show', U,
 '--property=ActiveState,SubState,Result,ExecMainCode,ExecMainStatus,MainPID,ControlGroup,MemoryMax,MemorySwapMax,Slice']
```

PASS requires **ActiveState=active, SubState=exited, Result=success, ExecMainCode=1, ExecMainStatus=0** and valid command-specific output. Require actual MemoryMax=9663676416, MemorySwapMax=0, Slice=agents.slice. Expected S25 RED requires Result=exit-code, ExecMainCode=1, ExecMainStatus=101 AND the intended single assertion failure. Timeout/OOM/signal/compile/setup/cache errors are invalid, not RED. Preserve full unit status/logs before stopping/resetting retained units. Do not rerun to recover logs.

## 3. Fresh current CVar copy with O_NOATIME guard

For EVERY executable invocation (including discovery, each single test and stock capture), resolve original from parent's unmodified absolute XDG_DATA_HOME, otherwise `/home/osso/.local/share`, plus `wow-sim/cvars.json`. Record original path and resolved symlink target. Missing/unreadable/unstable source is HOLD; do not create defaults. Never mutate parent environment, reset or restore the original.

Use this future snapshot helper, **not ordinary read_bytes on the original**:

```python
import os, hashlib, json
from pathlib import Path

STAT_KEYS = ('st_dev', 'st_ino', 'st_mode', 'st_uid', 'st_gid', 'st_size',
             'st_atime_ns', 'st_mtime_ns', 'st_ctime_ns')
def stat_fields(s):
    return {k: getattr(s, k) for k in STAT_KEYS}

def snapshot_original(path):
    path = Path(path)
    resolved = path.resolve(strict=True)
    link_before = stat_fields(path.lstat())
    before = stat_fields(path.stat())
    # Fail closed on absent O_NOATIME, EPERM or unsupported filesystem.
    # No fallback read that changes access time; never elevate/change ownership.
    fd = os.open(path, os.O_RDONLY | os.O_CLOEXEC | os.O_NOATIME)
    with os.fdopen(fd, 'rb') as f:
        assert stat_fields(os.fstat(f.fileno())) == before
        raw = f.read()
        assert stat_fields(os.fstat(f.fileno())) == before
    assert stat_fields(path.stat()) == before
    assert stat_fields(path.lstat()) == link_before
    assert path.resolve(strict=True) == resolved
    return {'path': str(path), 'resolved': str(resolved), 'stat': before,
            'lstat': link_before, 'sha256': hashlib.sha256(raw).hexdigest()}, raw
```

Immediately before each attempt save `before, raw = snapshot_original(ORIGINAL)` in a fresh 0600 receipt, before admission/launch. Create a new private CHILD directory under OUT/xdg/ATTEMPT (0700), `CHILD/wow-sim` (0700), and exclusive `CHILD/wow-sim/cvars.json` (0600) containing byte-exact raw. Verify child hash, pass CHILD only through launcher XDG_DATA_HOME. Each attempt copies the CURRENT original, never a prior child or saved old snapshot.

After admission/launch failure or exception, take/save after snapshot too. If a unit may have launched, first determine its state; defer final snapshot until all children exit. Compare entire original receipts including atime and hash. Any difference or inability to snapshot is HOLD with evidence retained; never repair original. Child final hash is separate, writes allowed. Keep exception receipts even if snapshot-after also fails. Tests under deps additionally use `default_storage_path()` automatic per-instance temp isolation; CHILD still protects escaped/standalone paths. Normal sealed wow-sim requires CHILD and must not inherit original storage.

## 4. Build once, exact discovery, single-run argv per profile

For OLD Wrath and GREEN each profile, compile separately via launch_argv, without timeout:

```python
PROFILES = ('wrath', 'mists', 'era', 'anniversary', 'retail', 'ptr', 'wowforever')
BUILD_ARGV = {p: ['cargo', 'test', '--locked', '--no-default-features',
    '--features', 'sound,gui,casc,client-' + p, '--test', 'integration',
    '--no-run', '-j1', '--message-format=json'] for p in PROFILES}
CHECK_ARGV = {p: ['cargo', 'check', '--locked', '--no-default-features',
    '--features', 'sound,gui,casc,client-' + p, '--lib', '--test', 'integration',
    '-j1', '--message-format=json'] for p in PROFILES}
```

Parse retained compiler JSON: successful build-finished, exactly one integration test executable (target.name integration, profile.test=true), package/profile/source identity and executable SHA-256. E[P] is that exact path, never a newest-file glob. Keep in deps or bind/seal all runtime libraries if moved. Capture before slot reuse. No source/profile changes while any artifact runs.

These are the complete GREEN selector inventories; aliases below are exact full names, not substring filters:

```python
G = 'lua_error_messages::global_nil_call_error_matches_active_profile'
S = 'classic_guild_roster_selection::friends_frame_onload_reproduces_missing_guild_roster_selection'
D = 'mists_dialog_helpers::money_frame_set_type_reproduces_missing_basic_message_dialog_helper'
C = 'mists_currency_list::token_frame_update_reproduces_missing_currency_list_size'
H = 'mists_compat_bootstrap::mists_honor_frame_shared_reproduces_missing_honor_system_enabled'
R = 'lua_error_messages::retail_nil_global_pcall_preserves_generic_error_and_two_results'
SELECTORS = {'wrath': (S, G), 'mists': (S, G, D, C, H),
             'era': (G,), 'anniversary': (G,), 'retail': (G, R),
             'ptr': (G,), 'wowforever': (G,)}
LIST_ARGV = {p: ['timeout', '--kill-after=2s', '88s', E[p],
                 '--list', '--format', 'terse'] for p in PROFILES}
RUN_ARGV = {p: [['timeout', '--kill-after=2s', '88s', E[p], selector,
                 '--exact', '--test-threads=1', '--nocapture']
                for selector in SELECTORS[p]] for p in PROFILES}
# OLD uses its OWN compiler-bound executable, never E from GREEN:
OLD_LIST_ARGV = ['timeout', '--kill-after=2s', '88s', OLD_E,
                 '--list', '--format', 'terse']
OLD_RUN_ARGV = ['timeout', '--kill-after=2s', '88s', OLD_E, S,
                '--exact', '--test-threads=1', '--nocapture']
```

Submit each LIST_ARGV and each individual RUN_ARGV separately with fresh admission and CHILD. OLD list must contain S exactly once; GREEN list must contain each SELECTORS[P] exactly once as `NAME: test`. Check nonapplicable rows absent: S only Wrath/Mists; D/C/H only Mists; R only Retail. The expected selected registration counts are Wrath2, Mists5, Era1, Anniversary1, Retail2, PTR1, Forever1 (13 GREEN executions). Do not assert total integration inventory equals these subset counts. Single runs require exactly 1 test executed and expected status, never successful zero-test filters. Standalone Cargo test targets may exist but are NOT substitutes for these integration names.

OLD S25 first: require `0 passed; 1 failed`, panic `expected SetGuildRosterSelection nil failure, got:` and generic `attempt to call a nil value` at the diagnostic assertion. If OLD already names the global or fails earlier, the diagnosis is falsified/unproved; do not label RED. Only after this receipt build GREEN. S must then PASS in Wrath and Mists, preserving the injected missing global, not testing guild API repair.

G must name `CooldownFrame_Set` in Wrath/Mists/Era/Anniversary and remain generic in Retail/PTR/Forever. Opposing expectations reject generic-only and named-everywhere policies; error-required calls reject no-op/success producers. S rejects a generic error; D/C/H additionally require precisely one injected `_G` lookup and the matching global name (`SetBasicMessageDialogText`, `GetCurrencyListSize`, `HonorSystemEnabled`). R is preservation, NOT RED: inner pcall exactly false/error, arity 2, generic suffix and no `RetailMissingGlobal`. Its varargs assertion rejects a third result, not merely an expected two-variable prefix. Native-caller errors without recoverable names are not named-global successes.

Run CHECK_ARGV once per GREEN profile after source integration. Require zero compiler-message warnings/errors, successful build-finished, status0 and no plain manifest/deprecation warnings in complete ANSI-stripped logs for both builds/checks. No suppression flags, `#[allow]`, `#[ignore]`, `cfg!` or assertion edits. Reuse valid same-source proof; no redundant broad checks or full suite/Wowless runs. No compilation/check/runtime was executed during this staging.

## 5. Paired stock comparison against MASTER; diagnostic piggyback

Stock is separate from fault-injection tests. For each of seven profiles compare source-bound MASTER versus final GREEN using identical toolchain, profile flags, selected immutable cache/assets and launch options. Use a valid same-source normal wow-sim artifact if already proven by compiler JSON/hash; otherwise conditional build once/profile/source:

```python
STOCK_BUILD_ARGV = {p: ['cargo', 'build', '--locked', '--no-default-features',
    '--features', 'sound,gui,casc,client-' + p, '--bin', 'wow-sim',
    '-j1', '--message-format=json'] for p in PROFILES}
# N[source][p]: immutable normal CLI, target.name wow-sim, kind bin,
# profile.test=false; bind source + profile + ELF + runtime libraries.
STOCK_RUN_ARGV = {source: {p: ['timeout', '--kill-after=2s', '88s',
    N[source][p], '--no-addons', '--no-saved-vars', 'lua-errors']
    for p in PROFILES} for source in ('MASTER', 'GREEN')}
```

Build before running; seal normal artifacts outside target caches (0700 executable, hash equals compiler artifact) before another build overwrites them. Record actual resource/cache resolution; working directory alone is not resource provenance. Every stock invocation gets a fresh current CVar copy and O_NOATIME before/after guard. Include `--setenv=WOW_SIM_LUA_ERRORS_BY_ADDON=1` in both MASTER/GREEN stock launchers so matching GREEN captures can supply the dependent diagnostic without another run. No `--exec-lua`, fault injection, repeated OnLoad or vendor alteration. Save complete stdout/stderr, final summary and unit result. Clean stock is valid JSON `[]` with exit0/active-exited/success. Nonempty JSON requires exit1/Result=exit-code plus matching unique/occurrence summary: completed diagnostic, not clean PASS. Missing/truncated JSON, timeout, OOM or setup failure invalidates comparison.

Compare full raw arrays and counts, not only selected rows or total count. Modern generic wording/errors must be preserved versus MASTER. Classic naming may split/coalesce rows; retain raw messages/wrappers, observed names, selected consumer locations, source/capture hashes and count reconciliation. Normalizing only the recovered named-global fragment back to generic can identify candidate families, not prove correspondence. Never assert opaque pairs fixed or missing APIs implemented. Explain any other changed row/count; ambiguous mappings remain gaps.

After accepted OLD→GREEN and naming/preservation matrix, immediately piggyback `H/nilcall-diag-recipe/SLOT-RECIPE.md` in the SAME ownership grant for Mists/Era/Anniversary plus Wrath clean control. That older recipe's `REV=1920b10c9...` must be explicitly rebound to the actual integrated GREEN and current artifact/cache receipts; do not reuse stale binaries. This recipe's durable exit/CVar O_NOATIME rules supersede its old atime-omitting snapshot. Reuse matching GREEN stock captures instead of rerunning; add its per-addon diagnostic export if not already captured. Six candidate rows/24 occurrences are diagnostic candidates, never repaired-row claims. Preserve native-caller generic residuals and distinguish unresolved attribution.

## 6. Static proof and final runtime ledger

Source inspection at GREEN integration: VM switch before app-data/bootstrap; all seven exhaustive match arms agree with G; Retail R gated by profile-retail (client-retail enables it); S gated Wrath/Mists, D/C/H Mists. `tests/integration.rs` expands automatic top-level discovery from `crates/test-harness/build.rs`; none of these five module paths is excluded. This proves source eligibility, NOT executed registration. Laptop Q1 supplies Wrath static name/false/error/arity2 evidence only, not modern native equivalence.

All five files equal OLD byte-for-byte; SHA-256:

| File under tests/ | SHA-256 |
|---|---|
| classic_guild_roster_selection.rs | fe010a0acc7b00f1605d5c2195a3afa35f63e29a301036863e3ed47c90ca161f |
| lua_error_messages.rs | 2005b5f514c4605e702394bc22e50b86cbb59cd70f561c7dce1282fc3bab575a |
| mists_dialog_helpers.rs | 3b2fa8dceef737a79d06d8b0189779e5577cb12cd5f106529f317a268c7a9d30 |
| mists_currency_list.rs | 1b857af9270518e756f02b054628e1659442afdd59377f113ce60c1db0c88fd6 |
| mists_compat_bootstrap.rs | 2559042eaa4aed99b3cea9302e472f77ed72bba1cdbc5aa8561579b212d83cda |

Parent records one proof ledger row per command: exact source/profile/slot/ELF or compiler binding, argv/environment, admission, unit status, counts/results, warnings, CVar before/after and invalidation scope. Runtime acceptance remains OPEN until original OLD S25 RED, 13 GREEN exact executions, seven warning-free checks/builds and paired stock reconciliation are recorded. READY-FOR-SLOT means only staging complete.
