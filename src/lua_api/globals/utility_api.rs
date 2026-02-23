//! Utility functions for WoW API.
//!
//! Contains table manipulation functions (wipe, tinsert, tremove, tContains, etc.),
//! string utilities (strsplit, strjoin), and other general-purpose functions.

use mlua::{Lua, Result, Value};

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
    table_lib.set("create", lua.create_function(|lua, (narray, nrec): (Option<i32>, Option<i32>)| {
        lua.create_table_with_capacity(
            narray.unwrap_or(0).max(0) as usize,
            nrec.unwrap_or(0).max(0) as usize,
        )
    })?)?;

    Ok(())
}

/// wipe, tinsert, tremove - core table mutation functions.
fn register_wipe_and_aliases(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

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
    table_lib.set("wipe", wipe)?;

    globals.set(
        "tinsert",
        lua.create_function(|lua, args: mlua::MultiValue| {
            let table_insert: mlua::Function =
                lua.globals().get::<mlua::Table>("table")?.get("insert")?;
            table_insert.call::<()>(args)?;
            Ok(())
        })?,
    )?;

    globals.set(
        "tremove",
        lua.create_function(|lua, args: mlua::MultiValue| {
            let table_remove: mlua::Function =
                lua.globals().get::<mlua::Table>("table")?.get("remove")?;
            table_remove.call::<Value>(args)
        })?,
    )?;

    Ok(())
}

/// tInvert, tContains, tIndexOf, tFilter - table search/filter functions.
fn register_table_search(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

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
    )?;

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
    )?;

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
    )?;

    globals.set(
        "tFilter",
        lua.create_function(
            |_, (tbl, pred, _keep_order): (mlua::Table, mlua::Function, Option<bool>)| {
                let mut to_remove = Vec::new();
                for pair in tbl.pairs::<Value, Value>() {
                    let (k, v) = pair?;
                    let keep: bool = pred.call((v.clone(),))?;
                    if !keep {
                        to_remove.push(k);
                    }
                }
                for k in to_remove {
                    tbl.set(k, Value::Nil)?;
                }
                Ok(tbl)
            },
        )?,
    )?;

    Ok(())
}

/// CopyTable, MergeTable - table copy/merge functions.
fn register_table_transform(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

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
    )?;

    globals.set(
        "MergeTable",
        lua.create_function(|_, (dest, source): (mlua::Table, mlua::Table)| {
            for pair in source.pairs::<Value, Value>() {
                let (k, v) = pair?;
                dest.set(k, v)?;
            }
            Ok(dest)
        })?,
    )?;

    Ok(())
}

