//! UnitHealPredictionCalculator table-backed proxy for WoW heal prediction API.
//!
//! Implements `CreateUnitHealPredictionCalculator()` which returns a table proxy
//! wrapping a hidden userdata. The proxy supports all heal prediction methods and
//! arbitrary per-instance field storage via the userdata's user-value table.

use crate::lua_api::proxy_helpers::{lookup_registered_method, proxy_userdata, wrap_fn_with_userdata};
use mlua::{AnyUserData, Lua, Result, UserData, UserDataMethods, Value};
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_HEAL_PRED_ID: AtomicU64 = AtomicU64::new(1);

const PROXY_MT_KEY: &str = "__unit_heal_pred_proxy_mt";
const BIND_METHOD_KEY: &str = "__unit_heal_pred_bind_method_helper";

/// Method names that are read-only (cannot be overwritten via __newindex).
const METHOD_NAMES: &[&str] = &[
    "EvaluateCurrentHealthPercent",
    "EvaluateMissingHealthPercent",
    "GetCurrentHealth",
    "GetCurrentHealthPercent",
    "GetDamageAbsorbClampMode",
    "GetDamageAbsorbs",
    "GetHealAbsorbClampMode",
    "GetHealAbsorbMode",
    "GetHealAbsorbs",
    "GetIncomingHealClampMode",
    "GetIncomingHealOverflowPercent",
    "GetIncomingHeals",
    "GetMaximumDamageAbsorbs",
    "GetMaximumHealAbsorbs",
    "GetMaximumHealth",
    "GetMaximumHealthMode",
    "GetMaximumIncomingHeals",
    "GetMissingHealth",
    "GetMissingHealthPercent",
    "GetPredictedValues",
    "GetTotalDamageAbsorbs",
    "GetTotalHealAbsorbs",
    "GetTotalIncomingHeals",
    "GetTotalIncomingHealsFromHealer",
    "HasSecretValues",
    "Reset",
    "ResetPredictedValues",
    "SetDamageAbsorbClampMode",
    "SetHealAbsorbClampMode",
    "SetHealAbsorbMode",
    "SetIncomingHealClampMode",
    "SetIncomingHealOverflowPercent",
    "SetMaximumHealthMode",
    "SetPredictedValues",
    "SetToDefaults",
];

/// Metamethod names that are read-only.
const META_NAMES: &[&str] = &["__eq", "__index", "__metatable", "__newindex", "__tostring"];

/// Per-instance state for UnitHealPredictionCalculator.
struct HealPredictionInner {
    /// Unique ID for tostring representation.
    id: u64,
    /// Stored heal prediction state.
    damage_absorb_clamp_mode: RefCell<i32>,
    heal_absorb_clamp_mode: RefCell<i32>,
    heal_absorb_mode: RefCell<i32>,
    incoming_heal_clamp_mode: RefCell<i32>,
    incoming_heal_overflow_percent: RefCell<f64>,
    incoming_heals: RefCell<f64>,
    damage_absorbs: RefCell<f64>,
    heal_absorbs: RefCell<f64>,
    maximum_health_mode: RefCell<i32>,
}

impl HealPredictionInner {
    fn new() -> Self {
        HealPredictionInner {
            id: NEXT_HEAL_PRED_ID.fetch_add(1, Ordering::Relaxed),
            damage_absorb_clamp_mode: RefCell::new(0),
            heal_absorb_clamp_mode: RefCell::new(0),
            heal_absorb_mode: RefCell::new(0),
            incoming_heal_clamp_mode: RefCell::new(0),
            incoming_heal_overflow_percent: RefCell::new(0.0),
            incoming_heals: RefCell::new(0.0),
            damage_absorbs: RefCell::new(0.0),
            heal_absorbs: RefCell::new(0.0),
            maximum_health_mode: RefCell::new(0),
        }
    }
}

