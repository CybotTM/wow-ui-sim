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

#[test]
fn test_profession_info_exposes_spell_slots_for_professions_book_buttons() {
    let env = env();
    let (num_spells_1, num_spells_2, bs_spell_id, mining_spell_id): (i32, i32, i32, i32) = env
        .eval(
            r#"
            local _, _, _, _, numSpells1, spellOffset1 = GetProfessionInfo(1)
            local _, _, _, _, numSpells2, spellOffset2 = GetProfessionInfo(2)

            local bsInfo = C_SpellBook.GetSpellBookItemInfo(spellOffset1 + 1, Enum.SpellBookSpellBank.Player)
            local miningInfo = C_SpellBook.GetSpellBookItemInfo(spellOffset2 + 1, Enum.SpellBookSpellBank.Player)
            return numSpells1, numSpells2, bsInfo and bsInfo.spellID or 0, miningInfo and miningInfo.spellID or 0
            "#,
        )
        .unwrap();

    assert!(num_spells_1 > 0);
    assert!(num_spells_2 > 0);
    assert_eq!(bs_spell_id, 2018);
    assert_eq!(mining_spell_id, 2575);
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

// ============================================================================
// Profession spells in SPELL_DB
// ============================================================================

#[test]
fn test_blacksmithing_spell_exists() {
    let env = env();
    let name: String = env.eval("return C_Spell.GetSpellInfo(2018).name").unwrap();
    assert_eq!(name, "Blacksmithing");
}

#[test]
fn test_smelt_copper_spell_exists() {
    let env = env();
    let name: String = env.eval("return C_Spell.GetSpellInfo(2657).name").unwrap();
    assert_eq!(name, "Smelt Copper");
}

#[test]
fn test_mining_spell_exists() {
    let env = env();
    let name: String = env.eval("return C_Spell.GetSpellInfo(2575).name").unwrap();
    assert_eq!(name, "Mining");
}

#[test]
fn test_mining_passive_spell_exists() {
    let env = env();
    let name: String = env.eval("return C_Spell.GetSpellInfo(2576).name").unwrap();
    assert_eq!(name, "Mining");
}

#[test]
fn test_blacksmithing_skill_line_in_spellbook() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            local n = C_SpellBook.GetNumSpellBookSkillLines()
            for i = 1, n do
                local info = C_SpellBook.GetSpellBookSkillLineInfo(i)
                if info.name == "Blacksmithing" then return info.name end
            end
            return ""
            "#,
        )
        .unwrap();
    assert_eq!(name, "Blacksmithing");
}

#[test]
fn test_mining_skill_line_in_spellbook() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            local n = C_SpellBook.GetNumSpellBookSkillLines()
            for i = 1, n do
                local info = C_SpellBook.GetSpellBookSkillLineInfo(i)
                if info.name == "Mining" then return info.name end
            end
            return ""
            "#,
        )
        .unwrap();
    assert_eq!(name, "Mining");
}

#[test]
fn test_spellbook_item_info_blacksmithing_slot() {
    let env = env();
    let (name, spell_id): (String, i32) = env
        .eval(
            r#"
            local n = C_SpellBook.GetNumSpellBookSkillLines()
            for i = 1, n do
                local line = C_SpellBook.GetSpellBookSkillLineInfo(i)
                if line.name == "Blacksmithing" then
                    local slot = line.itemIndexOffset + 1
                    local info = C_SpellBook.GetSpellBookItemInfo(slot)
                    return info.name, info.spellID
                end
            end
            return "", 0
            "#,
        )
        .unwrap();
    assert_eq!(name, "Blacksmithing");
    assert_eq!(spell_id, 2018);
}

#[test]
fn test_spellbook_item_info_mining_passive() {
    let env = env();
    let (name, is_passive): (String, bool) = env
        .eval(
            r#"
            local n = C_SpellBook.GetNumSpellBookSkillLines()
            for i = 1, n do
                local line = C_SpellBook.GetSpellBookSkillLineInfo(i)
                if line.name == "Mining" then
                    local slot = line.itemIndexOffset + 2
                    local info = C_SpellBook.GetSpellBookItemInfo(slot)
                    return info.name, info.isPassive
                end
            end
            return "", false
            "#,
        )
        .unwrap();
    assert_eq!(name, "Mining");
    assert!(is_passive);
}
