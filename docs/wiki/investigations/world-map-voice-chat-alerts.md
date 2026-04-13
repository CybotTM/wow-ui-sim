# World Map Voice Chat Alerts

Initial investigation found a reduced-harness artifact, not a proven `WorldMapFrame` render-order bug in the live/full addon stack. Follow-up probes now show two separate behaviors:

- voice prompt alerts sort below the world map panel in a combined world-map/chat/channels stack
- the live-like `1024x768` overlap comes from the standalone chat voice button (`ChatFrameChannelButton`), which physically intrudes into the map bounds while still rendering below the map border

## Content

## Symptoms

- In the isolated world-map stack, `VoiceChatPromptActivateChannel` and `VoiceChatChannelActivatedNotification` can appear above the map panel even before startup settles.
- The visible frames are top-level voice prompt alerts, not children of `WorldMapFrame`.
- `ChatAlertFrame` stays as the simulator stub, so any shown voice alert is unmanaged and falls back to the default top-level position instead of the real chat-alert anchor chain.

## Root Cause

### 1. Missing `Blizzard_SocialToast`

`Blizzard_Channels` depends on `Blizzard_SocialToast`:

- [`Interface/BlizzardUI/Blizzard_Channels/Blizzard_Channels.toc`](../../../Interface/BlizzardUI/Blizzard_Channels/Blizzard_Channels.toc)
- [`Interface/BlizzardUI/Blizzard_SocialToast/Blizzard_SocialToast.toc`](../../../Interface/BlizzardUI/Blizzard_SocialToast/Blizzard_SocialToast.toc)

The voice prompt frames inherit through:

- `VoiceChatPromptActivateChannel` / `VoiceChatChannelActivatedNotification`
- `VoiceChatPromptTemplate`
- `SocialToastTemplate`

`SocialToastTemplate` is where `hidden="true"` lives:

- [`Interface/BlizzardUI/Blizzard_Channels/VoiceChatPrompt.xml`](../../../Interface/BlizzardUI/Blizzard_Channels/VoiceChatPrompt.xml)
- [`Interface/BlizzardUI/Blizzard_SocialToast/SocialToast.xml`](../../../Interface/BlizzardUI/Blizzard_SocialToast/SocialToast.xml)

When `Blizzard_Channels` is loaded without `Blizzard_SocialToast`, the template chain is incomplete at frame-creation time, so the prompt frames do not inherit `hidden="true"` and start shown.

Evidence from a clean single-load probe:

- Without `Blizzard_SocialToast`, immediately after loading `Blizzard_Channels`: `prompt_shown=true`, `notif_shown=true`, both alpha `1`.
- With `Blizzard_SocialToast` included before `Blizzard_Channels`: immediately after loading `Blizzard_Channels`: `prompt_shown=false`, `notif_shown=false`, both alpha `0`.

This rules out a generic XML hidden-inheritance bug. A synthetic probe with an explicit `SocialToastTemplate -> VoiceChatPromptTemplate -> VoiceChatPromptActivateChannel` chain still started hidden as expected.

### 2. Missing real `ChatAlertFrame`

The reduced stack also omits the Blizzard chat-frame addons that define the real `ChatAlertFrame`:

- [`Interface/BlizzardUI/Blizzard_ChatFrame/Mainline/FloatingChatFrameAlertFrame.xml`](../../../Interface/BlizzardUI/Blizzard_ChatFrame/Mainline/FloatingChatFrameAlertFrame.xml)
- [`Interface/BlizzardUI/Blizzard_ChatFrameBase/Mainline/ChatAlertFrameMixin.lua`](../../../Interface/BlizzardUI/Blizzard_ChatFrameBase/Mainline/ChatAlertFrameMixin.lua)

Without those addons, the simulator falls back to the stub created in:

- [`src/lua_api/globals/global_frames.rs`](../../../src/lua_api/globals/global_frames.rs)

That stub only provides no-op alert-container methods (`AddAutoAnchoredSubSystem`, `SetSubSystemAnchorPriority`, `UpdateAnchors`). So if the voice prompt frames become visible, they are not re-anchored into the chat-alert stack and remain at the default top-level position.

## Scope

This root cause was confirmed in the reduced world-map harness, not in a fully loaded game UI stack.

Follow-up check:

