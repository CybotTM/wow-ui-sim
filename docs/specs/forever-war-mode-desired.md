# Forever War Mode desired

Forever `Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:1134` declares `C_PvP.IsWarModeDesired` with one non-null boolean result. `Blizzard_MicroMenu/Mainline/MainMenuBarMicroButtons.lua:811` uses that result to choose whether to query battlefield state.

The query returns the simulator's supplied War Mode desired state. Its initial false value is existing simulator scenario policy; the native Forever default is unknown. This does not publish the War Mode reward query on Forever.

`tests/forever_war_mode_desired.rs` exercises the selected micro-menu consumer with both desired values after a specialization event.
