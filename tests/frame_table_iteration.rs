use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn frame_pairs_enumerates_identity_and_array_slots() {
    let env = env();
    let (has_identity, has_inserted_value): (bool, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            tinsert(frame, "foo")

            local seen = {}
            for key, value in pairs(frame) do
                seen[key] = value
            end

            return type(seen[0]) == "userdata", seen[1] == "foo"
            "#,
        )
        .unwrap();

    assert!(
        has_identity,
        "pairs(frame) should include the frame[0] userdata identity"
    );
    assert!(
        has_inserted_value,
        "pairs(frame) should include array slots written by tinsert"
    );
}

#[test]
fn c_string_util_escape_quoted_codes_returns_displayable_string() {
    let env = env();
    let (plain, escaped_pipe): (String, String) = env
        .eval(
            r#"
            return C_StringUtil.EscapeQuotedCodes('"foo"'),
                   C_StringUtil.EscapeQuotedCodes('"|cff00ff00foo|r"')
            "#,
        )
        .unwrap();

    assert_eq!(plain, r#""foo""#);
    assert_eq!(escaped_pipe, r#""||cff00ff00foo||r""#);
}

#[test]
fn frame_identity_userdata_is_opaque_to_dumpobject() {
    let env = env();
    let (identity_type, dumpobject_type, dump_result_is_nil): (String, String, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local identity = frame[0]
            return type(identity), type(dumpobject), dumpobject(identity) == nil
            "#,
        )
        .unwrap();

    assert_eq!(identity_type, "userdata");
    assert_eq!(dumpobject_type, "function");
    assert!(dump_result_is_nil);
}
