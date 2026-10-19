//! Wrath12340 history reports, reconstruction0e4e9fe0; not native-run.
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
fn b6_c5_worked_physical_prefix_example() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(3);
    env.exec("for _,v in ipairs({'A','B','C','D'}) do H:AddHistoryLine(v) end; H:SetHistoryLines(2); H:SetText('draft')").unwrap();
    for expected in ["D", "B", "D"] {
        recall(&env, "UP", expected);
    }
}

#[test]
fn b6_c5_default_zero_disables_and_duplicates_remain() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(0);
    env.exec("assert(H:GetHistoryLines()==0); H:AddHistoryLine('discard'); H:SetText('draft')")
        .unwrap();
    recall(&env, "UP", "draft");
    recall(&env, "DOWN", "draft");
    env.exec(
        "H:SetHistoryLines(3); H:AddHistoryLine('X'); H:AddHistoryLine('X'); H:AddHistoryLine('Y')",
    )
    .unwrap();
    for expected in ["Y", "X", "X", "Y"] {
        recall(&env, "UP", expected);
    }
}

#[test]
fn b6_c5_first_down_reads_a_first_up_reads_c() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    for (key, expected) in [("DOWN", "A"), ("UP", "C")] {
        let env = history_env(4);
        env.exec("assert(H:GetHistoryLines()==4); H:AddHistoryLine('A'); H:AddHistoryLine('B'); H:AddHistoryLine('C'); H:SetText('draft')").unwrap();
        recall(&env, key, expected);
    }
}

#[test]
fn b6_c5_clear_keeps_capacity_display_and_resets_shared_index() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(4);
    env.exec("H:AddHistoryLine('A'); H:AddHistoryLine('B'); H:AddHistoryLine('C'); H:SetText('draft'); H:ClearHistory(); assert(H:GetHistoryLines()==4 and H:GetText()=='draft')").unwrap();
    recall(&env, "UP", "draft");
    recall(&env, "DOWN", "draft");
    env.exec("H:AddHistoryLine('X'); H:SetHistoryLines(2)")
        .unwrap();
    recall(&env, "UP", "X");
}

#[test]
fn b6_c5_empty_slots_skip_and_growth_preserves_slots() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(2);
    env.exec(
        "H:AddHistoryLine('A'); H:AddHistoryLine('B'); H:SetHistoryLines(4); H:SetText('draft')",
    )
    .unwrap();
    recall(&env, "UP", "B");
    recall(&env, "DOWN", "A");
    env.exec("H:ClearHistory(); H:AddHistoryLine('only')")
        .unwrap();
    for key in ["UP", "UP", "DOWN"] {
        recall(&env, key, "only");
    }
}

#[test]
fn b6_c5_lua_setter_conversion_and_rejected_inputs_are_atomic() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    for input in [
        "0",
        "-1",
        "0.7",
        "'not-a-number'",
        "nil",
        "false",
        "setmetatable({}, {__tostring=function() return '4' end})",
    ] {
        let env = history_env(4);
        env.exec("H:AddHistoryLine('A'); H:AddHistoryLine('B'); H:AddHistoryLine('C')")
            .unwrap();
        env.exec(&format!("local ok,err=pcall(H.SetHistoryLines,H,{input}); assert(not ok); assert(string.find(tostring(err),'Usage: B6History:SetHistoryLines(numLines)',1,true)); assert(H:GetHistoryLines()==4)")).unwrap();
        recall(&env, "DOWN", "A");
        recall(&env, "UP", "C");
    }
    let env = history_env(4);
    env.exec("H:SetHistoryLines(2.7); assert(H:GetHistoryLines()==2); H:SetHistoryLines('4'); assert(H:GetHistoryLines()==4); local b=CreateFrame('EditBox'); local ok,err=pcall(b.SetHistoryLines,b,0); assert(not ok and string.find(tostring(err),'Usage: <unnamed>:SetHistoryLines(numLines)',1,true))").unwrap();
}

#[test]
fn b6_c5_shrink_clamps_index_and_next_append_overwrites_physical_b() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(4);
    env.exec(
        "H:AddHistoryLine('A'); H:AddHistoryLine('B'); H:AddHistoryLine('C'); H:SetHistoryLines(2)",
    )
    .unwrap();
    recall(&env, "UP", "A");
    recall(&env, "DOWN", "B");
    env.exec("H:AddHistoryLine('D')").unwrap();
    recall(&env, "UP", "D");
    recall(&env, "DOWN", "A");
}

#[test]
fn b6_c5_recall_obeys_alt_admission_without_arrow_script() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = history_env(2);
    env.exec("H:AddHistoryLine('A'); H:AddHistoryLine('B'); H:SetText('draft'); assert(select('#',H:HasScript('OnArrowPressed'))==1 and H:HasScript('OnArrowPressed')==nil)").unwrap();
    recall(&env, "UP", "B");
    env.exec("H:SetAltArrowKeyMode(true)").unwrap();
    recall(&env, "UP", "B");
    recall(&env, "ALT-UP", "A");
}

#[test]
fn b6_c5_xml_history_capacity_clamps_nonpositive_before_onload() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("History.xml"), r#"<Ui>
        <EditBox name="B6HistoryTemplate" virtual="true" historyLines="4"><Scripts><OnLoad>self.capacityAtLoad=self:GetHistoryLines()</OnLoad></Scripts></EditBox>
        <EditBox name="B6HistoryPositive" inherits="B6HistoryTemplate"/>
        <EditBox name="B6HistoryZero" inherits="B6HistoryTemplate" historyLines="0"/>
        <EditBox name="B6HistoryNegative" inherits="B6HistoryTemplate" historyLines="-3"/>
    </Ui>"#).unwrap();
    let toc = dir.path().join("History.toc");
    std::fs::write(&toc, "## Title: History attributes\nHistory.xml\n").unwrap();
    let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(loaded.warnings.is_empty(), "SETUP: {:?}", loaded.warnings);
    env.exec("assert(B6HistoryPositive:GetHistoryLines()==4 and B6HistoryPositive.capacityAtLoad==4); assert(B6HistoryZero:GetHistoryLines()==0 and B6HistoryZero.capacityAtLoad==0); assert(B6HistoryNegative:GetHistoryLines()==0 and B6HistoryNegative.capacityAtLoad==0)").unwrap();
}
