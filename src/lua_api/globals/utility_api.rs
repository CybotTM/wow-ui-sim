//! Utility functions for WoW API.
//!
//! Contains table manipulation functions (wipe, tinsert, tremove, tContains, etc.),
//! string utilities (strsplit, strjoin), and other general-purpose functions.

use mlua::{Function, Lua, MultiValue, Result, Value};

/// Register all utility API functions.
pub fn register_utility_api(lua: &Lua) -> Result<()> {
    register_table_functions(lua)?;
    register_string_functions(lua)?;
    register_global_access(lua)?;
    super::security_api::register_security_functions(lua)?;
    register_error_handlers(lua)?;
    register_misc_stubs(lua)?;
    register_lua_stdlib_aliases(lua)?;
    register_mixin_system(lua)?;
    Ok(())
}

/// Table manipulation: wipe, tinsert, tremove, tInvert, tContains, tIndexOf,
/// tFilter, CopyTable, MergeTable.
fn register_table_functions(lua: &Lua) -> Result<()> {
    register_wipe_and_aliases(lua)?;
    register_table_search(lua)?;
    register_table_transform(lua)?;

    // table.create(narray, nrec) — WoW-specific pre-allocation hint.
    let table_lib: mlua::Table = lua.globals().get("table")?;
    table_lib.set(
        "create",
        lua.create_function(|lua, (narray, nrec): (Option<i32>, Option<i32>)| {
            lua.create_table_with_capacity(
                narray.unwrap_or(0).max(0) as usize,
                nrec.unwrap_or(0).max(0) as usize,
            )
        })?,
    )?;

    Ok(())
}

/// wipe, tinsert, tremove - core table mutation functions.
fn register_wipe_and_aliases(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    register_wipe(lua, &globals)?;
    register_tinsert(lua, &globals)?;
    register_tremove(lua, &globals)
}

fn register_wipe(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    let wipe = lua.create_function(|_, table: mlua::Table| {
        let keys: Vec<Value> = table
            .pairs::<Value, Value>()
            .filter_map(|r| r.ok().map(|(k, _)| k))
            .collect();
        for key in keys {
            table.set(key, Value::Nil)?;
        }
        Ok(table)
    })?;
    globals.set("wipe", wipe.clone())?;
    let table_lib: mlua::Table = globals.get("table")?;
    table_lib.set("wipe", wipe)
}

fn register_tinsert(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "tinsert",
        lua.create_function(|lua, args: mlua::MultiValue| {
            let table_insert: mlua::Function =
                lua.globals().get::<mlua::Table>("table")?.get("insert")?;
            table_insert.call::<()>(args)?;
            Ok(())
        })?,
    )
}

fn register_tremove(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "tremove",
        lua.create_function(|lua, args: mlua::MultiValue| {
            let table_remove: mlua::Function =
                lua.globals().get::<mlua::Table>("table")?.get("remove")?;
            table_remove.call::<Value>(args)
        })?,
    )
}

/// tInvert, tContains, tIndexOf, tFilter - table search/filter functions.
fn register_table_search(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    register_tinvert(lua, &globals)?;
    register_tcontains(lua, &globals)?;
    register_tindexof(lua, &globals)?;
    register_tfilter(lua, &globals)
}

fn register_tinvert(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "tInvert",
        lua.create_function(|lua, tbl: mlua::Table| {
            let result = lua.create_table()?;
            for pair in tbl.pairs::<Value, Value>() {
                let (k, v) = pair?;
                result.set(v, k)?;
            }
            Ok(result)
        })?,
    )
}

