//! UnitHealPredictionCalculator UserData type for WoW heal prediction API.
//!
//! Implements `CreateUnitHealPredictionCalculator()` which returns a UserData
//! object with methods for querying and setting heal prediction values.
//!
//! The metatable is automatically hidden by mlua (`getmetatable` returns `false`).

use mlua::{AnyUserData, Lua, MetaMethod, Result, UserData, UserDataMethods, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_HEAL_PRED_ID: AtomicU64 = AtomicU64::new(1);

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

/// Per-instance state for UnitHealPredictionCalculator.
struct HealPredictionInner {
    /// Unique ID for tostring representation.
    id: u64,
    /// Per-instance user field storage.
    fields: RefCell<HashMap<String, Value>>,
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
    fn new() -> Rc<Self> {
        Rc::new(HealPredictionInner {
            id: NEXT_HEAL_PRED_ID.fetch_add(1, Ordering::Relaxed),
            fields: RefCell::new(HashMap::new()),
            damage_absorb_clamp_mode: RefCell::new(0),
            heal_absorb_clamp_mode: RefCell::new(0),
            heal_absorb_mode: RefCell::new(0),
            incoming_heal_clamp_mode: RefCell::new(0),
            incoming_heal_overflow_percent: RefCell::new(0.0),
            incoming_heals: RefCell::new(0.0),
            damage_absorbs: RefCell::new(0.0),
            heal_absorbs: RefCell::new(0.0),
            maximum_health_mode: RefCell::new(0),
        })
    }
}

/// WoW UnitHealPredictionCalculator userdata object.
///
/// Tracks heal prediction values (incoming heals, absorbs, overflow) for a unit.
/// Arbitrary field storage is supported via `__index`/`__newindex`.
/// Methods are read-only (assignment fails with WoW's error message).
pub struct UnitHealPredictionCalculator {
    inner: Rc<HealPredictionInner>,
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
        add_index_metamethod(methods);
        add_newindex_metamethod(methods);
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(format!(
                "UnitHealPredictionCalculator: 0x{:016x}",
                this.inner.id
            ))
        });
    }
}

fn add_getter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    add_health_query_methods(methods);
    add_stored_getter_methods(methods);
    add_total_getter_methods(methods);
    methods.add_method("GetPredictedValues", |_, this, _unit: Value| {
        let incoming = *this.inner.incoming_heals.borrow();
        let absorbs = *this.inner.heal_absorbs.borrow();
        let damage_absorbs = *this.inner.damage_absorbs.borrow();
        Ok((incoming, absorbs, damage_absorbs))
    });
    methods.add_method("HasSecretValues", |_, _, ()| Ok(false));
}

fn add_health_query_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_method("EvaluateCurrentHealthPercent", |_, _, _unit: Value| Ok(1.0f64));
    methods.add_method("EvaluateMissingHealthPercent", |_, _, _unit: Value| Ok(0.0f64));
    methods.add_method("GetCurrentHealth", |_, _, ()| Ok(0.0f64));
    methods.add_method("GetCurrentHealthPercent", |_, _, ()| Ok(1.0f64));
    methods.add_method("GetMaximumHealth", |_, _, ()| Ok(0.0f64));
    methods.add_method("GetMaximumHealthMode", |_, this, ()| {
        Ok(*this.inner.maximum_health_mode.borrow())
    });
    methods.add_method("GetMissingHealth", |_, _, ()| Ok(0.0f64));
    methods.add_method("GetMissingHealthPercent", |_, _, ()| Ok(0.0f64));
}

fn add_stored_getter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_method("GetDamageAbsorbClampMode", |_, this, ()| {
        Ok(*this.inner.damage_absorb_clamp_mode.borrow())
    });
    methods.add_method("GetDamageAbsorbs", |_, this, ()| {
        Ok(*this.inner.damage_absorbs.borrow())
    });
    methods.add_method("GetHealAbsorbClampMode", |_, this, ()| {
        Ok(*this.inner.heal_absorb_clamp_mode.borrow())
    });
    methods.add_method("GetHealAbsorbMode", |_, this, ()| {
        Ok(*this.inner.heal_absorb_mode.borrow())
    });
    methods.add_method("GetHealAbsorbs", |_, this, ()| {
        Ok(*this.inner.heal_absorbs.borrow())
    });
    methods.add_method("GetIncomingHealClampMode", |_, this, ()| {
        Ok(*this.inner.incoming_heal_clamp_mode.borrow())
    });
    methods.add_method("GetIncomingHealOverflowPercent", |_, this, ()| {
        Ok(*this.inner.incoming_heal_overflow_percent.borrow())
    });
    methods.add_method("GetIncomingHeals", |_, this, ()| {
        Ok(*this.inner.incoming_heals.borrow())
    });
}

fn add_total_getter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_method("GetMaximumDamageAbsorbs", |_, _, ()| Ok(0.0f64));
    methods.add_method("GetMaximumHealAbsorbs", |_, _, ()| Ok(0.0f64));
    methods.add_method("GetMaximumIncomingHeals", |_, _, ()| Ok(0.0f64));
    methods.add_method("GetTotalDamageAbsorbs", |_, this, ()| {
        Ok(*this.inner.damage_absorbs.borrow())
    });
    methods.add_method("GetTotalHealAbsorbs", |_, this, ()| {
        Ok(*this.inner.heal_absorbs.borrow())
    });
    methods.add_method("GetTotalIncomingHeals", |_, this, ()| {
        Ok(*this.inner.incoming_heals.borrow())
    });
    methods.add_method("GetTotalIncomingHealsFromHealer", |_, _, ()| Ok(0.0f64));
}

