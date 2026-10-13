//! Catalog-backed numeric icons, explicit nil misses and sourced input policy.
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn zone_ability_icon_returns_catalog_file_id_for_known_spell() {
    const FIREBALL_SPELL_ID: u32 = 133;
    let spell = wow_ui_sim::spells::get_spell(FIREBALL_SPELL_ID)
        .expect("concrete Fireball fixture exists in spell catalog");
    let icon = spell.icon_file_data_id;
    let env = WowLuaEnv::new().unwrap();
    env.exec(&format!(
        r#"
        local icon = C_ZoneAbility.GetZoneAbilityIcon({FIREBALL_SPELL_ID})
        assert(type(icon) == 'number', 'zone ability icon must be numeric file ID')
        assert(icon == {icon}, 'zone ability icon must match supplied spell catalog')
        assert(select('#', C_ZoneAbility.GetZoneAbilityIcon({FIREBALL_SPELL_ID})) == 1)
        "#,
    ))
    .expect("known spell returns exactly one numeric catalog icon");
}

#[test]
fn zone_ability_icon_absent_valid_u32_returns_exactly_one_nil() {
    // Explicit simulator model policy; native miss behavior remains unknown.
    const ABSENT_SPELL_ID: u32 = 4_000_000_000;
    assert!(wow_ui_sim::spells::get_spell(ABSENT_SPELL_ID).is_none());
    let env = WowLuaEnv::new().expect("zone ability environment");
    env.exec(&format!(
        r#"
        assert(select('#', C_ZoneAbility.GetZoneAbilityIcon({ABSENT_SPELL_ID})) == 1,
            'catalog miss returns exactly one value')
        assert(C_ZoneAbility.GetZoneAbilityIcon({ABSENT_SPELL_ID}) == nil,
            'catalog miss returns nil under simulator model policy')
        "#,
    ))
    .expect("absent representable spell ID returns exactly one nil; native unknown");
}

#[test]
fn zone_ability_icon_preserves_ordinary_numeric_u32_rejection() {
    let env = WowLuaEnv::new().expect("zone ability environment");
    env.exec(
        r#"
        local function reject(...)
            local ok, err = pcall(C_ZoneAbility.GetZoneAbilityIcon, ...)
            assert(not ok and type(err) == 'string' and #err > 0,
                'malformed ordinary argument must fail explicitly')
        end
        reject()
        reject(nil)
        for _, value in ipairs({'133', 'Fireball', false, true, {},
            function() end, coroutine.create(function() end),
            -1, 133.5, 4294967296, 0/0, math.huge, -math.huge}) do
            reject(value)
        end
        reject(CreateFrame('Frame'))
        "#,
    )
    .expect("retain required number, exact integer and u32 range rejection");
}

#[cfg(feature = "retail-12-0-5")]
#[test]
fn zone_ability_icon_authenticates_native_host_secrets_by_caller_taint() {
    use rilua::LuaApiMut;
    use rilua::table_security::wrap_host_secret_number;

    const KNOWN_SPELL_ID: u32 = 133;
    const ABSENT_SPELL_ID: u32 = 4_000_000_000;
    let icon = wow_ui_sim::spells::get_spell(KNOWN_SPELL_ID)
        .expect("concrete Fireball fixture exists in spell catalog")
        .icon_file_data_id;
    assert!(wow_ui_sim::spells::get_spell(ABSENT_SPELL_ID).is_none());
    let env = WowLuaEnv::new().expect("zone ability environment");
    {
        // Same actual VM secret construction/rooting as mount_spell_identifier.rs.
        let loader = env.loader_env();
        let mut lua = loader.rilua_mut();
        rilua::table_security::register_table_security(&mut lua).expect("VM security helpers");
        for (name, number) in [
            ("ZoneSecretKnown", f64::from(KNOWN_SPELL_ID)),
            ("ZoneSecretAbsent", f64::from(ABSENT_SPELL_ID)),
        ] {
            let value = wrap_host_secret_number(lua.state_mut(), number);
            lua.state_mut().push(value);
            let inserted = lua.set_global_val(name, value);
            lua.state_mut().pop();
            inserted.expect("root actual secret NUMBER");
        }
    }
    env.exec(&format!(
        r#"
        debug.settaintmode(true)
        local known, absent = ZoneSecretKnown, ZoneSecretAbsent
        collectgarbage('collect')
        local function preserved(expectedTaint)
            assert(issecretvalue(known) and issecretvalue(absent),
                'inputs remain actual VM secrets')
            assert(rawequal(known, ZoneSecretKnown) and rawequal(absent, ZoneSecretAbsent),
                'host wrappers retain identity')
            assert(debug.getstacktaint() == expectedTaint, 'caller taint unchanged')
        end
        local function check(value, expected, expectedTaint)
            assert(select('#', C_ZoneAbility.GetZoneAbilityIcon(value)) == 1,
                'exactly one result')
            local result = C_ZoneAbility.GetZoneAbilityIcon(value)
            assert(result == expected, 'catalog result or explicit modeled miss')
            if expected ~= nil then
                assert(type(result) == 'number' and not issecretvalue(result),
                    'ordinary numeric catalog icon; no inferred secret output propagation')
            end
            preserved(expectedTaint)
        end
        local function reject(value)
            local ok, err = pcall(C_ZoneAbility.GetZoneAbilityIcon, value)
            assert(not ok and type(err) == 'string' and #err > 0,
                'tainted secret call rejects, even when catalog ID is absent')
            assert(string.find(err, 'untainted caller', 1, true),
                'existing secret authentication guard must reject: ' .. tostring(err))
            preserved('ZoneIconFixture')
        end
        assert(issecure() and debug.getstacktaint() == nil)
        preserved(nil)
        check(known, {icon}, nil)
        check(absent, nil, nil)
        local function addon()
            preserved('ZoneIconFixture')
            reject(known)
            reject(absent)
            check({KNOWN_SPELL_ID}, {icon}, 'ZoneIconFixture')
        end
        debug.setobjecttaint(addon, 'ZoneIconFixture')
        addon()
        assert(issecure() and debug.getstacktaint() == nil)
        check(known, {icon}, nil)
        check(absent, nil, nil)
        preserved(nil)
        "#,
    ))
    .expect(
        "AllowedWhenUntainted authenticates host secrets without altering wrappers or caller taint",
    );
}
