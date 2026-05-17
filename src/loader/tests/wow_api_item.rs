//! C_Item API coverage extracted from `wow_api.rs`.

use super::*;

#[test]
fn test_c_item_subclass_names_match_en_us_keywords() {
    let env = WowLuaEnv::new().unwrap();
    let names_match_keywords: bool = env
        .eval(
            r#"
            local expected = {
                {Enum.ItemClass.Armor, 6, "shields"},
                {Enum.ItemClass.Weapon, 0, "one-handed axes"},
                {Enum.ItemClass.Weapon, 1, "two-handed axes"},
                {Enum.ItemClass.Weapon, 4, "one-handed maces"},
                {Enum.ItemClass.Weapon, 5, "two-handed maces"},
                {Enum.ItemClass.Weapon, 6, "polearms"},
                {Enum.ItemClass.Weapon, 12, "catclaws"},
                {Enum.ItemClass.Weapon, 13, "fist weapons"},
            }
            for _, entry in ipairs(expected) do
                if C_Item.GetItemSubClassInfo(entry[1], entry[2]):lower() ~= entry[3] then
                    return false
                end
            end
            return true
            "#,
        )
        .unwrap();
    assert!(names_match_keywords);
}
