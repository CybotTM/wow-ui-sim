//! 12340 evidence applies to Wrath; Retail retains its existing color tuple.
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn plain_message_frame_has_no_scrolling_history_surface() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        -- Exercise both cache creation orders: these share a Rust widget type.
        for _, types in ipairs({{'MessageFrame', 'ScrollingMessageFrame'},
                                {'ScrollingMessageFrame', 'MessageFrame'}}) do
            for _, kind in ipairs(types) do
                local f = CreateFrame(kind)
                assert(type(f.AddMessage) == 'function')
                assert(type(f.HasMessageByID) == 'function')
                assert(type(f.ResetMessageFadeByID) == 'function')
                assert(type(f.SetTimeVisible) == 'function', 'shared fade methods remain')
                if kind == 'MessageFrame' then
                    for _, name in ipairs({'GetNumMessages', 'GetMessageInfo',
                        'RemoveMessagesByAccessID', 'AddMsg', 'BackFillMessage',
                        'GetMaxLines', 'SetMaxLines', 'ScrollUp'}) do
                        assert(f[name] == nil, kind .. ':' .. name .. ' must be absent')
                    end
                else
                    assert(type(f.GetNumMessages) == 'function')
                    assert(type(f.GetMessageInfo) == 'function')
                    assert(type(f.RemoveMessagesByAccessID) == 'function')
                end
            end
        end
    "#).unwrap();
}

#[test]
#[cfg(feature = "client-wrath")]
fn wrath_smf_access_id_zero_unfiltered_tuple_and_errors() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local f = CreateFrame('ScrollingMessageFrame')
        f:AddMessage('untagged')
        f:AddMessage('chosen', 0.2, 0.3, 0.4, 902, false, 11, 101)
        f:AddMessage('other', 0.5, 0.6, 0.7, 903, false, 22, 202)
        f:AddMessage('zero', 1, 1, 1, 904, false, 0, 203)
        assert(f:GetNumMessages(0) == 4, 'zero selects total history')
        assert(f:GetNumMessages(11) == 1)
        assert(f:GetNumMessages(999) == 0)
        assert(select('#', f:GetMessageInfo(2)) == 4, 'native tuple arity')
        local text, access, typeID, extra = f:GetMessageInfo(2)
        assert(text == 'chosen' and access == 11 and typeID == 902 and extra == 101,
            'unfiltered metadata, not colors')
        text, access, typeID, extra = f:GetMessageInfo(1, 0)
        assert(text == 'untagged' and access == 0 and typeID == 0 and extra == 0)
        assert(not pcall(f.GetMessageInfo, f, 0), 'index zero must error')
        assert(not pcall(f.GetMessageInfo, f, 5), 'missing row must error')
        assert(not pcall(f.GetMessageInfo, f, 2, 11), 'filtered bounds must error')
        assert(not pcall(f.RemoveMessagesByAccessID, f), 'numeric ID required')
        assert(select('#', f:RemoveMessagesByAccessID(0)) == 0)
        assert(f:GetNumMessages() == 2, 'remove zero matches untagged and explicit zero only')
        assert(f:GetMessageInfo(1) == 'chosen' and f:GetMessageInfo(2) == 'other')
        f:RemoveMessagesByAccessID(11)
        assert(f:GetNumMessages() == 1 and f:GetMessageInfo(1) == 'other')
    "#).unwrap();
}

#[test]
#[cfg(feature = "client-retail")]
fn retail_smf_history_retains_color_tuple_and_access_filtering() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local f = CreateFrame('ScrollingMessageFrame')
        f:AddMessage('retail', 0.25, 0.5, 0.75, 0.8, 42)
        assert(select('#', f:GetMessageInfo(1)) == 6)
        local text, r, g, b, alpha, timestamp = f:GetMessageInfo(1)
        assert(text == 'retail' and r == 0.25 and g == 0.5 and b == 0.75)
        assert(math.abs(alpha - 0.8) < 0.00001 and type(timestamp) == 'number')
        assert(f:HasMessageByID(42))
        assert(f:GetNumMessages() == 1)
    "#).unwrap();
}
