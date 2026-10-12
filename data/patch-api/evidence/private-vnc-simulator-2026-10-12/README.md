# Dedicated simulator VNC — pending independent setup gate

User selected new WoW simulator display, native VNC over SSH, and90-second simulator launch only. New display`:22`/1280×960; VNC binds loopback5904 (IPv4/IPv6), no public listener. Existing5902/5903 sessions unchanged. New user units are on-demand, not enabled at boot.

Main observed mapped1024×768 "WoW UI Simulator" window and actual UI frame in `frame-retry.png`; local RFB3.8 ServerInit reports1280×960. This is local setup/frame evidence, not authenticated remote-user viewing or native/whole-profile acceptance.

Initial display-readiness race corrected with bounded socket readiness helper. First app launch failed on missing `libxkbcommon-x11.so`; user approved official matching1.13.2-1 runtime package only, installed successfully. No simulator build, system/kernel/GPU-driver upgrade or provider change. Existing Retail GUI artifact/authentication reused, not rehashed.

App unit uses software Lavapipe Vulkan, CPU200%/memory4G/runtime90s/whole-control-group teardown. After90s, Result=timeout/MainPID0 is expected by the selected limit, not an unbounded process or a claimed clean application exit. VNC/display continue running withCPU50%/memory256M/512M. Launch another bounded session explicitly with `systemctl --user start wow-ui-vnc-app.service`.

SSH client example (authenticate normally; never disable host verification):

```text
ssh -N -L 127.0.0.1:5904:127.0.0.1:5904 osso@agent-server
```

Viewer host127.0.0.1, port5904 (TigerVNC syntax `127.0.0.1::5904`). VNC itself has no separate password; privacy depends on loopback binding and authenticated SSH transport. X11 cookie is held in a700 directory/600 Xauthority file; no cookie/key/credential material is retained here.

Stop only these units when finished: `systemctl --user stop wow-ui-vnc-app.service wow-ui-vnc.service wow-ui-vnc-display.service`. Full handoff/frozenBA3FAIL remain OPEN. User clarified there was no403issue; do not carry an invented403blocker.
