//! Lua file loading functionality.

use crate::lua_api::LoaderEnv;
use crate::lua_api::globals::security::mark_secure_state;
use crate::lua_api::methods::create_string;
use crate::lua_api::script_helpers::call_error_handler_state;
use crate::lua_api::taint::stamp_addon_taint_state;
use rilua::LuaApiMut;
use rilua::vm::state::LuaState;
use std::borrow::Cow;
use std::path::Path;
use std::time::Instant;

use super::LoadTiming;
use super::addon::AddonContext;
use super::bytecode_cache;
use super::error::LoadError;

/// Load a Lua file into the environment with addon varargs.
pub fn load_lua_file(
    env: &LoaderEnv<'_>,
    path: &Path,
    ctx: &AddonContext,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let io_start = Instant::now();
    let bytes = std::fs::read(path)?;
    timing.io_time += io_start.elapsed();

    let chunk_name = wow_chunk_name(path);
    let patched_source = patch_lua_source(&bytes, &chunk_name);

    let compile_start = Instant::now();
    let func_result =
        env.with_state(|state| load_cached_or_compile(state, &patched_source, &chunk_name, timing));
    let compile_elapsed = compile_start.elapsed();
    timing.lua_compile_time += compile_elapsed;
    timing.lua_exec_time += compile_elapsed;
    let func = func_result?;

    let call_start = Instant::now();
    // Stamp addon taint on the compiled function's GC header.
    // When the VM executes it, fixedtaint = cl->taint blocks read-propagation
    // and inner closures inherit via writetaint.
    let exec_result = env.with_state(|state| {
        if ctx.taint {
            set_object_taint(state, &func, ctx.name);
        }
        if ctx.use_secure_env {
            mark_secure_state(state, &func).map_err(|e| report_lua_load_error(state, e))?;
        }
        exec_addon_func(state, func, ctx).map_err(|e| {
            if let LoadError::Lua(msg) = &e {
                let contextual = format!("{chunk_name}: {msg}");
                call_error_handler_state(state, &contextual);
                return LoadError::Lua(contextual);
            }
            e
        })
    });
    let call_elapsed = call_start.elapsed();
    timing.lua_call_time += call_elapsed;
    timing.lua_exec_time += call_elapsed;
    exec_result?;

    Ok(())
}

/// Transform path to WoW-style chunk name for debugstack.
fn wow_chunk_name(path: &Path) -> String {
    let path_str = path.display().to_string();
    if let Some(pos) = path_str.find("AddOns/") {
        format!("@Interface/{}", &path_str[pos..])
    } else {
        format!("@{}", path_str)
    }
}

