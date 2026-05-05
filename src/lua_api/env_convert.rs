//! Type-conversion traits for decoding rilua result values back into Rust types.
//!
//! `FromRiluaResults` decodes the full multi-value result list returned by a
//! Lua call. `FromRiluaValue` decodes a single `Val` from the list. Both are
//! implemented for the primitive scalar types used by `WowLuaEnv::eval<T>`.

use crate::Result;
use rilua::{Val, vm::state::LuaState};

pub trait FromRiluaResults: Sized {
    fn from_results(state: &LuaState, results: Vec<Val>) -> Result<Self>;
}

pub trait FromRiluaValue: Sized {
    fn from_value(state: &LuaState, value: Val) -> Result<Self>;
}

// ── Internal helpers ─────────────────────────────────────────────────────────

pub(super) fn first_result(results: &[Val]) -> Val {
    results.first().copied().unwrap_or(Val::Nil)
}

/// Unpack the packed-table result from `eval<T>` into a flat results list.
pub(super) fn unpack_eval_results(state: &LuaState, results: Val) -> crate::Result<Vec<Val>> {
    let Val::Table(table_ref) = results else {
        return Ok(match results {
            Val::Nil => Vec::new(),
            value => vec![value],
        });
    };
    let Some(table) = state.gc.tables.get(table_ref) else {
        return Err(crate::Error::Other(
            "eval result table was collected".into(),
        ));
    };
    let len = table.len(&state.gc.string_arena);
    Ok((1..=len).map(|idx| table.get_int(idx as i64)).collect())
}

fn decode_string(state: &LuaState, value: Val) -> Result<String> {
    crate::lua_api::methods::val_to_string(state, value).ok_or_else(|| {
        crate::Error::Other(format!("expected string result, got {}", value.type_name()))
    })
}

fn decode_number(value: Val) -> Result<f64> {
    match value {
        Val::Num(n) => Ok(n),
        other => Err(crate::Error::Other(format!(
            "expected numeric result, got {}",
            other.type_name()
        ))),
    }
}

fn integer_decode_error(number: f64) -> crate::Error {
    crate::Error::Other(format!(
        "expected integer result, got non-integer number {number}"
    ))
}

fn decode_integer<T>(value: Val, convert: impl FnOnce(f64) -> Option<T>) -> Result<T> {
    let number = decode_number(value)?;
    convert(number).ok_or_else(|| integer_decode_error(number))
}

fn decode_i32(value: Val) -> Result<i32> {
    decode_integer(value, |number| {
        let int = number as i32;
        (int as f64 == number).then_some(int)
    })
}

fn decode_i64(value: Val) -> Result<i64> {
    decode_integer(value, |number| {
        let int = number as i64;
        (int as f64 == number).then_some(int)
    })
}

// ── Scalar impls ─────────────────────────────────────────────────────────────

impl FromRiluaResults for () {
    fn from_results(_state: &LuaState, _results: Vec<Val>) -> Result<Self> {
        Ok(())
    }
}

impl FromRiluaResults for bool {
    fn from_results(_state: &LuaState, results: Vec<Val>) -> Result<Self> {
        Ok(!matches!(
            first_result(&results),
            Val::Nil | Val::Bool(false)
        ))
    }
}

impl FromRiluaValue for bool {
    fn from_value(_state: &LuaState, value: Val) -> Result<Self> {
        Ok(!matches!(value, Val::Nil | Val::Bool(false)))
    }
}

impl FromRiluaResults for f64 {
    fn from_results(_state: &LuaState, results: Vec<Val>) -> Result<Self> {
        decode_number(first_result(&results))
    }
}

impl FromRiluaValue for f64 {
    fn from_value(_state: &LuaState, value: Val) -> Result<Self> {
        decode_number(value)
    }
}

impl FromRiluaResults for f32 {
    fn from_results(_state: &LuaState, results: Vec<Val>) -> Result<Self> {
        Ok(decode_number(first_result(&results))? as f32)
    }
}

impl FromRiluaValue for f32 {
    fn from_value(_state: &LuaState, value: Val) -> Result<Self> {
        Ok(decode_number(value)? as f32)
    }
}

impl FromRiluaResults for i32 {
    fn from_results(_state: &LuaState, results: Vec<Val>) -> Result<Self> {
        decode_i32(first_result(&results))
    }
}

impl FromRiluaValue for i32 {
    fn from_value(_state: &LuaState, value: Val) -> Result<Self> {
        decode_i32(value)
    }
}

impl FromRiluaResults for i64 {
    fn from_results(_state: &LuaState, results: Vec<Val>) -> Result<Self> {
        decode_i64(first_result(&results))
    }
}

