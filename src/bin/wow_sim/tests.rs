use super::*;
use clap::Parser;

#[test]
fn legacy_character_select_flag_maps_to_character_select_screen() {
    let args = Args::try_parse_from(["wow-sim", "--character-select"])
        .expect("legacy character-select flag should parse");
    assert_eq!(args.effective_screen(), ScreenKind::CharacterSelect);
}

#[test]
fn explicit_screen_still_parses_character_select() {
    let args = Args::try_parse_from(["wow-sim", "--screen", "character-select"])
        .expect("screen option should parse character-select");
    assert_eq!(args.effective_screen(), ScreenKind::CharacterSelect);
}

#[test]
fn explicit_screen_parses_character_create() {
    let args = Args::try_parse_from(["wow-sim", "--screen", "character-create"])
        .expect("screen option should parse character-create");
    assert_eq!(args.effective_screen(), ScreenKind::CharacterCreate);
}

#[test]
#[cfg(feature = "gui")]
fn debug_elements_enable_borders_and_anchors() {
    let env = WowLuaEnv::new().expect("failed to create Lua env");
    let dispatch = CommandDispatch {
        command: None,
        env,
        font_system: Rc::new(RefCell::new(WowFontSystem::new())),
        delay: None,
        exec_lua: None,
        saved_stdout: None,
        saved_vars: None,
        debug_borders: false,
        debug_anchors: false,
        debug_elements: true,
    };

    let debug = dispatch.debug_options();
    assert!(debug.borders);
    assert!(debug.anchors);
}