/// WoW UnitHealPredictionCalculator userdata object.
///
/// Tracks heal prediction values (incoming heals, absorbs, overflow) for a unit.
/// Arbitrary field storage is supported via the proxy's user-value table.
/// Methods are read-only (assignment fails with an error message).
pub struct UnitHealPredictionCalculator {
    inner: HealPredictionInner,
}

impl UnitHealPredictionCalculator {
    fn new() -> Self {
        UnitHealPredictionCalculator {
            inner: HealPredictionInner::new(),
        }
    }
}

impl UserData for UnitHealPredictionCalculator {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_getter_methods(methods);
        add_setter_methods(methods);
    }
}

fn add_getter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    add_health_query_methods(methods);
    add_stored_getter_methods(methods);
    add_total_getter_methods(methods);
    methods.add_function("GetPredictedValues", |_, (ud, _unit): (AnyUserData, Value)| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        let incoming = *this.inner.incoming_heals.borrow();
        let absorbs = *this.inner.heal_absorbs.borrow();
        let damage_absorbs = *this.inner.damage_absorbs.borrow();
        Ok((incoming, absorbs, damage_absorbs))
    });
    methods.add_function("HasSecretValues", |_, _ud: AnyUserData| Ok(false));
}

fn add_health_query_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_function(
        "EvaluateCurrentHealthPercent",
        |_, (ud, _unit): (AnyUserData, Value)| {
            let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
            Ok(1.0f64)
        },
    );
    methods.add_function(
        "EvaluateMissingHealthPercent",
        |_, (ud, _unit): (AnyUserData, Value)| {
            let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
            Ok(0.0f64)
        },
    );
    methods.add_function("GetCurrentHealth", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(0.0f64)
    });
    methods.add_function("GetCurrentHealthPercent", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(1.0f64)
    });
    methods.add_function("GetMaximumHealth", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(0.0f64)
    });
    methods.add_function("GetMaximumHealthMode", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.maximum_health_mode.borrow())
    });
    methods.add_function("GetMissingHealth", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(0.0f64)
    });
    methods.add_function("GetMissingHealthPercent", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(0.0f64)
    });
}

fn add_stored_getter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_function("GetDamageAbsorbClampMode", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.damage_absorb_clamp_mode.borrow())
    });
    methods.add_function("GetDamageAbsorbs", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.damage_absorbs.borrow())
    });
    methods.add_function("GetHealAbsorbClampMode", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.heal_absorb_clamp_mode.borrow())
    });
    methods.add_function("GetHealAbsorbMode", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.heal_absorb_mode.borrow())
    });
    methods.add_function("GetHealAbsorbs", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.heal_absorbs.borrow())
    });
    methods.add_function("GetIncomingHealClampMode", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.incoming_heal_clamp_mode.borrow())
    });
    methods.add_function("GetIncomingHealOverflowPercent", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.incoming_heal_overflow_percent.borrow())
    });
    methods.add_function("GetIncomingHeals", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.incoming_heals.borrow())
    });
}

fn add_total_getter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_function("GetMaximumDamageAbsorbs", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(0.0f64)
    });
    methods.add_function("GetMaximumHealAbsorbs", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(0.0f64)
    });
    methods.add_function("GetMaximumIncomingHeals", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(0.0f64)
    });
    methods.add_function("GetTotalDamageAbsorbs", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.damage_absorbs.borrow())
    });
    methods.add_function("GetTotalHealAbsorbs", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.heal_absorbs.borrow())
    });
    methods.add_function("GetTotalIncomingHeals", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(*this.inner.incoming_heals.borrow())
    });
    methods.add_function("GetTotalIncomingHealsFromHealer", |_, ud: AnyUserData| {
        let _this = ud.borrow::<UnitHealPredictionCalculator>()?;
        Ok(0.0f64)
    });
}

fn add_setter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    add_reset_methods(methods);
    add_mode_setter_methods(methods);
    add_value_setter_methods(methods);
}

