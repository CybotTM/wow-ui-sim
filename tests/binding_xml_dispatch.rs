use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn xml_binding_body_runs_for_named_action_and_shadows_hardcoded_action() {
    let directory = tempfile::tempdir().expect("temporary addon directory");
    std::fs::write(
        directory.path().join("Bindings.xml"),
        r#"<Bindings>
            <Binding name="TOGGLESPELLBOOK">BindingCount = (BindingCount or 0) + 1</Binding>
        </Bindings>"#,
    )
    .expect("write binding file");
    let toc = directory.path().join("Test.toc");
    std::fs::write(&toc, "Bindings.xml\n").expect("write addon TOC");
    let env = WowLuaEnv::new().expect("create Lua environment");
    load_addon(&env.loader_env(), &toc).expect("load addon");
    env.exec("assert(SetBinding('S', 'TOGGLESPELLBOOK'))")
        .expect("bind action");

    env.send_key_press("S", None).expect("first key press");
    env.send_key_press("S", None).expect("second key press");
    env.exec("assert(BindingCount == 2)")
        .expect("XML body runs for both key presses");
}

#[test]
fn xml_binding_runs_on_key_up_only_when_declared() {
    let directory = tempfile::tempdir().expect("temporary addon directory");
    std::fs::write(
        directory.path().join("Bindings.xml"),
        r#"<Bindings>
            <Binding name="TEST_EDGES" runOnUp="true">EdgeTrace = (EdgeTrace or '') .. keystate .. ','</Binding>
            <Binding name="TEST_DOWN">DownTrace = (DownTrace or '') .. keystate .. ','</Binding>
        </Bindings>"#,
    )
    .expect("write binding file");
    let toc = directory.path().join("Test.toc");
    std::fs::write(&toc, "Bindings.xml\n").expect("write addon TOC");
    let env = WowLuaEnv::new().expect("create Lua environment");
    load_addon(&env.loader_env(), &toc).expect("load addon");
    env.exec("assert(SetBinding('H', 'TEST_EDGES')); assert(SetBinding('G', 'TEST_DOWN'))")
        .expect("bind XML actions");

    env.send_key_down("H", None).expect("press edge key");
    env.exec("assert(EdgeTrace == 'down,')")
        .expect("down state is passed to XML body");
    env.send_key_up("H");
    env.exec("assert(EdgeTrace == 'down,up,')")
        .expect("runOnUp dispatches up state");
    env.send_key_press("G", None).expect("press down-only key");
    env.exec("assert(DownTrace == 'down,')")
        .expect("binding without runOnUp runs once");
}
