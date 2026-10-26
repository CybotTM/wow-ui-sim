//! Wrath's original container frame projects supplied bag names.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_full_ui.rs"]
mod wrath_full_ui;

#[test]
fn original_container_frame_displays_changing_bag_name() {
    let env = wrath_full_ui::load_original_wrath_startup();
    {
        let mut state = env.state().borrow_mut();
        let first = state.bag_info.get_mut(&1).unwrap();
        first.name = Some("Travel Pack".into());
        first.item_id = Some(6948);
        let second = state.bag_info.get_mut(&2).unwrap();
        second.name = Some("Herb Satchel".into());
        second.item_id = Some(109799);
    }
    env.exec(
        r#"
        CloseAllBags()
        ToggleBag(1)
        local frame = _G["ContainerFrame" .. IsBagOpen(1)]
        assert(_G[frame:GetName() .. "Name"]:GetText() == "Travel Pack")
        assert(_G[frame:GetName() .. "Portrait"]:GetTexture() == 134414)
        CloseAllBags()
        "#,
    )
    .expect("original Wrath container must display the first supplied bag name");

    env.exec(
        r#"
        ToggleBag(2)
        local frame = _G["ContainerFrame" .. IsBagOpen(2)]
        assert(_G[frame:GetName() .. "Name"]:GetText() == "Herb Satchel")
        assert(_G[frame:GetName() .. "Portrait"]:GetTexture() == 960145)
        "#,
    )
    .expect("original Wrath container must display the changed bag name");
}