fn register_tcontains(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "tContains",
        lua.create_function(|_, (tbl, value): (Option<mlua::Table>, Value)| {
            if let Some(tbl) = tbl {
                for pair in tbl.pairs::<Value, Value>() {
                    let (_, v) = pair?;
                    if v == value {
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        })?,
    )
}

fn register_tindexof(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "tIndexOf",
        lua.create_function(|_, (tbl, value): (mlua::Table, Value)| {
            for pair in tbl.pairs::<i32, Value>() {
                let (k, v) = pair?;
                if v == value {
                    return Ok(Value::Integer(k as i64));
                }
            }
            Ok(Value::Nil)
        })?,
    )
}

fn register_tfilter(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "tFilter",
        lua.create_function(
            |_, (tbl, pred, _keep_order): (mlua::Table, mlua::Function, Option<bool>)| {
                let mut to_remove = Vec::new();
                for pair in tbl.pairs::<Value, Value>() {
                    let (k, v) = pair?;
                    if !pred.call((v.clone(),))? {
                        to_remove.push(k);
                    }
                }
                for k in to_remove {
                    tbl.set(k, Value::Nil)?;
                }
                Ok(tbl)
            },
        )?,
    )
}

/// CopyTable, MergeTable - table copy/merge functions.
fn register_table_transform(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    register_copy_table(lua, &globals)?;
    register_merge_table(lua, &globals)
}

fn register_copy_table(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "CopyTable",
        lua.create_function(|lua, (tbl, seen): (mlua::Table, Option<mlua::Table>)| {
            let seen = seen.unwrap_or_else(|| lua.create_table().unwrap());
            let result = lua.create_table()?;
            seen.set(tbl.clone(), result.clone())?;
            for pair in tbl.pairs::<Value, Value>() {
                let (k, v) = pair?;
                let new_v = if let Value::Table(inner) = v.clone() {
                    if let Ok(cached) = seen.get::<mlua::Table>(inner.clone()) {
                        Value::Table(cached)
                    } else {
                        let copy_table: mlua::Function = lua.globals().get("CopyTable")?;
                        copy_table.call((inner, seen.clone()))?
                    }
                } else {
                    v
                };
                result.set(k, new_v)?;
            }
            Ok(result)
        })?,
    )
}

fn register_merge_table(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "MergeTable",
        lua.create_function(|_, (dest, source): (mlua::Table, mlua::Table)| {
            for pair in source.pairs::<Value, Value>() {
                let (k, v) = pair?;
                dest.set(k, v)?;
            }
            Ok(dest)
        })?,
    )
}

/// String functions: strsplit.
fn register_string_functions(lua: &Lua) -> Result<()> {
    lua.globals()
        .set("strsplit", lua.create_function(strsplit_impl)?)
}

/// Implementation of WoW's strsplit(delimiter, str, limit).
fn strsplit_impl(lua: &Lua, args: mlua::MultiValue) -> Result<mlua::MultiValue> {
    let args: Vec<Value> = args.into_iter().collect();
    let delimiter = lua_string_arg(&args, 0).unwrap_or_else(|| " ".to_string());
    let input = lua_string_arg(&args, 1).unwrap_or_default();
    let limit = args.get(2).and_then(|v| match v {
        Value::Integer(n) => Some(*n as usize),
        Value::Number(n) => Some(*n as usize),
        _ => None,
    });
    let parts: Vec<&str> = match limit {
        Some(n) => input.splitn(n, &delimiter).collect(),
        None => input.split(&delimiter).collect(),
    };
    let mut result = mlua::MultiValue::new();
    for part in parts {
        result.push_back(Value::String(lua.create_string(part)?));
    }
    Ok(result)
}

/// Extract a Lua string argument from a positional arg list.
fn lua_string_arg(args: &[Value], index: usize) -> Option<String> {
    if let Some(Value::String(s)) = args.get(index) {
        Some(s.to_string_lossy().to_string())
    } else {
        None
    }
}

/// Global access: getglobal, setglobal, loadstring, GetCurrentEnvironment.
fn register_global_access(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "getglobal",
        lua.create_function(|lua, name: String| {
            let value: Value = lua.globals().get(name.as_str()).unwrap_or(Value::Nil);
            Ok(value)
        })?,
    )?;

    globals.set(
        "setglobal",
        lua.create_function(|lua, (name, value): (String, Value)| {
            lua.globals().set(name.as_str(), value)?;
            Ok(())
        })?,
    )?;

    // loadstring: provided by Elune's luaopen_base, wrapped with taint in env.rs

    // Environment functions must be Lua (not Rust closures) for correct
    // getfenv/setfenv stack levels.  newsecurefunction wraps the inner Lua
    // function in a C closure, adding one extra stack frame.  So level 3
    // (not 2) reaches the actual caller.
    lua.load(
        r#"
        local nsf = debug.newsecurefunction
        GetCurrentEnvironment = nsf(function()
            return getfenv(3)
        end)
        IsInGlobalEnvironment = nsf(function()
            return getfenv(3) == _G
        end)
    "#,
    )
    .exec()?;

    Ok(())
}

