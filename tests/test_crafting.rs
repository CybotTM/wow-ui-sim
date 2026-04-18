//! Tests for the SimState-backed crafting surface.
//!
//! Uses recipe 100001 (Khaz Algar Helm) from BLACKSMITHING_RECIPES:
//!   reagents: item_id=210934 qty=12, item_id=210937 qty=2
//!   output_item_id: 211993

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

/// Lua helper: count total stack_count for a given item_id across bag 0 (backpack).
/// AddBagItem uses bag 0 in these tests so slots are accessible via C_Container.
const COUNT_ITEM: &str = r#"
local function count_item(item_id)
    local total = 0
    for slot = 1, C_Container.GetContainerNumSlots(0) do
        local info = C_Container.GetContainerItemInfo(0, slot)
        if info and info.itemID == item_id then
            total = total + (info.stackCount or 0)
        end
    end
    return total
end
"#;

#[test]
fn selected_profession_round_trips_through_admin_and_c_trade_skill_ui() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.SetSelectedProfession(164)
            local info = C_TradeSkillUI.GetChildProfessionInfo()
            if info == nil then return "nil_info" end
            if info.professionID ~= 164 then
                return "professionID=" .. tostring(info.professionID)
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "SetSelectedProfession(164) should be visible via GetChildProfessionInfo: {result}"
    );
}

#[test]
fn learning_a_recipe_makes_it_visible_via_is_recipe_learned() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local id = 100001
            if C_TradeSkillUI.IsRecipeLearned(id) ~= false then
                return "initial_not_false"
            end
            A_Admin.LearnRecipe(id)
            if C_TradeSkillUI.IsRecipeLearned(id) ~= true then
                return "after_learn_not_true"
            end
            A_Admin.UnlearnRecipe(id)
            if C_TradeSkillUI.IsRecipeLearned(id) ~= false then
                return "after_unlearn_not_false"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "LearnRecipe/UnlearnRecipe round-trip: {result}");
}

#[test]
fn is_recipe_craftable_returns_false_when_reagents_missing() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            A_Admin.ClearBags()
            return C_TradeSkillUI.IsRecipeCraftable(100001)
            "#,
        )
        .unwrap();
    assert!(!result, "IsRecipeCraftable should be false with empty bags");
}

#[test]
fn is_recipe_craftable_returns_true_when_all_reagents_present() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            A_Admin.ClearBags()
            -- reagents for recipe 100001: 210934 x12, 210937 x2
            A_Admin.AddBagItem(0, 1, 210934, 12)
            A_Admin.AddBagItem(0, 2, 210937, 2)
            return C_TradeSkillUI.IsRecipeCraftable(100001)
            "#,
        )
        .unwrap();
    assert!(result, "IsRecipeCraftable should be true when all reagents present");
}

#[test]
fn is_recipe_craftable_handles_count_arg() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.ClearBags()
            -- seed exactly 1 craft worth of reagents
            A_Admin.AddBagItem(0, 1, 210934, 12)
            A_Admin.AddBagItem(0, 2, 210937, 2)
            local one = C_TradeSkillUI.IsRecipeCraftable(100001, 1)
            local two = C_TradeSkillUI.IsRecipeCraftable(100001, 2)
            if one ~= true then return "one=" .. tostring(one) end
            if two ~= false then return "two=" .. tostring(two) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsRecipeCraftable with count arg: {result}");
}

#[test]
fn craft_recipe_consumes_reagents_and_adds_output() {
    let env = env();
    let result: String = env
        .eval(
            &format!(
                r#"
            {COUNT_ITEM}
            A_Admin.ClearBags()
            A_Admin.AddBagItem(0, 1, 210934, 12)
            A_Admin.AddBagItem(0, 2, 210937, 2)
            local ok = C_TradeSkillUI.CraftRecipe(100001, 1)
            if ok ~= true then return "craft_failed" end

            -- reagent 210934 should be gone
            local r1 = count_item(210934)
            if r1 ~= 0 then return "reagent_210934=" .. tostring(r1) end
            local r2 = count_item(210937)
            if r2 ~= 0 then return "reagent_210937=" .. tostring(r2) end

            -- output 211993 should be in bags (synthetic slot 99)
            local out = count_item(211993)
            if out < 1 then return "output=" .. tostring(out) end
            return "ok"
            "#,
            ),
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "CraftRecipe should consume reagents and add output: {result}"
    );
}

#[test]
fn craft_recipe_with_count_consumes_proportionally() {
    let env = env();
    let result: String = env
        .eval(
            &format!(
                r#"
            {COUNT_ITEM}
            A_Admin.ClearBags()
            -- 3× reagents
            A_Admin.AddBagItem(0, 1, 210934, 36)
            A_Admin.AddBagItem(0, 2, 210937, 6)
            local ok = C_TradeSkillUI.CraftRecipe(100001, 3)
            if ok ~= true then return "craft_failed" end

            local r1 = count_item(210934)
            if r1 ~= 0 then return "reagent_210934=" .. tostring(r1) end
            local r2 = count_item(210937)
            if r2 ~= 0 then return "reagent_210937=" .. tostring(r2) end

            local out = count_item(211993)
            if out ~= 3 then return "output=" .. tostring(out) end
            return "ok"
            "#,
            ),
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "CraftRecipe(id, 3) should produce 3 outputs and zero reagents: {result}"
    );
}

#[test]
fn craft_recipe_returns_false_and_no_op_when_reagents_missing() {
    let env = env();
    let result: String = env
        .eval(
            &format!(
                r#"
            {COUNT_ITEM}
            A_Admin.ClearBags()
            local ok = C_TradeSkillUI.CraftRecipe(100001, 1)
            if ok ~= false then return "expected_false got=" .. tostring(ok) end
            -- bags should still be empty
            local r1 = count_item(210934)
            local r2 = count_item(210937)
            local out = count_item(211993)
            if r1 ~= 0 or r2 ~= 0 or out ~= 0 then
                return "bags_changed r1=" .. r1 .. " r2=" .. r2 .. " out=" .. out
            end
            return "ok"
            "#,
            ),
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "CraftRecipe with missing reagents should be a no-op: {result}"
    );
}