fn add_setter_methods<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_method("Reset", |_, this, ()| {
        *this.inner.incoming_heals.borrow_mut() = 0.0;
        *this.inner.damage_absorbs.borrow_mut() = 0.0;
        *this.inner.heal_absorbs.borrow_mut() = 0.0;
        *this.inner.incoming_heal_overflow_percent.borrow_mut() = 0.0;
        *this.inner.damage_absorb_clamp_mode.borrow_mut() = 0;
        *this.inner.heal_absorb_clamp_mode.borrow_mut() = 0;
        *this.inner.heal_absorb_mode.borrow_mut() = 0;
        *this.inner.incoming_heal_clamp_mode.borrow_mut() = 0;
        *this.inner.maximum_health_mode.borrow_mut() = 0;
        Ok(())
    });
    methods.add_method("ResetPredictedValues", |_, this, ()| {
        *this.inner.incoming_heals.borrow_mut() = 0.0;
        *this.inner.damage_absorbs.borrow_mut() = 0.0;
        *this.inner.heal_absorbs.borrow_mut() = 0.0;
        Ok(())
    });
    methods.add_method("SetDamageAbsorbClampMode", |_, this, mode: i32| {
        *this.inner.damage_absorb_clamp_mode.borrow_mut() = mode;
        Ok(())
    });
    methods.add_method("SetHealAbsorbClampMode", |_, this, mode: i32| {
        *this.inner.heal_absorb_clamp_mode.borrow_mut() = mode;
        Ok(())
    });
    methods.add_method("SetHealAbsorbMode", |_, this, mode: i32| {
        *this.inner.heal_absorb_mode.borrow_mut() = mode;
        Ok(())
    });
    methods.add_method("SetIncomingHealClampMode", |_, this, mode: i32| {
        *this.inner.incoming_heal_clamp_mode.borrow_mut() = mode;
        Ok(())
    });
    methods.add_method("SetIncomingHealOverflowPercent", |_, this, pct: f64| {
        *this.inner.incoming_heal_overflow_percent.borrow_mut() = pct;
        Ok(())
    });
    methods.add_method("SetMaximumHealthMode", |_, this, mode: i32| {
        *this.inner.maximum_health_mode.borrow_mut() = mode;
        Ok(())
    });
    methods.add_method("SetPredictedValues", |_, this, (unit, incoming, absorbs, damage_absorbs): (Value, f64, f64, f64)| {
        let _ = unit;
        *this.inner.incoming_heals.borrow_mut() = incoming;
        *this.inner.heal_absorbs.borrow_mut() = absorbs;
        *this.inner.damage_absorbs.borrow_mut() = damage_absorbs;
        Ok(())
    });
    methods.add_method("SetToDefaults", |_, this, ()| {
        *this.inner.incoming_heals.borrow_mut() = 0.0;
        *this.inner.damage_absorbs.borrow_mut() = 0.0;
        *this.inner.heal_absorbs.borrow_mut() = 0.0;
        *this.inner.incoming_heal_overflow_percent.borrow_mut() = 0.0;
        *this.inner.damage_absorb_clamp_mode.borrow_mut() = 0;
        *this.inner.heal_absorb_clamp_mode.borrow_mut() = 0;
        *this.inner.heal_absorb_mode.borrow_mut() = 0;
        *this.inner.incoming_heal_clamp_mode.borrow_mut() = 0;
        *this.inner.maximum_health_mode.borrow_mut() = 0;
        Ok(())
    });
}

fn add_index_metamethod<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_meta_function(
        MetaMethod::Index,
        |_lua: &Lua, (ud, key): (AnyUserData, Value)| {
            let calc = ud.borrow::<UnitHealPredictionCalculator>()?;
            let inner = Rc::clone(&calc.inner);
            drop(calc);

            let key_str = match &key {
                Value::String(s) => s.to_string_lossy().to_string(),
                _ => return Ok(Value::Nil),
            };

            // Metamethods are not exposed through __index
            if key_str.starts_with("__") {
                return Ok(Value::Nil);
            }

            // Method names: mlua's generated __index checks methods table first,
            // then calls our __index. So we only reach here for non-method keys.
            let fields = inner.fields.borrow();
            Ok(fields.get(&key_str).cloned().unwrap_or(Value::Nil))
        },
    );
}

fn add_newindex_metamethod<M: UserDataMethods<UnitHealPredictionCalculator>>(methods: &mut M) {
    methods.add_meta_function(
        MetaMethod::NewIndex,
        |_lua: &Lua, (ud, key, value): (AnyUserData, String, Value)| {
            let calc = ud.borrow::<UnitHealPredictionCalculator>()?;
            let inner = Rc::clone(&calc.inner);
            drop(calc);

            // Block method name assignment
            if METHOD_NAMES.contains(&key.as_str()) {
                return Err(mlua::Error::RuntimeError(format!(
                    "Attempted to assign to read-only key {}",
                    key
                )));
            }

            // Block metamethod assignment
            if key.starts_with("__") {
                return Err(mlua::Error::RuntimeError(format!(
                    "Attempted to assign to read-only key {}",
                    key
                )));
            }

            // Store or remove from per-instance field table
            let mut fields = inner.fields.borrow_mut();
            if let Value::Nil = value {
                fields.remove(&key);
            } else {
                fields.insert(key, value);
            }
            Ok(())
        },
    );
}

/// Register `CreateUnitHealPredictionCalculator` in the Lua globals.
pub fn register_unit_heal_prediction(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "CreateUnitHealPredictionCalculator",
        lua.create_function(|_, ()| Ok(UnitHealPredictionCalculator::new()))?,
    )
}
