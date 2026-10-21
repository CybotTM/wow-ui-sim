//! Native 3.3.5 FrameXML fixture: no modern autocomplete libraries or adapters.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state_types::SocialFriend;

pub fn assert_native(probe: &'static str) {
    super::with_perf_lock(|| {
        super::with_timeout(90, move || {
            let env = WowLuaEnv::new().expect("native autocomplete environment");
            env.set_screen_size(1024.0, 768.0);
            super::panel_fixtures::seed_addon_search_paths(&env);
            super::load_profile_framexml(&env);
            seed_completion_names(&env);
            env.exec(EDITBOX_LUA).expect("native editbox fixture");
            env.exec(probe).expect("native Wrath autocomplete consumer");
            let errors = super::panel_fixtures::recorded_lua_errors(&env);
            assert!(errors.is_empty(), "native FrameXML Lua errors: {errors:?}");
        });
    });
}

fn seed_completion_names(env: &WowLuaEnv) {
    let names = ["F09Alpha", "F09Beta", "F09Gamma", "F09Delta", "F09Epsilon", "F09Zeta"];
    env.state().borrow_mut().social_friends = names
        .into_iter()
        .enumerate()
        .map(|(index, name)| SocialFriend {
            name: name.into(),
            level: 80,
            area: "Dalaran".into(),
            class_name: "Paladin".into(),
            note: String::new(),
            is_online: true,
            guid: format!("Player-1-F09000{index:02X}"),
        })
        .collect();
}

const EDITBOX_LUA: &str = r#"
local editBox = CreateFrame("EditBox", "F09EditBox", UIParent, "AutoCompleteEditBoxTemplate")
editBox:SetSize(200, 20)
editBox:SetPoint("BOTTOMLEFT", UIParent, "BOTTOMLEFT", 200, 400)
editBox.autoCompleteParams = AUTOCOMPLETE_LIST.ALL
editBox:SetText("F09")
editBox:SetCursorPosition(3)
AutoCompleteBox:Hide()
AutoCompleteBox.parent = editBox
AutoComplete_SetSelectedIndex(AutoCompleteBox, 0)
"#;
