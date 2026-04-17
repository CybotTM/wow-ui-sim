//! Tests for spell_api.rs: C_SpellBook, C_Spell, C_Traits.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// C_SpellBook
// ============================================================================

#[test]
fn test_spellbook_get_spell_name_slot1() {
    let env = env();
    let name: String = env
        .eval("return C_SpellBook.GetSpellBookItemName(1)")
        .unwrap();
    assert!(!name.is_empty(), "Slot 1 should have a spell name");
}

#[test]
fn test_spellbook_get_spell_name_nil_invalid() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_SpellBook.GetSpellBookItemName(9999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_spellbook_get_num_skill_lines() {
    let env = env();
    let count: i32 = env
        .eval("return C_SpellBook.GetNumSpellBookSkillLines()")
        .unwrap();
    assert!(count > 0, "Static spellbook data should have skill lines");
}

#[test]
fn test_spellbook_get_skill_line_info_valid() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_SpellBook.GetSpellBookSkillLineInfo(1)) == 'table'")
        .unwrap();
    assert!(is_table, "Skill line 1 should return a table");
}

#[test]
fn test_spellbook_get_skill_line_info_nil_invalid() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_SpellBook.GetSpellBookSkillLineInfo(9999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_spellbook_get_item_info_valid() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_SpellBook.GetSpellBookItemInfo(1)) == 'table'")
        .unwrap();
    assert!(is_table, "Slot 1 should return a table");
}

#[test]
fn test_spellbook_get_item_info_nil_invalid() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_SpellBook.GetSpellBookItemInfo(9999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_spellbook_get_item_texture_valid() {
    let env = env();
    let is_number: bool = env
        .eval("return type(C_SpellBook.GetSpellBookItemTexture(1, Enum.SpellBookSpellBank.Player)) == 'number'")
        .unwrap();
    assert!(
        is_number,
        "Slot 1 texture should return an icon file data id"
    );
}

#[test]
fn test_spellbook_has_pet_spells() {
    let env = env();
    let has: bool = env.eval("return C_SpellBook.HasPetSpells()").unwrap();
    assert!(!has);
}

#[test]
fn test_spellbook_namespace_functions_survive_gc() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            collectgarbage("collect")

            if type(C_SpellBook.GetNumSpellBookSkillLines) ~= "function" then
                return "missing_count_fn"
            end

            if type(C_SpellBook.GetSpellBookSkillLineInfo) ~= "function" then
                return "missing_info_fn"
            end

            local count = C_SpellBook.GetNumSpellBookSkillLines()
            if count < 1 then
                return "bad_count"
            end

            local info = C_SpellBook.GetSpellBookSkillLineInfo(1)
            if type(info) ~= "table" then
                return "bad_info"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[test]
fn test_spellbook_get_override_spell() {
    let env = env();
    let id: i32 = env.eval("return C_SpellBook.GetOverrideSpell(42)").unwrap();
    assert_eq!(id, 42, "GetOverrideSpell should return the same ID");
}

#[test]
fn test_spellbook_is_spell_known() {
    let env = env();
    let known: bool = env.eval("return C_SpellBook.IsSpellKnown(1)").unwrap();
    assert!(!known);
}

#[test]
fn test_spellbook_pickup_item_fires_cursor_changed() {
    let env = env();
    let changed_count: i32 = env
        .eval(
            r#"
            local f = CreateFrame("Frame")
            local count = 0
            f:SetScript("OnEvent", function(_, event)
                if event == "CURSOR_CHANGED" then
                    count = count + 1
                end
            end)
            f:RegisterEvent("CURSOR_CHANGED")

            C_SpellBook.PickupSpellBookItem(1)

            return count
            "#,
        )
        .unwrap();
    assert_eq!(
        changed_count, 1,
        "PickupSpellBookItem should fire CURSOR_CHANGED"
    );
}

#[test]
fn test_spellbook_get_loss_of_control_cooldown_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local info = C_SpellBook.GetSpellBookItemLossOfControlCooldownInfo(1, Enum.SpellBookSpellBank.Player)
            if type(info) ~= "table" then
                return "expected_loc_info_table"
            end
            if info.isActive ~= false then
                return "expected_inactive_loc_info"
            end
            if info.startTime ~= 0 or info.duration ~= 0 then
                return "expected_zero_loc_cooldown"
            end
            if info.modRate ~= 1 then
                return "expected_default_loc_mod_rate"
            end
            if info.shouldReplaceNormalCooldown ~= false then
                return "expected_loc_not_to_replace_normal_cooldown"
            end
            if C_SpellBook.GetSpellBookItemLossOfControlCooldownInfo(9999, Enum.SpellBookSpellBank.Player) ~= nil then
                return "invalid_spellbook_slot_should_not_have_loc_info"
            end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

// ============================================================================
// C_Spell
// ============================================================================

#[test]
fn test_spell_get_spell_info() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Spell.GetSpellInfo(100)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_spell_get_spell_info_has_name() {
    let env = env();
    let has_name: bool = env
        .eval("return C_Spell.GetSpellInfo(100).name ~= nil")
        .unwrap();
    assert!(has_name);
}

#[test]
fn test_spell_get_spell_charges() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Spell.GetSpellCharges(100)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_spell_is_spell_passive() {
    let env = env();
    let passive: bool = env.eval("return C_Spell.IsSpellPassive(100)").unwrap();
    assert!(!passive);
}

#[test]
fn test_spell_get_override_spell() {
    let env = env();
    let id: i32 = env.eval("return C_Spell.GetOverrideSpell(55)").unwrap();
    assert_eq!(id, 55);
}

#[test]
fn test_spell_get_school_string() {
    let env = env();
    // Bitmask 1 = Physical, 2 = Holy, etc.
    let school: String = env.eval("return C_Spell.GetSchoolString(1)").unwrap();
    assert!(!school.is_empty());
}

// ============================================================================
// C_AssistedCombat
// ============================================================================

#[test]
fn test_assisted_combat_rotation_spells_returns_table() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local spells = C_AssistedCombat.GetRotationSpells()
            if type(spells) ~= "table" then
                return "not_table:" .. tostring(type(spells))
            end
            local count = 0
            for _ in ipairs(spells) do
                count = count + 1
            end
            return "ok:" .. tostring(count)
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok:0");
}

