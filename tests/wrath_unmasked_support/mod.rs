//! Clean-room cached-consumer inputs. Missing/drifted files are setup failures.
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn original(file: &str) -> (PathBuf, String) {
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().expect("Wrath cache required");
    let path = toc.parent().unwrap().join(file);
    let bytes = std::fs::read(&path).unwrap();
    let expected = include_str!("pins.tsv")
        .lines()
        .find_map(|line| {
            line.split_once('\t')
                .filter(|(name, _)| *name == file)
                .map(|(_, hash)| hash)
        })
        .expect("consumer needs a pin");
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        expected,
        "changed {}",
        path.display()
    );
    (path, String::from_utf8(bytes).unwrap())
}

pub fn env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    let fonts = wow_ui_sim::render::font::WowFontSystem::new_without_casc();
    assert!(
        fonts
            .family_name(Some(wow_ui_sim::render::font::DEFAULT_WOW_FONT))
            .is_some()
    );
    env.set_font_system(std::rc::Rc::new(std::cell::RefCell::new(fonts)));
    env.set_screen_size(1600.0, 1200.0);
    run(
        &env,
        "UnmaskedErrors = {}; seterrorhandler(function(e) UnmaskedErrors[#UnmaskedErrors+1] = tostring(e) end)",
    );
    env
}

pub fn run(env: &WowLuaEnv, code: &str) {
    let result = env.exec(code);
    env.exec("assert(#UnmaskedErrors == 0, table.concat(UnmaskedErrors, '\\n'))")
        .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
    result.unwrap();
}

pub fn load(env: &WowLuaEnv, files: &[&str]) {
    // Validate every transitive Script/Include input before invoking the loader.
    for line in include_str!("pins.tsv").lines() {
        original(line.split_once('\t').unwrap().0);
    }
    let root = tempfile::tempdir().unwrap();
    let mut toc = String::from("## Interface: 30300\n## Title: WrathUnmasked\n");
    for file in files {
        let (path, _) = original(file);
        let common = root
            .path()
            .ancestors()
            .find(|base| path.starts_with(base))
            .unwrap();
        let mut relative = PathBuf::new();
        for _ in root.path().strip_prefix(common).unwrap().components() {
            relative.push("..");
        }
        relative.push(path.strip_prefix(common).unwrap());
        toc.push_str(relative.to_str().unwrap());
        toc.push('\n');
    }
    let path = root.path().join("WrathUnmasked.toc");
    std::fs::write(&path, toc).unwrap();
    let result = wow_ui_sim::loader::load_addon(&env.loader_env(), &path);
    run(env, "assert(true)");
    let loaded = result.unwrap();
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
}
