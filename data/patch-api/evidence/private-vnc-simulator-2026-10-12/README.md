# Dedicated simulator VNC — operational checks, visual/remote proof partial

User selected new WoW simulator display, native VNC over SSH, and90-second simulator launch only. New display`:22`/1280×960; VNC binds loopback5904 (IPv4/IPv6), no public listener. Existing5902/5903 sessions unchanged. New user units are on-demand, not enabled at boot.

Saved mapped1024×768 "WoW UI Simulator" window/capture and FIRST_FRAME_RENDERED receipts; local RFB3.8 ServerInit reports1280×960. [Independent operational gate](independent-report.md) confirms loopback/service/teardown/copy checks. [Visual gate](visual-report.md) remains PARTIAL because image-tool pixels were omitted; saved-image statistics establish nonuniform pixels only, not visual UI identification. No authenticated remote-view/user/native acceptance claim.

Initial display-readiness race corrected with bounded socket readiness helper. First app launch failed on missing `libxkbcommon-x11.so`; user approved official matching1.13.2-1 runtime package only, installed successfully. No simulator build, system/kernel/GPU-driver upgrade or provider change. Existing Retail GUI artifact/authentication reused, not rehashed.

App unit uses software Lavapipe Vulkan, CPU200%/memory4G/runtime90s/whole-control-group teardown. After90s, Result=timeout/MainPID0 is expected by the selected limit, not an unbounded process or a claimed clean application exit. VNC/display continue running withCPU50%/memory256M/512M. Launch another bounded session explicitly with `systemctl --user start wow-ui-vnc-app.service`.

SSH client example (authenticate normally; never disable host verification):

```text
ssh -N -L 127.0.0.1:5904:127.0.0.1:5904 osso@agent-server
```

Viewer host127.0.0.1, port5904 (TigerVNC syntax `127.0.0.1::5904`). VNC itself has no separate password; privacy depends on loopback binding and authenticated SSH transport. X11 cookie is held in a700 directory/600 Xauthority file; no cookie/key/credential material is retained here.

Stop only these units when finished: `systemctl --user stop wow-ui-vnc-app.service wow-ui-vnc.service wow-ui-vnc-display.service`. Full handoff/frozenBA3FAIL remain OPEN. User clarified there was no403issue; do not carry an invented403blocker.

## Current bounded GUI publication — 2026-10-12

[New90-second WoW API publication only; not Pi/LLM provider diagnostics](../live-retail-gui-providers-2026-10-12/README.md) · [Independent publication gate; logged event, not pixels; intentional124 expiry, not assertion failure or service repair](../live-retail-gui-providers-2026-10-12/live-provider-report.md). [Owned-process/socket cleanup, existing display preserved;15 copies/39,405 bytes retention independently2905 PASS at5e1b7bab4; native/visual/authenticated-remote/user acceptance OPEN](../live-retail-gui-providers-2026-10-12/README.md).

