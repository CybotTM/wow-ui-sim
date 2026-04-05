//! Tests for reputation: factions, expand/collapse, selection.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn test_get_num_factions_positive() {
    let env = env();
    let count: i32 = env.eval("return C_Reputation.GetNumFactions()").unwrap();
    assert!(count > 0);
}

#[test]
fn test_first_faction_is_header() {
    let env = env();
    let is_header: bool = env
        .eval("return C_Reputation.GetFactionDataByIndex(1).isHeader")
        .unwrap();
    assert!(is_header);
}

#[test]
fn test_second_faction_is_child() {
    let env = env();
    let (name, is_child): (String, bool) = env
        .eval(
            "local d = C_Reputation.GetFactionDataByIndex(2); \
             return d.name, d.isChild",
        )
        .unwrap();
    assert_eq!(name, "Council of Dornogal");
    assert!(is_child);
}

#[test]
fn test_out_of_range_returns_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Reputation.GetFactionDataByIndex(9999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_faction_by_id() {
    let env = env();
    let name: String = env
        .eval("return C_Reputation.GetFactionDataByID(72).name")
        .unwrap();
    assert_eq!(name, "Stormwind");
}

#[test]
fn test_faction_by_id_unknown() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Reputation.GetFactionDataByID(999999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_selected_faction_returns_zero() {
    let env = env();
    let sel: i32 = env
        .eval("return C_Reputation.GetSelectedFaction()")
        .unwrap();
    assert_eq!(sel, 0);
}

#[test]
fn test_watched_faction_data() {
    let env = env();
    let has_name: bool = env
        .eval("local d = C_Reputation.GetWatchedFactionData(); return d ~= nil and d.name ~= nil")
        .unwrap();
    assert!(has_name);
}

#[test]
fn test_collapse_header_hides_children() {
    let env = env();
    env.exec("C_Reputation.ExpandAllFactionHeaders()").unwrap();
    let before: i32 = env.eval("return C_Reputation.GetNumFactions()").unwrap();
    env.exec("C_Reputation.CollapseFactionHeader(1)").unwrap();
    let after: i32 = env.eval("return C_Reputation.GetNumFactions()").unwrap();
    assert!(after < before, "after={after} should be < before={before}");
}

#[test]
fn test_expand_header_restores_children() {
    let env = env();
    env.exec("C_Reputation.ExpandAllFactionHeaders()").unwrap();
    let full: i32 = env.eval("return C_Reputation.GetNumFactions()").unwrap();
    env.exec("C_Reputation.CollapseFactionHeader(1)").unwrap();
    env.exec("C_Reputation.ExpandFactionHeader(1)").unwrap();
    let restored: i32 = env.eval("return C_Reputation.GetNumFactions()").unwrap();
    assert_eq!(full, restored);
}

#[test]
fn test_collapse_all_shows_only_headers() {
    let env = env();
    env.exec("C_Reputation.ExpandAllFactionHeaders()").unwrap();
    env.exec("C_Reputation.CollapseAllFactionHeaders()")
        .unwrap();
    let count: i32 = env.eval("return C_Reputation.GetNumFactions()").unwrap();
    let headers: i32 = env
        .eval(
            "local h = 0; \
             for i = 1, C_Reputation.GetNumFactions() do \
                 if C_Reputation.GetFactionDataByIndex(i).isHeader then h = h + 1 end \
             end; \
             return h",
        )
        .unwrap();
    assert_eq!(count, headers);
}

#[test]
fn test_expand_all_shows_more_than_headers() {
    let env = env();
    env.exec("C_Reputation.CollapseAllFactionHeaders()")
        .unwrap();
    env.exec("C_Reputation.ExpandAllFactionHeaders()").unwrap();
    let count: i32 = env.eval("return C_Reputation.GetNumFactions()").unwrap();
    assert!(count > 4, "should have headers + children: {count}");
}

#[test]
fn test_max_reputation_reaction_constant() {
    let env = env();
    let val: i32 = env.eval("return MAX_REPUTATION_REACTION").unwrap();
    assert_eq!(val, 8);
}

#[test]
fn test_standing_text_via_gettext() {
    let env = env();
    let text: String = env
        .eval(
            "local d = C_Reputation.GetFactionDataByIndex(2); \
             local gender = UnitSex('player'); \
             return GetText('FACTION_STANDING_LABEL' .. d.reaction, gender)",
        )
        .unwrap();
    assert!(!text.is_empty(), "standing text should not be empty");
    assert!(
        [
            "Hated",
            "Hostile",
            "Unfriendly",
            "Neutral",
            "Friendly",
            "Honored",
            "Revered",
            "Exalted"
        ]
        .contains(&text.as_str()),
        "unexpected standing text: {text}"
    );
}