fn add_reset_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_function("Reset", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        reset_all_prediction_state(&this.inner);
        Ok(())
    });
    methods.add_function("ResetPredictedValues", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        reset_predicted_values(&this.inner);
        Ok(())
    });
    methods.add_function("SetToDefaults", |_, ud: AnyUserData| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        reset_all_prediction_state(&this.inner);
        Ok(())
    });
}

fn add_mode_setter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_function(
        "SetDamageAbsorbClampMode",
        |_, (ud, mode): (AnyUserData, i32)| {
            let this = ud.borrow::<UnitHealPredictionCalculator>()?;
            *this.inner.damage_absorb_clamp_mode.borrow_mut() = mode;
            Ok(())
        },
    );
    methods.add_function(
        "SetHealAbsorbClampMode",
        |_, (ud, mode): (AnyUserData, i32)| {
            let this = ud.borrow::<UnitHealPredictionCalculator>()?;
            *this.inner.heal_absorb_clamp_mode.borrow_mut() = mode;
            Ok(())
        },
    );
    methods.add_function("SetHealAbsorbMode", |_, (ud, mode): (AnyUserData, i32)| {
        let this = ud.borrow::<UnitHealPredictionCalculator>()?;
        *this.inner.heal_absorb_mode.borrow_mut() = mode;
        Ok(())
    });
    methods.add_function(
        "SetIncomingHealClampMode",
        |_, (ud, mode): (AnyUserData, i32)| {
            let this = ud.borrow::<UnitHealPredictionCalculator>()?;
            *this.inner.incoming_heal_clamp_mode.borrow_mut() = mode;
            Ok(())
        },
    );
    methods.add_function(
        "SetMaximumHealthMode",
        |_, (ud, mode): (AnyUserData, i32)| {
            let this = ud.borrow::<UnitHealPredictionCalculator>()?;
            *this.inner.maximum_health_mode.borrow_mut() = mode;
            Ok(())
        },
    );
}

fn add_value_setter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_function(
        "SetIncomingHealOverflowPercent",
        |_, (ud, pct): (AnyUserData, f64)| {
            let this = ud.borrow::<UnitHealPredictionCalculator>()?;
            *this.inner.incoming_heal_overflow_percent.borrow_mut() = pct;
            Ok(())
        },
    );
    methods.add_function(
        "SetPredictedValues",
        |_,
         (ud, _unit, incoming, absorbs, damage_absorbs): (
            AnyUserData,
            Value,
            f64,
            f64,
            f64,
        )| {
            let this = ud.borrow::<UnitHealPredictionCalculator>()?;
            *this.inner.incoming_heals.borrow_mut() = incoming;
            *this.inner.heal_absorbs.borrow_mut() = absorbs;
            *this.inner.damage_absorbs.borrow_mut() = damage_absorbs;
            Ok(())
        },
    );
}

fn reset_predicted_values(inner: &HealPredictionInner) {
    *inner.incoming_heals.borrow_mut() = 0.0;
    *inner.damage_absorbs.borrow_mut() = 0.0;
    *inner.heal_absorbs.borrow_mut() = 0.0;
}

fn reset_all_prediction_state(inner: &HealPredictionInner) {
    reset_predicted_values(inner);
    *inner.incoming_heal_overflow_percent.borrow_mut() = 0.0;
    *inner.damage_absorb_clamp_mode.borrow_mut() = 0;
    *inner.heal_absorb_clamp_mode.borrow_mut() = 0;
    *inner.heal_absorb_mode.borrow_mut() = 0;
    *inner.incoming_heal_clamp_mode.borrow_mut() = 0;
    *inner.maximum_health_mode.borrow_mut() = 0;
}

fn ensure_proxy_support(lua: &Lua) -> Result<()> {
    register_bind_method_helper(lua)?;
    install_proxy_metatable(lua)
}

