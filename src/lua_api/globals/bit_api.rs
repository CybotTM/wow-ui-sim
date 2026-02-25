//! Bitwise operations (native Rust implementation of WoW's bit library).
//!
//! Provides `bit.band`, `bit.bor`, `bit.bxor`, `bit.bnot`, `bit.lshift`,
//! `bit.rshift`, `bit.arshift`, and `bit.mod`.

use mlua::{Lua, Result, Value};

/// Register the `bit` table with all bitwise operations.
pub fn register_bit_library(lua: &Lua) -> Result<()> {
    let bit = lua.create_table()?;
    bit.set("band", lua.create_function(bit_fold_op(|a, b| a & b, 0xFFFFFFFF))?)?;
    bit.set("bor", lua.create_function(bit_fold_op(|a, b| a | b, 0))?)?;
    bit.set("bxor", lua.create_function(bit_fold_op(|a, b| a ^ b, 0))?)?;
    bit.set("bnot", lua.create_function(|_, a: mlua::Number| Ok(!to_u32(a)))?)?;
    bit.set("lshift", bit_shift_fn(lua, |a, n| a << n)?)?;
    bit.set("rshift", bit_shift_fn(lua, |a, n| a >> n)?)?;
    bit.set("arshift", bit_arshift_fn(lua)?)?;
    bit.set("mod", lua.create_function(bit_mod)?)?;
    lua.globals().set("bit", bit)?;
    Ok(())
}

fn to_u32(n: mlua::Number) -> u32 {
    n as u32
}

/// Create a variadic fold function for bitwise logic ops (band, bor, bxor).
fn bit_fold_op(
    op: fn(u32, u32) -> u32,
    identity: u32,
) -> impl Fn(&Lua, mlua::MultiValue) -> Result<u32> {
    move |_, args: mlua::MultiValue| {
        let mut result = identity;
        for val in args {
            let n = match val {
                Value::Number(n) => n,
                Value::Integer(n) => n as mlua::Number,
                _ => 0.0,
            };
            result = op(result, to_u32(n));
        }
        Ok(result)
    }
}

/// Create a shift function (lshift, rshift) with u32 shift clamping.
fn bit_shift_fn(lua: &Lua, op: fn(u32, u32) -> u32) -> Result<mlua::Function> {
    lua.create_function(move |_, (a, n): (mlua::Number, mlua::Number)| {
        let shift = to_u32(n);
        if shift >= 32 {
            return Ok(0u32);
        }
        Ok(op(to_u32(a), shift))
    })
}

/// Arithmetic right shift: preserves sign bit by casting through i32.
fn bit_arshift_fn(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|_, (a, n): (mlua::Number, mlua::Number)| {
        let shift = to_u32(n);
        if shift >= 32 {
            return Ok(if (to_u32(a) as i32) < 0 { 0xFFFFFFFFu32 } else { 0u32 });
        }
        Ok((to_u32(a) as i32 >> shift) as u32)
    })
}

/// Integer modulo: `a % b`.
fn bit_mod(_: &Lua, (a, b): (mlua::Number, mlua::Number)) -> Result<u32> {
    let b = to_u32(b);
    if b == 0 {
        return Err(mlua::Error::RuntimeError("bit.mod: division by zero".into()));
    }
    Ok(to_u32(a) % b)
}
