//! Tests for frame hierarchy methods: GetChildren, GetNumChildren, GetRegions.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_children_excludes_regions() {
    let env = env();
    env.exec(
        r#"
        local parent = CreateFrame("Frame", "HierarchyParent", UIParent)
        local child = CreateFrame("Frame", "HierarchyChild", parent)
        local tex = parent:CreateTexture("HierarchyTexture", "ARTWORK")
        local text = parent:CreateFontString("HierarchyFontString", "ARTWORK")

        assert(parent:GetNumChildren() == 1, "regions should not count as children")
        assert(parent:GetNumRegions() == 2, "texture and font string should count as regions")

        local onlyChild = parent:GetChildren()
        assert(onlyChild == child, "GetChildren should return only child frames")

        local firstRegion, secondRegion = parent:GetRegions()
        assert(firstRegion == tex, "GetRegions should include texture regions")
        assert(secondRegion == text, "GetRegions should include font string regions")
    "#,
    )
    .unwrap();
}
