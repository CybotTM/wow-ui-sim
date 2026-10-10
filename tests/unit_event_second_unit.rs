use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn second_unit_filter_receives_modeled_player_and_target_health_events() {
    let env = WowLuaEnv::new().expect("create unit-event environment");
    env.exec(
        r#"
        A_Admin.SetPlayerHealth(50000, 200000)
        A_Admin.SetTarget("Boss", 63, 1, true)
        A_Admin.SetTargetHealth(50000, 200000)
        local bothCalls, playerCalls, targetCalls = {}, {}, {}
        local function register(calls, unit1, unit2)
            local frame = CreateFrame("Frame")
            frame:RegisterUnitEvent("UNIT_HEALTH", unit1, unit2)
            frame:SetScript("OnEvent", function(_, event, unit)
                assert(event == "UNIT_HEALTH")
                calls[#calls + 1] = unit
            end)
            return frame
        end
        local both = register(bothCalls, "player", "target")
        register(playerCalls, "player")
        register(targetCalls, "target")

        CastSpellByID(85673) -- Word of Glory heals player with an enemy target.
        assert(UnitHealth("player") == 70000, "modeled heal must change player health")
        assert(table.concat(bothCalls, ",") == "player", "first unit must receive player heal")
        assert(table.concat(playerCalls, ",") == "player")
        assert(#targetCalls == 0, "target-only filter must suppress player heal")

        CastSpellByID(35395) -- Crusader Strike damages the target.
        assert(UnitHealth("target") == 35000, "modeled damage must change target health")
        assert(table.concat(targetCalls, ",") == "target", "single-unit control receives damage")
        assert(table.concat(playerCalls, ",") == "player", "player-only filter suppresses target damage")
        assert(table.concat(bothCalls, ",") == "player,target", "second unit must receive target damage")
        local registered, firstUnit = both:IsEventRegistered("UNIT_HEALTH")
        assert(registered and firstUnit == "player", "existing first-unit query remains unchanged")
        "#,
    )
    .expect("both registered units receive modeled health events");
}

#[test]
fn reregistering_and_unregistering_clear_the_second_unit_filter() {
    let env = WowLuaEnv::new().expect("create unit-event lifecycle environment");
    env.exec(
        r#"
        local calls = {}
        local frame = CreateFrame("Frame")
        frame:SetScript("OnEvent", function(_, _, unit)
            calls[#calls + 1] = unit
        end)
        frame:RegisterUnitEvent("UNIT_HEALTH", "player", "target")
        FireEvent("UNIT_HEALTH", "focus")
        assert(#calls == 0, "two-unit filter must not become an unfiltered registration")
        frame:RegisterUnitEvent("UNIT_HEALTH", "player")
        FireEvent("UNIT_HEALTH", "target")
        FireEvent("UNIT_HEALTH", "player")
        assert(table.concat(calls, ",") == "player", "single-unit registration replaces both-unit filter")

        frame:RegisterUnitEvent("UNIT_HEALTH", "player", "target")
        frame:UnregisterEvent("UNIT_HEALTH")
        FireEvent("UNIT_HEALTH", "player")
        FireEvent("UNIT_HEALTH", "target")
        assert(table.concat(calls, ",") == "player", "unregister removes both units")
        assert(not frame:IsEventRegistered("UNIT_HEALTH"))

        frame:RegisterUnitEvent("UNIT_HEALTH", "player", "target")
        frame:UnregisterAllEvents()
        FireEvent("UNIT_HEALTH", "player")
        FireEvent("UNIT_HEALTH", "target")
        assert(table.concat(calls, ",") == "player", "unregister-all removes both units")
        "#,
    )
    .expect("unit-event filter lifecycle preserves single-unit and unregister behavior");
}
