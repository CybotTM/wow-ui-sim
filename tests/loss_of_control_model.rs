use crate::common::panel_fixtures::setup_env;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn loss_of_control_fresh_world_is_empty() {
    let env = WowLuaEnv::new().unwrap();
    if cfg!(feature = "client-wrath") {
        env.exec("assert(C_LossOfControl == nil, 'Wrath has no declared LOC namespace')")
            .unwrap();
        return;
    }
    env.exec(
        r#"
        assert(C_LossOfControl.GetActiveLossOfControlDataCount() == 0)
        assert(C_LossOfControl.GetActiveLossOfControlDataCountByUnit('player') == 0)
        assert(C_LossOfControl.GetActiveLossOfControlData(1) == nil)
        assert(C_LossOfControl.GetActiveLossOfControlDataByUnit('player', 1) == nil)
        assert(C_LossOfControl.GetActiveLossOfControlDuration('player', 1) == nil)
        "#,
    )
    .unwrap();
}

#[test]
fn loss_of_control_original_display_tracks_supplied_state() {
    if cfg!(feature = "client-wrath") {
        return;
    }
    let env = setup_env();
    env.exec(
        r#"
        assert(LossOfControlFrame, 'original selected FrameXML consumer must load')
        assert(type(LossOfControlFrame.UpdateDisplay) == 'function')
        Constants.LossOfControlConsts = { LOSS_OF_CONTROL_ACTIVE_INDEX = 1 }
        LossOfControlFrame.isInEditMode = false
        LossOfControlFrame:UpdateDisplay()
        assert(not LossOfControlFrame:IsShown(), 'fresh original consumer must hide without LOC')
        A_Admin.AddLossOfControl('player', {
            locType = 'STUN', spellID = 408, displayText = 'Kidney Shot',
            iconTexture = 132298, timeRemaining = 4, duration = 4,
            lockoutSchool = 0, priority = 1, displayType = 2,
        })
        assert(C_LossOfControl.GetActiveLossOfControlDataCount() == 1)
        LossOfControlFrame:UpdateDisplay()
        assert(LossOfControlFrame:IsShown(), 'supplied entry must show original consumer')
        assert(LossOfControlFrame.AbilityName:GetText() == 'Kidney Shot')
        assert(LossOfControlFrame.Icon:GetTexture() == 132298)
        A_Admin.ClearLossOfControl('player')
        assert(C_LossOfControl.GetActiveLossOfControlDataCount() == 0)
        LossOfControlFrame:UpdateDisplay()
        assert(not LossOfControlFrame:IsShown(), 'removed entry must hide original consumer')
        "#,
    )
    .unwrap();
}
