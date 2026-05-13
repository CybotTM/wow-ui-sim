# Mists Load-On-Demand Addon Audit

Captured 2026-05-12 on `classic-profile-rollout`.

This audit accounts for every Blizzard addon directory under
`Interface/BlizzardUI/Mists/AddOns` that has at least one `.toc` file with
`## LoadOnDemand: 1`. User-facing frames outside the prior panel matrix were
promoted into `docs/baselines/mists-panels.md` and
`scripts/mists-panel-parity.sh`.

| Addon | Classification | Coverage |
|---|---|---|
| Blizzard_APIDocumentation | Support/debug | Not a player-facing Mists panel. |
| Blizzard_APIDocumentationGenerated | Support/debug | Data for API documentation. |
| Blizzard_AccountSaveUI | Retail/account surface | Not a Mists in-game panel target. |
| Blizzard_AchievementUI | Covered | `Achievements and Calendar`. |
| Blizzard_ArchaeologyUI | Added | `Archaeology panel`. |
| Blizzard_ArenaUI | Added | `Arena enemy unit frames LoD panel`. |
| Blizzard_AuctionHouseUI | Covered | `Auction House: browse, bid, post, cancel`. |
| Blizzard_AuctionUI | Covered | Legacy auction path in `Auction House: browse, bid, post, cancel`. |
| Blizzard_AutoCompletePopupList | Support | Autocomplete popup support, not standalone panel. |
| Blizzard_BarbershopUI | Added | `NPC service LoD panels: barber and black market`. |
| Blizzard_BattlefieldMap | Added | `Battlefield map LoD panel`. |
| Blizzard_BehavioralMessaging | Support | Toast/message system loaded by events, not standalone panel. |
| Blizzard_BindingUI | Covered | `Macro and key bindings`. |
| Blizzard_BlackMarketUI | Added | `NPC service LoD panels: barber and black market`. |
| Blizzard_Calendar | Covered | `Achievements and Calendar`. |
| Blizzard_ChallengesUI | Added | `Challenge mode LoD panel`. |
| Blizzard_Collections | Covered | `Collections: mounts, pets, toys, heirlooms, transmog` and `Pet Journal and Battle Pet UI`. |
| Blizzard_CombatLog | Covered by startup/chat surface | Chat/combat log support, not standalone panel. |
| Blizzard_CombatText | Covered by startup surface | Floating combat text support, not standalone panel. |
| Blizzard_Commentator | Out of Mists panel scope | Commentator/broadcast surface, not normal player UI parity. |
| Blizzard_CraftUI | Gap task added | `CraftFrame_LoadUI()` is present, but `CraftFrame` is not created under the Mists runtime path; needs a focused LoD construction fix before entering the panel matrix. |
| Blizzard_CustomizationUI | Retail/account surface | Not a Mists in-game panel target. |
| Blizzard_DebugTools | Support/debug | Developer/debug UI, not player panel parity. |
| Blizzard_Dispatcher | Support | Dispatcher support, not standalone panel. |
| Blizzard_EncounterJournal | Covered | `Encounter Journal`. |
| Blizzard_EngravingUI | Non-Mists feature surface | Season of Discovery/rune surface, not Pandaria panel parity. |
| Blizzard_EventTrace | Support/debug | Developer event trace, not player panel parity. |
| Blizzard_GMChatUI | Support/service | GM chat surface; covered indirectly by help/support flows, not standalone panel. |
| Blizzard_GlyphUI | Covered | `Talents and glyphs`. |
| Blizzard_GroupFinder_VanillaStyle | Covered | `LFG, LFR, Raid Browser`. |
| Blizzard_GuildBankUI | Covered | `Bank, ReagentBank, Void Storage, Guild Bank`. |
| Blizzard_GuildControlUI | Added | `Inspect and guild control LoD panels`. |
| Blizzard_InspectUI | Added | `Inspect and guild control LoD panels`. |
| Blizzard_ItemSocketingUI | Added | `Item socketing, reforging, and upgrade LoD panels`. |
| Blizzard_ItemUpgradeUI | Added | `Item socketing, reforging, and upgrade LoD panels`. |
| Blizzard_Kiosk | Support | Kiosk mode guard surface, not standalone panel. |
| Blizzard_MacroUI | Covered | `Macro and key bindings`. |
| Blizzard_MapCanvas | Covered as dependency | `World map` and `Challenge mode LoD panel`. |
| Blizzard_MovePad | Added | `Time manager and move pad LoD utilities`. |
| Blizzard_PVPUI | Covered | `PvP UI: HonorFrame, BG queue, Conquest`. |
| Blizzard_QuestChoice | Added | `Quest choice LoD dialog`. |
| Blizzard_RaidUI | Added | `Raid unit frames LoD panel`. |
| Blizzard_ReforgingUI | Added | `Item socketing, reforging, and upgrade LoD panels`. |
| Blizzard_RemixArtifactTutorialUI | Non-Mists feature surface | Remix tutorial surface, not Pandaria panel parity. |
| Blizzard_SharedMapDataProviders | Covered as dependency | `World map` and `Challenge mode LoD panel`. |
| Blizzard_SpellSearch | Covered as dependency | `Spellbook and professions`. |
| Blizzard_StatusUI | Support | Status/dialog support dependency, not standalone panel. |
| Blizzard_TalentUI | Covered | `Talents and glyphs`. |
| Blizzard_TimeManager | Added | `Time manager and move pad LoD utilities`. |
| Blizzard_TradeSkillUI | Gap task added | TradeSkillFrame loads, but the legacy Mists frame still needs `GetTradeSkill*` backing globals before it can enter the panel matrix. |
| Blizzard_TrainerUI | Gap task added | ClassTrainerFrame loads, but the Mists runtime is still missing legacy trainer-service globals and template wiring before it can enter the panel matrix. |
| Blizzard_WowSurveyUI | Support/service | Survey prompt surface, not normal panel parity. |