fn patch_lua_source<'a>(bytes: &'a [u8], chunk_name: &str) -> Cow<'a, [u8]> {
    let Ok(source) = std::str::from_utf8(bytes) else {
        return Cow::Borrowed(bytes);
    };

    let patched = if chunk_name.ends_with("/ChatFrameUtil.lua") {
        source
            .replace(
                "local info = ChatTypeInfo[\"SYSTEM\"];",
                "local info = ChatTypeInfo[\"SYSTEM\"] or { r = 1, g = 1, b = 0, id = 1 };",
            )
            .replacen(
                "previousValue:Hide();",
                "if type(previousValue.Hide) == \"function\" then previousValue:Hide(); end",
                1,
            )
            .replacen(
                "FCFClickAnywhereButton_UpdateState(previousValue.chatFrame.clickAnywhereButton);",
                "if previousValue.chatFrame and previousValue.chatFrame.clickAnywhereButton then FCFClickAnywhereButton_UpdateState(previousValue.chatFrame.clickAnywhereButton); end",
                1,
            )
            .replacen(
                "FCFClickAnywhereButton_UpdateState(editBox.chatFrame.clickAnywhereButton);",
                "if editBox.chatFrame and editBox.chatFrame.clickAnywhereButton then FCFClickAnywhereButton_UpdateState(editBox.chatFrame.clickAnywhereButton); end",
                1,
            )
    } else if chunk_name.ends_with("/Deprecated_ArenaUI.lua") {
        source
            .replacen(
                "self.layoutIndex = self:GetParent().layoutIndex + 1;",
                "self.layoutIndex = (self:GetParent().layoutIndex or ((id * 2) - 1)) + 1;",
                1,
            )
            .replacen(
                "_G[prefix..\"HealthBar\"]:SetBarTextZeroText(DEAD);",
                "if _G[prefix..\"HealthBar\"] then _G[prefix..\"HealthBar\"]:SetBarTextZeroText(DEAD); end",
                1,
            )
            .replacen(
                "_G[prefix..\"Name\"]:Hide();",
                "if _G[prefix..\"Name\"] then _G[prefix..\"Name\"]:Hide(); end",
                1,
            )
    } else if chunk_name.ends_with("/VoiceChatTranscriptionFrame.lua") {
        source.replace(
            "chatInfo = ChatTypeInfo[chatType];",
            "chatInfo = ChatTypeInfo[chatType] or ChatTypeInfo.SYSTEM or { r = 1, g = 1, b = 0, id = 1 };",
        )
    } else if chunk_name.ends_with("/EventUtil.lua") {
        format!(
            "if EventUtil ~= nil then return end\n{}",
            source.replace(
                "callback();",
                "if type(callback) == \"function\" then callback(); end",
            )
        )
    } else if chunk_name.ends_with("/EditModeManager.lua") {
        source
            .replacen(
                "function EditModeManagerFrameMixin:ReconcileLayoutsWithModern()\n\tlocal somethingChanged = false;",
                "function EditModeManagerFrameMixin:ReconcileLayoutsWithModern()\n\tif type(self.layoutInfo) ~= \"table\" or type(self.layoutInfo.layouts) ~= \"table\" then\n\t\treturn false;\n\tend\n\tlocal somethingChanged = false;",
                1,
            )
            .replacen(
                "function EditModeManagerFrameMixin:UpdateLayoutInfo(layoutInfo, reconcileLayouts)\n\tself.layoutApplyInProgress = true;\n\tself.layoutInfo = layoutInfo;",
                "function EditModeManagerFrameMixin:UpdateLayoutInfo(layoutInfo, reconcileLayouts)\n\tself.layoutApplyInProgress = true;\n\tself.layoutInfo = layoutInfo or self.layoutInfo or { layouts = {}, activeLayout = 1 };\n\tif type(self.layoutInfo.layouts) ~= \"table\" then\n\t\tself.layoutInfo.layouts = {};\n\tend",
                1,
            )
    } else if chunk_name.ends_with("/MainMenuBarMicroButtons.lua") {
        source
            .replace(
                "local wasShown = CatalogShopInboundInterface.IsShown();",
                "local wasShown = false;\n\t\tif CatalogShopInboundInterface and type(CatalogShopInboundInterface.IsShown) == \"function\" then\n\t\t\tlocal ok, value = pcall(CatalogShopInboundInterface.IsShown);\n\t\t\twasShown = ok and value or false;\n\t\tend",
            )
            .replace(
                "local wasShown = StoreFrame_IsShown();",
                "local wasShown = false;\n\t\tif type(StoreFrame_IsShown) == \"function\" then\n\t\t\tlocal ok, value = pcall(StoreFrame_IsShown);\n\t\t\twasShown = ok and value or false;\n\t\tend",
            )
    } else if chunk_name.ends_with("/UIParent.lua") {
        source
            .replacen(
                "if ( lastTalkedToGM ~= \"\" ) then",
                "if false and ( lastTalkedToGM ~= \"\" ) then",
                1,
            )
            .replacen(
                "NPETutorial_AttemptToBegin(event);",
                "if type(NPETutorial_AttemptToBegin) == \"function\" then NPETutorial_AttemptToBegin(event); end",
                1,
            )
            .replacen(
                "UpdateMicroButtons();",
                "pcall(UpdateMicroButtons);",
                1,
            )
            .replacen(
                "CatalogShopInboundInterface.CheckForFree(event);",
                "if CatalogShopInboundInterface and type(CatalogShopInboundInterface.CheckForFree) == \"function\" then CatalogShopInboundInterface.CheckForFree(event); end",
                1,
            )
            .replacen(
                "StoreFrame_CheckForFree(event);",
                "if type(StoreFrame_CheckForFree) == \"function\" then StoreFrame_CheckForFree(event); end",
                1,
            )
            .replacen(
                "EventUtil.TriggerOnVariablesLoaded();",
                "-- EventUtil.TriggerOnVariablesLoaded() skipped in rilua startup",
                1,
            )
    } else if chunk_name.ends_with("/Blizzard_Shared_StoreUIInbound.lua") {
        source.replace(
            "function StoreFrame_IsShown()\n\treturn StoreFrame:GetAttribute(\"isshown\");\nend",
            "function StoreFrame_IsShown()\n\tif type(StoreFrame) ~= \"table\" or type(StoreFrame.GetAttribute) ~= \"function\" then\n\t\treturn false;\n\tend\n\tlocal ok, shown = pcall(StoreFrame.GetAttribute, StoreFrame, \"isshown\");\n\treturn ok and shown or false;\nend",
        )
    } else if chunk_name.ends_with("/MinimalSlider.lua") {
        source.replace(
            "self.Slider.Thumb:SetAlpha(alpha);",
            "if self.Slider and self.Slider.Thumb then self.Slider.Thumb:SetAlpha(alpha); end",
        )
    } else if chunk_name.ends_with("/FloatingChatFrame.lua") {
        source
            .replace(
                "UIFrameFadeIn(object, CHAT_FRAME_FADE_TIME, object:GetAlpha(), max(chatFrame.oldAlpha, DEFAULT_CHATFRAME_ALPHA));",
                "UIFrameFadeIn(object, CHAT_FRAME_FADE_TIME, object:GetAlpha(), max(chatFrame.oldAlpha or DEFAULT_CHATFRAME_ALPHA, DEFAULT_CHATFRAME_ALPHA));",
            )
            .replace(
                "UIFrameFadeOut(object, CHAT_FRAME_FADE_OUT_TIME, max(object:GetAlpha(), chatFrame.oldAlpha), chatFrame.oldAlpha);",
                "UIFrameFadeOut(object, CHAT_FRAME_FADE_OUT_TIME, max(object:GetAlpha() or 0, chatFrame.oldAlpha or DEFAULT_CHATFRAME_ALPHA), chatFrame.oldAlpha or DEFAULT_CHATFRAME_ALPHA);",
            )
    } else if chunk_name.ends_with("/TextToSpeechFrame.lua") {
        source.replace(
            "TextToSpeechFrame_SetupVoiceDropdown(self);\n\t\tTextToSpeechFrame_SetupAlternateVoiceDropdown(self);",
            "if type(TextToSpeechFrame_SetupVoiceDropdown) ~= \"function\" then\n\t\t\tfunction TextToSpeechFrame_SetupVoiceDropdown(self)\n\t\t\t\tSetupVoiceMenu(self.PanelContainer.TtsVoiceDropdown, Enum.TtsVoiceType.Standard);\n\t\t\tend\n\t\tend\n\t\tif type(TextToSpeechFrame_SetupAlternateVoiceDropdown) ~= \"function\" then\n\t\t\tfunction TextToSpeechFrame_SetupAlternateVoiceDropdown(self)\n\t\t\t\tSetupVoiceMenu(self.PanelContainer.TtsVoiceAlternateDropdown, Enum.TtsVoiceType.Alternate);\n\t\t\tend\n\t\tend\n\n\t\tTextToSpeechFrame_SetupVoiceDropdown(self);\n\t\tTextToSpeechFrame_SetupAlternateVoiceDropdown(self);",
        )
    } else if chunk_name.ends_with("/Blizzard_PetBattleUI.lua") {
        source
            .replace(
                "cooldown = max(currentCooldown, currentLockdown);",
                "cooldown = max(currentCooldown or 0, currentLockdown or 0);",
            )
            .replace(
                "self.XPBar:SetWidth(max((xp / max(maxXp,1)) * self.xpBarWidth, 1));",
                "self.XPBar:SetWidth(max(((xp or 0) / max(maxXp or 1,1)) * self.xpBarWidth, 1));",
            )
            .replace(
                "self.ActualHealthBar:SetWidth((health / max(maxHealth,1)) * self.healthBarWidth);",
                "self.ActualHealthBar:SetWidth(((health or 0) / max(maxHealth or 1,1)) * self.healthBarWidth);",
            )
    } else {
        return Cow::Borrowed(bytes);
    };
    if patched == source {
        return Cow::Borrowed(bytes);
    }
    Cow::Owned(patched.into_bytes())
}

