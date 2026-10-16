//! T4 simulator-policy contract, not a native-behavior oracle.
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn alpha_gradient_completion_uses_visible_unicode_scalars_and_can_restart() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local f = CreateFrame('Frame'):CreateFontString()
        f:SetText('|cffff0000Hé H|r\nH') -- six scalars, including space/newline
        assert(f:SetAlphaGradient(5, 30), 'still incomplete before scalar six')
        assert(not f:SetAlphaGradient(6, 30), 'completion must return false')
        assert(f:HasAlphaGradient(), 'completion retains stored gradient')
        assert(f:SetAlphaGradient(0, 30), 'replacement restarts rendering')
        f:ClearAlphaGradient()
        assert(not f:HasAlphaGradient())
        f:SetText('')
        assert(not f:SetAlphaGradient(0, 30), 'empty text completes immediately')
    "#,
    )
    .unwrap();
}
