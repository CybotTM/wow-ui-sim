//! Tests for profession spells exposed through C_Spell and C_SpellBook.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

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