fn arg_to_i32(v: &Value) -> Option<i32> {
    match v {
        Value::Integer(n) => Some(*n as i32),
        Value::Number(n) => Some(*n as i32),
        _ => None,
    }
}

/// debuglocals(level, skipFunctionsAndUserdata) - returns a string of local variables.
/// Stub: returns empty string. Only used by Blizzard_ScriptErrors for error display.
fn register_debuglocals(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "debuglocals",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(String::new()))?,
    )
}

/// debugstack(start, count1, count2) - returns a stack trace string.
/// WoW's debugstack is used by error handlers and BugSack.
fn register_debugstack(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "debugstack",
        lua.create_function(|lua, args: mlua::MultiValue| {
            let start = args.front().and_then(arg_to_i32).unwrap_or(2);
            let count1 = args.get(1).and_then(arg_to_i32).unwrap_or(12) as usize;
            let count2 = args.get(2).and_then(arg_to_i32).unwrap_or(10) as usize;
            let tb: mlua::Function = lua
                .globals()
                .get::<mlua::Table>("debug")?
                .get("traceback")?;
            let trace: String = tb.call(("", start))?;
            let lines: Vec<&str> = trace.lines().filter(|l| !l.is_empty()).collect();
            let total = lines.len();
            if total <= count1 + count2 {
                Ok(lines.join("\n"))
            } else {
                let top = &lines[..count1];
                let bottom = &lines[total - count2..];
                Ok(format!("{}\n...\n{}", top.join("\n"), bottom.join("\n")))
            }
        })?,
    )
}

/// Error handler functions: geterrorhandler, seterrorhandler.
///
/// Stores the handler in the Lua registry under `__wow_error_handler`.
/// Script dispatch errors are routed through this handler (see script_helpers).
fn register_error_handlers(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    register_debugstack(lua)?;
    register_debuglocals(lua)?;

    // geterrorhandler / seterrorhandler: provided by Elune's baselib_shared,
    // using LUA_ERRORHANDLERINDEX (-9999) which securecall's lua_pcall references.

    // Internal error reporter used by generated Lua code (chained handlers,
    // lifecycle scripts).  Unlike `print()`, this always logs to stderr and
    // invokes the Lua error handler regardless of whether Blizzard_PrintHandler
    // has overridden `print`.
    // Stored in registry; also set as a temporary global so that Lua code
    // compiled before sandbox cleanup can capture it as an upvalue.
    let report_fn = lua.create_function(|lua, msg: String| {
        super::super::script_helpers::call_error_handler(lua, &msg);
        Ok(())
    })?;
    lua.set_named_registry_value("__report_script_error", report_fn.clone())?;
    globals.set("__report_script_error", report_fn)?;

    Ok(())
}

/// Misc stubs: nop function, mapvalues.
fn register_misc_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let nop = lua.create_function(|_, _: mlua::MultiValue| Ok(()))?;
    globals.set("nop", nop)?;

    // mapvalues(func, ...) - apply func to each value, return mapped results
    globals.set(
        "mapvalues",
        lua.create_function(|_, args: mlua::MultiValue| {
            let mut iter = args.into_iter();
            let func = match iter.next() {
                Some(Value::Function(f)) => f,
                _ => return Ok(mlua::MultiValue::new()),
            };
            let mut result = mlua::MultiValue::new();
            for val in iter {
                let mapped: Value = func.call(val)?;
                result.push_back(mapped);
            }
            Ok(result)
        })?,
    )?;

    Ok(())
}

/// Lua stdlib global aliases (string, math, table, bit, os) for WoW compatibility.
fn register_lua_stdlib_aliases(lua: &Lua) -> Result<()> {
    register_string_aliases(lua)?;
    register_math_aliases(lua)?;
    register_table_aliases(lua)?;
    super::bit_api::register_bit_library(lua)?;
    register_os_aliases(lua)?;
    Ok(())
}

