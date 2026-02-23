
WowBehaviorTestDB = {
["summary"] = {
["total"] = 161,
["passed"] = 160,
["failed"] = 1,
},
["timestamp"] = "2026-02-23",
["version"] = 1,
["results"] = {
["scroll.SetVerticalScroll_basic"] = {
["pass"] = true,
["actual"] = 50.00000381469727,
},
["tooltip.GetLine"] = {
["note"] = "Tooltip text accessible via named FontString regions",
["pass"] = true,
["actual"] = {
["text"] = "NO_TEXT_REGION",
},
},
["font.SetAlphaGradient"] = {
["note"] = "SetAlphaGradient — still available?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["err"] = "nil",
},
},
["frame.global_name_registration"] = {
["note"] = "Named frames register in _G",
["pass"] = true,
["actual"] = {
["registered"] = true,
},
},
["animation.loop_types"] = {
["pass"] = true,
["actual"] = {
["bounce"] = "BOUNCE",
["rep"] = "REPEAT",
["none"] = "NONE",
},
},
["scroll.create"] = {
["pass"] = true,
["actual"] = {
["type"] = "ScrollFrame",
},
},
["frame.GetPoint_out_of_range"] = {
["note"] = "GetPoint(2) when only 1 anchor exists",
["pass"] = true,
["actual"] = {
["n"] = 0,
},
},
["anchor.self_anchor_error"] = {
["note"] = "Expected error",
["pass"] = true,
["actual"] = "Interface/AddOns/WowBehaviorTest/tests_anchor.lua:62: Action[SetPoint] failed because[Cannot anchor to itself]: attempted from: Frame:SetPoint.",
},
["editbox.Insert_max_letters"] = {
["note"] = "Insert beyond MaxLetters — truncated or rejected?",
["pass"] = true,
["actual"] = {
["numLetters"] = 5,
["text"] = "hello",
},
},
["animation.create_alpha"] = {
["note"] = "CreateAnimation('Alpha') type",
["pass"] = true,
["actual"] = {
["type"] = "Alpha",
},
},
["editbox.default_text"] = {
["note"] = "Default text is empty string?",
["pass"] = true,
["actual"] = {
["text"] = "",
},
},
["event.RegisterEvent_hidden_frame"] = {
["note"] = "Can register events on hidden frame?",
["pass"] = true,
["actual"] = true,
},
["animation.create_group"] = {
["note"] = "CreateAnimationGroup returns AnimationGroup",
["pass"] = true,
["actual"] = {
["parent"] = true,
["type"] = "AnimationGroup",
},
},
["tooltip.multiple_SetOwner"] = {
["note"] = "Second SetOwner clears lines?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["count"] = 0,
},
},
["animation.SetToFinalAlpha"] = {
["pass"] = false,
["error"] = "Interface/AddOns/WowBehaviorTest/tests_animation.lua:147: attempt to call method 'GetToFinalAlpha' (a nil value)",
},
["editbox.GetNumLetters_multibyte"] = {
["note"] = "GetNumLetters vs byte length with accented chars",
["pass"] = true,
["actual"] = {
["numLetters"] = 5,
["byteLen"] = 6,
},
},
["event.invalid_event_error"] = {
["note"] = "Expected error",
["pass"] = true,
["actual"] = "Frame:RegisterEvent(): Frame:RegisterEvent(): Attempt to register unknown event \"NOT_A_REAL_EVENT_XYZ\"",
},
["script.SetScript_invalid_type"] = {
["note"] = "Expected error",
["pass"] = true,
["actual"] = "Frame:GetScript(): Doesn't have a \"OnNotARealScript\" script",
},
["mixin.CreateFromMixins_multiple"] = {
["pass"] = true,
["actual"] = {
["a"] = 1,
["b"] = 2,
},
},
["frame.GetObjectType_button"] = {
["pass"] = true,
["actual"] = "Button",
},
["mixin.with_frame"] = {
["note"] = "Can Mixin add methods to a frame?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["result"] = "custom",
},
},
["mixin.nil_mixin"] = {
["note"] = "Mixin(t, nil) — error or no-op?",
["pass"] = true,
["actual"] = {
["ok"] = false,
["err"] = "Usage: local outObject = Mixin(object, ...)\nLua Taint: WowBehaviorTest",
},
},
["editbox.HighlightText"] = {
["note"] = "HighlightText() selects all text?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["err"] = "nil",
},
},
["frame.GetWidth_default"] = {
["note"] = "Default width",
["pass"] = true,
["actual"] = 0,
},
["animation.finish_vs_stop"] = {
["note"] = "Finish() — applies final state? What alpha?",
["pass"] = true,
["actual"] = {
["alpha"] = 1,
["playing"] = true,
},
},
["frame.visibility_default"] = {
["note"] = "Default visibility state",
["pass"] = true,
["actual"] = {
["shown"] = true,
["visible"] = true,
},
},
["editbox.focus"] = {
["pass"] = true,
["actual"] = {
["had_focus"] = true,
["set_ok"] = true,
},
},
["propagation.scale_zero"] = {
["note"] = "SetScale(0) — error or allowed?",
["pass"] = true,
["actual"] = {
["ok"] = false,
["err"] = "Frame:SetScale(): Scale must be > 0\nLua Taint: WowBehaviorTest",
},
},
["event.UnregisterEvent"] = {
["note"] = "After unregister",
["pass"] = true,
["actual"] = false,
},
["font.max_lines"] = {
["note"] = "SetMaxLines/GetMaxLines",
["pass"] = true,
["actual"] = {
["maxlines"] = 3,
["get_ok"] = true,
["set_ok"] = true,
},
},
["propagation.scale_negative"] = {
["note"] = "SetScale(-1) — error or allowed?",
["pass"] = true,
["actual"] = {
["ok"] = false,
["err"] = "Frame:SetScale(): Scale must be > 0\nLua Taint: WowBehaviorTest",
},
},
["editbox.number_mode"] = {
["pass"] = true,
["actual"] = {
["number"] = 42,
["type"] = "number",
},
},
["frame.GetPoint_no_anchors"] = {
["note"] = "GetPoint on unanchored frame",
["pass"] = true,
["actual"] = {
["n"] = 0,
},
},
["script.Button_HasScript_OnClick"] = {
["note"] = "Button supports OnClick, Frame doesn't",
["pass"] = true,
["actual"] = {
["onclick"] = true,
["onshow"] = true,
},
},
["frame.id_roundtrip"] = {
["pass"] = true,
["actual"] = 42,
},
["propagation.Raise_effect"] = {
["note"] = "Does Raise() change frame level?",
["pass"] = true,
["actual"] = {
["before"] = 1,
["after"] = 1,
},
},
["frame.GetHeight_default"] = {
["note"] = "Default height",
["pass"] = true,
["actual"] = 0,
},
["script.SetScript_basic"] = {
["pass"] = true,
["actual"] = {
["has_handler"] = true,
["type"] = "function",
},
},
["frame.level_default"] = {
["note"] = "Default frame level",
["pass"] = true,
["actual"] = 0,
},
["animation.play_state"] = {
["note"] = "State after Play()",
["pass"] = true,
["actual"] = {
["stopped"] = false,
["paused"] = false,
["playing"] = true,
},
},
["anchor.SetPoint_nil_relativeTo"] = {
["note"] = "nil relativeTo defaults to parent?",
["pass"] = true,
["actual"] = {
["relativeTo_name"] = "nil",
["is_parent"] = false,
},
},
["frame.strata_default"] = {
["note"] = "Default frame strata",
["pass"] = true,
["actual"] = "MEDIUM",
},
["tooltip.ClearLines"] = {
["note"] = "NumLines after ClearLines",
["pass"] = true,
["actual"] = {
["ok"] = true,
["count"] = 0,
},
},
["font.GetStringWidth_text"] = {
["note"] = "GetStringWidth returns positive number",
["pass"] = true,
["actual"] = {
["type"] = "number",
["width"] = 73.06666564941406,
},
},
["font.IsTruncated_not_truncated"] = {
["note"] = "IsTruncated on short text",
["pass"] = true,
["actual"] = {
["ok"] = true,
["truncated"] = false,
},
},
["frame.CreateFontString_returns_fontstring"] = {
["pass"] = true,
["actual"] = {
["parent"] = true,
["type"] = "FontString",
},
},
["propagation.child_strata_default"] = {
["note"] = "Does child inherit parent strata?",
["pass"] = true,
["actual"] = {
["child_strata"] = "HIGH",
["parent_strata"] = "HIGH",
},
},
["propagation.alpha_after_reparent"] = {
["note"] = "Effective alpha updates after SetParent",
["pass"] = true,
["actual"] = {
["under_p1"] = 0.501960813999176,
["under_p2"] = 1,
},
},
["script.GetScript_after_hook"] = {
["note"] = "GetScript returns original or wrapper?",
["pass"] = true,
["actual"] = {
["same_as_original"] = false,
["type"] = "function",
},
},
["propagation.child_strata_override"] = {
["note"] = "Can child have lower strata than parent?",
["pass"] = true,
["actual"] = {
["child_strata"] = "LOW",
["parent_strata"] = "HIGH",
},
},
["scroll.horizontal_overflow"] = {
["note"] = "SetHorizontalScroll(999) on 100px range — clamped?",
["pass"] = true,
["actual"] = {
["value"] = 999.0000610351562,
},
},
["font.GetWrappedWidth"] = {
["note"] = "GetWrappedWidth vs GetStringWidth when text wraps",
["pass"] = true,
["actual"] = {
["ok"] = true,
["wrapped"] = 53.86666870117188,
["string_width"] = 209.066650390625,
},
},
["anchor.ClearAllPoints"] = {
["note"] = "NumPoints after ClearAllPoints",
["pass"] = true,
["actual"] = 0,
},
["animation.default_duration"] = {
["note"] = "Default animation duration",
["pass"] = true,
["actual"] = 0,
},
["editbox.cursor_position"] = {
["pass"] = true,
["actual"] = 3,
},
["event.IsEventRegistered"] = {
["pass"] = true,
["actual"] = {
["not_registered"] = false,
["registered"] = true,
},
},
["frame.GetRect_no_anchors"] = {
["note"] = "GetRect on unanchored frame returns nothing?",
["pass"] = true,
["actual"] = {
["n"] = 0,
},
},
["frame.id_default"] = {
["note"] = "Default ID",
["pass"] = true,
["actual"] = 0,
},
["frame.GetPoint_format"] = {
["note"] = "GetPoint returns 5 values: point, relativeTo, relativePoint, xOfs, yOfs",
["pass"] = true,
["actual"] = {
["relativeTo_type"] = "table",
["point"] = "CENTER",
["relativePoint"] = "CENTER",
["yOfs"] = 20.00000190734863,
["xOfs"] = 10.00000095367432,
},
},
["frame.IsObjectType_direct"] = {
["note"] = "Button IsObjectType for Button and Frame (inheritance)",
["pass"] = true,
["actual"] = {
["button"] = true,
["frame"] = true,
},
},
["scroll.negative_scroll"] = {
["note"] = "Negative scroll — clamped to 0?",
["pass"] = true,
["actual"] = -50.00000381469727,
},
["animation.play_during_play"] = {
["note"] = "Play() during already-playing — restart or no-op?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["still_playing"] = true,
},
},
["script.HasScript"] = {
["note"] = "HasScript checks if script type is valid for frame type",
["pass"] = true,
["actual"] = {
["onshow"] = true,
["onupdate"] = true,
["onclick"] = false,
},
},
["tooltip.NumLines_default"] = {
["note"] = "NumLines before any AddLine",
["pass"] = true,
["actual"] = {
["ok"] = true,
["count"] = 0,
},
},
["font.GetStringWidth_color_codes"] = {
["note"] = "Color codes affect width?",
["pass"] = true,
["actual"] = {
["plain"] = 32.53333282470703,
["same"] = true,
["colored"] = 32.53333282470703,
},
},
["event.multiple_frames_same_event"] = {
["note"] = "Multiple frames can register for same event",
["pass"] = true,
["actual"] = {
["f2"] = true,
["f1"] = true,
},
},
["animation.elapsed_initial"] = {
["note"] = "Initial elapsed time",
["pass"] = true,
["actual"] = {
["ok"] = true,
["elapsed"] = 0,
},
},
["editbox.Insert_beginning"] = {
["pass"] = true,
["actual"] = {
["cursor"] = 6,
["text"] = "hello world",
},
},
["editbox.GetNumLetters"] = {
["note"] = "GetNumLetters for 'hello'",
["pass"] = true,
["actual"] = 5,
},
["editbox.cursor_negative"] = {
["note"] = "SetCursorPosition(-1) — error or clamps?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["err"] = "nil",
["pos"] = 0,
},
},
["propagation.Lower_effect"] = {
["note"] = "Does Lower() change frame level?",
["pass"] = true,
["actual"] = {
["after"] = 5,
},
},
["animation.pause_resume"] = {
["pass"] = true,
["actual"] = {
["paused"] = true,
["resumed_playing"] = true,
},
},
["editbox.max_letters_default"] = {
["note"] = "Default max letters (0 = unlimited?)",
["pass"] = true,
["actual"] = 0,
},
["event.RegisterEvent_return"] = {
["note"] = "RegisterEvent return value",
["pass"] = true,
["actual"] = {
["value"] = true,
["type"] = "boolean",
["n"] = 1,
},
},
["scroll.GetScrollChild_none"] = {
["note"] = "GetScrollChild with no child — nil?",
["pass"] = true,
["actual"] = {
["type"] = "nil",
},
},
["scroll.default_offset"] = {
["note"] = "Default scroll offsets",
["pass"] = true,
["actual"] = {
["v"] = 0,
["h"] = 0,
},
},
["font.SetFont_size"] = {
["pass"] = true,
["actual"] = {
["size"] = 20.00000190734863,
},
},
["font.GetFont"] = {
["note"] = "GetFont returns path, size, flags",
["pass"] = true,
["actual"] = {
["flags"] = "",
["font"] = "Fonts\\FRIZQT__.TTF",
["size"] = 12,
},
},
["font.text_color"] = {
["pass"] = true,
["actual"] = {
["a"] = 0.8000000715255737,
["b"] = 0.501960813999176,
["g"] = 0,
["r"] = 1,
},
},
["event.handler_receives_self"] = {
["note"] = "OnEvent handler is set",
["pass"] = true,
["actual"] = {
["has_handler"] = true,
},
},
["font.justify_v"] = {
["pass"] = true,
["actual"] = "MIDDLE",
},
["propagation.effective_alpha_chain"] = {
["note"] = "3-level alpha chain: 0.5 * 0.8 * 0.5 = 0.2?",
["pass"] = true,
["actual"] = {
["child_alpha"] = 0.501960813999176,
["child_effective"] = 0.2000000178813934,
},
},
["font.word_wrap"] = {
["note"] = "SetWordWrap / CanWordWrap",
["pass"] = true,
["actual"] = {
["ok"] = true,
["can_wrap"] = true,
},
},
["font.SetText_number"] = {
["note"] = "SetText(42) — coerced to string?",
["pass"] = true,
["actual"] = {
["type"] = "string",
["text"] = "42",
},
},
["font.SetText_nil"] = {
["note"] = "SetText(nil) — empty string or error?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["err"] = "nil",
},
},
["font.SetText_GetText"] = {
["pass"] = true,
["actual"] = "test string",
},
["frame.GetNumPoints_default"] = {
["note"] = "Default anchor count",
["pass"] = true,
["actual"] = 0,
},
["tooltip.SetOwner"] = {
["note"] = "Tooltip visible after SetOwner?",
["pass"] = true,
["actual"] = {
["visible"] = false,
},
},
["font.GetStringWidth_empty"] = {
["note"] = "GetStringWidth('') — 0 or minimum?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["width"] = 0,
},
},
["editbox.number_non_numeric"] = {
["note"] = "GetNumber on non-numeric text — 0?",
["pass"] = true,
["actual"] = {
["number"] = 0,
},
},
["anchor.SetAllPoints"] = {
["note"] = "SetAllPoints creates TOPLEFT + BOTTOMRIGHT anchors?",
["pass"] = true,
["actual"] = {
["numPoints"] = 2,
["p2"] = "BOTTOMRIGHT",
["p1"] = "TOPLEFT",
},
},
["mixin.method_self"] = {
["pass"] = true,
["actual"] = {
["value"] = 42,
},
},
["frame.GetName_nil_unnamed"] = {
["note"] = "Unnamed frame GetName",
["pass"] = true,
["actual"] = {
["type"] = "nil",
},
},
["animation.duration_roundtrip"] = {
["pass"] = true,
["actual"] = 0.5,
},
["mixin.return_value"] = {
["note"] = "Mixin returns the target table?",
["pass"] = true,
["actual"] = {
["same_table"] = true,
},
},
["mixin.CreateFromMixins_basic"] = {
["note"] = "Creates new table with mixin fields",
["pass"] = true,
["actual"] = {
["x"] = 1,
["is_new"] = true,
["foo"] = "foo",
},
},
["frame.mouse_enabled_default"] = {
["note"] = "Default mouse enabled state",
["pass"] = true,
["actual"] = {
["ok"] = true,
["value"] = false,
},
},
["tooltip.hide"] = {
["pass"] = true,
["actual"] = {
["visible"] = false,
},
},
["animation.GetAnimationGroups"] = {
["note"] = "GetAnimationGroups returns all groups",
["pass"] = true,
["actual"] = {
["count"] = 2,
},
},
["mixin.preserves_existing"] = {
["pass"] = true,
["actual"] = {
["new_field"] = true,
["existing"] = true,
},
},
["script.HookScript_no_existing"] = {
["note"] = "HookScript with no existing handler — error or creates handler?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["called"] = true,
},
},
["mixin.basic"] = {
["pass"] = true,
["actual"] = {
["has_Foo"] = true,
["foo_result"] = "foo",
["bar"] = 42,
},
},
["frame.alpha_effective"] = {
["note"] = "Effective alpha = parent * child",
["pass"] = true,
["actual"] = {
["effective"] = 0.250980406999588,
["own"] = 0.501960813999176,
},
},
["propagation.ignore_parent_scale"] = {
["note"] = "SetIgnoreParentScale — effective scale ignores parent?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["after"] = 1,
["before"] = 2,
},
},
["animation.duration_zero"] = {
["note"] = "SetDuration(0) — allowed or error?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["duration"] = 0,
["err"] = "nil",
},
},
["propagation.ignore_parent_alpha"] = {
["note"] = "SetIgnoreParentAlpha — effective alpha ignores parent?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["after"] = 1,
["before"] = 0.501960813999176,
},
},
["event.RegisterEvent_idempotent"] = {
["note"] = "After registering twice and unregistering once",
["pass"] = true,
["actual"] = false,
},
["editbox.max_letters_enforcement"] = {
["note"] = "SetText longer than MaxLetters — truncated?",
["pass"] = true,
["actual"] = {
["numLetters"] = 5,
["text"] = "hello",
},
},
["propagation.frame_level_child"] = {
["note"] = "Child frame level relative to parent",
["pass"] = true,
["actual"] = {
["child_level"] = 6,
["parent_level"] = 5,
},
},
["frame.GetNumPoints_after_set"] = {
["note"] = "After setting 2 anchors",
["pass"] = true,
["actual"] = 2,
},
["editbox.cursor_past_end"] = {
["note"] = "SetCursorPosition(100) on 5-char string — clamps?",
["pass"] = true,
["actual"] = 5,
},
["editbox.Insert_middle"] = {
["pass"] = true,
["actual"] = {
["cursor"] = 4,
["text"] = "hello",
},
},
["animation.order"] = {
["pass"] = true,
["actual"] = {
["order2"] = 2,
["order1"] = 1,
},
},
["propagation.effective_scale_chain"] = {
["note"] = "Scale chain: 2.0 * 3.0 = 6.0?",
["pass"] = true,
["actual"] = {
["child_scale"] = 3,
["child_effective"] = 6,
},
},
["propagation.alpha_clamp"] = {
["note"] = "Does SetAlpha clamp to 0..1?",
["pass"] = true,
["actual"] = {
["over"] = 1,
["under"] = 0,
},
},
["animation.types_available"] = {
["note"] = "Which animation types are creatable",
["pass"] = true,
["actual"] = {
["Scale"] = true,
["Rotation"] = true,
["Alpha"] = true,
["Translation"] = true,
},
},
["scroll.default_range"] = {
["note"] = "Default scroll range with no child",
["pass"] = true,
["actual"] = {
["h_range"] = 0,
["ok_v"] = true,
["v_range"] = 0,
["ok_h"] = true,
},
},
["font.justify_h"] = {
["pass"] = true,
["actual"] = "CENTER",
},
["tooltip.SetOwner_with_offsets"] = {
["note"] = "SetOwner with x,y offset args",
["pass"] = true,
["actual"] = {
["ok"] = true,
["err"] = "nil",
},
},
["script.SetScript_after_hook"] = {
["note"] = "SetScript after HookScript — does hook survive?",
["pass"] = true,
["actual"] = {
"replaced",
},
},
["frame.GetName_named"] = {
["pass"] = true,
["actual"] = "WBT_TestFrame1",
},
["mixin.CreateFromMixins_no_modify_original"] = {
["note"] = "Modifying result doesn't affect original",
["pass"] = true,
["actual"] = {
["copy"] = 99,
["original"] = 1,
},
},
["mixin.multiple_last_wins"] = {
["note"] = "m2.y overwrites m1.y (last wins)?",
["pass"] = true,
["actual"] = {
["y"] = 3,
["x"] = 1,
["z"] = 4,
},
},
["scroll.SetScrollChild_replace"] = {
["note"] = "Replace scroll child — old child reparented?",
["pass"] = true,
["actual"] = {
["old_parent"] = false,
["is_child2"] = true,
},
},
["animation.stop_state"] = {
["note"] = "State after Stop()",
["pass"] = true,
["actual"] = {
["paused"] = false,
["playing"] = false,
},
},
["script.HookScript_order"] = {
["note"] = "Hook runs after original?",
["pass"] = true,
["actual"] = {
},
},
["event.RegisterAllEvents"] = {
["note"] = "RegisterAllEvents makes all events registered",
["pass"] = true,
["actual"] = {
["player_login"] = false,
},
},
["editbox.SetAutoFocus"] = {
["pass"] = true,
["actual"] = false,
},
["script.SetScript_nil_no_handler"] = {
["note"] = "SetScript(nil) on empty — error or no-op?",
["pass"] = true,
["actual"] = {
["ok"] = true,
},
},
["editbox.auto_focus_default"] = {
["note"] = "Default auto focus state",
["pass"] = true,
["actual"] = {
["ok"] = true,
["value"] = true,
},
},
["script.SetScript_nil_removes"] = {
["note"] = "SetScript(nil) removes handler",
["pass"] = true,
["actual"] = {
},
},
["scroll.UpdateScrollChildRect"] = {
["pass"] = true,
["actual"] = {
["ok"] = true,
["err"] = "nil",
},
},
["tooltip.AddLine_color_clamp"] = {
["note"] = "Color values > 1.0 — clamped or passed through?",
["pass"] = true,
["actual"] = {
["ok"] = true,
},
},
["anchor.canonical_order"] = {
["note"] = "Points returned in canonical order? TOPLEFT first regardless of insertion order?",
["pass"] = true,
["actual"] = {
["second"] = "BOTTOMRIGHT",
["first"] = "TOPLEFT",
},
},
["mixin.frame_method_precedence"] = {
["note"] = "Does Mixin overwrite frame's GetName? Or does metatable win?",
["pass"] = true,
["actual"] = {
["name"] = "mixin_name",
},
},
["editbox.Insert_end"] = {
["pass"] = true,
["actual"] = {
["cursor"] = 11,
["text"] = "hello world",
},
},
["anchor.all_nine_points_order"] = {
["note"] = "Order of all 9 anchor points",
["pass"] = true,
["actual"] = {
"TOPLEFT",
"TOP",
"TOPRIGHT",
"LEFT",
"CENTER",
"RIGHT",
"BOTTOMLEFT",
"BOTTOM",
"BOTTOMRIGHT",
},
},
["tooltip.AddDoubleLine"] = {
["note"] = "AddDoubleLine counts as 1 line?",
["pass"] = true,
["actual"] = {
["ok"] = true,
["count"] = 0,
},
},
["tooltip.AddLine"] = {
["note"] = "NumLines after 2 AddLine calls",
["pass"] = true,
["actual"] = {
["ok"] = true,
["count"] = 0,
},
},
["frame.SetParent_reparent"] = {
["pass"] = true,
["actual"] = {
["parent_is_p2"] = true,
},
},
["anchor.SetPoint_defaults"] = {
["note"] = "SetPoint('CENTER') default relativeTo and offsets",
["pass"] = true,
["actual"] = {
["y"] = 0,
["x"] = 0,
["point"] = "CENTER",
["relativePoint"] = "CENTER",
["relativeTo_name"] = "nil",
},
},
["script.HookScript_chain"] = {
["note"] = "Multiple hooks chain in order",
["pass"] = true,
["actual"] = {
"original",
"hook1",
"hook2",
},
},
["mixin.overwrites_existing"] = {
["note"] = "Mixin overwrites existing fields?",
["pass"] = true,
["actual"] = {
["x"] = 2,
},
},
["anchor.SetPoint_replace"] = {
["note"] = "Setting same point replaces, doesn't add",
["pass"] = true,
["actual"] = {
["y"] = 20.00000190734863,
["x"] = 10.00000095367432,
["numPoints"] = 1,
},
},
["frame.visibility_hidden_parent"] = {
["note"] = "Child shown but not visible when parent hidden",
["pass"] = true,
["actual"] = {
["shown"] = true,
["visible"] = false,
},
},
["tooltip.SetOwner_default_offset"] = {
["note"] = "Default offsets after SetOwner ANCHOR_RIGHT",
["pass"] = true,
["actual"] = {
["ok"] = true,
["x"] = 0,
["y"] = 0,
},
},
["tooltip.create"] = {
["pass"] = true,
["actual"] = {
["name"] = "WBT_TestTooltip1",
["type"] = "GameTooltip",
},
},
["event.UnregisterAllEvents"] = {
["note"] = "All events unregistered after UnregisterAllEvents",
["pass"] = true,
["actual"] = {
["login"] = false,
["leaving"] = false,
},
},
["tooltip.anchor_types"] = {
["note"] = "Which anchor types are valid",
["pass"] = true,
["actual"] = {
["ANCHOR_TOPRIGHT"] = true,
["ANCHOR_TOP"] = true,
["ANCHOR_BOTTOMRIGHT"] = true,
["ANCHOR_BOTTOM"] = true,
["ANCHOR_NONE"] = true,
["ANCHOR_CURSOR"] = true,
["ANCHOR_LEFT"] = true,
["ANCHOR_TOPLEFT"] = true,
["ANCHOR_BOTTOMLEFT"] = true,
["ANCHOR_RIGHT"] = true,
},
},
["frame.scale_default"] = {
["note"] = "Default scale",
["pass"] = true,
["actual"] = 1,
},
["scroll.SetScrollChild"] = {
["pass"] = true,
["actual"] = {
["same_child"] = true,
["has_child"] = true,
},
},
["anchor.SetPoint_explicit_relative"] = {
["pass"] = true,
["actual"] = {
["y"] = -5.000000476837158,
["x"] = 5.000000476837158,
["point"] = "TOPLEFT",
["relativePoint"] = "BOTTOMLEFT",
["relativeTo_is_parent"] = true,
},
},
["anchor.GetPoint_1_based"] = {
["note"] = "GetPoint(0) vs GetPoint(1) — 1-based indexing?",
["pass"] = true,
["actual"] = {
["index_1_returns"] = 5,
["index_0_returns"] = 0,
},
},
["frame.GetObjectType_frame"] = {
["pass"] = true,
["actual"] = "Frame",
},
["editbox.text_roundtrip"] = {
["pass"] = true,
["actual"] = "hello world",
},
["frame.scale_effective"] = {
["note"] = "Effective scale = parent * child",
["pass"] = true,
["actual"] = {
["effective"] = 1,
["own"] = 0.5,
},
},
["editbox.create"] = {
["pass"] = true,
["actual"] = {
["type"] = "EditBox",
},
},
["frame.alpha_default"] = {
["note"] = "Default alpha",
["pass"] = true,
["actual"] = 1,
},
["scroll.range_with_child"] = {
["note"] = "Range should be child_size - frame_size",
["pass"] = true,
["actual"] = {
["h_range"] = 0,
["ok_v"] = true,
["v_range"] = 0,
["ok_h"] = true,
},
},
["frame.SetSize_roundtrip"] = {
["pass"] = true,
["actual"] = {
["w"] = 100.0000076293945,
["h"] = 200.0000152587891,
},
},
["frame.CreateTexture_returns_texture"] = {
["pass"] = true,
["actual"] = {
["parent"] = true,
["type"] = "Texture",
},
},
},
}
