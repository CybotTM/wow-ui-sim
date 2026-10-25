//! Finite PTR 70493 declarations used by original I4 consumers.
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{create_table, table_set};
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register(state: &mut LuaState) {
    if ACTIVE != ClientProfile::Ptr {
        return;
    }
    let enums = super::helpers::ensure_global_table(state, "Enum");
    register_usage_locations(state, enums);
    register_recent_allies_categories(state, enums);
}

fn register_usage_locations(state: &mut LuaState, enums: Val) {
    // 70493 GameRulesDocumentation.lua:467–477.
    publish(
        state,
        enums,
        "SdToggleUsageLocationUI",
        &[
            ("BeforeLogin", 0),
            ("CharacterCreation", 1),
            ("BarberShop", 2),
            ("OptionsMenu", 3),
        ],
    );
    publish(
        state,
        enums,
        "SdToggleUsageLocationUIMeta",
        &[("MinValue", 0), ("MaxValue", 3), ("NumValues", 4)],
    );
}

fn register_recent_allies_categories(state: &mut LuaState, enums: Val) {
    // 70493 RecentAlliesConstantsDocumentation.lua:6–18.
    publish(
        state,
        enums,
        "RecentAlliesInteractionCategoryFilter",
        &[
            ("Professions", 0),
            ("PvP", 1),
            ("Raiding", 2),
            ("Dungeons", 3),
            ("Delves", 4),
            ("Questing", 5),
        ],
    );
    publish(
        state,
        enums,
        "RecentAlliesInteractionCategoryFilterMeta",
        &[("MinValue", 0), ("MaxValue", 5), ("NumValues", 6)],
    );
}

fn publish(state: &mut LuaState, enums: Val, name: &str, fields: &[(&str, i32)]) {
    let table = create_table(state);
    for &(field, value) in fields {
        table_set(state, table, field, Val::Num(f64::from(value)));
    }
    table_set(state, enums, name, table);
}
