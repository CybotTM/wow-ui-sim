#![cfg(feature = "client-wowforever")]

use wow_ui_sim::c_api::c_housing::HousingArea;
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_startup(env: &WowLuaEnv) {
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    crate::common::load_profile_framexml(env);
    for (name, toc) in wow_ui_sim::loader::discover_blizzard_addons(&ui) {
        wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
    }
}

fn startup() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    load_startup(&env);
    env
}

#[test]
fn forever_original_guild_menu_projects_discord_cvar() {
    let env = startup();
    env.state()
        .borrow()
        .cvars
        .register("discordClientEnabled", Some("0"));
    for enabled in [false, true] {
        assert!(
            env.state()
                .borrow()
                .cvars
                .set("discordClientEnabled", if enabled { "1" } else { "0" })
        );
        let found = env
            .eval::<bool>(
                r#"
            GuildControlUI_SetupSelected(1)
            GuildControlUI.dropdown:GenerateMenu()
            local description = GuildControlUI.dropdown:GetMenuDescription()
            for _, entry in description:EnumerateElementDescriptions() do
                if entry:GetData() == 4 then return true end
            end
            return false
            "#,
            )
            .unwrap();
        assert_eq!(found, enabled);
        assert_eq!(
            env.eval::<bool>("return C_Discord.IsEnabled()").unwrap(),
            enabled
        );
    }
}

#[test]
fn forever_original_raid_manager_projects_ping_availability() {
    let env = startup();
    env.exec(
        r#"
        CompactRaidFrameManager_OnLoad(CompactRaidFrameManager)
        local menu = CompactRaidFrameManager.displayFrame.RestrictPingsDropdown:GetMenuDescription()
        assert(not menu or not menu:HasElements())
        "#,
    )
    .unwrap();
    env.state().borrow_mut().forever_startup.ping_enabled = true;
    env.exec(
        r#"
        CompactRaidFrameManager_OnLoad(CompactRaidFrameManager)
        CompactRaidFrameManager.displayFrame.RestrictPingsDropdown:GenerateMenu()
        local menu = CompactRaidFrameManager.displayFrame.RestrictPingsDropdown:GetMenuDescription()
        assert(menu:HasElements())
        assert(C_Ping.IsPingSystemEnabled() == true)
        "#,
    )
    .unwrap();
}

#[test]
fn forever_original_bank_lock_consumer_projects_nullable_reason() {
    let env = startup();
    env.exec(
        r#"
        lockProbe = CreateFromMixins(BankPanelMixin)
        lockProbe.bankType = Enum.BankType.Character
        assert(lockProbe:IsBankTypeLocked() == false)
        assert(C_Bank.FetchBankLockedReason(Enum.BankType.Character) == nil)
        assert(select('#', C_Bank.FetchBankLockedReason(Enum.BankType.Character)) == 1)
        "#,
    )
    .unwrap();
    let bank_type = env.eval::<i32>("return Enum.BankType.Character").unwrap();
    let reason = env
        .eval::<i32>("return Enum.BankLockedReason.BankConversionFailed")
        .unwrap();
    env.state()
        .borrow_mut()
        .forever_startup
        .bank_locked_reasons
        .insert(bank_type, reason);
    env.exec(
        r#"
        assert(lockProbe:IsBankTypeLocked() == true)
        assert(C_Bank.FetchBankLockedReason(Enum.BankType.Character) == Enum.BankLockedReason.BankConversionFailed)
        assert(C_Bank.FetchBankLockedReason(Enum.BankType.Account) == nil)
        "#,
    ).unwrap();
}

#[test]
fn forever_original_tts_consumer_marks_character_settings_saved() {
    let env = startup();
    env.state()
        .borrow_mut()
        .forever_startup
        .character_tts_settings_saved = false;
    env.exec(
        r#"
        assert(C_TTSSettings.GetCharacterSettingsSaved() == false)
        TextToSpeechFrame.loaded = false
        TextToSpeechFrame.loadedEvents = {
            PLAYER_ENTERING_WORLD = true,
            VARIABLES_LOADED = true,
            ADDON_LOADED = true,
        }
        TextToSpeechFrame_CheckLoad(TextToSpeechFrame)
        assert(C_TTSSettings.GetCharacterSettingsSaved() == true)
        assert(select('#', C_TTSSettings.MarkCharacterSettingsSaved()) == 0)
        "#,
    )
    .unwrap();
    assert!(
        env.state()
            .borrow()
            .forever_startup
            .character_tts_settings_saved
    );
}

#[test]
fn forever_original_game_housing_consumer_reads_spatial_state() {
    let env = startup();
    for area in [
        None,
        Some(HousingArea::HouseInterior),
        Some(HousingArea::PlotExterior),
    ] {
        env.state().borrow_mut().housing.current_housing_area = area;
        assert_eq!(
            env.eval::<bool>("return C_Housing.IsInsideHouseOrPlot()")
                .unwrap(),
            area.is_some()
        );
        env.exec("assert(GameEvent.HandlePlayerEnteringWorld(nil, 'PLAYER_ENTERING_WORLD', false, false) == true)").unwrap();
    }
    assert!(
        env.eval::<bool>("return C_AddOns.IsAddOnLoaded('Blizzard_HousingControls')")
            .unwrap()
    );
}

#[test]
fn forever_original_gamepad_core_removes_temporary_cvars() {
    let env = WowLuaEnv::new().unwrap();
    env.state()
        .borrow_mut()
        .forever_startup
        .temporary_cvars
        .insert("softtargetenemy".to_owned(), "7".to_owned());
    load_startup(&env);
    assert!(
        env.state()
            .borrow()
            .forever_startup
            .temporary_cvars
            .is_empty()
    );
    let saved = env.state().borrow().cvars.get("SoftTargetEnemy");
    env.state()
        .borrow_mut()
        .forever_startup
        .temporary_cvars
        .insert("softtargetenemy".to_owned(), "9".to_owned());
    assert_eq!(
        env.eval::<String>("return GetCVar('SoftTargetEnemy')")
            .unwrap(),
        "9"
    );
    assert_eq!(
        env.eval::<String>("return C_CVar.GetCVar('SoftTargetEnemy')")
            .unwrap(),
        "9"
    );
    let core = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path()
        .unwrap()
        .join("Blizzard_Gamepad/Core.lua");
    env.exec(&std::fs::read_to_string(core).unwrap()).unwrap();
    assert_eq!(
        env.eval::<Option<String>>("return GetCVar('SoftTargetEnemy')")
            .unwrap(),
        saved
    );
    assert!(
        env.state()
            .borrow()
            .forever_startup
            .temporary_cvars
            .is_empty()
    );
    assert_eq!(env.state().borrow().cvars.get("SoftTargetEnemy"), saved);
}

#[test]
fn forever_original_social_consumer_accepts_raid_status_event() {
    let env = startup();
    env.exec(
        r#"
        local frame = CreateFrame('Frame')
        frame:RegisterEvent('SOCIAL_UI_RAID_LIST_SYSTEM_STATUS_UPDATED')
        assert(frame:IsEventRegistered('SOCIAL_UI_RAID_LIST_SYSTEM_STATUS_UPDATED'))
        "#,
    )
    .unwrap();
    assert!(
        !env.state()
            .borrow()
            .lua_errors
            .iter()
            .any(|error| error.contains("SOCIAL_UI_RAID_LIST_SYSTEM_STATUS_UPDATED"))
    );
}
