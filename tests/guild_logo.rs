//! `GetGuildLogoInfo()` round-trip coverage.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn guild_logo_global_is_classified_as_real_lua_api() {
    let globals_mod = include_str!("../src/lua_api/globals/mod.rs");
    let real_mod = include_str!("../src/lua_api/globals/real/mod.rs");
    let registrar = include_str!("../src/lua_api/globals/register.rs");

    assert!(
        !globals_mod.contains("pub mod guild_logo;"),
        "state-backed guild logo surface should not live in the globals base module"
    );
    assert!(
        real_mod.contains("pub mod guild_logo;"),
        "state-backed guild logo surface should be classified under globals::real"
    );
    assert!(
        registrar.contains("real::guild_logo::register_all"),
        "global registrar should wire GetGuildLogoInfo through globals::real"
    );
}

fn all_ten(env: &WowLuaEnv) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, String) {
    env.eval(
        r#"
        local bkgR, bkgG, bkgB, borderR, borderG, borderB,
              emblemR, emblemG, emblemB, filename = GetGuildLogoInfo()
        return bkgR, bkgG, bkgB, borderR, borderG, borderB,
               emblemR, emblemG, emblemB, filename
        "#,
    )
    .unwrap()
}

#[test]
fn defaults_zero_channels_and_empty_filename() {
    let env = WowLuaEnv::new().unwrap();
    let (br, bg, bb, rr, rg, rb, er, eg, eb, fname) = all_ten(&env);
    assert_eq!(br, 0.0);
    assert_eq!(bg, 0.0);
    assert_eq!(bb, 0.0);
    assert_eq!(rr, 0.0);
    assert_eq!(rg, 0.0);
    assert_eq!(rb, 0.0);
    assert_eq!(er, 0.0);
    assert_eq!(eg, 0.0);
    assert_eq!(eb, 0.0);
    assert_eq!(fname, "");
}

#[test]
fn returns_ten_values() {
    // Matches Blizzard's destructuring shape.
    let env = WowLuaEnv::new().unwrap();
    let count: i32 = env
        .eval(r#"return select('#', GetGuildLogoInfo())"#)
        .unwrap();
    assert_eq!(count, 10);
}

#[test]
fn admin_set_guild_emblem_drives_all_channels() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        A_Admin.SetGuildEmblem(
            "Interface\\GuildFrame\\GuildEmblems_01",
            0.8, 0.1, 0.1,
            0.5, 0.5, 0.5,
            1.0, 1.0, 0.0
        )
        "#,
    )
    .unwrap();
    let (br, bg, bb, rr, rg, rb, er, eg, eb, fname) = all_ten(&env);
    assert!((br - 0.8).abs() < 1e-9);
    assert!((bg - 0.1).abs() < 1e-9);
    assert!((bb - 0.1).abs() < 1e-9);
    assert!((rr - 0.5).abs() < 1e-9);
    assert!((rg - 0.5).abs() < 1e-9);
    assert!((rb - 0.5).abs() < 1e-9);
    assert!((er - 1.0).abs() < 1e-9);
    assert!((eg - 1.0).abs() < 1e-9);
    assert!((eb - 0.0).abs() < 1e-9);
    assert_eq!(fname, "Interface\\GuildFrame\\GuildEmblems_01");
}

#[test]
fn partial_admin_args_default_to_zero_or_empty() {
    let env = WowLuaEnv::new().unwrap();
    // Only filename + bkg RGB set; border and emblem default to 0.
    env.exec(r#"A_Admin.SetGuildEmblem("Emblems/Skull", 0.5, 0.0, 0.0)"#)
        .unwrap();
    let (br, _, _, rr, _, _, er, _, _, fname) = all_ten(&env);
    assert!((br - 0.5).abs() < 1e-9);
    assert_eq!(rr, 0.0, "border defaults to 0");
    assert_eq!(er, 0.0, "emblem defaults to 0");
    assert_eq!(fname, "Emblems/Skull");
}

#[test]
fn no_arg_admin_resets_to_defaults() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetGuildEmblem("X", 1, 1, 1, 1, 1, 1, 1, 1, 1)"#)
        .unwrap();
    env.exec("A_Admin.SetGuildEmblem()").unwrap();
    let (br, _, _, _, _, _, _, _, _, fname) = all_ten(&env);
    assert_eq!(br, 0.0);
    assert_eq!(fname, "");
}

#[test]
fn emblem_filename_can_be_set_without_colours() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetGuildEmblem("foo/bar.tga")"#)
        .unwrap();
    let (_, _, _, _, _, _, _, _, _, fname) = all_ten(&env);
    assert_eq!(fname, "foo/bar.tga");
}
