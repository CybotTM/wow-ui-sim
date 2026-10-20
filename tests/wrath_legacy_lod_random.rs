#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn random_original_capture_87_and_calendar_1193() {
    let env = WowLuaEnv::new().unwrap();
    let source = original_lines("Blizzard_Calendar", 87, 87);
    let consumer = original_lines("Blizzard_Calendar", 1193, 1196);
    env.exec(&format!(
        r#"
        {source}
        CALENDAR_DAYBUTTON_NORMALIZED_TEX_WIDTH = 0.5
        CALENDAR_DAYBUTTON_NORMALIZED_TEX_HEIGHT = 0.5
        {consumer}
        assert(texLeft == 0 or texLeft == 0.5)
        assert(texRight == texLeft + 0.5)
        assert(texTop == 0 or texTop == 0.5)
        assert(texBottom == texTop + 0.5)
        assert(random == math.random)
    "#
    ))
    .unwrap();
}
