//! Tests for A_Admin encounter & loot roll simulation.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SimulateBossKill
// ============================================================================

#[test]
fn test_boss_kill_fires_encounter_end() {
    let env = env();
    let got: bool = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("ENCOUNTER_END")
            f:SetScript("OnEvent", function(self, event, eid, ename, diff, size, success)
                if event == "ENCOUNTER_END" and eid == 2902 and ename == "Ky'veza"
                   and diff == 16 and size == 20 and success == 1 then
                    fired = true
                end
            end)
            A_Admin.SimulateBossKill(2902, "Ky'veza", 16, 20)
            return fired
            "#,
        )
        .unwrap();
    assert!(got);
}

#[test]
fn test_boss_kill_fires_boss_kill_event() {
    let env = env();
    let got: bool = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("BOSS_KILL")
            f:SetScript("OnEvent", function(self, event, eid, ename)
                if event == "BOSS_KILL" and eid == 2902 and ename == "Ky'veza" then
                    fired = true
                end
            end)
            A_Admin.SimulateBossKill(2902, "Ky'veza", 16, 20)
            return fired
            "#,
        )
        .unwrap();
    assert!(got);
}

// ============================================================================
// StartLootRoll / EndLootRoll
// ============================================================================

#[test]
fn test_start_loot_roll_fires_event() {
    let env = env();
    let (roll_id, roll_time): (i32, f64) = env
        .eval(
            r#"
            local rid, rtime
            local f = CreateFrame("Frame")
            f:RegisterEvent("START_LOOT_ROLL")
            f:SetScript("OnEvent", function(self, event, id, time)
                rid = id; rtime = time
            end)
            A_Admin.StartLootRoll(42, 30, "Heroic Sword", "Interface\\Icons\\inv_sword", 4, 639)
            return rid, rtime
            "#,
        )
        .unwrap();
    assert_eq!(roll_id, 42);
    assert!((roll_time - 30.0).abs() < 0.001);
}

#[test]
fn test_get_loot_roll_item_info_returns_data() {
    let env = env();
    let (texture, name, quality, ilvl): (String, String, i32, i32) = env
        .eval(
            r#"
            A_Admin.StartLootRoll(1, 30, "Heroic Sword", "Interface\\Icons\\inv_sword", 4, 639)
            local tex, n, count, q, bop, need, greed, de, deLvl, il = GetLootRollItemInfo(1)
            return tex, n, q, il
            "#,
        )
        .unwrap();
    assert_eq!(texture, "Interface\\Icons\\inv_sword");
    assert_eq!(name, "Heroic Sword");
    assert_eq!(quality, 4);
    assert_eq!(ilvl, 639);
}

#[test]
fn test_get_loot_roll_item_info_unknown_returns_nil() {
    let env = env();
    let is_nil: bool = env.eval("return GetLootRollItemInfo(999) == nil").unwrap();
    assert!(is_nil);
}

#[test]
fn test_get_loot_roll_item_link() {
    let env = env();
    let link: String = env
        .eval(
            r#"
            A_Admin.StartLootRoll(1, 30, "Sword", "tex", 4, 600, "|cffff8000|Hitem:12345|h[Sword]|h|r")
            return GetLootRollItemLink(1)
            "#,
        )
        .unwrap();
    assert!(link.contains("12345"));
}

#[test]
fn test_get_loot_roll_item_link_nil_when_missing() {
    let env = env();
    let is_nil: bool = env.eval("return GetLootRollItemLink(999) == nil").unwrap();
    assert!(is_nil);
}

#[test]
fn test_get_loot_roll_time_left() {
    let env = env();
    let time: f64 = env
        .eval(
            r#"
            A_Admin.StartLootRoll(1, 25, "Sword", "tex", 4, 600)
            return GetLootRollTimeLeft(1)
            "#,
        )
        .unwrap();
    assert!((time - 25.0).abs() < 0.001);
}

#[test]
fn test_get_active_loot_roll_ids() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.StartLootRoll(10, 30, "Sword", "tex", 4, 600)
            A_Admin.StartLootRoll(20, 30, "Shield", "tex2", 3, 600)
            return #GetActiveLootRollIDs()
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
}

#[test]
fn test_end_loot_roll_removes_and_fires_event() {
    let env = env();
    let (fired, count): (bool, i32) = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("LOOT_ROLLS_COMPLETE")
            f:SetScript("OnEvent", function(self, event, handle)
                if event == "LOOT_ROLLS_COMPLETE" and handle == 1 then fired = true end
            end)
            A_Admin.StartLootRoll(1, 30, "Sword", "tex", 4, 600)
            A_Admin.EndLootRoll(1)
            return fired, #GetActiveLootRollIDs()
            "#,
        )
        .unwrap();
    assert!(fired);
    assert_eq!(count, 0);
}

#[test]
fn test_get_active_loot_roll_ids_empty_by_default() {
    let env = env();
    let count: i32 = env.eval("return #GetActiveLootRollIDs()").unwrap();
    assert_eq!(count, 0);
}