/// Execute a compiled addon function.
/// Taint is already stamped on the function's GC header by the caller.
fn exec_addon_func(
    state: &mut LuaState,
    func: rilua::Function,
    ctx: &AddonContext,
) -> Result<(), LoadError> {
    let name = create_string(state, ctx.name);
    crate::lua_api::methods::call_function_state(
        state,
        rilua::Val::Function(func.gc_ref()),
        &[name, ctx.table],
    )
    .map(|_| ())
    .map_err(|e| LoadError::Lua(e.to_string()))
}

/// Try loading from bytecode cache; compile and cache on miss.
///
/// NOTE on secureenv: this function returns a fresh `rilua::Function` handle
/// whether the compiled body came from the cache or from source. The caller
/// (`load_lua_file`) applies `mark_secure_state` to that handle *after* this
/// function returns, so cache-replayed chunks and fresh compilations are
/// indistinguishable from secureenv's point of view — both get their fenv
/// swapped before the chunk ever runs. That ordering must be preserved if
/// anyone wires actual cache `get`/`put` calls here.
fn load_cached_or_compile(
    lua: &mut LuaState,
    bytes: &[u8],
    chunk_name: &str,
    timing: &mut LoadTiming,
) -> Result<rilua::Function, LoadError> {
    let hash = bytecode_cache::content_hash(bytes, chunk_name);
    let legacy_hash = bytecode_cache::legacy_content_hash(bytes, chunk_name);

    if !bytecode_cache::is_disabled() {
        if let Some(bytecode) = bytecode_cache::get_with_legacy_fallback(hash, legacy_hash)
            && let Ok(func) = compile_with_rilua(lua, &bytecode, chunk_name)
        {
            timing.cache_hits += 1;
            return Ok(func);
        }
    }

    timing.cache_misses += 1;
    let func = compile_from_source(lua, bytes, chunk_name)?;
    if !bytecode_cache::is_disabled() {
        let bytecode = crate::loader::bytecode::dump_function(lua, &func)?;
        bytecode_cache::put(hash, &bytecode);
    }
    Ok(func)
}

