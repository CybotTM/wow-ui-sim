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
use super::bytecode_cache::PutResult;
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

    let func = compile_lua_file(env, &patched_source, &chunk_name, timing)?;
    execute_lua_file(env, func, ctx, &chunk_name, timing)?;

    Ok(())
}

fn compile_lua_file(
    env: &LoaderEnv<'_>,
    patched_source: &[u8],
    chunk_name: &str,
    timing: &mut LoadTiming,
) -> Result<rilua::Function, LoadError> {
    let compile_start = Instant::now();
    let func_result = env.with_state(|state| {
        load_cached_or_compile_for_chunk(state, patched_source, chunk_name, timing)
    });
    let compile_elapsed = compile_start.elapsed();
    timing.lua_compile_time += compile_elapsed;
    timing.lua_exec_time += compile_elapsed;

    func_result
}

fn load_cached_or_compile_for_chunk(
    state: &mut LuaState,
    patched_source: &[u8],
    chunk_name: &str,
    timing: &mut LoadTiming,
) -> Result<rilua::Function, LoadError> {
    if chunk_name.starts_with("@Interface/") {
        return load_cached_or_compile(state, patched_source, chunk_name, timing);
    }

    let saved_slots = state.global_slots.take();
    let result = load_cached_or_compile(state, patched_source, chunk_name, timing);
    state.global_slots = saved_slots;
    result
}

fn execute_lua_file(
    env: &LoaderEnv<'_>,
    func: rilua::Function,
    ctx: &AddonContext,
    chunk_name: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let call_start = Instant::now();
    let exec_result =
        env.with_state(|state| execute_compiled_lua_file(state, func, ctx, chunk_name));
    let call_elapsed = call_start.elapsed();
    timing.lua_call_time += call_elapsed;
    timing.lua_exec_time += call_elapsed;

    exec_result
}

fn execute_compiled_lua_file(
    state: &mut LuaState,
    func: rilua::Function,
    ctx: &AddonContext,
    chunk_name: &str,
) -> Result<(), LoadError> {
    // Stamp addon taint on the compiled function's GC header. When the VM executes
    // it, fixedtaint blocks read-propagation and inner closures inherit writetaint.
    if ctx.taint {
        set_object_taint(state, &func, ctx.name);
    }
    if ctx.use_secure_env {
        mark_secure_state(state, &func).map_err(|e| report_lua_load_error(state, e))?;
    }
    exec_addon_func(state, func, ctx).map_err(|e| contextual_lua_load_error(state, e, chunk_name))
}

fn contextual_lua_load_error(
    state: &mut LuaState,
    error: LoadError,
    chunk_name: &str,
) -> LoadError {
    let LoadError::Lua(msg) = error else {
        return error;
    };

    let contextual = format!("{chunk_name}: {msg}");
    call_error_handler_state(state, &contextual);
    LoadError::Lua(contextual)
}

/// Transform path to WoW-style chunk name for debugstack.
fn wow_chunk_name(path: &Path) -> String {
    let path_str = path.display().to_string().replace('\\', "/");
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

    let mut patched = Cow::Borrowed(source);
    if let Some(patch) = lua_source_patch_for_chunk(chunk_name) {
        patched = Cow::Owned(apply_lua_source_patch(&patched, patch.operations));
    }

    if patched.as_ref() == source {
        return Cow::Borrowed(bytes);
    }
    Cow::Owned(patched.into_owned().into_bytes())
}

struct LuaSourcePatch {
    suffix: &'static str,
    operations: &'static [LuaSourcePatchOp],
}

