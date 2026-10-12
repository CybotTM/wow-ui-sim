# Independent gate 7f32cab67

Verdict: **PARTIAL**. Current operational setup checks pass; requested independent visual check remains unproved.

## Current state
One read-only systemctl --user show batch for the three named units and one ss -ltnp invocation; both exit0. Display active/running PID635115; VNC active/running PID635118. New5904 listens only on127.0.0.1 and[::1]. Prior5902 PID916 and5903 PID401841 remain present; no service mutation performed.
App Result=timeout, MainPID0, ExecMainCode2, ExecMainStatus15, empty ControlGroup and absent expected app cgroup: no current app children. This is expected chosen90-second termination, not successful natural exit. Display/VNC remain running.
App RuntimeMax90s/CPU200%/memory4GiB; display/VNC CPU50%, memory512/256MiB. All KillMode=control-group, TimeoutStop5s, UnitFileState=static.

## Installed evidence
20/20 unit/helper/nonbinary JSON receipt copies match public versus installed/private bytes. Root0700/Xauthority0600, UID/GID1000 via stat only; no authority payload read.
Installed ExecStartPost invokes wait-display.py and records exit0. Source waits for the specific UNIX socket with10s deadline,1s attempt timeout,0.1s retry sleep; normal break on success, SystemExit on exhaustion. Source-only success-case validation, not synthetic unit PASS. Socket readiness is not authenticated X11 readiness; saved xdpyinfo exit0 proves earlier authenticated1280x720 display before resize, not a fresh1280x960 probe.
Reused existing sealed Retail normal-auth metadata from /home/osso/.local/state/wow-ui-sim/handoff/vnc-existing-retail-gui-artifact-current.md only. Identity metadata is not current-source/build/runtime-library equivalence. No binary/library payload access, hashing, rebuild or old proof replay.

## Saved frame and fix boundary
Frame: /home/osso/.local/state/wow-ui-sim/vnc-simulator/frame-retry.png. Saved capture exit0/1280x960, window-tree-retry.json reports mapped1024x768 “WoW UI Simulator”. Saved RFB3.8/security type1/security_result0/ServerInit1280x960 proves local negotiation only; no separate VNC password, privacy relies on loopback plus authenticated SSH.
**Concrete limitation:** image reads returned “Previously processed image omitted”; pixels were not independently inspectable in this verifier context. Cannot honestly certify visible simulator UI versus blank desktop. No new capture attempted. LOCAL_PRIVATE_VNC_SETUP_PASS withheld for this missing visual proof.
Initial xdpyinfo exit1/display race, missing libxkbcommon-x11.so panic/status101 and initial zero-window receipt preserved. Bounded helper wired post-start; saved approved single-package libxkbcommon-x111.13.2-1 installation exit0 precedes actual retry mapped-window evidence. Later timeout is the chosen limit, not recurrence of the initial library panic.

## Limits
Remote SSH/viewer/user acceptance UNPROVED. No native/full-profile/broad BA pass. Full goal OPEN; frozen BA3 FAIL unchanged. No403issue; invented403blocker dropped. No delegation, fake operational authority, new connections, launch/capture, service changes, package installation or stress.

Outputs:
/home/osso/.local/state/wow-ui-sim/vnc-simulator/independent-report.md
/home/osso/.local/state/wow-ui-sim/vnc-simulator/independent-proof.json
/home/osso/.local/state/wow-ui-sim/vnc-simulator/current-state.json
