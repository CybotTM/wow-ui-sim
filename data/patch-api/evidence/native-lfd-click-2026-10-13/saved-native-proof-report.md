# Saved native proof — bounded verification

**PASS: one actual native LFDMicroButton click. Parent OPEN. Intended user-view size acceptance UNKNOWN.**

## Evidence and coverage

| Claim | Result / evidence |
|---|---|
| Source / artifact | Saved seal identifies `5bd208dbabe90828cf5c7b8f064457745650b0dd`, Retail compiler features. Saved authentication receipt PASS; launch stat exactly matches seal and receipt before/after (device 66306, inode 140549026, size 408213728, mtime_ns 1791887011185378516). No rehash or live identity check. |
| New owned app | Launch unit `wow-ui-vnc-atlas-app-20261013t104528326211z`, invocation `eff8e38d68d646098528c540445fee8d`; saved service MainPID 382127 (bwrap), child/app PID 382142, sealed executable path in `owned-process.json`. `app-metadata.json`: LoadState loaded, ActiveState active, SubState running, ExecMainStatus 0, MemoryCurrent 1502027776, MemoryPeak 1714454528. These are saved observations, not current liveness proof. |
| Isolation / caps | `launch.json` records root RO, private G/tmp bound to /tmp, --unshare-net, authenticated display :22, Xauthority path, MemoryMax 4G, MemoryHigh 3G, CPUQuota 200%. Admission allows launch. Launch configuration is not independent live enforcement proof. |
| Socket bridge | Initial long host alias failed AF_UNIX length check with zero queries/inputs. Saved deviation records `/proc/382142/root/tmp/wow-lua-382142.sock` and same_inode true; ownership records socket_exists true. No current /proc/socket inspection performed. |
| Native focus / hover | Saved focus and window-tree establish `0x800002`, 1920x1080. XTest move to (1783,926), zero button pairs; actual saved GetMouseFocus is LFDMicroButton, PVE false. Native screen coordinates are not Lua coordinates: saved cursor (1776,26.200012207031), Lua rect (1760,6,32,40). |
| Actual backend click | `native-input.py` calls XTestFakeButtonEvent(button 1, down), then release; native-click receipt exit 0, exactly one pair. No Lua Click or MouseClick RPC in inspected action/query helpers. |
| Exact dispatch boundary | Full `native-click-dispatch.stderr` and full app.stderr reviewed. Recorded byte offset 17026 yields exactly one new dispatch record; release frame 18750 and down frame 18750 both LFDMicroButton, identical rect (1760,868.2,32,40), clicks_on_up true, AnyUp registered. |
| Observable result | Saved queries: PVE false before/hover, true after; Guild/CommunitiesFrame false after; same LFDMicroButton rect before/after. Saved native PNGs visually confirm Dungeon Finder panel absent before, visible after. |

## Exact new dispatch

```text
[click-dispatch] release frame=18750 LFDMicroButton(rect=Some(LayoutRect { x: 1760.0, y: 868.2, width: 32.0, height: 40.0 })) mouse_down_frame=Some(18750) Some("LFDMicroButton(rect=Some(LayoutRect { x: 1760.0, y: 868.2, width: 32.0, height: 40.0 }))") clicks_on_up=true registered=Some({"AnyUp"})
```

## Capture authenticity and preserved failures

`native-capture-before.json` preserves exit 1, `ModuleNotFoundError: No module named 'PIL'`. Archived initial helper imports PIL before opening X11: zero native captures on that attempt. Successful before/after receipts both exit 0, 1920x1080, window 0x800002. Inspected corrected helpers use XGetImage, real image bytes, BGRX-to-RGB conversion, and stdlib PNG encoding; no generated/fake pixels. Both `<PRIVATE_BEFORE_IMAGE>` and `<PRIVATE_AFTER_IMAGE>` present and privately inspected. Images not copied or published.

## Limits and exclusions

- **Modifier limitation:** X11 mask 4 persisted; saved IsControlKeyDown is true. This is not evidence of plain/unmodified click parity or universal modifier parity.
- **Size acceptance UNKNOWN:** native window 1920x1080; saved Lua screen 1906x914.2 and app canvas log 1906x914. Native window dimensions do not establish intended user-view size acceptance.
- Static VNC/display untouched/still-active state is user-supplied context, not freshly proven by G. Older saved current-state snapshot (2026-10-12T06:27:37.862888+00:00) records display MainPID 635115 and VNC MainPID 635118 active; cannot certify current state. No static config or service mutation performed.
- Only this bounded LFDMicroButton click is verified. Parent remains OPEN; no broad micro-menu, atlas, keyboard, or native WoW parity acceptance.

## Verifier boundary

Saved evidence only. Zero live queries, input, captures, restarts, process/service operations, builds, hashes, delegation, or tracked edits. Owned app, display, VNC left untouched. Only two private report files created in G, mode 0600.
