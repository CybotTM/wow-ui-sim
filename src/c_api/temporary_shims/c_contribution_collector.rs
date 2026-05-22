//! Temporary `C_ContributionCollector` fallback surface.
//!
//! Contribution collectors are not modeled yet. These methods preserve the
//! empty/default return shapes Blizzard startup code expects until a backing
//! contribution state model exists.

use crate::c_api::ensure_namespace;
use crate::c_api::helpers::global_val;
use crate::lua_api::methods::{create_string, create_table, table_get, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

type LuaTableRef = GcRef<Table>;

pub(crate) fn register_c_contribution_collector(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ContributionCollector")?;
    register_lifecycle_methods(state, ns)?;
    register_query_methods(state, ns)?;
    register_reward_methods(state, ns)?;
    Ok(())
}

fn register_lifecycle_methods(state: &mut LuaState, ns: LuaTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, ns, "Close", contribution_noop)?;
    table_set_rust_fn_static(state, ns, "Contribute", contribution_noop)?;
    table_set_rust_fn_static(state, ns, "GetActive", contribution_no_result)?;
    table_set_rust_fn_static(state, ns, "HasPendingContribution", contribution_false)?;
    table_set_rust_fn_static(state, ns, "IsAwaitingRewardQuestData", contribution_false)?;
    Ok(())
}

fn register_query_methods(state: &mut LuaState, ns: LuaTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, ns, "GetAtlases", contribution_empty_table)?;
    table_set_rust_fn_static(state, ns, "GetBuffs", contribution_no_result)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetContributionAppearance",
        contribution_get_appearance,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetContributionCollectorsForMap",
        contribution_empty_table,
    )?;
    table_set_rust_fn_static(state, ns, "GetContributionResult", contribution_get_result)?;
    table_set_rust_fn_static(state, ns, "GetDescription", contribution_empty_string)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetManagedContributionsForCreatureID",
        contribution_empty_table,
    )?;
    table_set_rust_fn_static(state, ns, "GetName", contribution_empty_string)?;
    table_set_rust_fn_static(state, ns, "GetOrderIndex", contribution_get_order_index)?;
    table_set_rust_fn_static(state, ns, "GetState", contribution_get_state)?;
    Ok(())
}

fn register_reward_methods(state: &mut LuaState, ns: LuaTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        ns,
        "GetRequiredContributionCurrency",
        contribution_no_result,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetRequiredContributionItem",
        contribution_no_result,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetRewardQuestID",
        contribution_get_reward_quest_id,
    )?;
    Ok(())
}

fn contribution_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn contribution_no_result(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn contribution_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn contribution_get_appearance(state: &mut LuaState) -> LuaResult<u32> {
    let appearance = create_table(state);
    let empty = create_string(state, "");
    table_set(state, appearance, "stateName", empty);
    let color = default_color(state)?;
    table_set(state, appearance, "stateColor", color);
    let empty = create_string(state, "");
    table_set(state, appearance, "tooltipLine", empty);
    table_set(
        state,
        appearance,
        "tooltipUseTimeRemaining",
        Val::Bool(false),
    );
    let empty = create_string(state, "");
    table_set(state, appearance, "statusBarAtlas", empty);
    let empty = create_string(state, "");
    table_set(state, appearance, "borderAtlas", empty);
    let empty = create_string(state, "");
    table_set(state, appearance, "bannerAtlas", empty);
    state.push(appearance);
    Ok(1)
}

fn default_color(state: &mut LuaState) -> LuaResult<Val> {
    let color = create_table(state);
    let Val::Table(color_ref) = color else {
        unreachable!("create_table must return table");
    };
    table_set_rust_fn_static(state, color_ref, "GetRGB", color_get_rgb)?;
    table_set_rust_fn_static(state, color_ref, "GetRGBA", color_get_rgba)?;
    Ok(Val::Table(color_ref))
}

fn color_get_rgb(state: &mut LuaState) -> LuaResult<u32> {
    push_rgb(state);
    Ok(3)
}

fn color_get_rgba(state: &mut LuaState) -> LuaResult<u32> {
    push_rgb(state);
    state.push(Val::Num(1.0));
    Ok(4)
}

fn push_rgb(state: &mut LuaState) {
    state.push(Val::Num(1.0));
    state.push(Val::Num(1.0));
    state.push(Val::Num(1.0));
}

fn contribution_get_result(state: &mut LuaState) -> LuaResult<u32> {
    let result = enum_variant_number(state, "ContributionResult", "Success", 0.0);
    state.push(Val::Num(result));
    Ok(1)
}

fn contribution_empty_string(state: &mut LuaState) -> LuaResult<u32> {
    let value = create_string(state, "");
    state.push(value);
    Ok(1)
}

fn contribution_get_order_index(state: &mut LuaState) -> LuaResult<u32> {
    let contribution_id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    state.push(Val::Num(contribution_id));
    Ok(1)
}

fn contribution_get_reward_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn contribution_get_state(state: &mut LuaState) -> LuaResult<u32> {
    let none = enum_variant_number(state, "ContributionState", "None", 0.0);
    state.push(Val::Num(none));
    state.push(Val::Num(0.0));
    state.push(Val::Nil);
    state.push(Val::Num(0.0));
    Ok(4)
}

fn contribution_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn enum_variant_number(
    state: &mut LuaState,
    enum_name: &'static str,
    variant: &'static str,
    fallback: f64,
) -> f64 {
    let enum_root = global_val(state, "Enum");
    let enum_table = table_get(state, enum_root, enum_name);
    match table_get(state, enum_table, variant) {
        Val::Num(value) => value,
        _ => fallback,
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn contribution_collector_returns_empty_default_shapes() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: (bool, String, bool, bool, i32, i32, bool) = env
            .eval(
                r#"
                local appearance = C_ContributionCollector.GetContributionAppearance(1)
                local r, g, b, a = appearance.stateColor:GetRGBA()
                local state, _, itemName, _ = C_ContributionCollector.GetState(1)
                return
                    type(C_ContributionCollector.GetAtlases(1)) == "table",
                    appearance.stateName,
                    appearance.tooltipUseTimeRemaining,
                    r == 1 and g == 1 and b == 1 and a == 1,
                    C_ContributionCollector.GetOrderIndex(17),
                    C_ContributionCollector.GetRewardQuestID(1),
                    itemName == nil and state == Enum.ContributionState.None
                "#,
            )
            .expect("contribution collector defaults should be readable");

        assert_eq!(result, (true, String::new(), false, true, 17, 0, true));
    }

    #[test]
    fn contribution_collector_reports_no_active_or_pending_work() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: (bool, bool, bool, bool) = env
            .eval(
                r#"
                return
                    C_ContributionCollector.GetActive() == nil,
                    C_ContributionCollector.GetRequiredContributionCurrency(1) == nil,
                    C_ContributionCollector.HasPendingContribution(1),
                    C_ContributionCollector.IsAwaitingRewardQuestData(1)
                "#,
            )
            .expect("contribution collector no-op methods should be readable");

        assert_eq!(result, (true, true, false, false));
    }
}
