# Live provider bounded verification

**PASS — publication only. Parent OPEN.**

Epoch: `/home/osso/.local/state/wow-ui-sim/verification/vnc-current-provider-binding/20261012T174131944689Z`.

## Evidence
- Source applicability: independent `git diff ab8dd8739 -- src Cargo.toml Cargo.lock build.rs .cargo`, exit 0, empty stdout/stderr; matches recorded production-applicability. Data/validator-only drift does not establish whole-library identity.
- Reused normal compiler metadata: Retail GUI binary, `test=false`, sealed path matches submission and owned-server record. Existing independent auth receipt authenticated at 2026-10-12T15:22:16.202613Z. No binary read/hash/reauth or library authentication performed here.
- Submission exit 0; RuntimeMaxSec=95, timeout=90, existing DISPLAY=:22, software Lavapipe, WOW_SIM_CASC=0. Admission 27.884967803955078 GiB >=12; load 11.61 <=24.
- Main ownership observations identify timeout PID 2270040 and app PID 2270041 under exact new unit cgroup; owned-server records exact sealed executable and `/tmp/wow-lua-2270041.sock`. Reused recorded ownership, not a fresh live symlink observation after expiry.
- One targeted query, exit 0, empty stderr. Exact stdout:

```
C_FriendList.GetFriendInfo=function:C
C_MountJournal.GetMountFromItem=function:C
```

Script uses rawget for namespace/field, type/debug.getinfo for sourcekind, and appends only to reserved `__repl_captured`; no provider invocation, namespace fabrication, model seeds, chat print, vendor override, or images. Assertions cover diagnostic transport only.

Full app logs read: 19 stdout lines, 265 stderr lines. Actual app log: `[  6.919s] [Startup] Firing FIRST_FRAME_RENDERED`. This proves logged startup event, not rendered pixels or CASC/texture correctness. Existing app workarounds are logged; verifier/query added none. App itself logged stale-socket cleanup; verifier performed no unlink.

## Terminal observation
Owned unit: ActiveState=failed, SubState=failed, Result=exit-code, ExecMainCode=1, ExecMainStatus=124, MainPID=0, ControlGroup empty. Start 2026-10-12 12:41:31 CDT; exit 12:43:02 CDT. **Expected timeout90 expiry, not provider assertion failure.** RuntimeMaxUSec=1min35s is outer bound, not observed timeout cause.

Owned app PID 2270041 and timeout PID 2270040 absent. Lua socket absent; iced-debug socket still present. No stop/unlink; main owns cleanup. Existing display unit remains active/running, MainPID=635115, matching recorded display-state.

Historic `wow-ui-vnc-app.service` was already failed timeout/MainPID0 (caller evidence); preserved, not claimed repaired. No unrelated-unit/global-process inventory or secret/Xauthority read.

## Limits
Publication of two C functions only. No behavior/native parity, visual/user acceptance, persistent-service repair, whole-library identity, pixels/textures/CASC claim. Parent goal remains OPEN.
