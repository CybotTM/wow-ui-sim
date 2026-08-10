//! Enum and constant globals: `Enum.*`, `Constants.*`, LE_* values.

use crate::client_profile::{ACTIVE_RETAIL_API_EPOCH, RetailApiEpoch};
use crate::lua_api::globals::enum_data::{EXPLICIT_ENUMS, SEQUENTIAL_ENUMS};
use crate::lua_api::methods::{create_table, table_get, table_set};
use rilua::LuaApiMut;
use rilua::Val;

const MISSING_ENUMS_LUA: &str = include_str!("../globals/enum_data/missing_enums.lua");
const COMPAT_ENUMS_LUA: &str = include_str!("../globals/enum_data/compat_enums.lua");
const RETAIL_12_0_0_ENUM_OVERRIDES_LUA: &str = r#"
Enum.EncounterEventFlags = {
    Disabled = 1,
}
Enum.EncounterEventFlagsMeta = {
    MaxValue = 1,
    MinValue = 1,
    NumValues = 1,
}
Enum.CooldownViewerAlertEventType.OnAuraApplied = nil
Enum.CooldownViewerAlertEventType.OnAuraRemoved = nil
Enum.CooldownViewerAlertEventTypeMeta = {
    MaxValue = 4,
    MinValue = 1,
    NumValues = 4,
}
Enum.CombatAudioAlertThrottle = {
    Sample = 0,
    PlayerHealth = 1,
    TargetHealth = 2,
    PlayerCast = 3,
    TargetCast = 4,
    PlayerResource1 = 5,
    PlayerResource2 = 6,
}
Enum.CombatAudioAlertThrottleMeta = {
    MaxValue = 6,
    MinValue = 0,
    NumValues = 7,
}
Enum.DamageMeterSpellDetailsDisplayType.Deaths = nil
Enum.DamageMeterSpellDetailsDisplayType.EnemyDamageTaken = nil
Enum.DamageMeterSpellDetailsDisplayTypeMeta.MaxValue = 2
Enum.DamageMeterSpellDetailsDisplayTypeMeta.NumValues = 3
Enum.DamageMeterStorageType.Deaths = nil
Enum.DamageMeterStorageType.EnemyDamageTaken = nil
Enum.DamageMeterStorageTypeMeta.MaxValue = 6
Enum.DamageMeterStorageTypeMeta.NumValues = 7
Enum.DamageMeterType.Deaths = nil
Enum.DamageMeterType.EnemyDamageTaken = nil
Enum.DamageMeterTypeMeta.MaxValue = 8
Enum.DamageMeterTypeMeta.NumValues = 9
Enum.EditModeAccountSetting.ShowTotemActionBar = nil
Enum.EditModeAccountSettingMeta.MaxValue = 32
Enum.EditModeAccountSettingMeta.NumValues = 33
Enum.SecretAspect.Attributes = nil
Enum.SecretAspect.CooldownStyle = nil
Enum.SecretAspectMeta = {
    MaxValue = 262144,
    MinValue = 1,
    NumValues = 24,
}
"#;
const RETAIL_12_0_0_POST_COMPAT_ENUM_OVERRIDES_LUA: &str = r#"
if Enum.ExpansionLandingPageType then
    Enum.ExpansionLandingPageType.None = nil
    Enum.ExpansionLandingPageType.Dragonflight = nil
    Enum.ExpansionLandingPageType.WarWithin = nil
end
if Enum.ExpansionLandingPageTypeMeta then
    Enum.ExpansionLandingPageTypeMeta.MaxValue = nil
    Enum.ExpansionLandingPageTypeMeta.MinValue = nil
    Enum.ExpansionLandingPageTypeMeta.NumValues = nil
end
"#;
const MISSING_CONSTANTS_LUA: &str = include_str!("../globals/enum_data/missing_constants.lua");
const CONSTANTS_VALUES_LUA: &str = include_str!("../globals/enum_data/constants_values.lua");
const COMPAT_CONSTANTS_LUA: &str = include_str!("../globals/enum_data/compat_constants.lua");

pub(crate) fn init_enum_globals(lua: &mut rilua::Lua) -> crate::Result<()> {
    {
        let state = lua.state_mut();
        let enum_table = ensure_global_table(state, "Enum");
        for &(enum_name, entries) in EXPLICIT_ENUMS.iter() {
            let enum_values = create_table(state);
            for &(variant_name, value) in entries {
                table_set(state, enum_values, variant_name, Val::Num(value as f64));
            }
            table_set(state, enum_table, enum_name, enum_values);
        }
        for &(enum_name, entries) in SEQUENTIAL_ENUMS.iter() {
            let enum_values = create_table(state);
            for (index, &variant_name) in entries.iter().enumerate() {
                table_set(state, enum_values, variant_name, Val::Num(index as f64));
            }
            table_set(state, enum_table, enum_name, enum_values);
        }
    }
    lua.exec(MISSING_ENUMS_LUA)?;
    if ACTIVE_RETAIL_API_EPOCH == RetailApiEpoch::Retail12_0_0 {
        lua.exec(RETAIL_12_0_0_ENUM_OVERRIDES_LUA)?;
    }
    lua.exec(COMPAT_ENUMS_LUA)?;
    if ACTIVE_RETAIL_API_EPOCH == RetailApiEpoch::Retail12_0_0 {
        lua.exec(RETAIL_12_0_0_POST_COMPAT_ENUM_OVERRIDES_LUA)?;
    }
    #[cfg(feature = "retail-12-1-0")]
    {
        let state = lua.state_mut();
        let enum_table = ensure_global_table(state, "Enum");
        ensure_on_update_mode_enum(state, enum_table);
    }
    lua.exec(
        r#"
        Constants = Constants or {}
        setmetatable(Constants, {
            __index = function(t, key)
                local value = {}
                rawset(t, key, value)
                return value
            end,
        })
        "#,
    )?;
    lua.exec(MISSING_CONSTANTS_LUA)?;
    lua.exec(CONSTANTS_VALUES_LUA)?;
    lua.exec(COMPAT_CONSTANTS_LUA)?;
    Ok(())
}

#[cfg(feature = "retail-12-1-0")]
fn ensure_on_update_mode_enum(state: &mut rilua::vm::state::LuaState, enum_table: Val) {
    let existing = table_get(state, enum_table, "OnUpdateMode");
    if matches!(existing, Val::Table(_)) {
        return;
    }
    let mode = create_table(state);
    for name in [
        "Disabled",
        "RunWhenVisible",
        "RunWhenVisibleOnce",
        "RunOnce",
        "RunAlways",
    ] {
        let value = crate::lua_api::methods::create_string(state, name);
        table_set(state, mode, name, value);
    }
    table_set(state, enum_table, "OnUpdateMode", mode);
    table_set(state, enum_table, "ScriptObjectOnUpdateMode", mode);
}

fn ensure_global_table(state: &mut rilua::vm::state::LuaState, key: &str) -> Val {
    let global = Val::Table(state.global);
    let existing = table_get(state, global, key);
    if matches!(existing, Val::Table(_)) {
        return existing;
    }
    let table = create_table(state);
    table_set(state, global, key, table);
    table
}
