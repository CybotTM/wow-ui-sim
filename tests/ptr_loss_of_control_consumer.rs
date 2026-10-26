//! Original PTR LossOfControlFrame proof after Edit Mode display-info initialization.
#![cfg(feature = "client-ptr")]

use crate::common::panel_fixtures::setup_env;
use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_original_consumer_selection() {
    use wow_ui_sim::loader::discover_blizzard_addon_closure_for_screen_with_overrides;
    use wow_ui_sim::screen::ScreenKind;
    use wow_ui_sim::toc::TocFile;

    let root = crate::common::panel_fixtures::blizzard_ui_dir();
    let addons = discover_blizzard_addon_closure_for_screen_with_overrides(
        &root,
        ScreenKind::Game,
        &["Blizzard_FrameXML"],
        &[],
    );
    let (_, toc_path) = addons
        .iter()
        .find(|(name, _)| name == "Blizzard_FrameXML")
        .unwrap();
    let toc = TocFile::from_file(toc_path).unwrap();
    // PTR FrameXML.toc:135-137 selects the mainline Lua/XML, not the Mists override.
    for relative in ["LossOfControlFrame.lua", "LossOfControlFrame.xml"] {
        assert!(
            toc.files
                .iter()
                .any(|path| path == std::path::Path::new(relative))
        );
    }
}

fn load_original_consumer() -> WowLuaEnv {
    assert_original_consumer_selection();
    let env = setup_env();
    env.exec(
        r#"
        assert(LossOfControlFrame, 'original selected FrameXML consumer must load')
        assert(type(LossOfControlFrame.UpdateDisplay) == 'function')
        assert(LossOfControlFrame.TimeLeft.secondsWidth ==
            LossOfControlFrame.TimeLeft.SecondsText:GetStringWidth(), 'original OnLoad completed')
        -- PTR LossOfControlConstantsDocumentation.lua:10 declares active index 1.
        Constants.LossOfControlConsts = { LOSS_OF_CONTROL_ACTIVE_INDEX = 1 }
        LossOfControlFrame.isInEditMode = false
        "#,
    )
    .unwrap();
    env
}

fn assert_empty_original_display(env: &WowLuaEnv) {
    env.exec(
        r#"
        -- Start shown so an empty-state no-op cannot satisfy the hide assertion.
        LossOfControlFrame:Show()
        LossOfControlFrame:UpdateDisplay()
        assert(not LossOfControlFrame:IsShown(), 'fresh original consumer must hide without LOC')
        "#,
    )
    .expect("original PTR UpdateDisplay:230-240 must consume empty LOC state and hide");
}

fn supply_player_effect(env: &WowLuaEnv) {
    env.exec(
        r#"
        A_Admin.AddLossOfControl('player', {
            locType = 'STUN', spellID = 408, displayText = 'Kidney Shot',
            iconTexture = 132298, timeRemaining = 4, duration = 4,
            lockoutSchool = 0, priority = 1, displayType = 2,
        })
        assert(C_LossOfControl.GetActiveLossOfControlDataCount() == 1)
        "#,
    )
    .unwrap();
}

fn assert_supplied_original_display(env: &WowLuaEnv) {
    env.exec(
        r#"
        LossOfControlFrame:UpdateDisplay()
        assert(LossOfControlFrame:IsShown(), 'supplied entry must show original consumer')
        assert(LossOfControlFrame.AbilityName:GetText() == 'Kidney Shot')
        assert(LossOfControlFrame.Icon:GetTexture() == 132298)
        assert(LossOfControlFrame.spellID == 408 and LossOfControlFrame.priority == 1)
        assert(LossOfControlFrame.TimeLeft.timeRemaining == 4)
        "#,
    )
    .expect("original PTR SetUpDisplay:149-208 must present supplied fields");
}

#[test]
fn original_display_tracks_supplied_state() {
    let env = load_original_consumer();
    assert_empty_original_display(&env);
    supply_player_effect(&env);
    assert_supplied_original_display(&env);
    env.exec(
        r#"
        A_Admin.ClearLossOfControl('player')
        assert(C_LossOfControl.GetActiveLossOfControlDataCount() == 0)
        LossOfControlFrame:UpdateDisplay()
        assert(not LossOfControlFrame:IsShown(), 'removed entry must hide original consumer')
        "#,
    )
    .unwrap();
}

#[test]
fn original_displays_are_environment_local() {
    let first = load_original_consumer();
    let second = load_original_consumer();
    assert_empty_original_display(&first);
    assert_empty_original_display(&second);
    supply_player_effect(&first);
    assert_supplied_original_display(&first);
    assert_empty_original_display(&second);
    supply_player_effect(&second);
    first.exec("A_Admin.ClearLossOfControl('player')").unwrap();
    assert_empty_original_display(&first);
    assert_supplied_original_display(&second);
    second.exec("A_Admin.ClearLossOfControl('player')").unwrap();
    assert_empty_original_display(&second);
}
