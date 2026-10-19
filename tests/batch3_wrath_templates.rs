//! Loader trust origins + observable size/script output; not registry-shape assertions.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::{loader::load_addon, lua_api::WowLuaEnv, xml::clear_templates};

fn declaration(
    env: &WowLuaEnv,
    parent: &std::path::Path,
    addon_name: &str,
    template: &str,
    width: u32,
    marker: &str,
) {
    let dir = parent.join(addon_name);
    std::fs::create_dir(&dir).unwrap();
    let toc = dir.join(format!("{addon_name}.toc"));
    std::fs::write(&toc, "## Interface: 30300\nTemplate.xml\n").unwrap();
    std::fs::write(
        dir.join("Template.xml"),
        format!(
            r#"<Ui>
        <Frame name="{template}" virtual="true" protected="true" hidden="true">
            <Size x="{width}" y="20"/>
            <Scripts><OnShow>
                self.batch3Origin = '{marker}'; self.batch3Secure = issecure()
            </OnShow></Scripts>
        </Frame>
    </Ui>"#
        ),
    )
    .unwrap();
    load_addon(&env.loader_env(), &toc).unwrap();
}

fn check_instance(env: &WowLuaEnv, name: &str, width: u32, marker: &str, secure: bool) {
    env.exec(&format!(
        r#"
        local frame = CreateFrame('Frame', nil, UIParent, '{name}')
        frame:Show()
        assert(frame:GetWidth() == {width}, 'duplicate selection size')
        assert(frame.batch3Origin == '{marker}', 'duplicate selection script')
        assert(frame.batch3Secure == {secure}, 'winning declaration script taint')
    "#
    ))
    .unwrap();
}

fn collision(first_trusted: bool, incoming_trusted: bool, mixed_case: bool) {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    let root = tempfile::tempdir().unwrap();
    let first = if first_trusted {
        "Blizzard_Batch3First"
    } else {
        "Batch3First"
    };
    let second = if incoming_trusted {
        "Blizzard_Batch3Second"
    } else {
        "Batch3Second"
    };
    let old_name = "Batch3CollisionTemplate";
    let new_name = if mixed_case {
        "bATCH3cOLLISIONtEMPLATE"
    } else {
        old_name
    };
    declaration(&env, root.path(), first, old_name, 111, "first");
    check_instance(&env, old_name, 111, "first", first_trusted);
    declaration(&env, root.path(), second, new_name, 222, "second");
    let replacement = !first_trusted && incoming_trusted;
    let (width, marker, secure) = if replacement {
        (222, "second", true)
    } else {
        (111, "first", first_trusted)
    };
    // Warmed inheritance/lifecycle caches and exact/case-fold lookups must agree.
    for name in [
        old_name,
        new_name,
        "batch3collisiontemplate",
        "BATCH3COLLISIONTEMPLATE",
    ] {
        check_instance(&env, name, width, marker, secure);
    }
    assert!(env.state().borrow().lua_errors.is_empty());
}

#[test]
fn x1_clean_existing_beats_clean_incoming() {
    collision(true, true, false);
}
#[test]
fn x1_clean_existing_beats_tainted_incoming() {
    collision(true, false, false);
}
#[test]
fn x1_tainted_existing_beats_tainted_incoming() {
    collision(false, false, false);
}
#[test]
fn x1_clean_incoming_replaces_tainted_existing() {
    collision(false, true, false);
}
#[test]
fn x1_casefold_clean_existing_beats_clean_incoming() {
    collision(true, true, true);
}
#[test]
fn x1_casefold_clean_existing_beats_tainted_incoming() {
    collision(true, false, true);
}
#[test]
fn x1_casefold_tainted_existing_beats_tainted_incoming() {
    collision(false, false, true);
}
#[test]
fn x1_casefold_clean_incoming_replaces_tainted_existing() {
    collision(false, true, true);
}
