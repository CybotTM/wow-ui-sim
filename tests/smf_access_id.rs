//! Authenticated 12340 methods/tuples apply only to Wrath. Retail is unchanged.
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
#[cfg(feature = "client-wrath")]
fn wrath_plain_message_frame_has_only_native_message_methods() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        -- Exercise both cache creation orders: these share a Rust widget type.
        for _, types in ipairs({{'MessageFrame', 'ScrollingMessageFrame'},
                                {'ScrollingMessageFrame', 'MessageFrame'}}) do
            for _, kind in ipairs(types) do
                local f = CreateFrame(kind)
                if kind == 'MessageFrame' then
                    for _, name in ipairs({'SetFontObject', 'GetFontObject', 'SetFont', 'GetFont',
                        'SetTextColor', 'GetTextColor', 'SetShadowColor', 'GetShadowColor',
                        'SetShadowOffset', 'GetShadowOffset', 'SetSpacing', 'GetSpacing',
                        'SetJustifyH', 'GetJustifyH', 'SetJustifyV', 'GetJustifyV',
                        'SetInsertMode', 'GetInsertMode', 'SetIndentedWordWrap', 'GetIndentedWordWrap',
                        'SetFading', 'GetFading', 'SetTimeVisible', 'GetTimeVisible',
                        'SetFadeDuration', 'GetFadeDuration', 'AddMessage', 'Clear'}) do
                        assert(type(f[name]) == 'function', 'native method missing: ' .. name)
                    end
                    for _, name in ipairs({'GetNumMessages', 'GetMessageInfo', 'RemoveMessagesByAccessID',
                        'AddMsg', '_AddMessageSilent', 'BackFillMessage', 'ClearText', 'GetMaxLines',
                        'SetMaxLines', 'SetTextCopyable', 'IsTextCopyable', 'ScrollUp', 'ScrollDown',
                        'PageUp', 'PageDown', 'ScrollToTop', 'ScrollToBottom', 'AtTop', 'AtBottom',
                        'GetMaxScrollRange', 'SetScrollOffset', 'GetScrollOffset', 'SetScrollAllowed',
                        'IsScrollAllowed', 'SetOnScrollChangedCallback', 'SetOnLineRightClickedCallback',
                        'AddOnDisplayRefreshedCallback', 'SetOnTextCopiedCallback', 'MarkDisplayDirty',
                        'ResetAllFadeTimes', 'RemoveMessagesByPredicate', 'TransformMessages',
                        'AdjustMessageColors', 'HasMessageByID', 'ResetMessageFadeByID',
                        'SetFadePower', 'GetFadePower', 'GetFontStringByID'}) do
                        assert(f[name] == nil, kind .. ':' .. name .. ' must be absent')
                    end
                else
                    assert(type(f.AddMessage) == 'function')
                    assert(type(f.GetNumMessages) == 'function')
                    assert(type(f.GetMessageInfo) == 'function')
                    assert(type(f.RemoveMessagesByAccessID) == 'function')
                end
            end
        end
    "#).unwrap();
}