/// Set taint on a Lua function's GC object header via `debug.setobjecttaint`.
fn set_object_taint(state: &mut LuaState, func: &rilua::Function, taint: &str) {
    stamp_addon_taint_state(state, func, taint);
}

fn report_lua_load_error(state: &mut LuaState, err: impl ToString) -> LoadError {
    let msg = err.to_string();
    call_error_handler_state(state, &msg);
    LoadError::Lua(msg)
}

/// Compile Lua source code into a function.
fn compile_from_source(
    lua: &mut LuaState,
    bytes: &[u8],
    chunk_name: &str,
) -> Result<rilua::Function, LoadError> {
    compile_with_rilua(lua, bytes, chunk_name).map_err(|e| report_lua_load_error(lua, e))
}

/// Compile Lua source code using rilua's compiler (pure Rust).
///
/// This is the rilua-side equivalent of `compile_from_source`. It compiles
/// source code and returns a rilua Function handle. Used as a parallel
/// compilation path for Phase 3 migration — the mlua path remains active
/// until the full VM switch.
pub fn compile_with_rilua<L: LuaApiMut>(
    lua: &mut L,
    bytes: &[u8],
    chunk_name: &str,
) -> Result<rilua::Function, LoadError> {
    // Strip UTF-8 BOM if present
    let bytes = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &bytes[3..]
    } else {
        bytes
    };
    lua.load_bytes(bytes, chunk_name)
        .map_err(|e| LoadError::Lua(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_cache_chunk_name(prefix: &str) -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        format!("@{prefix}_{}_{}", std::process::id(), nanos)
    }

    #[test]
    fn rilua_compilation_matches_source_semantics() {
        let mut lua = rilua::Lua::new().unwrap();
        let func = compile_with_rilua(&mut lua, b"return 40 + 2", "@test")
            .expect("rilua should compile simple expression");
        let results = lua.call_function(&func, &[]).unwrap();
        assert_eq!(results, vec![rilua::Val::Num(42.0)]);
    }

    #[test]
    fn rilua_compilation_strips_bom() {
        let mut lua = rilua::Lua::new().unwrap();
        let source = b"\xEF\xBB\xBFreturn 1";
        let func = compile_with_rilua(&mut lua, source, "@bom_test")
            .expect("rilua should handle BOM-prefixed source");
        let results = lua.call_function(&func, &[]).unwrap();
        assert_eq!(results, vec![rilua::Val::Num(1.0)]);
    }

    /// A function handle reloaded from cached bytecode must accept `setfenv`
    /// exactly like one produced by fresh compilation. This protects the
    /// invariant documented on `load_cached_or_compile`: regardless of
    /// whether `compile_with_rilua` consumed source or bytecode,
    /// `state_set_fenv` can still retarget the returned closure so
    /// downstream `mark_secure_state` works on cache hits.
    #[test]
    fn bytecode_replayed_function_accepts_setfenv() {
        use rilua::LuaApiMut;

        let mut lua = rilua::Lua::new().unwrap();
        // Fresh compile -> get a Function whose prototype we can dump.
        let func_from_source =
            compile_with_rilua(&mut lua, b"return MARK_SECURE_PROBE", "@cache_test")
                .expect("source compile should succeed");

        let proto = {
            let state = lua.state_mut();
            let closure = state
                .gc
                .closures
                .get(func_from_source.gc_ref())
                .expect("closure exists");
            match closure {
                rilua::vm::closure::Closure::Lua(cl) => cl.proto.clone(),
                rilua::vm::closure::Closure::Rust(_) => {
                    panic!("compiled source should produce a Lua closure")
                }
            }
        };

        // Dump to Lua 5.1 bytecode bytes — what bytecode_cache would store.
        let bytecode = {
            let state = lua.state_mut();
            rilua::vm::dump::dump(&proto, Some(&state.gc.string_arena), false)
        };

        // Simulate a cache hit: feed bytecode back through the same entry
        // point the loader uses on miss.
        let func_from_bytecode = compile_with_rilua(&mut lua, &bytecode, "@cache_test")
            .expect("bytecode replay should succeed");

        // Build a fresh env table, point it at a sentinel value.
        let env_table = LuaApiMut::create_table(&mut lua);
        {
            let state = lua.state_mut();
            let sentinel = rilua::Val::Str(state.gc.intern_string_static(b"from-secureenv"));
            let key = rilua::Val::Str(state.gc.intern_string_static(b"MARK_SECURE_PROBE"));
            env_table.raw_set(state, key, sentinel).unwrap();
        }

        // Swap the replayed closure's fenv — the secureenv path.
        rilua::api::state_set_fenv(lua.state_mut(), &func_from_bytecode, &env_table)
            .expect("state_set_fenv should accept bytecode-replayed closure");

        // The replayed chunk should resolve MARK_SECURE_PROBE through the
        // swapped env, not _G (where it's nil).
        let results = lua.call_function(&func_from_bytecode, &[]).unwrap();
        let resolved = results.into_iter().next().expect("chunk returns a value");
        let rilua::Val::Str(s) = resolved else {
            panic!("expected string, got {resolved:?}");
        };
        assert_eq!(
            lua.val_as_bytes(rilua::Val::Str(s)).unwrap(),
            b"from-secureenv"
        );
    }

    #[test]
    fn load_cached_or_compile_hits_bytecode_cache_on_second_load() {
        let chunk_name = unique_cache_chunk_name("lua_file_cache");
        let source = format!("return {:?}", chunk_name);

        let mut first_lua = rilua::Lua::new().unwrap();
        let mut first_timing = LoadTiming::default();
        let first_func = {
            let state = first_lua.state_mut();
            load_cached_or_compile(state, source.as_bytes(), &chunk_name, &mut first_timing)
        }
        .expect("first compile should succeed");
        let first_results = first_lua.call_function(&first_func, &[]).unwrap();
        let first_value = first_results
            .into_iter()
            .next()
            .expect("first call returns value");
        assert_eq!(first_timing.cache_hits, 0);
        assert_eq!(first_timing.cache_misses, 1);
        assert_eq!(
            first_lua.val_as_bytes(first_value).unwrap(),
            chunk_name.as_bytes()
        );

        let mut second_lua = rilua::Lua::new().unwrap();
        let mut second_timing = LoadTiming::default();
        let second_func = {
            let state = second_lua.state_mut();
            load_cached_or_compile(state, source.as_bytes(), &chunk_name, &mut second_timing)
        }
        .expect("second compile should succeed");
        let second_results = second_lua.call_function(&second_func, &[]).unwrap();
        let second_value = second_results
            .into_iter()
            .next()
            .expect("second call returns value");
        assert_eq!(
            second_timing.cache_hits, 1,
            "second load should reuse cached bytecode"
        );
        assert_eq!(second_timing.cache_misses, 0);
        assert_eq!(
            second_lua.val_as_bytes(second_value).unwrap(),
            chunk_name.as_bytes()
        );
    }
}
