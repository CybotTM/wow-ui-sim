//! Wrath's original container frame projects supplied bag names.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_full_ui.rs"]
mod wrath_full_ui;

#[test]
fn original_container_frame_displays_changing_bag_name() {
    let env = wrath_full_ui::load_original_wrath_startup();
    env.state().borrow_mut().bag_info.get_mut(&1).unwrap().name = Some("Travel Pack".into());
    env.exec(
        r#"
        CloseAllBags()
        ToggleBag(1)
        local frame = _G["ContainerFrame" .. IsBagOpen(1)]
        assert(_G[frame:GetName() .. "Name"]:GetText() == "Travel Pack")
        CloseAllBags()
        "#,
    )
    .expect("original Wrath container must display the first supplied bag name");

    env.state().borrow_mut().bag_info.get_mut(&1).unwrap().name = Some("Herb Satchel".into());
    env.exec(
        r#"
        ToggleBag(1)
        local frame = _G["ContainerFrame" .. IsBagOpen(1)]
        assert(_G[frame:GetName() .. "Name"]:GetText() == "Herb Satchel")
        "#,
    )
    .expect("original Wrath container must display the changed bag name");
}
