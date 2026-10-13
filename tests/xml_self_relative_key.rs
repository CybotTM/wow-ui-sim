#![cfg(feature = "client-mists")]

use std::path::Path;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{self, UiXml, XmlElement};

fn parse_cached_xml(path: &Path) -> UiXml {
    xml::parse_xml_file(path)
        .unwrap_or_else(|error| panic!("required cached XML {}: {error}", path.display()))
}

fn register_cached_frame(ui: &UiXml, name: &str, widget_type: &str) {
    let frame = ui
        .elements
        .iter()
        .find_map(|element| match element {
            XmlElement::Frame(frame) | XmlElement::ModelScene(frame)
                if frame.name.as_deref() == Some(name) =>
            {
                Some(frame)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("required cached virtual frame {name}"));
    assert_eq!(frame.is_virtual, Some(true), "fixture must be virtual");
    xml::register_template(name, widget_type, frame.clone());
}

fn register_product_choice_textures(ui: &UiXml) {
    // Only dependencies used by the unchanged item-display AST, including its subtrees.
    for name in [
        "productchoice-card",
        "productchoice-card-petshadow",
        "productchoice-itemicon-border",
        "productchoice-icon-magnifyingglass",
        "productchoice-icon-checkmark",
    ] {
        let texture = ui
            .elements
            .iter()
            .find_map(|element| match element {
                XmlElement::Texture(texture) if texture.name.as_deref() == Some(name) => {
                    Some(texture)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("required cached virtual texture {name}"));
        assert_eq!(texture.is_virtual, Some(true), "fixture must be virtual");
        xml::register_texture_template(name, texture.clone());
    }
}

#[test]
fn xml_self_relative_key_product_choice_targets_parent_and_preserves_later_sibling() {
    let env = WowLuaEnv::new().expect("create Mists Lua environment");
    let addons = wow_ui_sim::client_profile::blizzard_ui_addons_dir();
    let product_choice =
        parse_cached_xml(&addons.join("Blizzard_UIPanels_Game/Classic/ProductChoice.xml"));
    let model_scene = parse_cached_xml(&addons.join("Blizzard_SharedXML/ModelSceneMixin.xml"));
    let model_scene_lua_path = addons.join("Blizzard_SharedXML/ModelSceneMixin.lua");
    let model_scene_lua = std::fs::read_to_string(&model_scene_lua_path).unwrap_or_else(|error| {
        panic!(
            "required cached Lua {}: {error}",
            model_scene_lua_path.display()
        )
    });
    // Real vendor definitions; no invented ModelScene handlers or stripped child frames.
    env.exec_named(&model_scene_lua, &model_scene_lua_path.to_string_lossy())
        .expect("load unchanged cached ModelScene mixin definitions");
    register_cached_frame(
        &model_scene,
        "NonInteractableModelSceneMixinTemplate",
        "ModelScene",
    );
    register_cached_frame(&model_scene, "ModelSceneMixinTemplate", "ModelScene");
    register_product_choice_textures(&product_choice);
    register_cached_frame(&product_choice, "ProductChoiceItemDisplayTemplate", "Frame");
    env.exec(
        r#"
        assert(type(CreateFromMixins) == "function", "builtin mixin support required")
        assert(GameFontHighlight and GameFontNormalSmall, "builtin font objects required")
        assert(type(ModelSceneMixin.OnLoad) == "function", "real ModelScene OnLoad required")
        "#,
    )
    .expect("satisfy real template prerequisites before disputed anchor");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "setup Lua errors: {:?}",
        env.state().borrow().lua_errors
    );

    // CreateFrame inheritance enters production template layers and finalization.
    env.exec(
        r#"
        ProductChoiceAnchorCard = CreateFrame("Frame", "ProductChoiceAnchorCard", UIParent,
            "ProductChoiceItemDisplayTemplate")
        "#,
    )
    .expect("instantiate unchanged ProductChoice template through production XML layers");
    env.exec(
        r#"
        local card = ProductChoiceAnchorCard
        local icon, border = card.Icon, card.IconBorder
        assert(icon ~= nil, "Icon must be created")
        assert(border ~= nil, "later IconBorder producer must complete")
        assert(icon ~= border and icon ~= card and border ~= card, "distinct regions required")
        assert(icon:GetParent() == card and border:GetParent() == card, "region parent identity")
        assert(icon:GetParentKey() == "Icon" and border:GetParentKey() == "IconBorder", "parent keys")
        assert(icon:GetWidth() == 63 and icon:GetHeight() == 63, "authored Icon size")
        assert(icon:GetNumPoints() == 1, "Icon must retain its anchor")
        local point, relative, relativePoint, x, y = icon:GetPoint(1)
        assert(point == "CENTER" and relative == card and relativePoint == "CENTER"
            and x == 0 and y == 15, "XML self-relativeKey must target parent card, not Icon")
        assert(border:GetNumPoints() == 1, "later sibling must retain its anchor")
        point, relative, relativePoint, x, y = border:GetPoint(1)
        assert(point == "CENTER" and relative == icon and relativePoint == "CENTER"
            and x == 0 and y == -5, "IconBorder must still target sibling Icon")
        assert(card.Background and card.Shadow and card.Name and card.SubTitle, "decorations preserved")
        assert(card.ModelScene and card.ModelScene:GetParent() == card, "ModelScene subtree preserved")
        assert(card.ModelScene.cameras and card.ModelScene.tagToActor, "real ModelScene OnLoad completed")
        assert(card.ModelScene.PreviewButton:GetParent() == card.ModelScene, "preview subtree preserved")
        assert(card.ModelScene.PreviewButton:GetNormalTexture(), "preview inherited texture preserved")
        assert(card.Covers and card.Covers.CheckMark and card.Covers.Disabled, "later Covers subtree preserved")
        "#,
    )
    .expect("observe parent-relative Icon, later sibling anchor, and unchanged subtree progress");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "whole Lua error accumulator must remain empty: {:?}",
        env.state().borrow().lua_errors
    );
}

#[test]
fn xml_self_relative_key_direct_lua_self_setpoint_rejects_without_mutation() {
    let env = WowLuaEnv::new().expect("create Mists Lua environment");
    env.exec(
        r#"
        local receiver = CreateFrame("Frame", nil, UIParent)
        local firstTarget = CreateFrame("Frame", nil, UIParent)
        local secondTarget = CreateFrame("Frame", nil, UIParent)
        receiver:SetPoint("TOPLEFT", firstTarget, "BOTTOMRIGHT", 7, -11)
        receiver:SetPoint("BOTTOMRIGHT", secondTarget, "TOPLEFT", -13, 17)
        local before = {}
        for i = 1, receiver:GetNumPoints() do
            before[i] = { receiver:GetPoint(i) }
        end
        assert(#before == 2, "two observable points required before rejection")
        local ok = pcall(function()
            receiver:SetPoint("TOPLEFT", receiver, "CENTER", 29, 31)
        end)
        assert(ok == false, "direct Lua self SetPoint must remain invalid")
        assert(receiver:GetNumPoints() == #before, "rejection must not change point count")
        for i = 1, #before do
            local point, relative, relativePoint, x, y = receiver:GetPoint(i)
            local old = before[i]
            assert(point == old[1] and relative == old[2] and relativePoint == old[3]
                and x == old[4] and y == old[5], "rejection must preserve every existing point")
        end
        ok = pcall(function()
            receiver:SetPoint("TOPLEFT", secondTarget, "CENTER", 19, -23)
        end)
        assert(ok == true, "distinct-target SetPoint must succeed")
        assert(receiver:GetNumPoints() == 2, "successful replacement preserves other point")
        local found = false
        for i = 1, receiver:GetNumPoints() do
            local point, relative, relativePoint, x, y = receiver:GetPoint(i)
            if point == "TOPLEFT" then
                assert(relative == secondTarget and relativePoint == "CENTER" and x == 19 and y == -23,
                    "distinct-target identity and offsets must be exact")
                found = true
            else
                assert(point == "BOTTOMRIGHT" and relative == secondTarget and relativePoint == "TOPLEFT"
                    and x == -13 and y == 17, "successful replacement preserves untouched point")
            end
        end
        assert(found, "successful distinct-target point must be observable")
        "#,
    )
    .expect("direct Lua rejection is atomic while distinct-target anchoring still works");
}