#[test]
fn test_assisted_combat_is_available_shape() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local available, reason = C_AssistedCombat.IsAvailable()
            return tostring(available) .. ":" .. tostring(reason)
            "#,
        )
        .unwrap();
    assert_eq!(result, "false:Not available");
}

#[test]
fn test_spell_get_spell_texture() {
    let env = env();
    let tex: String = env.eval("return C_Spell.GetSpellTexture(100)").unwrap();
    assert!(!tex.is_empty(), "Spell texture path should be non-empty");
}

#[test]
fn test_spell_get_spell_link() {
    let env = env();
    let link: String = env.eval("return C_Spell.GetSpellLink(100)").unwrap();
    assert!(link.contains("100"), "Link should contain spell ID");
}

#[test]
fn test_spell_get_spell_name() {
    let env = env();
    // Spell 100 = "Charge"
    let name: String = env.eval("return C_Spell.GetSpellName(100)").unwrap();
    assert_eq!(name, "Charge");
}

#[test]
fn test_spell_get_spell_name_unknown() {
    let env = env();
    let name: String = env.eval("return C_Spell.GetSpellName(999999999)").unwrap();
    assert_eq!(name, "Unknown");
}

#[test]
fn test_spell_get_maw_power_border_atlas_by_spell_id_is_stubbed() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Spell.GetMawPowerBorderAtlasBySpellID(12345) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_spell_pickup_fires_cursor_changed() {
    let env = env();
    let changed_count: i32 = env
        .eval(
            r#"
            local f = CreateFrame("Frame")
            local count = 0
            f:SetScript("OnEvent", function(_, event)
                if event == "CURSOR_CHANGED" then
                    count = count + 1
                end
            end)
            f:RegisterEvent("CURSOR_CHANGED")

            C_Spell.PickupSpell(100)

            return count
            "#,
        )
        .unwrap();
    assert_eq!(changed_count, 1, "PickupSpell should fire CURSOR_CHANGED");
}

#[test]
fn test_spell_get_spell_cooldown() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Spell.GetSpellCooldown(100)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_spell_does_spell_exist() {
    let env = env();
    let exists: bool = env.eval("return C_Spell.DoesSpellExist(100)").unwrap();
    assert!(exists);
    let no_exist: bool = env.eval("return C_Spell.DoesSpellExist(0)").unwrap();
    assert!(!no_exist);
}

