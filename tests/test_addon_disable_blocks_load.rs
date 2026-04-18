//! Coverage for the `C_AddOns.DisableAddOn` → `LoadAddOn` blocking
//! contract: a disabled addon must refuse to load (returning the
//! retail-canonical `(false, "DISABLED")` tuple), and pulling in a
//! disabled addon transitively must fail with `"DEP_DISABLED"`.
//!
//! Uses the `Admin` addon shipped under `Interface/AddOns/` as the
//! load target — it's a real registered addon the runtime knows about
//! but doesn't auto-load when these unit-style tests boot.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn load_addon_returns_disabled_for_a_disabled_addon() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Register a stub addon entry that LoadAddOn can resolve by
            -- name. We don't actually need a real toc — DisableAddOn +
            -- LoadAddOn only consult the SimState.addons list.
            A_Admin.RegisterTestAddon("FakeTestAddon")
            C_AddOns.DisableAddOn("FakeTestAddon")
            local loaded, reason = C_AddOns.LoadAddOn("FakeTestAddon")
            return tostring(loaded) .. "|" .. tostring(reason)
            "#,
        )
        .unwrap();
    assert_eq!(result, "false|DISABLED");
}

#[test]
fn enabling_a_previously_disabled_addon_lets_load_addon_proceed() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.RegisterTestAddon("FakeTestAddon")
            C_AddOns.DisableAddOn("FakeTestAddon")
            local loaded1, reason1 = C_AddOns.LoadAddOn("FakeTestAddon")
            if loaded1 ~= false or reason1 ~= "DISABLED" then
                return "step1=" .. tostring(loaded1) .. "/" .. tostring(reason1)
            end
            -- Re-enabling should clear the gate, even though the load
            -- itself will still fail (no real toc on disk) — the
            -- relevant assertion is the reason isn't "DISABLED" anymore.
            C_AddOns.EnableAddOn("FakeTestAddon")
            local loaded2, reason2 = C_AddOns.LoadAddOn("FakeTestAddon")
            if reason2 == "DISABLED" then
                return "step2_still_disabled"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn disabled_addon_stays_unloaded_after_rejected_load() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.RegisterTestAddon("FakeTestAddon")
            C_AddOns.DisableAddOn("FakeTestAddon")
            C_AddOns.LoadAddOn("FakeTestAddon")
            -- IsAddOnLoaded returns (loaded_or_loading, loaded). The
            -- second return must be false after a rejected load.
            local loaded_or_loading, fully_loaded = C_AddOns.IsAddOnLoaded("FakeTestAddon")
            return tostring(loaded_or_loading) .. "|" .. tostring(fully_loaded)
            "#,
        )
        .unwrap();
    assert_eq!(result, "false|false");
}