fn register_bind_method_helper(lua: &Lua) -> Result<()> {
    if lua
        .named_registry_value::<mlua::Function>(BIND_METHOD_KEY)
        .is_ok()
    {
        return Ok(());
    }
    lua.set_named_registry_value(
        BIND_METHOD_KEY,
        crate::lua_api::cfunc_wrap::create_bind_factory(lua)?,
    )
}

fn install_proxy_metatable(lua: &Lua) -> Result<()> {
    if lua
        .named_registry_value::<mlua::Table>(PROXY_MT_KEY)
        .is_ok()
    {
        return Ok(());
    }
    let mt = create_proxy_metatable(lua)?;
    lua.set_named_registry_value(PROXY_MT_KEY, mt)
}

fn create_proxy(lua: &Lua, userdata: mlua::AnyUserData) -> Result<Value> {
    userdata.set_user_value(lua.create_table()?)?;
    let proxy = lua.create_table()?;
    proxy.raw_set("__lud", userdata)?;
    let mt: mlua::Table = lua.named_registry_value(PROXY_MT_KEY)?;
    proxy.set_metatable(Some(mt));
    Ok(Value::Table(proxy))
}

fn create_proxy_metatable(lua: &Lua) -> Result<mlua::Table> {
    let mt = lua.create_table()?;
    mt.raw_set("__index", create_proxy_index(lua)?)?;
    mt.raw_set("__newindex", create_proxy_newindex(lua)?)?;
    mt.raw_set("__tostring", create_proxy_tostring(lua)?)?;
    Ok(mt)
}

fn create_proxy_index(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|lua, (this, key): (mlua::Table, Value)| {
        // Metamethod names always return nil.
        if let Value::String(ref s) = key {
            if s.to_string_lossy().starts_with("__") {
                return Ok(Value::Nil);
            }
        }

        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok(Value::Nil);
        };

        // Check per-instance fields first.
        if let Ok(fields) = userdata.user_value::<mlua::Table>() {
            let field_value: Value = fields.raw_get(key.clone())?;
            if !field_value.is_nil() {
                return Ok(field_value);
            }
        }

        // Fall back to registered methods on the userdata metatable.
        let registered = lookup_registered_method(&userdata, &key)?;
        if let Value::Function(function) = registered {
            return Ok(Value::Function(wrap_fn_with_userdata(
                lua, function, userdata, BIND_METHOD_KEY,
            )?));
        }
        Ok(registered)
    })
}

fn create_proxy_newindex(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|_, (this, key, value): (mlua::Table, Value, Value)| {
        // Reject writes to method and metamethod names.
        if let Value::String(ref s) = key {
            let key_str = s.to_string_lossy();
            if is_readonly_key(&key_str) {
                return Err(mlua::Error::RuntimeError(format!(
                    "Attempted to assign to read-only key {}",
                    key_str
                )));
            }
        }

        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok(());
        };
        let fields: mlua::Table = userdata.user_value()?;
        fields.raw_set(key, value)?;
        Ok(())
    })
}

fn create_proxy_tostring(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|_, this: mlua::Table| {
        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok("UnitHealPredictionCalculator: 0x0000000000000000".to_string());
        };
        let id = userdata
            .borrow::<UnitHealPredictionCalculator>()
            .map(|c| c.inner.id)
            .unwrap_or(0);
        Ok(format!("UnitHealPredictionCalculator: 0x{:016x}", id))
    })
}

fn is_readonly_key(key: &str) -> bool {
    METHOD_NAMES.contains(&key) || META_NAMES.contains(&key)
}

/// Register `CreateUnitHealPredictionCalculator` in the Lua globals.
pub fn register_unit_heal_prediction(lua: &Lua) -> Result<()> {
    ensure_proxy_support(lua)?;
    lua.globals().set(
        "CreateUnitHealPredictionCalculator",
        lua.create_function(|lua, ()| {
            ensure_proxy_support(lua)?;
            let userdata = lua.create_userdata(UnitHealPredictionCalculator::new())?;
            create_proxy(lua, userdata)
        })?,
    )
}
