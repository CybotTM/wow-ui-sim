//! Consumer-derived simulator contract, not native-parity evidence.
use wow_ui_sim::lua_api::WowLuaEnv;

#[cfg(feature = "client-wrath")]
fn create_tagged_env(kind: &str) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec(&format!(
        r#"
        C11Frame = CreateFrame('{kind}')
        assert(C11Frame:GetNumMessages() == 0, 'new history is empty')
        C11Frame:AddMessage('other-first', 1, 0, 0, 901, false, 22, 202)
        C11Frame:AddMessage('chosen-first', 0, 1, 0, 902, false, 11, 101)
        C11Frame:AddMessage('untagged')
        C11Frame:AddMessage('other-second', 1, 0, 0, 903, false, 22, 202)
        C11Frame:AddMessage('chosen-second', 0, 0, 1, 904, false, 11, 103)
    "#
    ))
    .unwrap();
    env
}

#[cfg(feature = "client-wrath")]
fn assert_access_counts(kind: &str) {
    let env = create_tagged_env(kind);
    env.exec(
        r#"
        assert(C11Frame:GetNumMessages() == 5, 'unfiltered count before')
        assert(C11Frame:GetNumMessages(11) == 2, 'access 11 count')
        assert(C11Frame:GetNumMessages(22) == 2, 'access 22 count')
        assert(C11Frame:GetNumMessages(999) == 0, 'absent access count')
        assert(C11Frame:GetNumMessages() == 5, 'filtered reads must preserve history')
        assert(C11Frame:GetMessageInfo(1) == 'other-first', 'unfiltered first')
        assert(C11Frame:GetMessageInfo(5) == 'chosen-second', 'unfiltered last')
        C11Frame:Clear()
        assert(C11Frame:GetNumMessages() == 0, 'clear after filtered reads')
    "#,
    )
    .unwrap();
}

#[test]
#[cfg(feature = "client-wrath")]
fn c11_message_frame_counts_only_selected_access_id() {
    assert_access_counts("MessageFrame");
}

#[test]
#[cfg(feature = "client-wrath")]
fn c11_scrolling_message_frame_counts_only_selected_access_id() {
    assert_access_counts("ScrollingMessageFrame");
}

#[test]
#[cfg(feature = "client-wrath")]
fn c11_filtered_info_indexes_selected_history_and_returns_transfer_metadata() {
    for kind in ["MessageFrame", "ScrollingMessageFrame"] {
        let env = create_tagged_env(kind);
        env.exec(
            r#"
            local text, access, line, extra = C11Frame:GetMessageInfo(1, 11)
            assert(text == 'chosen-first', 'filtered index 1')
            assert(access == 11 and line == 902 and extra == 101, 'first metadata')
            text, access, line, extra = C11Frame:GetMessageInfo(2, 11)
            assert(text == 'chosen-second', 'filtered index 2')
            assert(access == 11 and line == 904 and extra == 103, 'second metadata')
            assert(C11Frame:GetMessageInfo(1, 22) == 'other-first')
            assert(C11Frame:GetMessageInfo(2, 22) == 'other-second')
            assert(C11Frame:GetMessageInfo(1) == 'other-first', 'unfiltered order')
            assert(C11Frame:GetNumMessages() == 5)
        "#,
        )
        .unwrap();
    }
}

#[test]
#[cfg(feature = "client-wrath")]
fn c11_remove_selected_access_id_preserves_other_and_untagged_messages() {
    for kind in ["MessageFrame", "ScrollingMessageFrame"] {
        let env = create_tagged_env(kind);
        env.exec(
            r#"
            C11Frame:RemoveMessagesByAccessID(11)
            assert(C11Frame:GetNumMessages() == 3)
            assert(C11Frame:GetNumMessages(11) == 0)
            assert(C11Frame:GetNumMessages(22) == 2)
            assert(C11Frame:GetMessageInfo(1) == 'other-first')
            assert(C11Frame:GetMessageInfo(2) == 'untagged')
            assert(C11Frame:GetMessageInfo(3) == 'other-second')
        "#,
        )
        .unwrap();
    }
}

#[cfg(feature = "client-wrath")]
fn read_wrath_source(name: &str, expected_digest: &str) -> String {
    use sha2::{Digest, Sha256};
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("SETUP FAILURE: populate pinned Wrath cache");
    let path = toc.parent().unwrap().join(name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("SETUP FAILURE: {}: {error}", path.display()));
    let digest = format!("{:x}", Sha256::digest(&bytes));
    assert_eq!(
        digest,
        expected_digest,
        "SETUP FAILURE: {} changed",
        path.display()
    );
    eprintln!("Wrath source={} sha256={digest}", path.display());
    String::from_utf8(bytes).unwrap()
}

