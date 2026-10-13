//! Host-supplied kit tests staged; no executed RED/GREEN or native proof.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use rilua::LuaApiMut;
use rilua::table_security::{wrap_host_secret_number, wrap_host_secret_string};
use wow_ui_sim::c_api::PingTextureKits;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::paths::default_blizzard_ui_addons_path;

fn supply_kits(env: &WowLuaEnv, pairs: &[(i32, &str)]) {
    env.state().borrow_mut().ping_texture_kits = PingTextureKits {
        by_type: pairs
            .iter()
            .map(|(key, kit)| (*key, (*kit).to_owned()))
            .collect(),
    };
}

#[test]
fn pass2_ping_kit_exact_host_strings_live_replacement_and_environment_isolation() {
    let first = WowLuaEnv::new().expect("first environment");
    let second = WowLuaEnv::new().expect("second environment");
    // Synthetic keys and kits, NOT native enum values or native pairs.
    supply_kits(
        &first,
        &[
            (70001, "Fixture_NotRetail_A"),
            (70002, "Fixture_NotRetail_B"),
        ],
    );
    supply_kits(&second, &[(70001, "SeparateHostKit")]);
    first
        .exec(
            r#"
        assert(select('#', C_Ping.GetTextureKitForType(70001)) == 1)
        assert(type(C_Ping.GetTextureKitForType(70001)) == 'string')
        assert(C_Ping.GetTextureKitForType(70001) == 'Fixture_NotRetail_A')
        assert(C_Ping.GetTextureKitForType(70002) == 'Fixture_NotRetail_B')
    "#,
        )
        .expect("exact configured output");
    supply_kits(&first, &[(70001, "ReplacementHostKit")]);
    let changed: String = first
        .eval("return C_Ping.GetTextureKitForType(70001)")
        .expect("next lookup uses replacement input");
    let isolated: String = second
        .eval("return C_Ping.GetTextureKitForType(70001)")
        .expect("independent environment retains its input");
    assert_eq!(changed, "ReplacementHostKit");
    assert_eq!(isolated, "SeparateHostKit");
}

#[test]
fn pass2_ping_kit_missing_mapping_errors_by_simulator_policy_not_native_parity() {
    let env = WowLuaEnv::new().expect("ping environment");
    env.exec(
        r#"
        assert(type(C_Ping.GetTextureKitForType) == 'function')
        local ok, err = pcall(C_Ping.GetTextureKitForType, 70001)
        assert(not ok, 'empty host input fails explicitly by simulator policy')
        assert(type(err) == 'string')
        assert(err:find('unmapped ping type 70001', 1, true))
        assert(err:find('host-supplied mapping required', 1, true))
    "#,
    )
    .expect("empty-map simulator policy");
    supply_kits(&env, &[(70001, "ConfiguredOnly")]);
    env.exec(
        r#"
        assert(C_Ping.GetTextureKitForType(70001) == 'ConfiguredOnly')
        local ok, err = pcall(C_Ping.GetTextureKitForType, 70002)
        assert(not ok)
        assert(err:find('unmapped ping type 70002', 1, true))
    "#,
    )
    .expect("missing key does not use another entry");
    supply_kits(&env, &[]);
    env.exec(
        r#"
        local ok, err = pcall(C_Ping.GetTextureKitForType, 70001)
        assert(not ok)
        assert(err:find('unmapped ping type 70001', 1, true))
    "#,
    )
    .expect("cleared input does not keep a cached kit");
}

#[test]
fn pass2_ping_kit_public_invalid_keys_do_not_coerce_or_reach_lookup() {
    let env = WowLuaEnv::new().expect("ping validation environment");
    supply_kits(&env, &[(70001, "ConfiguredOnly")]);
    env.exec(
        r#"
        local query = C_Ping.GetTextureKitForType
        assert(type(query) == 'function', 'real query must be published')
        local function reject(...)
            local ok, message = pcall(query, ...)
            assert(not ok and type(message) == 'string', 'invalid key rejected')
            assert(message:find('ping type', 1, true), 'argument error context')
            assert(not message:find('unmapped ping type', 1, true),
                'invalid keys must not become map lookups')
        end
        reject()
        reject(nil)
        for _, value in ipairs({'70001', true, false, {}, 70001.5,
                0/0, math.huge, -math.huge, 2147483648, -2147483649}) do
            reject(value)
        end
        assert(query(70001) == 'ConfiguredOnly', 'failed calls preserve input')
        "#,
    )
    .expect("explicit i32 key policy, not native coercion or enum bounds");
}

fn install_secret_ping_inputs(env: &WowLuaEnv) {
    let loader = env.loader_env();
    let mut lua = loader.rilua_mut();
    rilua::table_security::register_table_security(&mut lua).expect("VM security helpers");
    for (name, number) in [
        ("PingSecretConfigured", 70001.0),
        ("PingSecretUnmapped", 70002.0),
        ("PingSecretFractional", 70001.5),
    ] {
        let value = wrap_host_secret_number(lua.state_mut(), number);
        lua.state_mut().push(value);
        let inserted = lua.set_global_val(name, value);
        lua.state_mut().pop();
        inserted.expect("root actual secret number");
    }
    let value = wrap_host_secret_string(lua.state_mut(), "PRIVATE-PingInput");
    lua.state_mut().push(value);
    let inserted = lua.set_global_val("PingSecretString", value);
    lua.state_mut().pop();
    inserted.expect("root actual secret string");
}