// ============================================================================
// C_Traits
// ============================================================================

#[test]
fn test_traits_generate_import_string() {
    let env = env();
    let s: String = env.eval("return C_Traits.GenerateImportString(1)").unwrap();
    assert!(!s.is_empty());
}

#[test]
fn test_traits_get_config_id_by_system_id() {
    let env = env();
    let id: i32 = env
        .eval("return C_Traits.GetConfigIDBySystemID(1)")
        .unwrap();
    assert_eq!(id, 1);
}

#[test]
fn test_traits_get_config_id_by_tree_id() {
    let env = env();
    let id: i32 = env.eval("return C_Traits.GetConfigIDByTreeID(1)").unwrap();
    assert_eq!(id, 1);
}

#[test]
fn test_traits_get_config_info() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Traits.GetConfigInfo(1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_traits_get_config_info_exposes_tree_ids() {
    let env = env();
    let first_tree_id: i32 = env
        .eval("return C_Traits.GetConfigInfo(201).treeIDs[1]")
        .unwrap();
    assert_eq!(first_tree_id, 790);
}

#[test]
fn test_traits_get_node_info_unknown() {
    let env = env();
    // Unknown node returns an empty info table (ID=1, all zeroed fields)
    let id: i32 = env.eval("return C_Traits.GetNodeInfo(1, 1).ID").unwrap();
    assert_eq!(id, 1);
}

#[test]
fn test_traits_get_node_info_exposes_position_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local nodeID = C_Traits.GetTreeNodes(201, 790)[1]
            local info = C_Traits.GetNodeInfo(201, nodeID)
            if type(info) ~= "table" then
                return "expected_table"
            end
            if info.posX == nil or info.posY == nil then
                return "missing_position"
            end
            if info.type == nil or info.flags == nil then
                return "missing_node_shape"
            end
            if type(info.visibleEdges) ~= "table" then
                return "missing_edges"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_traits_get_entry_info_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Traits.GetEntryInfo(1, 1) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_traits_initialize_view_loadout() {
    let env = env();
    let ok: bool = env
        .eval("return C_Traits.InitializeViewLoadout(1, 1)")
        .unwrap();
    assert!(ok);
}

#[test]
fn test_traits_get_tree_info_valid() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Traits.GetTreeInfo(1, 1)) == 'table'")
        .unwrap();
    assert!(is_table, "Tree 1 exists in TRAIT_TREE_DB");
}

#[test]
fn test_traits_get_tree_currency_info_exposes_currency_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local info = C_Traits.GetTreeCurrencyInfo(201, 790, false)
            if type(info) ~= "table" then
                return "expected_table"
            end
            if type(info[1]) ~= "table" then
                return "expected_entry"
            end
            if info[1].traitCurrencyID == nil then
                return "missing_currency_id"
            end
            if info[1].quantity == nil then
                return "missing_quantity"
            end
            if info[1].spent == nil then
                return "missing_spent"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_traits_get_trait_currency_info_returns_currency_type() {
    let env = env();
    let has_currency_type: bool = env
        .eval(
            r#"
            local _, _, currencyTypesID = C_Traits.GetTraitCurrencyInfo(2801)
            return currencyTypesID ~= nil
            "#,
        )
        .unwrap();
    assert!(has_currency_type);
}

#[test]
fn test_traits_get_tree_info_nil_invalid() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Traits.GetTreeInfo(1, 999999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_traits_get_tree_nodes_empty() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Traits.GetTreeNodes(1, 1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_traits_get_all_tree_ids_empty() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Traits.GetAllTreeIDs()) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_traits_get_trait_system_flags() {
    let env = env();
    let flags: i32 = env.eval("return C_Traits.GetTraitSystemFlags(1)").unwrap();
    assert_eq!(flags, 0);
}

#[test]
fn test_traits_can_purchase_rank() {
    let env = env();
    let can: bool = env
        .eval("return C_Traits.CanPurchaseRank(1, 1, 1)")
        .unwrap();
    assert!(!can);
}

#[test]
fn test_traits_get_loadout_serialization_version() {
    let env = env();
    let ver: i32 = env
        .eval("return C_Traits.GetLoadoutSerializationVersion()")
        .unwrap();
    assert_eq!(ver, 2);
}
