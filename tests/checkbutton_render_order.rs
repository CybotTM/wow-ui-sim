use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn checked_texture_renders_after_normal_texture() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        local button = CreateFrame("CheckButton", "RenderOrderCheckButton", UIParent)
        button:SetSize(30, 29)
        button:SetPoint("CENTER")
        button:SetNormalTexture("Interface\\common\\minimalcheckbox")
        button:SetCheckedTexture("Interface\\common\\minimalcheckbox")
        button:SetChecked(true)
        button:Show()
        "#,
    )
    .unwrap();

    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();
    let _ = state.get_strata_buckets();

    let button_id = state
        .widgets
        .get_id_by_name("RenderOrderCheckButton")
        .unwrap();
    let button = state.widgets.get(button_id).unwrap();
    let normal_id = *button.children_keys.get("NormalTexture").unwrap();
    let checked_id = *button.children_keys.get("CheckedTexture").unwrap();

    let bucket = state
        .strata_buckets
        .as_ref()
        .unwrap()
        .iter()
        .find(|bucket| bucket.contains(&normal_id) && bucket.contains(&checked_id))
        .expect("normal and checked textures should share a render bucket");
    let normal_pos = bucket.iter().position(|id| *id == normal_id).unwrap();
    let checked_pos = bucket.iter().position(|id| *id == checked_id).unwrap();

    assert!(
        normal_pos < checked_pos,
        "checked texture must render after normal texture so it draws on top; bucket={bucket:?}"
    );
}
