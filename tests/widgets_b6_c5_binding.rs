//! Wrath12340 AddHistoryLine binding; null-conversion inputs stay unasserted.
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

fn history_env(capacity: usize) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec("H=CreateFrame('EditBox','B6History',UIParent); H:SetAutoFocus(false); H:SetMultiLine(false); H:SetAltArrowKeyMode(false); H:SetSize(300,40); H:Show(); H:SetFocus()").unwrap();
    if capacity > 0 {
        env.exec(&format!("H:SetHistoryLines({capacity})")).unwrap();
    }
    env
}

fn recall(env: &WowLuaEnv, key: &str, expected: &str) {
    env.send_key_press(key, None).unwrap();
    assert_eq!(env.eval::<String>("return H:GetText()").unwrap(), expected);
}

#[test]
fn b6_c5_binding_stored_empty_string_is_recalled_and_advances_index() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(2);
    env.exec("H:AddHistoryLine(''); H:AddHistoryLine('X'); H:SetText('draft')")
        .unwrap();
    recall(&env, "UP", "X");
    recall(&env, "UP", "");
    recall(&env, "DOWN", "X");
    recall(&env, "DOWN", "");
}

#[test]
fn b6_c5_binding_zero_capacity_does_not_store_or_advance() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(0);
    env.exec("H:AddHistoryLine('discard'); assert(H:GetHistoryLines()==0); H:SetHistoryLines(2); H:SetText('draft')").unwrap();
    recall(&env, "UP", "draft");
    recall(&env, "DOWN", "draft");
    env.exec("H:AddHistoryLine('A'); H:SetHistoryLines(1)")
        .unwrap();
    recall(&env, "UP", "A");
}

#[test]
fn b6_c5_binding_exact_arity_rejection_is_atomic() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    for arguments in ["", ", 'extra', 'second'"] {
        let env = history_env(2);
        env.exec("H:AddHistoryLine('A'); H:AddHistoryLine('B'); H:SetText('draft')")
            .unwrap();
        env.exec(&format!(
            r#"
            local ok,err=pcall(H.AddHistoryLine,H{arguments})
            assert(not ok)
            assert(string.find(tostring(err),'Usage: B6History:AddHistoryLine("text")',1,true))
            assert(H:GetHistoryLines()==2)
        "#
        ))
        .unwrap();
        recall(&env, "UP", "B");
    }
    let env = history_env(2);
    env.exec(r#"
        local b=CreateFrame('EditBox')
        local ok,err=pcall(b.AddHistoryLine,b)
        assert(not ok and string.find(tostring(err),'Usage: <unnamed>:AddHistoryLine("text")',1,true))
    "#).unwrap();
}

#[test]
fn b6_c5_binding_number_is_converted_to_string() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(2);
    env.exec("H:AddHistoryLine(42); H:SetText('draft')")
        .unwrap();
    recall(&env, "UP", "42");
}
