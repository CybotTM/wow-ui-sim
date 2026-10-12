# Dedicated simulator VNC — operational checks, visual/remote proof partial

Historical setup: user selected new WoW simulator display, native VNC over SSH, and initially a90-second simulator diagnostic launch. Display`:22`/1280×960 originally used loopback5904 (IPv4/IPv6). Existing5902/5903 sessions unchanged. User units are on-demand, not enabled at boot.

Current access policy, observed 2026-10-12: user expressly requested existing VNC over private LAN without a password. VNC now binds port5904 to one specific RFC1918 IPv4 interface with `-noipv6` and `-nopw`, not a wildcard/public-interface binding. Only VNC restarted; original display PID635115 was preserved and the exact private-interface listener was observed active. No app launch occurred in this change. This is a setup observation only—not remote viewing, rendering, native proof or user acceptance. LAN clients that can reach this listener need no VNC authentication; loopback+authenticated SSH is historical, not current access policy.

Address-bearing setup receipts remain private; no LAN IP/host, password, Xauthority payload or private pixels are published here.

Saved mapped1024×768 "WoW UI Simulator" window/capture and FIRST_FRAME_RENDERED receipts; local RFB3.8 ServerInit reports1280×960. [Independent operational gate](independent-report.md) confirms loopback/service/teardown/copy checks. [Visual gate](visual-report.md) remains PARTIAL because image-tool pixels were omitted; saved-image statistics establish nonuniform pixels only, not visual UI identification. No authenticated remote-view/user/native acceptance claim.

Initial display-readiness race corrected with bounded socket readiness helper. First app launch failed on missing `libxkbcommon-x11.so`; user approved official matching1.13.2-1 runtime package only, installed successfully. No simulator build, system/kernel/GPU-driver upgrade or provider change. Existing Retail GUI artifact/authentication reused, not rehashed.

App unit uses software Lavapipe Vulkan, CPU200%/memory4G/whole-control-group teardown. User later authorized stopping the app after each interactive test instead of automatic90s expiry; `RuntimeMaxSec` is removed, resource limits preserved. Main stops the owned app when each test ends. VNC/display continue running withCPU50%/memory256M/512M. Historical Result=timeout/MainPID0 receipts remain configured-expiry observations, not clean application exits. Start a test explicitly with `systemctl --user start wow-ui-vnc-app.service`.

The earlier loopback viewer/SSH-forwarding instructions belong to the historical receipt and no longer describe the current binding. Current viewer endpoint details remain private. X11 cookie is held in a700 directory/600 Xauthority file; no cookie/key/credential material is retained here.

Stop only these units when finished: `systemctl --user stop wow-ui-vnc-app.service wow-ui-vnc.service wow-ui-vnc-display.service`. Full handoff/frozenBA3FAIL remain OPEN. User clarified there was no403issue; do not carry an invented403blocker.

## Current bounded GUI publication — 2026-10-12

Later viewing run: [Edit Mode / Enter packet](../edit-mode-enter-2026-10-12/README.md) records the service's switch to authenticated `7d9d9f621`, native1920×1080 window and read-only Classic/horizontal observation. That initial bounded app expired naturally. User later accepted layout but reported both keyboard fields unresponsive; native focus was PointerRoot and no Enter reached the app. On the same authenticated artifact, assigning native focus to the mapped simulator window produced five Enter deliveries and user confirmation that WoW chat and `/command` both worked. Launch setup now maps/resizes/focuses the unique owned window and verifies native focus; fresh-launch ownership/focus/wiring/native-input gate independently PASS ([packet](../native-window-focus-2026-10-12/README.md)); main stopped the verified test app. Earlier focus-only failure on the historical artifact remains separate from this current-artifact success. Earlier1280×960 display,1024×768 window and historical artifact receipts below are not current geometry/source proof. Listener policy and original WTF data unchanged.

[New90-second WoW API publication only; not Pi/LLM provider diagnostics](../live-retail-gui-providers-2026-10-12/README.md) · [Independent publication gate; logged event, not pixels; intentional124 expiry, not assertion failure or service repair](../live-retail-gui-providers-2026-10-12/live-provider-report.md). [Owned-process/socket cleanup, existing display preserved;15 copies/39,405 bytes retention independently2905 PASS at5e1b7bab4; native/visual/authenticated-remote/user acceptance OPEN](../live-retail-gui-providers-2026-10-12/README.md).

