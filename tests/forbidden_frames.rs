//! Tests for forbidden frame proxy behavior.
//!
//! Forbidden frames use a proxy table instead of LightUserData. The proxy must
//! delegate both __index and __newindex to the underlying LightUserData so that
//! children_keys and __frame_fields stay in sync.

mod common;

use common::env_with_shared_xml;

/// Forbidden proxy __newindex must delegate to the underlying LightUserData.
/// Without this, assignments on the proxy table are invisible when the same frame
/// is later accessed as LightUserData (e.g., via children_keys lookup).
///
/// The template code pattern:
/// 1. `parent["Track"] = CreateFrame(...)` → lud stored in children_keys ✓
/// 2. `_G["__tpl_XXX"]` returns proxy table for Track
/// 3. `proxy["Thumb"] = CreateFrame(...)` → rawset on proxy, NOT synced ✗
/// 4. `parent.Track.Thumb` → children_keys returns lud → lud.Thumb = nil
#[test]
fn test_forbidden_frame_newindex_syncs_children_keys() {
    let env = env_with_shared_xml();

    // Step 1: Create a normal (non-forbidden) parent frame
    env.exec("TestParent = CreateFrame('Frame', 'TestParent', UIParent)")
        .unwrap();

    // Step 2: Enable forbidden, create named child. CreateFrame returns LUD but
    // _G["TestChild"] gets the proxy (set inside create_frame_userdata).
    // Assign the return LUD to parent via parentKey (goes through LUD __newindex).
    env.state().borrow_mut().loading_forbidden = true;
    env.exec("TestParent.Child = CreateFrame('Frame', 'TestChild', TestParent)")
        .unwrap();

    // Step 3: Access child via _G["TestChild"] (proxy), assign a grandchild on it.
    // This mimics template code: _G["__tpl_XXX"] returns proxy, proxy["Key"] = lud.
    env.exec(
        r#"
        local proxy = _G["TestChild"]
        proxy.GC = CreateFrame("Frame", nil, proxy)
    "#,
    )
    .unwrap();
    env.state().borrow_mut().loading_forbidden = false;

    // Step 4: Access via children_keys path.
    // TestParent.Child returns LUD (from children_keys), then .GC uses LUD __index.
    let result: String = env
        .eval(
            r#"
        local child_lud = TestParent.Child
        return tostring(child_lud ~= nil) .. "," .. tostring(child_lud.GC ~= nil)
    "#,
        )
        .unwrap();
    assert_eq!(
        result, "true,true",
        "property set on proxy should be visible via LightUserData __index"
    );
}

/// Template children created inside a forbidden scope should have their parentKey
/// accessible. This reproduces the ScrollBar.lua:30 error where Track.Thumb is nil
/// because Track was a forbidden frame and __newindex didn't sync to children_keys.
#[test]
fn test_forbidden_scrollbar_track_has_thumb() {
    let env = env_with_shared_xml();

    // Set forbidden loading context before creating the scrollbar
    env.state().borrow_mut().loading_forbidden = true;

    let result: String = env
        .eval(
            r#"
        local sb = CreateFrame("EventFrame", nil, UIParent, "MinimalScrollBar")
        local track = sb.Track
        local thumb = track and track.Thumb or nil
        return tostring(track ~= nil) .. "," .. tostring(thumb ~= nil)
    "#,
        )
        .unwrap();

    env.state().borrow_mut().loading_forbidden = false;

    assert_eq!(
        result, "true,true",
        "Forbidden MinimalScrollBar's Track.Thumb should be accessible"
    );
}

#[test]
fn test_forbidden_proxy_method_lookup_does_not_exhaust_aux_stack() {
    let env = env_with_shared_xml();
    env.state().borrow_mut().loading_forbidden = true;
    env.exec("CreateFrame('Frame', 'ForbiddenLoopFrame', UIParent)")
        .unwrap();
    env.state().borrow_mut().loading_forbidden = false;

    env.exec(
        r#"
        local proxy = _G["ForbiddenLoopFrame"]
        for i = 1, 9000 do
            local fn = proxy.GetName
            assert(type(fn) == "function")
            assert(fn(proxy) == "ForbiddenLoopFrame")
        end
    "#,
    )
    .unwrap();
}
