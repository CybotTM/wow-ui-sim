# Dedicated simulator VNC — operational checks, visual/remote proof partial

Historical setup: user selected new WoW simulator display, native VNC over SSH, and90-second simulator launch only. Display`:22`/1280×960 originally used loopback5904 (IPv4/IPv6). Existing5902/5903 sessions unchanged. User units are on-demand, not enabled at boot.

Current access policy, observed 2026-10-12: user expressly requested existing VNC over private LAN without a password. VNC now binds port5904 to one specific RFC1918 IPv4 interface with `-noipv6` and `-nopw`, not a wildcard/public-interface binding. Only VNC restarted; original display PID635115 was preserved and the exact private-interface listener was observed active. No app launch occurred in this change. This is a setup observation only—not remote viewing, rendering, native proof or user acceptance. LAN clients that can reach this listener need no VNC authentication; loopback+authenticated SSH is historical, not current access policy.

Address-bearing setup receipts remain private; no LAN IP/host, password, Xauthority payload or private pixels are published here.

Saved mapped1024×768 "WoW UI Simulator" window/capture and FIRST_FRAME_RENDERED receipts; local RFB3.8 ServerInit reports1280×960. [Independent operational gate](independent-report.md) confirms loopback/service/teardown/copy checks. [Visual gate](visual-report.md) remains PARTIAL because image-tool pixels were omitted; saved-image statistics establish nonuniform pixels only, not visual UI identification. No authenticated remote-view/user/native acceptance claim.

Initial display-readiness race corrected with bounded socket readiness helper. First app launch failed on missing `libxkbcommon-x11.so`; user approved official matching1.13.2-1 runtime package only, installed successfully. No simulator build, system/kernel/GPU-driver upgrade or provider change. Existing Retail GUI artifact/authentication reused, not rehashed.

App unit uses software Lavapipe Vulkan, CPU200%/memory4G/runtime90s/whole-control-group teardown. After90s, Result=timeout/MainPID0 is expected by the selected limit, not an unbounded process or a claimed clean application exit. VNC/display continue running withCPU50%/memory256M/512M. Launch another bounded session explicitly with `systemctl --user start wow-ui-vnc-app.service`.

The earlier loopback viewer/SSH-forwarding instructions belong to the historical receipt and no longer describe the current binding. Current viewer endpoint details remain private. X11 cookie is held in a700 directory/600 Xauthority file; no cookie/key/credential material is retained here.

Stop only these units when finished: `systemctl --user stop wow-ui-vnc-app.service wow-ui-vnc.service wow-ui-vnc-display.service`. Full handoff/frozenBA3FAIL remain OPEN. User clarified there was no403issue; do not carry an invented403blocker.

## Current bounded GUI publication — 2026-10-12

Later viewing run: [Edit Mode / Enter packet](../edit-mode-enter-2026-10-12/README.md) records the service's switch to authenticated `7d9d9f621`, native1920×1080 window and read-only Classic/horizontal observation. That bounded app expired naturally; intended-user keyboard/layout acceptance remains pending. Earlier1280×960 display,1024×768 window and historical artifact receipts below are not current geometry/source proof. Listener policy and original WTF data unchanged.

[New90-second WoW API publication only; not Pi/LLM provider diagnostics](../live-retail-gui-providers-2026-10-12/README.md) · [Independent publication gate; logged event, not pixels; intentional124 expiry, not assertion failure or service repair](../live-retail-gui-providers-2026-10-12/live-provider-report.md). [Owned-process/socket cleanup, existing display preserved;15 copies/39,405 bytes retention independently2905 PASS at5e1b7bab4; native/visual/authenticated-remote/user acceptance OPEN](../live-retail-gui-providers-2026-10-12/README.md).

