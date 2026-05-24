#[test]
fn recently_moved_startup_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for (needle, owner) in [
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
    ] {
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