/// String functions: strsplit.
fn register_string_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    // strsplit(delimiter, str, limit) - WoW string utility
    globals.set(
        "strsplit",
        lua.create_function(|lua, args: mlua::MultiValue| {
            let args: Vec<Value> = args.into_iter().collect();

            let delimiter = args
                .first()
                .and_then(|v| {
                    if let Value::String(s) = v {
                        Some(s.to_string_lossy().to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| " ".to_string());

            let input = args
                .get(1)
                .and_then(|v| {
                    if let Value::String(s) = v {
                        Some(s.to_string_lossy().to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();

            let limit = args.get(2).and_then(|v| match v {
                Value::Integer(n) => Some(*n as usize),
                Value::Number(n) => Some(*n as usize),
                _ => None,
            });

            let parts: Vec<&str> = if let Some(limit) = limit {
                input.splitn(limit, &delimiter).collect()
            } else {
                input.split(&delimiter).collect()
            };

            let mut result = mlua::MultiValue::new();
            for part in parts {
                result.push_back(Value::String(lua.create_string(part)?));
            }
            Ok(result)
        })?,
    )?;

    Ok(())
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
    // getfenv/setfenv stack levels — level 2 reaches the actual caller.
    lua.load(
        r#"
        function GetCurrentEnvironment()
            return getfenv(2)
        end
        function IsInGlobalEnvironment()
            return getfenv(2) == _G
        end
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
            let tb: mlua::Function =
                lua.globals().get::<mlua::Table>("debug")?.get("traceback")?;
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
    globals.set(
        "__report_script_error",
        lua.create_function(|lua, msg: String| {
            super::super::script_helpers::call_error_handler(lua, &msg);
            Ok(())
        })?,
    )?;

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
    register_bit_library(lua)?;
    register_os_aliases(lua)?;
    Ok(())
}

/// String library global aliases.
fn register_string_aliases(lua: &Lua) -> Result<()> {
    lua.load(
        r##"
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
        strsplittable = function(del, str) local t = {} for v in string.gmatch(str, "([^"..del.."]+)") do t[#t+1] = v end return t end
        strjoin = function(delimiter, ...) return table.concat({...}, delimiter) end
        string.join = strjoin
        format = string.format

        function string:split(delimiter)
            local result = {}
            local from = 1
            local delim_from, delim_to = string.find(self, delimiter, from, true)
            while delim_from do
                table.insert(result, string.sub(self, from, delim_from - 1))
                from = delim_to + 1
                delim_from, delim_to = string.find(self, delimiter, from, true)
            end
            table.insert(result, string.sub(self, from))
            return result
        end
        gsub = string.gsub
        gmatch = string.gmatch
    "##,
    )
    .exec()?;
    Ok(())
}

/// Math library global aliases.
fn register_math_aliases(lua: &Lua) -> Result<()> {
    lua.load(
        r##"
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
    "##,
    )
    .exec()?;
    Ok(())
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

/// Bitwise operations (native Rust implementation of WoW's bit library).
fn register_bit_library(lua: &Lua) -> Result<()> {
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
        if shift >= 32 { return Ok(0u32); }
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

/// Mixin system: Mixin, CreateFromMixins, CreateAndInitFromMixin.
fn register_mixin_system(lua: &Lua) -> Result<()> {
    register_mixin_overrides_table(lua)?;
    register_set_mixin_override(lua)?;
    register_mixin_globals(lua)?;
    register_scrollbox_globals(lua)?;
    register_utility_stubs(lua)
}

/// Initialize the __mixin_overrides registry table used by the frame __index.
/// Structure: __mixin_overrides[frame_id][method_name] = function.
/// Only function values are stored here; non-function properties go through __frame_fields.
fn register_mixin_overrides_table(lua: &Lua) -> Result<()> {
    let overrides_table = lua.create_table()?;
    lua.set_named_registry_value("__mixin_overrides", overrides_table)?;
    Ok(())
}

/// Register __SetMixinOverride(frame_lud, key, value): write a function override into
/// __mixin_overrides[frame_id][key]. Called by Mixin() for LightUserData targets.
/// Accepts any Value for the object and silently skips non-LightUserData values
/// (e.g. animation group full-userdata objects that don't use the frame __index).
fn register_set_mixin_override(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "__SetMixinOverride",
        lua.create_function(
            |lua, (obj, key, value): (Value, String, Value)| {
                let ud = match &obj {
                    Value::LightUserData(lud) => *lud,
                    _ => return Ok(()), // not a frame LightUserData, skip
                };
                let frame_id = crate::lua_api::frame::lud_to_id(ud);
                let overrides: mlua::Table = lua.named_registry_value("__mixin_overrides")?;
                let frame_overrides: mlua::Table = match overrides.get::<mlua::Table>(frame_id) {
                    Ok(t) => t,
                    Err(_) => {
                        let t = lua.create_table()?;
                        overrides.set(frame_id, t.clone())?;
                        t
                    }
                };
                frame_overrides.set(key, value)?;
                Ok(())
            },
        )?,
    )?;
    Ok(())
}

/// Register Mixin, CreateFromMixins, CreateAndInitFromMixin as Lua globals.
fn register_mixin_globals(lua: &Lua) -> Result<()> {
    lua.load(
        r##"
        function Mixin(object, ...)
            for i = 1, select("#", ...) do
                local mixin = select(i, ...)
                -- Live WoW errors on nil mixins: "Usage: local outObject = Mixin(object, ...)"
                if mixin == nil then
                    error("Usage: local outObject = Mixin(object, ...)")
                end
                -- For secure mixins (transformed by secureMixin XML attribute),
                -- use the stable methods table stored in __secureMixinMethods.
                -- This prevents user-added direct entries (e.g. test fixtures) from
                -- propagating to new frame instances created after the mixin is modified.
                local source = (__secureMixinMethods and __secureMixinMethods[mixin]) or mixin
                for k, v in pairs(source) do
                    -- For LightUserData frames, store function values in __mixin_overrides
                    -- so they shadow built-in Rust methods (mirrors WoW where frames are
                    -- tables and Mixin rawsets directly, taking precedence over __index).
                    if type(object) == "userdata" and type(v) == "function" then
                        __SetMixinOverride(object, k, v)
                    end
                    object[k] = v
                end
            end
            return object
        end

        function CreateFromMixins(...)
            return Mixin({}, ...)
        end

        function CreateAndInitFromMixin(mixin, ...)
            local object = CreateFromMixins(mixin)
            if object.Init then
                object:Init(...)
            end
            return object
        end
    "##,
    )
    .exec()?;
    Ok(())
}

/// Register ScrollBox factory functions: CreateScrollBoxPadding, CreateScrollBoxLinearView.
fn register_scrollbox_globals(lua: &Lua) -> Result<()> {
    lua.load(
        r##"
        function CreateScrollBoxPadding(top, bottom, left, right, spacing)
            return {
                top = top or 0,
                bottom = bottom or 0,
                left = left or 0,
                right = right or 0,
                spacing = spacing or 0,
                GetSpacing = function(self) return self.spacing end,
                GetLeft = function(self) return self.left end,
                GetRight = function(self) return self.right end,
                GetTop = function(self) return self.top end,
                GetBottom = function(self) return self.bottom end,
            }
        end

        function CreateScrollBoxLinearView(top, bottom, left, right, spacing)
            local view = CreateAndInitFromMixin(ScrollBoxLinearViewMixin, top, bottom, left, right, spacing)
            return view
        end
    "##,
    )
    .exec()?;
    Ok(())
}

/// Register utility factory functions: GetFinalNameFromTextureKit, NineSliceUtil, CreateFramePool.
fn register_utility_stubs(lua: &Lua) -> Result<()> {
    lua.load(
        r##"
        -- TextureKit utility
        function GetFinalNameFromTextureKit(kit)
            if not kit or not kit.name then
                return ""
            end
            return kit.name
        end

        -- NineSlice utility table
        NineSliceUtil = {
            GetBorderSizes = function(self, border)
                if not border then return 0, 0, 0, 0 end
                return border.top or 0, border.bottom or 0, border.left or 0, border.right or 0
            end,
        }

        -- Frame pool creation
        function CreateFramePool(frameType, parent, template)
            local pool = {}
            function pool:Acquire()
                return CreateFrame(frameType or "Frame", nil, parent, template)
            end
            function pool:Release(frame)
                if frame then
                    frame:Hide()
                    frame:ClearAllPoints()
                end
            end
            function pool:EnumerateActive()
                return function() end
            end
            return pool
        end

        function CreateTexturePool(parent, template)
            local pool = {}
            function pool:Acquire()
                local parent_frame = type(parent) == "string" and _G[parent] or parent
                if not parent_frame then parent_frame = UIParent end
                return parent_frame:CreateTexture(nil, template)
            end
            function pool:Release(texture)
                if texture then
                    texture:SetTexture(nil)
                end
            end
            function pool:EnumerateActive()
                return function() end
            end
            return pool
        end
    "##,
    )
    .exec()?;
    Ok(())
}