const STRING_ALIASES_LUA: &str = r##"
    strlen = string.len
    strsub = string.sub
    strfind = string.find
    strmatch = string.match
    strbyte = string.byte
    strchar = string.char
    strrep = string.rep
    strrev = string.reverse
    strlower = string.lower
    strupper = string.upper
    strtrim = function(s) return (s:gsub("^%s*(.-)%s*$", "%1")) end
    format = string.format
    gsub = string.gsub
    gmatch = string.gmatch
"##;

/// String library global aliases.
fn register_string_aliases(lua: &Lua) -> Result<()> {
    lua.load(STRING_ALIASES_LUA).exec()?;
    register_strjoin(lua)?;
    register_strsplittable(lua)?;
    register_string_split(lua)
}

/// strjoin(delimiter, ...) -> concatenate variadic args with delimiter.
fn register_strjoin(lua: &Lua) -> Result<()> {
    lua.globals()
        .set("strjoin", lua.create_function(strjoin_impl_join)?)?;
    lua.globals()
        .get::<mlua::Table>("string")?
        .set("join", lua.create_function(strjoin_impl_join)?)
}

fn strjoin_impl_join(_: &Lua, args: MultiValue) -> Result<String> {
    let mut iter = args.into_iter();
    let sep = match iter.next() {
        Some(Value::String(s)) => s.to_str()?.to_string(),
        Some(Value::Nil) | None => String::new(),
        Some(v) => v.to_string()?,
    };
    let parts: Vec<String> = iter
        .map(|v| match v {
            Value::String(s) => s.to_str().map(|s| s.to_string()).unwrap_or_default(),
            Value::Number(n) => format!("{}", n),
            Value::Integer(n) => format!("{}", n),
            _ => String::new(),
        })
        .collect();
    Ok(parts.join(&sep))
}

/// strsplittable(delimiter, str) -> table of non-empty parts split by delimiter.
/// Mirrors WoW's gmatch pattern `([^del]+)` which skips empty parts.
fn register_strsplittable(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "strsplittable",
        lua.create_function(|lua, (del, s): (String, String)| {
            let t = lua.create_table()?;
            if del.is_empty() {
                t.set(1, s)?;
            } else {
                let mut i = 1;
                for part in s.split(&*del) {
                    if !part.is_empty() {
                        t.set(i, part)?;
                        i += 1;
                    }
                }
            }
            Ok(t)
        })?,
    )
}

/// string:split(delimiter) -> table of parts split by delimiter, including empty strings.
/// Mirrors WoW's string.find with plain=true which preserves empty parts.
fn register_string_split(lua: &Lua) -> Result<()> {
    let string_table: mlua::Table = lua.globals().get("string")?;
    string_table.set(
        "split",
        lua.create_function(|lua, (s, del): (String, String)| {
            let t = lua.create_table()?;
            if del.is_empty() {
                t.set(1, s)?;
                return Ok(t);
            }
            let mut from = 0;
            let mut i = 1;
            while let Some(pos) = s[from..].find(&*del) {
                t.set(i, &s[from..from + pos])?;
                i += 1;
                from += pos + del.len();
            }
            t.set(i, &s[from..])?;
            Ok(t)
        })?,
    )
}

const MATH_ALIASES_LUA: &str = r##"
    abs = math.abs
    ceil = math.ceil
    floor = math.floor
    max = math.max
    min = math.min
    mod = math.fmod
    sqrt = math.sqrt
    sin = math.sin
    cos = math.cos
    tan = math.tan
    asin = math.asin
    acos = math.acos
    atan = math.atan
    atan2 = math.atan2
    deg = math.deg
    rad = math.rad
    random = math.random
    exp = math.exp
    log = math.log
    log10 = math.log10
    pow = math.pow
    frexp = math.frexp
    ldexp = math.ldexp

    sort = table.sort
    getn = function(t) return #t end
    tconcat = table.concat
"##;