fn create_secret_ping_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("ping secret environment");
    supply_kits(&env, &[(70001, "ConfiguredOnly")]);
    env.exec("assert(type(C_Ping.GetTextureKitForType) == 'function', 'real query required')")
        .expect("publication before secret fixture setup");
    install_secret_ping_inputs(&env);
    env
}

#[test]
fn pass2_ping_kit_untainted_secret_key_returns_exact_public_string() {
    let env = create_secret_ping_env();
    env.exec(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local key = PingSecretConfigured
        assert(issecretvalue(key), 'actual secret numeric input')
        local function check(...)
            assert(select('#', ...) == 1)
            local value = ...
            assert(not issecretvalue(value), 'public output model policy')
            assert(type(value) == 'string' and value == 'ConfiguredOnly')
        end
        check(C_Ping.GetTextureKitForType(key))
        assert(issecretvalue(key) and secretunwrap(key) == 70001)
        assert(debug.getstacktaint() == nil, 'query preserves caller context')
        "#,
    )
    .expect("AllowedWhenUntainted acceptance with public mapped output");
}

#[test]
fn pass2_ping_kit_tainted_secret_denial_precedes_validation_and_lookup() {
    let env = create_secret_ping_env();
    env.exec(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local keys = {PingSecretConfigured, PingSecretUnmapped,
            PingSecretFractional, PingSecretString}
        local query = C_Ping.GetTextureKitForType
        local function addon()
            assert(debug.getstacktaint() == 'PingInputFixture')
            for _, key in ipairs(keys) do
                assert(issecretvalue(key), 'actual VM secret fixture')
                local ok, message = pcall(query, key)
                assert(not ok and type(message) == 'string', 'tainted secret denied')
                assert(message:find('untainted caller', 1, true),
                    'authentication precedes validation and map lookup')
                assert(not message:find('PRIVATE-PingInput', 1, true), 'no payload leak')
                assert(issecretvalue(key), 'original input remains wrapped')
                assert(not pcall(secretunwrap, key), 'no tainted declassification')
                assert(debug.getstacktaint() == 'PingInputFixture')
            end
            assert(query(70001) == 'ConfiguredOnly', 'tainted public input permitted')
        end
        debug.setobjecttaint(addon, 'PingInputFixture')
        addon()
        assert(debug.getstacktaint() == nil, 'outer context restored')
        assert(query(PingSecretConfigured) == 'ConfiguredOnly', 'secure recovery')
        "#,
    )
    .expect("metadata authentication ordering, not native taint parity");
}

#[test]
fn pass2_ping_kit_actual_cached_consumer_formats_both_atlas_names() {
    let env = WowLuaEnv::new().expect("ping environment");
    supply_kits(&env, &[(70001, "Fixture_NotRetail_Glow")]);
    // Reach the missing query before cache discovery, I/O or vendor loading.
    // No query replacement: this prospective assertion requires future work.
    let kit: String = env
        .eval("return C_Ping.GetTextureKitForType(70001)")
        .expect("real query must read host input before cached fixture prerequisites");
    assert_eq!(kit, "Fixture_NotRetail_Glow");
    let source_path = default_blizzard_ui_addons_path()
        .expect("active-profile Blizzard UI cache required; no fallback")
        .join("Blizzard_PingUI/Blizzard_PingManager.lua");
    let source = std::fs::read_to_string(&source_path)
        .expect("active-profile cached PingManager source required; no fallback");
    env.exec_named(&source, &source_path.to_string_lossy())
        .expect("unchanged vendor definitions only, not XML/startup");
    env.exec(
        r#"
        -- Only controlled pool/frame/animation sinks; query/vendor unchanged.
        local observed = {}
        local spot = {
            GlowIn = {SetAtlas = function(_, name, useSize)
                observed.inName = name; observed.inSize = useSize
            end},
            GlowOut = {SetAtlas = function(_, name, useSize)
                observed.outName = name; observed.outSize = useSize
            end},
            PulseAnim = {SetScript = function() end, Restart = function() end},
            ClearAllPoints = function() end,
            SetPoint = function() end,
            Show = function() end,
            Hide = function() end,
        }
        local receiver = {pingSpotPool = {
            Acquire = function() return spot end,
            Release = function() end,
        }}
        -- Cached Retail/PTR declaration: PingManager:ShowPingSpot(type, posX, posY).
        -- Explicit self supplies exactly the pingSpotPool the method reads.
        PingManager.ShowPingSpot(receiver, 70001, 42, 84)
        assert(observed.inName == 'Ping_SpotGlw_Fixture_NotRetail_Glow_In')
        assert(observed.outName == 'Ping_SpotGlw_Fixture_NotRetail_Glow_Out')
        assert(observed.inSize == true and observed.outSize == true)
    "#,
    )
    .expect("actual consumer passes both exact atlas names to sinks");
}
