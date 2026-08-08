//! Startup publication and values for the 12.0.0 collection and secret-aspect enums.

#![cfg(feature = "retail-12-0-0")]

use super::super::*;

const ITEM_COLLECTION_TYPE_VALUES: &[(&str, i64)] = &[
    ("ExteriorFixture", 9),
    ("Heirloom", 2),
    ("HouseType", 13),
    ("None", 0),
    ("Room", 8),
    ("RoomMaterial", 11),
    ("RoomTheme", 10),
    ("RuneforgeLegendaryAbility", 5),
    ("Toy", 1),
    ("Transmog", 3),
    ("TransmogIllusion", 6),
    ("TransmogOutfit", 12),
    ("TransmogSetFavorite", 4),
    ("WarbandScene", 7),
];

const ITEM_COLLECTION_TYPE_META_VALUES: &[(&str, i64)] = &[("MaxValue", 13), ("NumValues", 14)];

const SECRET_ASPECT_VALUES: &[(&str, i64)] = &[
    ("Alpha", 128),
    ("BarValue", 16384),
    ("Cooldown", 32768),
    ("Cursor", 1024),
    ("Desaturation", 4096),
    ("FrameLevel", 256),
    ("Hierarchy", 1),
    ("ID", 2),
    ("MinimumWidth", 131072),
    ("ObjectDebug", 1),
    ("ObjectName", 1),
    ("ObjectSecrets", 1),
    ("ObjectSecurity", 1),
    ("ObjectType", 1),
    ("Padding", 262144),
    ("Rotation", 65536),
    ("Scale", 64),
    ("ScrollRange", 512),
    ("SecureText", 16),
    ("Shown", 32),
    ("TexCoords", 8192),
    ("Text", 8),
    ("Toplevel", 4),
    ("VertexColor", 2048),
];

fn assert_numeric_namespace(env: &WowLuaEnv, namespace_name: &str, expected: &[(&str, i64)]) {
    let expected_lua = expected
        .iter()
        .map(|(name, value)| format!("[{name:?}] = {value}"))
        .collect::<Vec<_>>()
        .join(",\n                ");
    let script = format!(
        r#"
            local namespace = {namespace_name}
            if type(namespace) ~= "table" then
                return "namespace:" .. type(namespace)
            end
            local expected = {{
                {expected_lua}
            }}
            for name, value in pairs(expected) do
                local actual = namespace[name]
                if type(actual) ~= "number" then
                    return name .. ":type=" .. type(actual)
                end
                if actual ~= value then
                    return name .. ":value=" .. tostring(actual)
                end
            end
            return "ok"
        "#,
        namespace_name = namespace_name,
        expected_lua = expected_lua,
    );
    let result: String = env.eval(&script).unwrap();
    assert_eq!(
        result, "ok",
        "{namespace_name} did not match the 12.0.0 source register"
    );
}

#[test]
fn test_patch_12_0_0_item_collection_and_secret_aspect_values() {
    let env = WowLuaEnv::new().unwrap();
    assert_numeric_namespace(&env, "Enum.ItemCollectionType", ITEM_COLLECTION_TYPE_VALUES);
    assert_numeric_namespace(
        &env,
        "Enum.ItemCollectionTypeMeta",
        ITEM_COLLECTION_TYPE_META_VALUES,
    );
    assert_numeric_namespace(&env, "Enum.SecretAspect", SECRET_ASPECT_VALUES);
}