enum LuaSourcePatchOp {
    Prefix(&'static str),
    Replace {
        from: &'static str,
        to: &'static str,
    },
    ReplaceOnce {
        from: &'static str,
        to: &'static str,
    },
}

const LUA_SOURCE_PATCHES: &[LuaSourcePatch] = &[
    LuaSourcePatch {
        suffix: "/ChatFrameUtil.lua",
        operations: &[
            LuaSourcePatchOp::Replace {
                from: "local info = ChatTypeInfo[\"SYSTEM\"];",
                to: "local info = ChatTypeInfo[\"SYSTEM\"] or { r = 1, g = 1, b = 0, id = 1 };",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "previousValue:Hide();",
                to: "if type(previousValue.Hide) == \"function\" then previousValue:Hide(); end",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "FCFClickAnywhereButton_UpdateState(previousValue.chatFrame.clickAnywhereButton);",
                to: "if previousValue.chatFrame and previousValue.chatFrame.clickAnywhereButton then FCFClickAnywhereButton_UpdateState(previousValue.chatFrame.clickAnywhereButton); end",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "FCFClickAnywhereButton_UpdateState(editBox.chatFrame.clickAnywhereButton);",
                to: "if editBox.chatFrame and editBox.chatFrame.clickAnywhereButton then FCFClickAnywhereButton_UpdateState(editBox.chatFrame.clickAnywhereButton); end",
            },
        ],
    },
    LuaSourcePatch {
        suffix: "/Deprecated_ArenaUI.lua",
        operations: &[
            LuaSourcePatchOp::ReplaceOnce {
                from: "self.layoutIndex = self:GetParent().layoutIndex + 1;",
                to: "self.layoutIndex = (self:GetParent().layoutIndex or ((id * 2) - 1)) + 1;",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "_G[prefix..\"HealthBar\"]:SetBarTextZeroText(DEAD);",
                to: "if _G[prefix..\"HealthBar\"] then _G[prefix..\"HealthBar\"]:SetBarTextZeroText(DEAD); end",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "_G[prefix..\"Name\"]:Hide();",
                to: "if _G[prefix..\"Name\"] then _G[prefix..\"Name\"]:Hide(); end",
            },
        ],
    },
    LuaSourcePatch {
        suffix: "/Blizzard_CodeOfConduct.lua",
        operations: &[LuaSourcePatchOp::Replace {
            from: "ChatFrameUtil.AddSystemMessage(ONLINE_SAFETY_NOTICE);",
            to: "-- suppressed in the simulator to keep chat history tests stable",
        }],
    },
    LuaSourcePatch {
        suffix: "/VoiceChatTranscriptionFrame.lua",
        operations: &[LuaSourcePatchOp::Replace {
            from: "chatInfo = ChatTypeInfo[chatType];",
            to: "chatInfo = ChatTypeInfo[chatType] or ChatTypeInfo.SYSTEM or { r = 1, g = 1, b = 0, id = 1 };",
        }],
    },
    LuaSourcePatch {
        suffix: "/EventUtil.lua",
        operations: &[
            LuaSourcePatchOp::Replace {
                from: "callback();",
                to: "if type(callback) == \"function\" then callback(); end",
            },
            LuaSourcePatchOp::Prefix("if EventUtil ~= nil then return end\n"),
        ],
    },
    LuaSourcePatch {
        suffix: "/LocalizationMachinery.lua",
        operations: &[LuaSourcePatchOp::Prefix(
            "if SetupLocalization ~= nil then return end\n",
        )],
    },
    LuaSourcePatch {
        suffix: "/Blizzard_AddOnList/AddonList.lua",
        operations: &[LuaSourcePatchOp::ReplaceOnce {
            from: "local group = C_AddOns.GetAddOnMetadata(i, \"Group\");",
            to: "local group = C_AddOns.GetAddOnMetadata(i, \"Group\");\n\t\tif type(group) ~= \"string\" or group == \"\" then\n\t\t\tgroup = C_AddOns.GetAddOnName(i);\n\t\tend",
        }],
    },
    LuaSourcePatch {
        suffix: "/EditModeManager.lua",
        operations: &[
            LuaSourcePatchOp::ReplaceOnce {
                from: "function EditModeManagerFrameMixin:ReconcileLayoutsWithModern()\n\tlocal somethingChanged = false;",
                to: "function EditModeManagerFrameMixin:ReconcileLayoutsWithModern()\n\tif type(self.layoutInfo) ~= \"table\" or type(self.layoutInfo.layouts) ~= \"table\" then\n\t\treturn false;\n\tend\n\tlocal somethingChanged = false;",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "function EditModeManagerFrameMixin:UpdateLayoutInfo(layoutInfo, reconcileLayouts)\n\tself.layoutApplyInProgress = true;\n\tself.layoutInfo = layoutInfo;",
                to: "function EditModeManagerFrameMixin:UpdateLayoutInfo(layoutInfo, reconcileLayouts)\n\tself.layoutApplyInProgress = true;\n\tself.layoutInfo = layoutInfo or self.layoutInfo or { layouts = {}, activeLayout = 1 };\n\tif type(self.layoutInfo.layouts) ~= \"table\" then\n\t\tself.layoutInfo.layouts = {};\n\tend",
            },
        ],
    },
    LuaSourcePatch {
        suffix: "/MainMenuBarMicroButtons.lua",
        operations: &[
            LuaSourcePatchOp::Replace {
                from: "local wasShown = CatalogShopInboundInterface.IsShown();",
                to: "local wasShown = false;\n\t\tif CatalogShopInboundInterface and type(CatalogShopInboundInterface.IsShown) == \"function\" then\n\t\t\tlocal ok, value = pcall(CatalogShopInboundInterface.IsShown);\n\t\t\twasShown = ok and value or false;\n\t\tend",
            },
            LuaSourcePatchOp::Replace {
                from: "local wasShown = StoreFrame_IsShown();",
                to: "local wasShown = false;\n\t\tif type(StoreFrame_IsShown) == \"function\" then\n\t\t\tlocal ok, value = pcall(StoreFrame_IsShown);\n\t\t\twasShown = ok and value or false;\n\t\tend",
            },
        ],
    },
    LuaSourcePatch {
        suffix: "/UIParent.lua",
        operations: &[
            LuaSourcePatchOp::ReplaceOnce {
                from: "if ( lastTalkedToGM ~= \"\" ) then",
                to: "if false and ( lastTalkedToGM ~= \"\" ) then",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "NPETutorial_AttemptToBegin(event);",
                to: "if type(NPETutorial_AttemptToBegin) == \"function\" then NPETutorial_AttemptToBegin(event); end",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "UpdateMicroButtons();",
                to: "pcall(UpdateMicroButtons);",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "CatalogShopInboundInterface.CheckForFree(event);",
                to: "if CatalogShopInboundInterface and type(CatalogShopInboundInterface.CheckForFree) == \"function\" then CatalogShopInboundInterface.CheckForFree(event); end",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "StoreFrame_CheckForFree(event);",
                to: "if type(StoreFrame_CheckForFree) == \"function\" then StoreFrame_CheckForFree(event); end",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "EventUtil.TriggerOnVariablesLoaded();",
                to: "-- EventUtil.TriggerOnVariablesLoaded() skipped in rilua startup",
            },
        ],
    },
    LuaSourcePatch {
        suffix: "/Blizzard_Shared_StoreUIInbound.lua",
        operations: &[LuaSourcePatchOp::Replace {
            from: "function StoreFrame_IsShown()\n\treturn StoreFrame:GetAttribute(\"isshown\");\nend",
            to: "function StoreFrame_IsShown()\n\tif type(StoreFrame) ~= \"table\" or type(StoreFrame.GetAttribute) ~= \"function\" then\n\t\treturn false;\n\tend\n\tlocal ok, shown = pcall(StoreFrame.GetAttribute, StoreFrame, \"isshown\");\n\treturn ok and shown or false;\nend",
        }],
    },
    LuaSourcePatch {
        suffix: "/MinimalSlider.lua",
        operations: &[LuaSourcePatchOp::Replace {
            from: "self.Slider.Thumb:SetAlpha(alpha);",
            to: "if self.Slider and self.Slider.Thumb then self.Slider.Thumb:SetAlpha(alpha); end",
        }],
    },
    LuaSourcePatch {
        suffix: "/FloatingChatFrame.lua",
        operations: &[
            LuaSourcePatchOp::Replace {
                from: "DEFAULT_TAB_SELECTED_COLOR_TABLE = { r = 1, g = 0.5, b = 0.25 };",
                to: "DEFAULT_TAB_SELECTED_COLOR_TABLE = { r = 1, g = 0.5, b = 0.25 };\n\nlocal function __wow_ensure_chat_tab_font_string(button)\n\tif not button then\n\t\treturn nil;\n\tend\n\n\tlocal fontString = button:GetFontString();\n\tif fontString then\n\t\treturn fontString;\n\tend\n\tif type(button.CreateFontString) ~= \"function\" then\n\t\treturn nil;\n\tend\n\n\tlocal name = button.GetName and button:GetName();\n\tlocal childName = type(name) == \"string\" and name ~= \"\" and (name..\"Text\") or nil;\n\tfontString = button:CreateFontString(childName, \"ARTWORK\");\n\tif fontString and type(button.SetFontString) == \"function\" then\n\t\tbutton:SetFontString(fontString);\n\tend\n\treturn fontString;\nend",
            },
            LuaSourcePatchOp::Replace {
                from: "UIFrameFadeIn(object, CHAT_FRAME_FADE_TIME, object:GetAlpha(), max(chatFrame.oldAlpha, DEFAULT_CHATFRAME_ALPHA));",
                to: "UIFrameFadeIn(object, CHAT_FRAME_FADE_TIME, object:GetAlpha(), max(chatFrame.oldAlpha or DEFAULT_CHATFRAME_ALPHA, DEFAULT_CHATFRAME_ALPHA));",
            },
            LuaSourcePatchOp::Replace {
                from: "UIFrameFadeOut(object, CHAT_FRAME_FADE_OUT_TIME, max(object:GetAlpha(), chatFrame.oldAlpha), chatFrame.oldAlpha);",
                to: "UIFrameFadeOut(object, CHAT_FRAME_FADE_OUT_TIME, max(object:GetAlpha() or 0, chatFrame.oldAlpha or DEFAULT_CHATFRAME_ALPHA), chatFrame.oldAlpha or DEFAULT_CHATFRAME_ALPHA);",
            },
            LuaSourcePatchOp::Replace {
                from: "self:GetFontString():SetTextColor(colorTable.r, colorTable.g, colorTable.b);",
                to: "do local fontString = __wow_ensure_chat_tab_font_string(self); if fontString then fontString:SetTextColor(colorTable.r, colorTable.g, colorTable.b); end end",
            },
            LuaSourcePatchOp::Replace {
                from: "self:GetFontString():SetTextColor(NORMAL_FONT_COLOR.r, NORMAL_FONT_COLOR.g, NORMAL_FONT_COLOR.b);",
                to: "do local fontString = __wow_ensure_chat_tab_font_string(self); if fontString then fontString:SetTextColor(NORMAL_FONT_COLOR.r, NORMAL_FONT_COLOR.g, NORMAL_FONT_COLOR.b); end end",
            },
            LuaSourcePatchOp::Replace {
                from: "minFrame:GetFontString():SetTextColor(colorTable.r, colorTable.g, colorTable.b);",
                to: "do local fontString = __wow_ensure_chat_tab_font_string(minFrame); if fontString then fontString:SetTextColor(colorTable.r, colorTable.g, colorTable.b); end end",
            },
            LuaSourcePatchOp::Replace {
                from: "minFrame:GetFontString():SetTextColor(NORMAL_FONT_COLOR.r, NORMAL_FONT_COLOR.g, NORMAL_FONT_COLOR.b);",
                to: "do local fontString = __wow_ensure_chat_tab_font_string(minFrame); if fontString then fontString:SetTextColor(NORMAL_FONT_COLOR.r, NORMAL_FONT_COLOR.g, NORMAL_FONT_COLOR.b); end end",
            },
            LuaSourcePatchOp::Replace {
                from: "button:GetFontString():SetTextColor(colorTable.r, colorTable.g, colorTable.b);",
                to: "do local fontString = __wow_ensure_chat_tab_font_string(button); if fontString then fontString:SetTextColor(colorTable.r, colorTable.g, colorTable.b); end end",
            },
            LuaSourcePatchOp::Replace {
                from: "button:GetFontString():SetTextColor(NORMAL_FONT_COLOR.r, NORMAL_FONT_COLOR.g, NORMAL_FONT_COLOR.b);",
                to: "do local fontString = __wow_ensure_chat_tab_font_string(button); if fontString then fontString:SetTextColor(NORMAL_FONT_COLOR.r, NORMAL_FONT_COLOR.g, NORMAL_FONT_COLOR.b); end end",
            },
        ],
    },
    LuaSourcePatch {
        suffix: "/MenuTemplates.lua",
        operations: &[
            LuaSourcePatchOp::Replace {
                from: "function DropdownTextMixin:UpdateText()\n\tself.Text:SetText(self:GetUpdateText());",
                to: "local function __wow_ensure_dropdown_text_font_string(self)\n\tif self.Text then\n\t\treturn self.Text;\n\tend\n\tif not MenuVariants or type(MenuVariants.CreateFontString) ~= \"function\" then\n\t\treturn nil;\n\tend\n\tlocal ok, fontString = pcall(MenuVariants.CreateFontString, self);\n\tif not ok or fontString == nil then\n\t\treturn nil;\n\tend\n\tself.Text = fontString;\n\treturn fontString;\nend\n\nfunction DropdownTextMixin:UpdateText()\n\tlocal text = __wow_ensure_dropdown_text_font_string(self);\n\tif not text then\n\t\treturn;\n\tend\n\ttext:SetText(self:GetUpdateText());",
            },
            LuaSourcePatchOp::ReplaceOnce {
                from: "local newWidth = self.Text:GetUnboundedStringWidth();",
                to: "local newWidth = text:GetUnboundedStringWidth();",
            },
        ],
    },
    LuaSourcePatch {
        suffix: "TextToSpeechFrame.lua",
        operations: &[LuaSourcePatchOp::Replace {
            from: "function TextToSpeechFrame_CheckLoad(self)",
            to: "local __wow_saved_text_to_speech_voice_dropdown = TextToSpeechFrame_SetupVoiceDropdown\nlocal __wow_saved_text_to_speech_alt_voice_dropdown = TextToSpeechFrame_SetupAlternateVoiceDropdown\nlocal function __wow_ensure_text_to_speech_dropdown_helpers(self)\n\tif type(TextToSpeechFrame_SetupVoiceDropdown) ~= \"function\" then\n\t\tif type(__wow_saved_text_to_speech_voice_dropdown) == \"function\" then\n\t\t\tTextToSpeechFrame_SetupVoiceDropdown = __wow_saved_text_to_speech_voice_dropdown\n\t\telse\n\t\t\tfunction TextToSpeechFrame_SetupVoiceDropdown(frame)\n\t\t\t\tSetupVoiceMenu(frame.PanelContainer.TtsVoiceDropdown, Enum.TtsVoiceType.Standard);\n\t\t\tend\n\t\tend\n\tend\n\tif type(TextToSpeechFrame_SetupAlternateVoiceDropdown) ~= \"function\" then\n\t\tif type(__wow_saved_text_to_speech_alt_voice_dropdown) == \"function\" then\n\t\t\tTextToSpeechFrame_SetupAlternateVoiceDropdown = __wow_saved_text_to_speech_alt_voice_dropdown\n\t\telse\n\t\t\tfunction TextToSpeechFrame_SetupAlternateVoiceDropdown(frame)\n\t\t\t\tSetupVoiceMenu(frame.PanelContainer.TtsVoiceAlternateDropdown, Enum.TtsVoiceType.Alternate);\n\t\t\tend\n\t\tend\n\tend\nend\n\nfunction TextToSpeechFrame_CheckLoad(self)\n\t__wow_ensure_text_to_speech_dropdown_helpers(self)",
        }],
    },
    LuaSourcePatch {
        suffix: "/Blizzard_PetBattleUI.lua",
        operations: &[
            LuaSourcePatchOp::Replace {
                from: "cooldown = max(currentCooldown, currentLockdown);",
                to: "cooldown = max(currentCooldown or 0, currentLockdown or 0);",
            },
            LuaSourcePatchOp::Replace {
                from: "self.XPBar:SetWidth(max((xp / max(maxXp,1)) * self.xpBarWidth, 1));",
                to: "self.XPBar:SetWidth(max(((xp or 0) / max(maxXp or 1,1)) * self.xpBarWidth, 1));",
            },
            LuaSourcePatchOp::Replace {
                from: "self.ActualHealthBar:SetWidth((health / max(maxHealth,1)) * self.healthBarWidth);",
                to: "self.ActualHealthBar:SetWidth(((health or 0) / max(maxHealth or 1,1)) * self.healthBarWidth);",
            },
        ],
    },
];

fn lua_source_patch_for_chunk(chunk_name: &str) -> Option<&'static LuaSourcePatch> {
    let normalized_chunk_name = chunk_name.replace('\\', "/");
    LUA_SOURCE_PATCHES
        .iter()
        .find(|patch| normalized_chunk_name.ends_with(patch.suffix))
}

