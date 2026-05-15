use crate::lua_api::methods::create_string;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(super) fn is_valid_unit_filter(unit: &str) -> bool {
    matches!(
        unit,
        "player" | "pet" | "vehicle" | "target" | "focus" | "mouseover" | "npc" | "none"
    ) || has_numbered_unit_prefix(unit)
}

pub(super) fn push_registered_unit(state: &mut LuaState, unit: Option<&str>) {
    if let Some(unit) = unit {
        let unit = create_string(state, unit);
        state.push(unit);
    } else {
        state.push(Val::Nil);
    }
}

fn has_numbered_unit_prefix(unit: &str) -> bool {
    const PREFIXES: &[&str] = &[
        "party",
        "partypet",
        "raid",
        "raidpet",
        "boss",
        "arena",
        "arenapet",
        "nameplate",
    ];
    PREFIXES
        .iter()
        .any(|prefix| has_numeric_suffix(unit, prefix))
}

fn has_numeric_suffix(unit: &str, prefix: &str) -> bool {
    let Some(suffix) = unit.strip_prefix(prefix) else {
        return false;
    };
    !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
}