impl FromRiluaValue for i64 {
    fn from_value(_state: &LuaState, value: Val) -> Result<Self> {
        decode_i64(value)
    }
}

impl FromRiluaResults for String {
    fn from_results(state: &LuaState, results: Vec<Val>) -> Result<Self> {
        decode_string(state, first_result(&results))
    }
}

impl FromRiluaValue for String {
    fn from_value(state: &LuaState, value: Val) -> Result<Self> {
        decode_string(state, value)
    }
}

// ── Generic container impls ──────────────────────────────────────────────────

impl<T> FromRiluaResults for Option<T>
where
    T: FromRiluaValue,
{
    fn from_results(state: &LuaState, results: Vec<Val>) -> Result<Self> {
        match first_result(&results) {
            Val::Nil => Ok(None),
            value => T::from_value(state, value).map(Some),
        }
    }
}

impl<T> FromRiluaValue for Option<T>
where
    T: FromRiluaValue,
{
    fn from_value(state: &LuaState, value: Val) -> Result<Self> {
        match value {
            Val::Nil => Ok(None),
            value => T::from_value(state, value).map(Some),
        }
    }
}

impl<T> FromRiluaResults for Vec<T>
where
    T: FromRiluaValue,
{
    fn from_results(state: &LuaState, results: Vec<Val>) -> Result<Self> {
        match first_result(&results) {
            Val::Nil => Ok(Vec::new()),
            value => <Vec<T> as FromRiluaValue>::from_value(state, value),
        }
    }
}

impl<T> FromRiluaValue for Vec<T>
where
    T: FromRiluaValue,
{
    fn from_value(state: &LuaState, value: Val) -> Result<Self> {
        let Val::Table(table_ref) = value else {
            return Err(crate::Error::Other(format!(
                "expected table result, got {}",
                value.type_name()
            )));
        };
        let Some(table) = state.gc.tables.get(table_ref) else {
            return Err(crate::Error::Other("table result was collected".into()));
        };
        let len = table.len(&state.gc.string_arena);
        (1..=len)
            .map(|idx| T::from_value(state, table.get_int(idx as i64)))
            .collect()
    }
}

impl FromRiluaResults for std::collections::BTreeMap<String, String> {
    fn from_results(state: &LuaState, results: Vec<Val>) -> Result<Self> {
        let value = first_result(&results);
        let Val::Table(table_ref) = value else {
            return Err(crate::Error::Other(format!(
                "expected table result, got {}",
                value.type_name()
            )));
        };
        let Some(table) = state.gc.tables.get(table_ref) else {
            return Err(crate::Error::Other("table result was collected".into()));
        };

        let mut key = Val::Nil;
        let mut map = std::collections::BTreeMap::new();
        while let Some((next_key, next_value)) = table.next(key, &state.gc.string_arena)? {
            let decoded_key = decode_string(state, next_key)?;
            let decoded_value = decode_string(state, next_value)?;
            map.insert(decoded_key, decoded_value);
            key = next_key;
        }

        Ok(map)
    }
}

impl FromRiluaResults for Val {
    fn from_results(_state: &LuaState, results: Vec<Val>) -> Result<Self> {
        Ok(first_result(&results))
    }
}

impl FromRiluaValue for Val {
    fn from_value(_state: &LuaState, value: Val) -> Result<Self> {
        Ok(value)
    }
}

impl FromRiluaResults for crate::lua_bridge::MultiValue {
    fn from_results(_state: &LuaState, results: Vec<Val>) -> Result<Self> {
        Ok(results.into())
    }
}

// ── Tuple impls ──────────────────────────────────────────────────────────────

macro_rules! impl_from_results_tuple {
    ($(($($name:ident),+)),+ $(,)?) => {
        $(
            impl<$($name),+> FromRiluaResults for ($($name,)+)
            where
                $($name: FromRiluaValue),+
            {
                fn from_results(
                    state: &LuaState,
                    results: Vec<Val>,
                ) -> Result<Self> {
                    let mut iter = results.into_iter();
                    Ok((
                        $(
                            <$name as FromRiluaValue>::from_value(
                                state,
                                iter.next().unwrap_or(Val::Nil),
                            )?,
                        )+
                    ))
                }
            }
        )+
    };
}

impl_from_results_tuple!(
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
    (A, B, C, D, E, F, G, H, I),
    (A, B, C, D, E, F, G, H, I, J),
    (A, B, C, D, E, F, G, H, I, J, K),
    (A, B, C, D, E, F, G, H, I, J, K, L),
    (A, B, C, D, E, F, G, H, I, J, K, L, M),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T),
);
