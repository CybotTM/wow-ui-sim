type BootstrapFallbackOwner = (&'static str, &'static str);

const RECENTLY_MOVED_BOOTSTRAP_FALLBACKS: &[BootstrapFallbackOwner] = &[
        ("function ReloadUI", "Rust event API"),
        ("HasArtifactEquipped", "temporary inert-global workaround"),
        ("IsPVPTimerRunning", "temporary inert-global workaround"),
        (
            "GetAlternativeDefaultLanguage",
            "temporary inert-global workaround",
        ),
        ("UI_SPECIAL_FRAMES", "Rust global table surface"),
        ("UISpecialFrames =", "Rust global table surface"),
        ("StaticPopup_Show", "temporary StaticPopup workaround"),
        ("StaticPopup_Hide", "temporary StaticPopup workaround"),
        (
            "StaticPopup_AddShowCondition",
            "temporary StaticPopup workaround",
        ),
        (
            "StaticPopupDialogs = StaticPopupDialogs or {}",
            "Rust global table surface",
        ),
        (
            "__wow_ensure_glue_character_select_surface",
            "temporary client-info workaround",
        ),
        (
            "__wow_ensure_spellbook_surface",
            "temporary legacy spell workaround",
        ),
        (
            "function RegisterEventCallback",
            "temporary dispatcher callback workaround",
        ),
        (
            "function UnregisterEventCallback",
            "temporary dispatcher callback workaround",
        ),
        (
            "function RegisterUnitEventCallback",
            "temporary dispatcher callback workaround",
        ),
        (
            "function UnregisterUnitEventCallback",
            "temporary dispatcher callback workaround",
        ),
        (
            "function DevTools_AddMessageHandler",
            "temporary dispatcher callback workaround",
        ),
        (
            "__wow_ensure_dispatcher_surface",
            "temporary Dispatcher surface workaround",
        ),
        ("DISPATCHER_VERSION = 2.0", "temporary Dispatcher surface workaround"),
        ("Dispatcher = dispatcher", "temporary Dispatcher surface workaround"),
        (
            "function GetSpecializationInfoForSpecID",
            "temporary glue character-select workaround",
        ),
        (
            "function GetCharacterUndeleteStatus",
            "temporary glue character-select workaround",
        ),
        (
            "function IsCharacterTimerunning",
            "temporary glue character-select workaround",
        ),
        (
            "function ShouldShowExpansionUpgradeBanner",
            "temporary glue character-select workaround",
        ),
        (
            "function GetCharacterListGroupsInfo",
            "temporary glue character-select workaround",
        ),
        ("ChatTypeInfo =", "temporary chat-window workaround"),
        ("function GetChatWindowInfo", "temporary chat-window workaround"),
        ("function SetChatWindowShown", "temporary chat-window workaround"),
        (
            "function GetChatWindowSavedDimensions",
            "temporary chat-window workaround",
        ),
        (
            "function SetChatWindowSavedDimensions",
            "temporary chat-window workaround",
        ),
        (
            "function GetChatWindowSavedPosition",
            "temporary chat-window workaround",
        ),
        (
            "function SetChatWindowSavedPosition",
            "temporary chat-window workaround",
        ),
        (
            "function GetCameraFOVDefaults",
            "temporary camera/tutorial workaround",
        ),
        (
            "function GetTutorialsEnabled",
            "temporary camera/tutorial workaround",
        ),
        ("MoveViewOutStart", "temporary camera/tutorial workaround"),
        ("MoveViewDownStop", "temporary camera/tutorial workaround"),
        (
            "function GetPVPLifetimeStats",
            "temporary difficulty/PVP utility workaround",
        ),
        (
            "function GetModifiedClick",
            "temporary modified-click settings workaround",
        ),
        (
            "function SetModifiedClick",
            "temporary modified-click settings workaround",
        ),
        (
            "function GetLFDRoleRestrictions",
            "temporary legacy LFG workaround",
        ),
        (
            "function GetLFGRoleShortageRewards",
            "temporary legacy LFG workaround",
        ),
        ("function UnitClass", "Rust unit query owner"),
        ("function UnitRace", "Rust unit state owner"),
        ("function UnitNameUnmodified", "Rust unit query owner"),
        ("function UnitSex", "Rust unit state owner"),
        ("function UnitIsDead", "Rust unit liveness owner"),
        (
            "function RegisterUIPanel",
            "temporary UIParent panel workaround",
        ),
        (
            "function CloseAllWindows",
            "temporary UIParent panel workaround",
        ),
        (
            "__wow_ensure_chat_voice_button_surface",
            "temporary chat voice button workaround",
        ),
        (
            "function __wow_apply_chat_voice_button_surface",
            "temporary chat voice button workaround",
        ),
        (
            "function C_Map.GetBestMapForUnit",
            "temporary map runtime workaround",
        ),
        (
            "function C_Map.GetFallbackWorldMapID",
            "temporary map runtime workaround",
        ),
        (
            "function C_Map.MapHasArt",
            "temporary map runtime workaround",
        ),
        (
            "function MapUtil.GetDisplayableMapForPlayer",
            "temporary map runtime workaround",
        ),
        (
            "function MapUtil.IsShadowlandsZoneMap",
            "temporary map runtime workaround",
        ),
        (
            "function GetIconForRole",
            "temporary legacy LFG workaround",
        ),
        (
            "function GetIconForRoleEnum",
            "temporary legacy LFG workaround",
        ),
        (
            "function UnitGroupRolesAssigned",
            "temporary legacy LFG workaround",
        ),
        (
            "function UnitGroupRolesAssignedEnum",
            "temporary legacy LFG workaround",
        ),
        (
            "function GetInventoryItemID",
            "Rust inventory state probe",
        ),
        (
            "function GetInventoryItemsForSlot",
            "temporary inventory query workaround",
        ),
        (
            "function GetChatWindowChannels",
            "Rust chat-window state probe",
        ),
        (
            "function IsInventoryItemLocked",
            "Rust inventory state probe",
        ),
        (
            "ContentTrackingUtil = {}",
            "temporary content tracking workaround",
        ),
        (
            "function ContentTrackingUtil.IsContentTrackingEnabled",
            "temporary content tracking workaround",
        ),
        (
            "function ContentTrackingUtil.MakeCombinedID",
            "temporary content tracking workaround",
        ),
        (
            "function UnitHasVehiclePlayerFrameUI",
            "Rust global false stub",
        ),
        (
            "function UnitInVehicle",
            "Rust vehicle possession state",
        ),
        (
            "function UnitGetAvailableRoles",
            "temporary legacy LFG workaround",
        ),
        (
            "function IsTutorialFlagged",
            "temporary camera/tutorial workaround",
        ),
        (
            "function GetDungeonDifficultyID",
            "temporary difficulty/PVP utility workaround",
        ),
        ("function UnitThreatSituation", "Rust unit threat surface"),
        (
            "function UnitDetailedThreatSituation",
            "temporary unit threat workaround",
        ),
        (
            "function UnitThreatPercentageOfLead",
            "temporary unit threat workaround",
        ),
        ("function GetSendMailPrice", "Rust mail verb surface"),
        ("function GetMerchantFilter", "temporary merchant filter state"),
        ("function SetMerchantFilter", "temporary merchant filter state"),
        (
            "function AbbreviateNumbers",
            "temporary formatting utility workaround",
        ),
        ("function BNGetInfo", "temporary Battle.net account workaround"),
        (
            "function GetLFGDeserterExpiration",
            "temporary legacy LFG workaround",
        ),
        ("function UnitStagger", "temporary unit stagger workaround"),
        ("function GetPossessInfo", "temporary possess info workaround"),
        (
            "function StoreSecureReference",
            "temporary secure reference workaround",
        ),
        ("function IsInJailersTower", "Rust Torghast state surface"),
        (
            "function IsInventoryItemProfessionBag",
            "temporary inventory query workaround",
        ),
        (
            "function SetItemButtonTexture",
            "temporary item-button helper workaround",
        ),
        (
            "function SetItemButtonCount",
            "temporary item-button helper workaround",
        ),
        (
            "function SetItemButtonTextureVertexColor",
            "temporary item-button helper workaround",
        ),
        (
            "function SetItemButtonNormalTextureVertexColor",
            "temporary item-button helper workaround",
        ),
        (
            "TooltipDataProcessor",
            "temporary tooltip data processor workaround",
        ),
        (
            "function AddTooltipDataAccessor",
            "temporary tooltip data processor workaround",
        ),
        (
            "UIWidgetManager = UIWidgetManager",
            "temporary UI widget manager workaround",
        ),
        (
            "RegisterWidgetVisTypeTemplate = __wow_noop",
            "temporary UI widget manager workaround",
        ),
        ("Settings = Settings", "temporary Settings surface workaround"),
        (
            "function InterfaceOptions_AddCategory",
            "temporary Settings surface workaround",
        ),
        (
            "function Settings.RegisterCanvasLayoutCategory",
            "temporary Settings surface workaround",
        ),
        (
            "function GetInventoryItemTexture",
            "Rust inventory probe surface",
        ),
        (
            "function GetNumArenaOpponentSpecs",
            "temporary inert global workaround",
        ),
        ("SecureTypes = {}", "temporary secure types workaround"),
        (
            "SecureTypes.CreateSecureMap",
            "temporary secure types workaround",
        ),
        (
            "SecureTypes.CreateSecureArray",
            "temporary secure types workaround",
        ),
        (
            "__wow_securecall_accepts_names",
            "temporary secure types workaround",
        ),
        ("function GetActionInfo", "Rust action slot state surface"),
        ("function IsTrialAccount", "temporary client-info workaround"),
        (
            "function IsRestrictedAccount",
            "temporary client-info workaround",
        ),
        (
            "function IsVeteranTrialAccount",
            "temporary client-info workaround",
        ),
        (
            "function IsAccountSecured",
            "temporary client-info workaround",
        ),
        (
            "function GetFileStreamingStatus",
            "temporary client-info workaround",
        ),
        (
            "function GetBackgroundLoadingStatus",
            "temporary client-info workaround",
        ),
        ("function GetWebTicket", "temporary client-info workaround"),
        ("PlayerLocation = {}", "temporary PlayerLocation workaround"),
        (
            "function PlayerLocation:CreateFromGUID",
            "temporary PlayerLocation workaround",
        ),
        (
            "function PlayerLocation:CreateFromUnit",
            "temporary PlayerLocation workaround",
        ),
        (
            "function PlayerLocation:CreateFromCommunityChatData",
            "temporary PlayerLocation workaround",
        ),
        (
            "function PlayerLocation:CreateFromBattleNetID",
            "temporary PlayerLocation workaround",
        ),
        (
            "function PlayerLocation:CreateFromVoiceID",
            "temporary PlayerLocation workaround",
        ),
        (
            "function ChatFrameUtil.ProcessMessageEventFilters",
            "temporary chat-window workaround",
        ),
        (
            "function ChatFrameUtil.GetChatWindowName",
            "temporary chat-window workaround",
        ),
        (
            "function ChatFrameUtil.GetCommunitiesChannelColor",
            "temporary chat-window workaround",
        ),
        (
            "function ChatFrameUtil.GetCommunitiesChannelLocalID",
            "temporary chat-window workaround",
        ),
        ("function GetChannelName", "Rust channel state surface"),
        (
            "function GetCurrentEnvironment",
            "temporary debug/environment workaround",
        ),
        (
            "function SwapToGlobalEnvironment",
            "temporary debug/environment workaround",
        ),
        (
            "function CreateSecureDelegate",
            "temporary debug/environment workaround",
        ),
        (
            "function secureexecuterange",
            "temporary secureexecuterange workaround",
        ),
        (
            "function debug.getfenv",
            "temporary debug/environment workaround",
        ),
];

#[test]
fn recently_moved_startup_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for &(needle, owner) in RECENTLY_MOVED_BOOTSTRAP_FALLBACKS {
        assert!(
            !bootstrap.contains(needle),
            "{needle} fallback must live in the explicit {owner}, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} fallback must live in the explicit {owner}, not shared bootstrap"
        );
    }
}
