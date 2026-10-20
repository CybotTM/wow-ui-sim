//! P3 numeric selectors: original meter consumers, not replacement providers.
#![cfg(all(feature = "retail-12-0-5", feature = "profile-retail"))]

use rilua::LuaApiMut;
use wow_ui_sim::lua_api::WowLuaEnv;

fn fixture() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    let original = std::fs::read_to_string(
        "/home/osso/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_DamageMeter/DamageMeterEntry.lua",
    ).unwrap();
    env.exec(&original)
        .expect("unchanged DamageMeterEntry source");
    install_secrets(&env);
    env
}

fn install_secrets(env: &WowLuaEnv) {
    let loader = env.loader_env();
    let mut lua = loader.rilua_mut();
    for (name, number) in [
        ("Selector", 19750.0),
        ("Missing", 900002.0),
        ("Fraction", 19750.5),
        ("Negative", -1.0),
        ("Infinite", f64::INFINITY),
        ("Nan", f64::NAN),
        ("Overflow", 4294967296.0),
    ] {
        let value = rilua::table_security::wrap_host_secret_number(lua.state_mut(), number);
        lua.state_mut().push(value);
        lua.set_global_val(name, value).unwrap();
        lua.state_mut().pop();
    }
    let value = rilua::table_security::wrap_host_secret_string(lua.state_mut(), "19750");
    lua.state_mut().push(value);
    lua.set_global_val("SecretString", value).unwrap();
    lua.state_mut().pop();
}

fn invoke(env: &WowLuaEnv, code: &str, tainted: bool) {
    env.exec(&format!(
        r#"
        local function probe()
            local before = debug.getstacktaint()
            assert(before == {expected})
            {code}
            assert(issecretvalue(Selector))
            if {tainted} then
                assert(not canaccessvalue(Selector))
                assert(not pcall(secretunwrap, Selector))
                assert(not pcall(function() return Selector + 1 end))
            end
            assert(debug.getstacktaint() == before, 'caller taint preserved')
        end
        if {tainted} then debug.setobjecttaint(probe, 'PresentationProbe') end
        probe()
    "#,
        expected = if tainted {
            "'PresentationProbe'"
        } else {
            "nil"
        }
    ))
    .expect(code);
}

#[test]
fn original_meter_icon_numeric_secret_selector() {
    let env = fixture();
    for tainted in [false, true] {
        for selector in ["19750", "Selector"] {
            invoke(
                &env,
                &format!(
                    r#"
                local row = {{spellID = {selector}}}
                local icon, original, override = DamageMeterSpellEntryMixin.GetIconTexture(row)
                assert(icon == 135907 and original == 135907 and override == nil, 'selected Flash of Light icon')
                assert(select('#', DamageMeterSpellEntryMixin.GetIconTexture(row)) == 3)
                assert(issecretvalue(row.spellID) == issecretvalue({selector}), 'row secrecy unchanged')
            "#
                ),
                tainted,
            );
        }
    }
}

#[test]
fn original_meter_name_numeric_secret_selector() {
    let env = fixture();
    for tainted in [false, true] {
        for selector in ["19750", "Selector"] {
            invoke(
                &env,
                &format!(
                    r#"
                local row = {{spellID = {selector}}}
                assert(DamageMeterSpellEntryMixin.GetNameText(row) == 'Flash of Light', 'selected name')
                assert(select('#', DamageMeterSpellEntryMixin.GetNameText(row)) == 1)
                assert(issecretvalue(row.spellID) == issecretvalue({selector}), 'row secrecy unchanged')
            "#
                ),
                tainted,
            );
        }
    }
}

#[test]
fn presentation_miss_and_invalid_numeric_secret_controls() {
    let env = fixture();
    for tainted in [false, true] {
        invoke(
            &env,
            r#"
            assert(select('#', C_Spell.GetSpellTexture(Missing)) == 0)
            assert(select('#', C_Spell.GetSpellName(Missing)) == 1)
            assert(C_Spell.GetSpellName(Missing) == 'Unknown')
            assert(select('#', C_Spell.GetSpellTexture(900002)) == 0)
            assert(C_Spell.GetSpellName(900002) == 'Unknown')
            for _, value in ipairs({Fraction, Negative, Infinite, Nan, Overflow, SecretString}) do
                assert(select('#', C_Spell.GetSpellTexture(value)) == 0)
                assert(not pcall(C_Spell.GetSpellName, value), 'invalid typed name selector')
                assert(issecretvalue(value), 'invalid wrapper unchanged')
            end
            assert(select('#', C_Spell.GetSpellTexture({})) == 0)
            assert(not pcall(C_Spell.GetSpellName, {}))
        "#,
            tainted,
        );
    }
}
