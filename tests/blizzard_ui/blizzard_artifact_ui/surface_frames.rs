//! Frame-surface probes for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArtifactUI";
const ARTIFACT_FRAME_CHILDREN: &[&str] = &[
    "PerksTab",
    "AppearancesTab",
    "AppearancesTabButton",
    "ForgeBadgeFrame",
    "ForgeLevelFrame",
    "VisitForgeOverlay",
];

#[test]
fn artifact_ui_exposes_frame_and_overlay_children_after_load_addon() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(&frame_surface_probe())
            .expect("ArtifactUI frame surface probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must expose ArtifactFrame, expected children, and sibling underlay; \
             mismatches: {mismatches:?}"
        );
    });
}

#[test]
fn artifact_ui_registers_uipanel_window_entry_after_load_addon() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(UIPANEL_WINDOW_SURFACE_PROBE)
            .expect("ArtifactFrame UIPanelWindows surface probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must register the ArtifactFrame UIPanelWindows entry; \
             mismatches: {mismatches:?}"
        );
    });
}

fn load_artifact_ui(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, error): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
        .expect("C_AddOns.LoadAddOn probe should run cleanly");
    assert!(
        loaded,
        "`{ROOT}` must load before frame surface probes; error={error:?}"
    );
}

const UIPANEL_WINDOW_SURFACE_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local entry = UIPanelWindows and UIPanelWindows["ArtifactFrame"]
expect(type(entry) == "table", "UIPanelWindows.ArtifactFrame:" .. type(entry))

if type(entry) == "table" then
    expect(entry.area == "doublewide", "area:" .. tostring(entry.area))
    expect(entry.pushable == 0, "pushable:" .. tostring(entry.pushable))
    expect(entry.xoffset == 35, "xoffset:" .. tostring(entry.xoffset))
    expect(entry.yoffset == -9, "yoffset:" .. tostring(entry.yoffset))
    expect(
        entry.bottomClampOverride == 100,
        "bottomClampOverride:" .. tostring(entry.bottomClampOverride)
    )
    expect(
        type(C_ArtifactUI.Clear) == "function",
        "C_ArtifactUI.Clear:" .. type(C_ArtifactUI.Clear)
    )
    expect(
        entry.showFailedFunc == C_ArtifactUI.Clear,
        "showFailedFunc direct reference"
    )
end

return mismatches
"#;

fn frame_surface_probe() -> String {
    let child_list = lua_array_literal(ARTIFACT_FRAME_CHILDREN);

    format!(
        r#"
        local children = {{{child_list}}}
        local mismatches = {{}}

        local function expect(condition, message)
            if not condition then
                table.insert(mismatches, message)
            end
        end

        local frame = ArtifactFrame
        local underlay = ArtifactFrameUnderlay
        expect(type(frame) == "table", "ArtifactFrame:" .. type(frame))
        expect(frame and frame:GetObjectType() == "Frame", "ArtifactFrame object type")
        expect(frame and frame:GetParent() == UIParent, "ArtifactFrame parent")

        for _, childName in ipairs(children) do
            local child = frame and frame[childName]
            expect(type(child) == "table", "ArtifactFrame." .. childName .. ":" .. type(child))
            expect(child and child:GetParent() == frame, "ArtifactFrame." .. childName .. " parent")
        end

        expect(type(underlay) == "table", "ArtifactFrameUnderlay:" .. type(underlay))
        expect(underlay and underlay:GetObjectType() == "Frame", "ArtifactFrameUnderlay object type")
        expect(underlay and underlay:GetParent() == UIParent, "ArtifactFrameUnderlay parent")
        expect(underlay and underlay ~= frame, "ArtifactFrameUnderlay is distinct from ArtifactFrame")
        expect(
            underlay and frame and underlay:GetParent() == frame:GetParent(),
            "ArtifactFrameUnderlay sibling parent"
        )

        return mismatches
        "#
    )
}

fn lua_array_literal(values: &[&str]) -> String {
    values
        .iter()
        .map(|value| format!("{value:?}"))
        .collect::<Vec<_>>()
        .join(", ")
}
