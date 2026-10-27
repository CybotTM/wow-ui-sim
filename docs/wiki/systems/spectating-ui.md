# Spectating UI scenario

`C_SpectatingUI.IsSpectating` reads the per-environment `SimState.spectating` fixture. Its initial false value is a user-authorized scenario default, not a native observation. The [contract](../../specs/spectating-ui.md) excludes spectating lifecycle and the other namespace members.

## Evidence

The pinned Retail, PTR, and WowForever UI caches call the namespace. The other four profile caches have no calls. Retail and PTR generated API documentation contains no SpectatingUI declaration, so the native signature and default are not established by it. Retail `Blizzard_Game_Mainline.toc:6–7,15` selects `Mainline/Game.lua` for the game/mainline startup. The handler at line 29 short-circuits before `ClearTarget` when spectating is true.

The existing explicit Enter-to-game-menu test previously failed at that handler because the namespace was absent. The two-value test exercises the same binding with a supplied target. Verification is pending.

## Sources

- [Contract](../../specs/spectating-ui.md)
- [Consumer tests](../../../tests/chat_enter_binding.rs)
- Profile-scoped pinned Blizzard UI caches, `Blizzard_Game/Mainline/Game.lua` and Retail `Blizzard_Game_Mainline.toc`.

## See Also

- [Consumer selection and RED proof](../investigations/consumer-selection-and-red-proof.md)