/// Math library global aliases.
fn register_math_aliases(lua: &Lua) -> Result<()> {
    lua.load(MATH_ALIASES_LUA).exec()
}

/// Table library global aliases.
fn register_table_aliases(_lua: &Lua) -> Result<()> {
    // Already registered in register_math_aliases (sort, getn, tconcat)
    Ok(())
}

/// OS library global aliases: date, time, difftime, clock.
fn register_os_aliases(lua: &Lua) -> Result<()> {
    lua.load(
        r##"
        date = os.date
        time = os.time
        difftime = os.difftime
        clock = os.clock
    "##,
    )
    .exec()?;
    Ok(())
}

/// Mixin system: Mixin, CreateFromMixins, CreateAndInitFromMixin.
fn register_mixin_system(lua: &Lua) -> Result<()> {
    register_mixin_overrides_table(lua)?;
    register_set_mixin_override(lua)?;
    register_lud_setter(lua)?;
    register_mixin_globals(lua)?;
    super::utility_stubs::register_scrollbox_and_utility_stubs(lua)
}

/// Register a Lua-level setter for UserData __newindex dispatch.
fn register_lud_setter(lua: &Lua) -> Result<()> {
    let f = lua
        .load("return function(obj, k, v) obj[k] = v end")
        .eval::<Function>()?;
    lua.set_named_registry_value("__mixin_lud_setter", f)
}

/// Initialize the __mixin_overrides registry table used by the frame __index.
/// Structure: __mixin_overrides[frame_id][method_name] = function.
/// Only function values are stored here; non-function properties go through __frame_fields.
fn register_mixin_overrides_table(lua: &Lua) -> Result<()> {
    let overrides_table = lua.create_table()?;
    lua.set_named_registry_value("__mixin_overrides", overrides_table)?;
    Ok(())
}

/// Register __SetMixinOverride(frame_ud, key, value): write a function override into
/// __mixin_overrides[frame_id][key]. Called by Mixin() for UserData targets.
/// Accepts any Value for the object and silently skips non-FrameRef values
/// (e.g. animation group userdata objects that don't use the frame __index).
/// Stored in registry; also exposed as a temporary global so Mixin() (compiled
/// immediately after) can reference it by name. Sandbox cleanup nils the global.
fn register_set_mixin_override(lua: &Lua) -> Result<()> {
    let f = lua.create_function(set_mixin_override_impl)?;
    lua.set_named_registry_value("__SetMixinOverride", f.clone())?;
    lua.globals().set("__SetMixinOverride", f)
}

fn set_mixin_override_impl(_lua: &Lua, (obj, key, value): (Value, String, Value)) -> Result<()> {
    // Store mixin function directly in the per-frame user_value table.
    // The UserData __index reads from this table, so mixin functions are found
    // without needing a separate __mixin_overrides lookup.
    if let Value::UserData(ud) = &obj {
        if ud.borrow::<crate::lua_api::frame::FrameRef>().is_ok() {
            if let Ok(fields) = ud.user_value::<mlua::Table>() {
                fields.raw_set(key, value)?;
            }
        }
    }
    Ok(())
}

/// Resolve the effective source table for a mixin, preferring __secureMixinMethods.
/// Returns (source_table, is_secure).
fn resolve_mixin_source(secure_methods: &Value, mixin: mlua::Table) -> Result<(mlua::Table, bool)> {
    if let Value::Table(sm) = secure_methods {
        if let Value::Table(t) = sm.get::<Value>(mixin.clone())? {
            return Ok((t, true));
        }
    }
    Ok((mixin, false))
}

/// Apply a mixin source table's k/v pairs into object.
/// For userdata objects, function values are also routed through __SetMixinOverride.
/// For UserData (FrameRef), uses a Lua-level setter to trigger __newindex.
fn apply_mixin_to_object(
    lua: &Lua,
    object: &Value,
    source: mlua::Table,
    is_userdata: bool,
    is_secure: bool,
    set_override: &Option<Function>,
) -> Result<()> {
    let ud_setter: Option<Function> = if is_userdata {
        Some(lua.named_registry_value("__mixin_lud_setter")?)
    } else {
        None
    };
    let mut newsecfn: Option<Function> = None;
    for pair in source.pairs::<Value, Value>() {
        let (k, mut v) = pair?;
        if is_secure && is_userdata {
            v = wrap_secure_function(lua, v, &mut newsecfn)?;
        }
        if is_userdata && matches!(&v, Value::Function(_)) {
            if let Some(f) = set_override {
                f.call::<()>((object.clone(), k.clone(), v.clone()))?;
            }
        }
        store_mixin_value(object, k, v, &ud_setter)?;
    }
    Ok(())
}