#[test]
#[cfg(feature = "client-wrath")]
fn c11_original_wrath_temporary_window_moves_only_selected_access_history() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("NUM_CHAT_WINDOWS = 10; C11Errors = {}; seterrorhandler(function(e) C11Errors[#C11Errors+1] = tostring(e) end)").unwrap();
    for (name, digest) in [
        (
            "HistoryKeeper.lua",
            "092303b898a4f00f9320503d549ef608847b03a9877441fec54c6c8ba35ecdf4",
        ),
        (
            "FloatingChatFrame.lua",
            "6d1a7ae674088c422098c19a0a7589ea80503caf8c97ec5ed7ea859c0df89a47",
        ),
    ] {
        env.loader_env()
            .exec(&read_wrath_source(name, digest))
            .expect("SETUP FAILURE: complete unchanged public Wrath file");
    }
    // Isolate window chrome/routing, never the transfer function or message APIs.
    // Reuse the temporary-frame branch, avoiding templates and full UI startup.
    env.exec(
        r#"
        FCF_CopyChatSettings = function() end
        FCF_SetTemporaryWindowType = function() end
        FCF_DockFrame = function() end
        ChatEdit_DeactivateChat = function() end
        ChatFrame_ExcludePrivateMessageTarget = function() end
        GENERAL_CHAT_DOCK = { DOCKED_CHAT_FRAMES = {} }
        C11Destination = CreateFrame('ScrollingMessageFrame', 'C11Destination')
        C11Destination.editBox = CreateFrame('EditBox')
        CreateFrame('Frame', 'C11DestinationTab')
        C11Destination.isTemporary = true
        CHAT_FRAMES = {'C11Destination'}
        C11Source = CreateFrame('ScrollingMessageFrame')
        ChatTypeInfo = { WHISPER = {r=0, g=1, b=0}, WHISPER_INFORM = {r=0, g=0, b=1} }
        C11Alice = ChatHistory_GetAccessID('WHISPER', 'Alice')
        C11Bob = ChatHistory_GetAccessID('WHISPER', 'Bob')
        C11Inform = ChatHistory_GetAccessID('WHISPER_INFORM', 'Alice')
        assert(ChatHistory_GetAccessID('whisper', 'ALICE') == C11Alice)
        assert(C11Alice ~= C11Bob and C11Inform ~= C11Alice)
        C11Source:AddMessage('bob-first', 1, 0, 0, 501, false, C11Bob, C11Bob)
        C11Source:AddMessage('alice-in', 0, 1, 0, 502, false, C11Alice, C11Alice)
        C11Source:AddMessage('untagged')
        C11Source:AddMessage('alice-out', 0, 0, 1, 503, false, C11Alice, C11Inform)
        C11Source:AddMessage('bob-last', 1, 0, 0, 504, false, C11Bob, C11Bob)
        C11Destination:AddMessage('stale')
    "#,
    )
    .expect("SETUP FAILURE: bounded window chrome and concrete history");
    env.exec(r#"
        assert(type(FCF_OpenTemporaryWindow) == 'function', 'SETUP FAILURE: missing public transfer')
        assert(#C11Errors == 0, 'SETUP FAILURE: ' .. table.concat(C11Errors, '\n'))
    "#)
        .expect("SETUP FAILURE: public fixture initialization");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "SETUP FAILURE: recorded Lua errors"
    );
    // Only the public transfer is captured. Fixture loading/setup above still panics.
    let observed: (bool, String, i64, i64) = env
        .eval(
            r#"
        local ok, result = pcall(FCF_OpenTemporaryWindow, 'WHISPER', 'Alice', C11Source, false)
        C11TransferredDestination = ok and result or nil
        return ok and result == C11Destination, ok and '' or tostring(result),
            C11Destination:GetNumMessages(), C11Source:GetNumMessages()
    "#,
        )
        .expect("observe public transfer result");
    assert_eq!(
        (observed.0, observed.2, observed.3),
        (true, 2, 3),
        "expected successful selective transfer; captured consumer error: {}",
        observed.1
    );
    env.exec(
        r#"
        local destination = C11TransferredDestination
        assert(destination == C11Destination)
        assert(destination:GetNumMessages() == 2, 'copy only selected access')
        local text, access, line, extra = destination:GetMessageInfo(1, C11Alice)
        assert(text == 'alice-in' and access == C11Alice and line == 502 and extra == C11Alice)
        text, access, line, extra = destination:GetMessageInfo(2, C11Alice)
        assert(text == 'alice-out' and access == C11Alice and line == 503 and extra == C11Inform)
        assert(destination:GetNumMessages(C11Bob) == 0)
        local _, r, g, b = destination:GetMessageInfo(1)
        assert(r == 0 and g == 1 and b == 0, 'incoming type color')
        _, r, g, b = destination:GetMessageInfo(2)
        assert(r == 0 and g == 0 and b == 1, 'outgoing type color')
        assert(C11Source:GetNumMessages() == 3)
        assert(C11Source:GetNumMessages(C11Alice) == 0)
        assert(C11Source:GetNumMessages(C11Bob) == 2)
        assert(C11Source:GetMessageInfo(1) == 'bob-first')
        assert(C11Source:GetMessageInfo(2) == 'untagged')
        assert(C11Source:GetMessageInfo(3) == 'bob-last')
    "#,
    )
    .expect("ASSERTION FAILURE: original Wrath transfer must preserve access, line, extraData");
    env.exec(r#"assert(#C11Errors == 0, table.concat(C11Errors, '\n'))"#)
        .unwrap();
    assert!(env.state().borrow().lua_errors.is_empty());
}