- A combined-stack regression in [`tests/render_order.rs`](../../../tests/render_order.rs) now loads world-map plus chat/voice addons together, forces `VoiceChatPromptActivateChannel` to overlap `WorldMapFrame`, and verifies the prompt renders before `WorldMapFrame.BorderFrame`.
- That means the simulator currently preserves the expected major ordering in this live-like configuration: voice prompt `LOW` strata, world map border `HIGH` strata.

Additional follow-up:

- In a live-like `1024x768` layout, [`ChatFrameChannelButton`](../../../Interface/BlizzardUI/Blizzard_ChatFrame/Mainline/FloatingChatFrameVoiceChat.xml) is visible by default and its icon atlas is `chatframe-button-icon-voicechat`.
- Its bounds overlap `WorldMapFrame` horizontally (`x=2..29` versus world map starting at `x=16`) and vertically in the lower-left of the map.
- A focused regression in [`tests/render_order.rs`](../../../tests/render_order.rs) confirms that this button still renders **before** `WorldMapFrame.BorderFrame`.

Inference:

- This live-like overlap is not currently explained by wrong major z-order.
- It is better described as a layout/placement overlap: the chat voice button remains anchored to the chat button frame while the minimized world map occupies the left panel area.
- The likely reason is that `WorldMapFrame` is registered as a regular left-side panel (`RegisterUIPanel(... { area = "left", ... })`) rather than a fullscreen frame, so it does not go through the `FCF_SetFullScreenFrame()` path in [`FloatingChatFrame.lua`](../../../Interface/BlizzardUI/Blizzard_ChatFrameBase/Mainline/FloatingChatFrame.lua) that reparents/controls chat-adjacent buttons for fullscreen UIs.

Inference:

- The main simulator path in [`src/bin/wow_sim/addon_loading.rs`](../../../src/bin/wow_sim/addon_loading.rs) loads the discovered Blizzard addons instead of the hand-picked reduced list.
- The reduced harness in [`tests/render_order.rs`](../../../tests/render_order.rs) manually narrows the addon set, so it can omit prerequisites that the full game load normally has.

Current conclusion:

- The reduced harness issue was real and is now understood.
- A live/full-stack **voice prompt** render-order bug has **not** been reproduced by this investigation.
- The live-like `1024x768` overlap that is reproducible today is the chat voice button, and current evidence points to layout overlap rather than incorrect z-order.
- If a user still sees the icon above the map in a real/full simulator run, that needs a separate reproduction against the exact frame/icon involved rather than more reduced-stack reasoning.

## Practical Fix Direction

- If a reduced stack wants to load `Blizzard_Channels`, it also needs `Blizzard_SocialToast`.
- If that stack expects alert positioning to match retail, it also needs the chat-alert system (`Blizzard_ChatFrameBase` / `Blizzard_ChatFrame`) instead of the `ChatAlertFrame` stub.
- If the goal is only world-map rendering coverage, the simpler option is to avoid pulling `Blizzard_Channels` into the reduced stack unless the voice/chat prerequisites are intentionally included too.

## Sources

- [tests/render_order.rs](../../../tests/render_order.rs) — reduced world-map addon list and startup harness
- [global_frames.rs](../../../src/lua_api/globals/global_frames.rs) — `ChatAlertFrame` stub setup
- [Blizzard_Channels.toc](../../../Interface/BlizzardUI/Blizzard_Channels/Blizzard_Channels.toc) — `Blizzard_SocialToast` dependency
- [VoiceChatPrompt.xml](../../../Interface/BlizzardUI/Blizzard_Channels/VoiceChatPrompt.xml) — voice prompt frame definitions
- [SocialToast.xml](../../../Interface/BlizzardUI/Blizzard_SocialToast/SocialToast.xml) — `SocialToastTemplate hidden="true"`
- [FloatingChatFrameAlertFrame.xml](../../../Interface/BlizzardUI/Blizzard_ChatFrame/Mainline/FloatingChatFrameAlertFrame.xml) — real `ChatAlertFrame`
- [ChatAlertFrameMixin.lua](../../../Interface/BlizzardUI/Blizzard_ChatFrameBase/Mainline/ChatAlertFrameMixin.lua) — real alert positioning behavior

## See Also

- [[world-map-frame-level-rebuilds]] — separate world-map-specific investigation
- [[transparent-wrapper-render-order]] — real world-map render-order bug, unrelated to the voice alert overlay
- [[addon-loading]] — addon discovery and load-order behavior