/// Wrap function values with `debug.newsecurefunction` for secure mixins.
/// Non-function values pass through unchanged.
fn wrap_secure_function(
    lua: &Lua,
    v: Value,
    newsecfn: &mut Option<Function>,
) -> Result<Value> {
    let Value::Function(ref f) = v else {
        return Ok(v);
    };
    if newsecfn.is_none() {
        *newsecfn = Some(lua.load("return debug.newsecurefunction").eval::<Function>()?);
    }
    let wrapped = newsecfn.as_ref().unwrap().call::<Function>(f.clone())?;
    Ok(Value::Function(wrapped))
}

/// Store a key-value pair on the target object (table rawset or userdata setter).
fn store_mixin_value(
    object: &Value,
    key: Value,
    value: Value,
    ud_setter: &Option<Function>,
) -> Result<()> {
    match object {
        Value::Table(t) => t.set(key, value),
        _ => {
            if let Some(setter) = ud_setter {
                setter.call::<()>((object.clone(), key, value))?;
            }
            Ok(())
        }
    }
}

/// Mixin(object, ...) — copy k/v from each mixin into object, return object.
fn mixin_impl(lua: &Lua, args: MultiValue) -> Result<Value> {
    let mut iter = args.into_iter();
    let object = iter.next().ok_or_else(|| {
        mlua::Error::RuntimeError("Usage: local outObject = Mixin(object, ...)".into())
    })?;
    let secure_methods: Value = lua.globals().get("__secureMixinMethods")?;
    let set_override: Option<Function> = lua.named_registry_value("__SetMixinOverride").ok();
    let is_userdata = matches!(&object, Value::UserData(_));
    for mixin_val in iter {
        let mixin = match mixin_val {
            Value::Table(t) => t,
            Value::Nil => {
                return Err(mlua::Error::RuntimeError(
                    "Usage: local outObject = Mixin(object, ...)".into(),
                ));
            }
            _ => continue,
        };
        let (source, is_secure) = resolve_mixin_source(&secure_methods, mixin)?;
        apply_mixin_to_object(lua, &object, source, is_userdata, is_secure, &set_override)?;
    }
    Ok(object)
}

/// CreateAndInitFromMixin(mixin, ...) — create from mixin, then call :Init(...).
fn create_and_init_from_mixin_impl(lua: &Lua, args: MultiValue) -> Result<Value> {
    let mut iter = args.into_iter();
    let mixin = iter.next().unwrap_or(Value::Nil);
    let create_fn: Function = lua.globals().get("CreateFromMixins")?;
    let obj: Value = create_fn.call(mixin)?;
    if let Value::Table(ref t) = obj {
        if let Ok(init) = t.get::<Function>("Init") {
            let mut call_args = vec![obj.clone()];
            call_args.extend(iter);
            init.call::<MultiValue>(MultiValue::from_iter(call_args))?;
        }
    }
    Ok(obj)
}

/// Register Mixin, CreateFromMixins, CreateAndInitFromMixin as Rust closures.
fn register_mixin_globals(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("Mixin", lua.create_function(mixin_impl)?)?;
    g.set(
        "CreateFromMixins",
        lua.create_function(|lua, args: MultiValue| {
            let obj = lua.create_table()?;
            let mixin_fn: Function = lua.globals().get("Mixin")?;
            let mut call_args = vec![Value::Table(obj)];
            call_args.extend(args);
            mixin_fn.call::<Value>(MultiValue::from_iter(call_args))
        })?,
    )?;
    g.set(
        "CreateAndInitFromMixin",
        lua.create_function(create_and_init_from_mixin_impl)?,
    )?;
    Ok(())
}