fn apply_lua_source_patch(source: &str, operations: &[LuaSourcePatchOp]) -> String {
    let mut patched = source.to_string();
    for operation in operations {
        patched = apply_lua_source_patch_operation(&patched, operation);
    }
    patched
}

fn apply_lua_source_patch_operation(source: &str, operation: &LuaSourcePatchOp) -> String {
    match operation {
        LuaSourcePatchOp::Prefix(prefix) => format!("{prefix}{source}"),
        LuaSourcePatchOp::Replace { from, to } => {
            replace_with_line_ending_fallback(source, from, to)
        }
        LuaSourcePatchOp::ReplaceOnce { from, to } => {
            replace_once_with_line_ending_fallback(source, from, to)
        }
    }
}

fn replace_with_line_ending_fallback(source: &str, from: &str, to: &str) -> String {
    let patched = source.replace(from, to);
    if patched != source {
        return patched;
    }
    source.replace(&from.replace('\n', "\r\n"), &to.replace('\n', "\r\n"))
}

fn replace_once_with_line_ending_fallback(source: &str, from: &str, to: &str) -> String {
    let patched = source.replacen(from, to, 1);
    if patched != source {
        return patched;
    }
    source.replacen(&from.replace('\n', "\r\n"), &to.replace('\n', "\r\n"), 1)
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
        match bytecode_cache::get_with_legacy_fallback(hash, legacy_hash) {
            Some(bytecode) => match compile_with_rilua(lua, &bytecode, chunk_name) {
                Ok(func) => {
                    timing.cache_hits += 1;
                    return Ok(func);
                }
                Err(_) => {
                    timing.cache_replay_failures += 1;
                }
            },
            None => {
                timing.cache_lookup_misses += 1;
            }
        }
    }

    timing.cache_misses += 1;
    let func = compile_from_source(lua, bytes, chunk_name)?;
    if !bytecode_cache::is_disabled() {
        let bytecode = crate::loader::bytecode::dump_function(lua, &func)?;
        match bytecode_cache::put(hash, &bytecode) {
            PutResult::Stored => timing.cache_store_successes += 1,
            PutResult::Unchanged => {}
            PutResult::Failed => timing.cache_store_failures += 1,
        }
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
    fn wow_chunk_name_normalizes_windows_addon_paths() {
        let path = Path::new(
            r"C:\repo\vendor\wow-ui-source\Interface\AddOns\Blizzard_UIParent\Mainline\UIParent.lua",
        );
        assert_eq!(
            wow_chunk_name(path),
            "@Interface/AddOns/Blizzard_UIParent/Mainline/UIParent.lua"
        );
    }

    #[test]
    fn wow_chunk_name_normalizes_windows_blizzard_ui_paths_for_patches() {
        let path =
            Path::new(r"C:\repo\Interface\BlizzardUI\Blizzard_UIParent\Mainline\UIParent.lua");
        assert!(wow_chunk_name(path).ends_with("/UIParent.lua"));
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

    #[test]
    fn load_cached_or_compile_counts_bytecode_cache_lookup_misses() {
        let chunk_name = unique_cache_chunk_name("lua_file_cache_lookup_miss");
        let source = format!("return {:?}", chunk_name);

        let mut lua = rilua::Lua::new().unwrap();
        let mut timing = LoadTiming::default();
        {
            let state = lua.state_mut();
            load_cached_or_compile(state, source.as_bytes(), &chunk_name, &mut timing)
        }
        .expect("source compile should succeed");

        assert_eq!(timing.cache_hits, 0);
        assert_eq!(timing.cache_misses, 1);
        assert_eq!(timing.cache_lookup_misses, 1);
        assert_eq!(timing.cache_replay_failures, 0);
        assert_eq!(timing.cache_store_successes, 1);
        assert_eq!(timing.cache_store_failures, 0);
    }

    #[test]
    fn load_cached_or_compile_counts_bytecode_replay_failures() {
        let chunk_name = unique_cache_chunk_name("lua_file_cache_replay_failure");
        let source = format!("return {:?}", chunk_name);
        let hash = bytecode_cache::content_hash(source.as_bytes(), &chunk_name);
        bytecode_cache::put(hash, b"not valid lua bytecode");

        let mut lua = rilua::Lua::new().unwrap();
        let mut timing = LoadTiming::default();
        {
            let state = lua.state_mut();
            load_cached_or_compile(state, source.as_bytes(), &chunk_name, &mut timing)
        }
        .expect("source compile should recover after cache replay failure");

        assert_eq!(timing.cache_hits, 0);
        assert_eq!(timing.cache_misses, 1);
        assert_eq!(timing.cache_lookup_misses, 0);
        assert_eq!(timing.cache_replay_failures, 1);
        assert_eq!(timing.cache_store_successes, 1);
        assert_eq!(timing.cache_store_failures, 0);
    }
}
