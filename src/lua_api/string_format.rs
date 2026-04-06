//! WoW-compatible `string.format` patch: `%F` support and positional arguments (`%1$s`).

use mlua::{Lua, Result, Value};

/// Patch `string.format` with a Rust implementation that handles:
/// - `%F` (uppercase float) which Lua 5.1 lacks; converted to `%f`
/// - Positional arguments (`%1$s`, `%2$d`) which WoW's patched LuaJIT supports
///
/// Being a Rust/mlua function, it appears as a C function to Lua's `coroutine.create`,
/// matching WoW's real behavior where `string.format` is a C function.
pub fn patch_string_format(lua: &Lua) -> Result<()> {
    let string_table: mlua::Table = lua.globals().get("string")?;
    let original: mlua::Function = string_table.get("format")?;
    lua.set_named_registry_value("__original_string_format", original)?;

    let format_fn = lua.create_function(wow_string_format)?;
    string_table.set("format", format_fn.clone())?;
    lua.globals().set("format", format_fn)?;
    Ok(())
}

/// Rust implementation of WoW's extended `string.format`.
fn wow_string_format(
    lua: &mlua::Lua,
    mut args: mlua::MultiValue,
) -> mlua::Result<mlua::MultiValue> {
    let original: mlua::Function = lua.named_registry_value("__original_string_format")?;

    // Non-string first arg: pass through to original C string.format
    let fmt = match args.iter().next() {
        Some(Value::String(s)) => match s.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return original.call(args),
        },
        _ => return original.call(args),
    };

    // Fast path: no %F or positional args
    if !fmt.contains('F') && !fmt.contains('$') {
        return original.call(args);
    }

    args.pop_front();
    let rest: Vec<Value> = args.into_vec();
    let (new_fmt, new_rest) = process_wow_format(&fmt, &rest)?;
    call_with_processed_args(lua, &original, &new_fmt, new_rest)
}

/// Build MultiValue from processed format + args and call original string.format.
fn call_with_processed_args(
    lua: &mlua::Lua,
    original: &mlua::Function,
    fmt: &str,
    rest: Vec<Value>,
) -> mlua::Result<mlua::MultiValue> {
    let mut new_args = mlua::MultiValue::new();
    new_args.push_back(Value::String(lua.create_string(fmt)?));
    for arg in rest {
        new_args.push_back(arg);
    }
    original.call(new_args)
}

/// Parse format string: replace `%F` → `%f` and reorder positional args (`%1$s`).
fn process_wow_format(fmt: &str, args: &[Value]) -> mlua::Result<(String, Vec<Value>)> {
    let bytes = fmt.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    let mut reordered: Vec<Value> = Vec::new();
    let mut seq: usize = 0;
    let mut has_positional = false;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] != b'%' {
            out.push(bytes[i] as char);
            i += 1;
        } else if i + 1 < bytes.len() && bytes[i + 1] == b'%' {
            out.push_str("%%");
            i += 2;
        } else {
            i = parse_format_specifier(
                bytes,
                i,
                args,
                &mut out,
                &mut reordered,
                &mut seq,
                &mut has_positional,
            )?;
        }
    }

    if has_positional {
        Ok((out, reordered))
    } else {
        Ok((out, args.to_vec()))
    }
}

/// Parse one format specifier starting at `%`, appending to `out` and collecting args.
/// Returns the index after the specifier.
fn parse_format_specifier(
    bytes: &[u8],
    start: usize,
    args: &[Value],
    out: &mut String,
    reordered: &mut Vec<Value>,
    seq: &mut usize,
    has_positional: &mut bool,
) -> mlua::Result<usize> {
    let mut i = start + 1; // skip the '%'

    // Check for positional %N$
    if let Some((n, after)) = parse_positional_index(bytes, i) {
        if n >= 100 {
            return Err(mlua::Error::RuntimeError(
                "invalid format (width or precision too long)".to_string(),
            ));
        }
        *has_positional = true;
        reordered.push(args.get(n - 1).cloned().unwrap_or(Value::Nil));
        out.push('%');
        i = after;
    } else {
        *seq += 1;
        reordered.push(args.get(*seq - 1).cloned().unwrap_or(Value::Nil));
        out.push('%');
    }

    i = skip_flags_width_precision(bytes, i, out);
    // Conversion character — %F → %f
    if i < bytes.len() && is_format_conversion(bytes[i]) {
        out.push(if bytes[i] == b'F' {
            'f'
        } else {
            bytes[i] as char
        });
        i += 1;
    }
    Ok(i)
}

/// Skip flags (`-+ #0`), width digits, and precision (`.N`) — appending to `out`.
fn skip_flags_width_precision(bytes: &[u8], start: usize, out: &mut String) -> usize {
    let mut i = start;
    while i < bytes.len() && matches!(bytes[i], b'-' | b'+' | b' ' | b'#' | b'0') {
        out.push(bytes[i] as char);
        i += 1;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        out.push(bytes[i] as char);
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        out.push('.');
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    i
}

/// Try to parse `N$` (digits followed by `$`) at `start`. Returns `(N, index_after_$)`.
fn parse_positional_index(bytes: &[u8], start: usize) -> Option<(usize, usize)> {
    let mut i = start;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == start || i >= bytes.len() || bytes[i] != b'$' {
        return None;
    }
    let n: usize = std::str::from_utf8(&bytes[start..i]).ok()?.parse().ok()?;
    Some((n, i + 1))
}

fn is_format_conversion(b: u8) -> bool {
    matches!(
        b,
        b'd' | b'i'
            | b'o'
            | b'u'
            | b'x'
            | b'X'
            | b'e'
            | b'E'
            | b'f'
            | b'F'
            | b'g'
            | b'G'
            | b'a'
            | b'A'
            | b'c'
            | b's'
            | b'p'
            | b'q'
            | b'n'
    )
}