#[test]
fn xml_message_frame_aliases_keep_distinct_history_surfaces() {
    let env = WowLuaEnv::new().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let toc = directory.path().join("MessageAliases.toc");
    std::fs::write(&toc, "## Title: MessageAliases\nframes.xml\n").unwrap();
    std::fs::write(directory.path().join("frames.xml"), r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <MessageFrame name="XmlPlainMessages"/>
            <ScrollingMessageFrame name="XmlScrollingMessages"/>
            <Frame name="MessageAliasesTemplate" virtual="true">
                <Frames><MessageFrame name="$parentPlain"/>
                    <ScrollingMessageFrame name="$parentScrolling"/></Frames>
            </Frame>
        </Ui>
    "#).unwrap();
    let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
    env.exec("CreateFrame('Frame', 'RuntimeMessages', nil, 'MessageAliasesTemplate')").unwrap();
    let wrath = cfg!(feature = "client-wrath");
    for name in ["XmlPlainMessages", "RuntimeMessagesPlain"] {
        let has_history: bool = env.eval(&format!("return type({name}.GetNumMessages) == 'function'")).unwrap();
        assert_eq!(has_history, !wrath, "{name}");
    }
    env.exec(r#"
        for _, f in ipairs({XmlScrollingMessages, RuntimeMessagesScrolling}) do
            f:AddMessage('retained')
            assert(f:GetNumMessages() == 1)
            assert(f:GetMessageInfo(1) == 'retained')
        end
    "#).unwrap();
}

#[test]
#[cfg(feature = "client-wrath")]
fn wrath_smf_access_id_zero_selects_total_history() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local f = CreateFrame('ScrollingMessageFrame')
        f:AddMessage('untagged')
        f:AddMessage('chosen', 0.2, 0.3, 0.4, 902, false, 11, 101)
        f:AddMessage('other', 0.5, 0.6, 0.7, 903, false, 22, 202)
        f:AddMessage('zero', 1, 1, 1, 904, false, 0, 203)
        assert(f:GetNumMessages(0) == 4, 'zero selects total history')
        assert(f:GetNumMessages(11) == 1 and f:GetNumMessages(999) == 0)
        assert(f:GetMessageInfo(1, 0) == 'untagged')
        assert(select('#', f:RemoveMessagesByAccessID(0)) == 0)
        assert(f:GetNumMessages() == 2, 'zero removes untagged and explicit zero only')
        assert(f:GetMessageInfo(1) == 'chosen' and f:GetMessageInfo(2) == 'other')
    "#).unwrap();
}

#[test]
#[cfg(feature = "client-wrath")]
fn wrath_smf_unfiltered_tuple_is_metadata_not_colors() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local f = CreateFrame('ScrollingMessageFrame')
        f:AddMessage('chosen', 0.2, 0.3, 0.4, 902, false, 11, 101)
        assert(select('#', f:GetMessageInfo(1)) == 4, 'native tuple arity')
        local text, access, typeID, extra = f:GetMessageInfo(1)
        assert(text == 'chosen' and access == 11 and typeID == 902 and extra == 101)
        f:AddMessage('untagged')
        text, access, typeID, extra = f:GetMessageInfo(2)
        assert(text == 'untagged' and access == 0 and typeID == 0 and extra == 0)
    "#).unwrap();
}

#[test]
#[cfg(feature = "client-wrath")]
fn wrath_smf_invalid_history_index_and_missing_removal_id_error() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local f = CreateFrame('ScrollingMessageFrame')
        f:AddMessage('chosen', 1, 1, 1, 902, false, 11, 101)
        assert(not pcall(f.GetMessageInfo, f, 0), 'index zero must error')
        assert(not pcall(f.GetMessageInfo, f, 2), 'missing row must error')
        assert(not pcall(f.GetMessageInfo, f, 2, 11), 'filtered bounds must error')
        assert(not pcall(f.RemoveMessagesByAccessID, f), 'numeric ID required')
        assert(f:GetNumMessages() == 1, 'failed calls must preserve history')
    "#).unwrap();
}

#[test]
#[cfg(feature = "client-wrath")]
fn wrath_smf_native_argument_positions_do_not_depend_on_backfill_false() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local f = CreateFrame('ScrollingMessageFrame')
        f:AddMessage('implicit', 1, 1, 1, 907)
        f:AddMessage('true', 1, 1, 1, 908, true, 33, 303)
        f:AddMessage('nil', 1, 1, 1, 909, nil, 44, 404)
        assert(f:GetNumMessages(33) == 1, 'true backFill retains access ID')
        assert(f:GetNumMessages(44) == 1, 'nil backFill retains access ID')
        local text, access, typeID, extra = f:GetMessageInfo(1, 33)
        assert(text == 'true' and access == 33 and typeID == 908 and extra == 303)
        text, access, typeID, extra = f:GetMessageInfo(1, 44)
        assert(text == 'nil' and access == 44 and typeID == 909 and extra == 404)
        -- Ordering for backFill is outside this lane; locate the untagged message.
        for i = 1, f:GetNumMessages() do
            text, access, typeID, extra = f:GetMessageInfo(i)
            if text == 'implicit' then
                assert(access == 0 and typeID == 907 and extra == 0)
            end
        end
    "#).unwrap();
}

#[test]
#[cfg(feature = "client-retail")]
fn retail_plain_message_frame_surface_is_unchanged() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local f = CreateFrame('MessageFrame')
        for _, method in ipairs({'GetNumMessages', 'GetMessageInfo', 'RemoveMessagesByAccessID',
            'HasMessageByID', 'ResetMessageFadeByID', 'GetFontStringByID', 'SetFadePower',
            'GetFadePower', 'SetMaxLines'}) do
            assert(type(f[method]) == 'function', 'Retail preservation: ' .. method)
        end
        f:AddMessage('retail', 0.25, 0.5, 0.75)
        assert(f:GetNumMessages() == 1)
        local text, r, g, b = f:GetMessageInfo(1)
        assert(text == 'retail' and r == 0.25 and g == 0.5 and b == 0.75)
    "#).unwrap();
}
