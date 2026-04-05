//! Tests for professions: GetProfessions, GetProfessionInfo, C_TradeSkillUI.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// GetProfessions / GetProfessionInfo
// ============================================================================

#[test]
fn test_get_professions_returns_two_indices() {
    let env = env();
    let (p1, p2): (i32, i32) = env
        .eval("local a, b = GetProfessions(); return a, b")
        .unwrap();
    assert_eq!(p1, 1);
    assert_eq!(p2, 2);
}

#[test]
fn test_get_professions_no_secondary() {
    let env = env();
    let is_nil: bool = env
        .eval("local _, _, a, b, c = GetProfessions(); return a == nil and b == nil and c == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_profession_info_blacksmithing() {
    let env = env();
    let (name, skill, max_skill): (String, i32, i32) = env
        .eval("local n, _, s, m = GetProfessionInfo(1); return n, s, m")
        .unwrap();
    assert_eq!(name, "Blacksmithing");
    assert_eq!(skill, 80);
    assert_eq!(max_skill, 100);
}

#[test]
fn test_profession_info_mining() {
    let env = env();
    let (name, skill): (String, i32) = env
        .eval("local n, _, s = GetProfessionInfo(2); return n, s")
        .unwrap();
    assert_eq!(name, "Mining");
    assert_eq!(skill, 90);
}

// ============================================================================
// C_TradeSkillUI
// ============================================================================

#[test]
fn test_base_profession_info() {
    let env = env();
    let (id, name): (i32, String) = env
        .eval(
            "local i = C_TradeSkillUI.GetBaseProfessionInfo(); \
             return i.professionID, i.professionName",
        )
        .unwrap();
    assert_eq!(id, 164);
    assert_eq!(name, "Blacksmithing");
}

#[test]
fn test_child_profession_info() {
    let env = env();
    let id: i32 = env
        .eval("return C_TradeSkillUI.GetChildProfessionInfo().professionID")
        .unwrap();
    assert_eq!(id, 164);
}

#[test]
fn test_is_trade_skill_ready() {
    let env = env();
    let ready: bool = env
        .eval("return C_TradeSkillUI.IsTradeSkillReady()")
        .unwrap();
    assert!(ready);
}

#[test]
fn test_all_recipe_ids_count() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TradeSkillUI.GetAllRecipeIDs()")
        .unwrap();
    assert_eq!(count, 10);
}

#[test]
fn test_filtered_recipe_ids_nonempty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TradeSkillUI.GetFilteredRecipeIDs()")
        .unwrap();
    assert!(count > 0);
}

#[test]
fn test_recipe_info_valid() {
    let env = env();
    let (id, name, learned, craftable): (i32, String, bool, bool) = env
        .eval(
            "local r = C_TradeSkillUI.GetRecipeInfo(100001); \
             return r.recipeID, r.name, r.learned, r.craftable",
        )
        .unwrap();
    assert_eq!(id, 100001);
    assert_eq!(name, "Khaz Algar Helm");
    assert!(learned);
    assert!(craftable);
}

#[test]
fn test_recipe_info_unknown() {
    let env = env();
    let id: i32 = env
        .eval("return C_TradeSkillUI.GetRecipeInfo(999999).recipeID")
        .unwrap();
    assert_eq!(id, 0);
}

#[test]
fn test_recipe_schematic_has_reagents() {
    let env = env();
    let (id, count): (i32, i32) = env
        .eval(
            "local s = C_TradeSkillUI.GetRecipeSchematic(100001); \
             return s.recipeID, #s.reagentSlotSchematics",
        )
        .unwrap();
    assert_eq!(id, 100001);
    assert!(count > 0);
}

#[test]
fn test_recipe_schematic_unknown() {
    let env = env();
    let id: i32 = env
        .eval("return C_TradeSkillUI.GetRecipeSchematic(999999).recipeID")
        .unwrap();
    assert_eq!(id, 0);
}

#[test]
fn test_category_info() {
    let env = env();
    let (id, name): (i32, String) = env
        .eval(
            "local c = C_TradeSkillUI.GetCategoryInfo(1); \
             return c.categoryID, c.name",
        )
        .unwrap();
    assert_eq!(id, 1);
    assert_eq!(name, "Armor");
}

#[test]
fn test_category_info_unknown() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_TradeSkillUI.GetCategoryInfo(9999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_all_recipes_retrievable() {
    let env = env();
    let all_valid: bool = env
        .eval(
            "for _, id in ipairs(C_TradeSkillUI.GetAllRecipeIDs()) do \
                 local r = C_TradeSkillUI.GetRecipeInfo(id); \
                 if not r or not r.name or not r.learned then return false end \
             end; \
             return true",
        )
        .unwrap();
    assert!(all_valid);
}

#[test]
fn test_greatsword_recipe_has_three_reagents() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TradeSkillUI.GetRecipeSchematic(100005).reagentSlotSchematics")
        .unwrap();
    assert_eq!(count, 3);
}
