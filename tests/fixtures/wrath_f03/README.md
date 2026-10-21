# Native Wrath panel fixtures

Window/child identities, geometry and free-function entry points in these fixtures are **INFERRED** from the committed shipped FrameXML inventory, not native-client execution. The shared loader reads the active profile's FrameXML TOC in order before dependent consumers. Retail keeps its existing fixture path.

| Shared behavior | Wrath consumer / identity |
|---|---|
| Character equipment/title | `ToggleCharacter`, `CharacterNameText`, globally named slot `IconTexture` children |
| Spellbook/talents | `ToggleSpellBook` / `SpellBookFrame`; `ToggleTalentFrame` / `PlayerTalentFrame` |
| Guild | `ToggleFriendsFrame(3)`, native rank dropdown and global guild information children |
| Mail/macros/socketing | `MAIL_SHOW` consumer; `ShowMacroFrame`; `ItemSocketingFrame_OnEvent` |
| Options/reputation | Native interface/audio dropdowns; third character tab and named FauxScrollFrame/detail children |

Modern Communities streams, menu-description objects, guild-control category dropdowns and assignment-only rank exclusion have no established equivalent in the native corpus. Their original tests remain registered; report-only classification does not turn a failure into a pass. Shared guild MOTD/details assertions remain; modern guild-challenge rows are a separate unsupported target.

`wrath_guild_rank_flags.rs` additionally tests the laptop-authenticated build 12340 positional flag contract through the original `GuildControlCheckboxUpdate` consumer. Unselected-rank native behavior is undefined and is not asserted.
