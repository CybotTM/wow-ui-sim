use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";
const FRAME_TEMPLATE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local function expectCorner(parent, key, point, x, y)
    local corner = parent and parent[key]
    expect(corner ~= nil, "AzeriteRespecFrame." .. key .. " must exist")
    if corner == nil then
        return
    end

    expect(corner:GetObjectType() == "Texture",
        "AzeriteRespecFrame." .. key .. " must be a Texture")
    expect(corner:GetParent() == parent,
        "AzeriteRespecFrame." .. key .. " must be parented to AzeriteRespecFrame")
    expect(corner:GetWidth() == 64 and corner:GetHeight() == 64,
        "AzeriteRespecFrame." .. key .. " must keep EtherealFrameTemplate 64x64 size")
    expect(corner:GetNumPoints() == 1,
        "AzeriteRespecFrame." .. key .. " must have exactly one anchor after OnLoad")

    local actualPoint, relativeTo, relativePoint, actualX, actualY = corner:GetPoint(1)
    expect(actualPoint == point,
        "AzeriteRespecFrame." .. key .. " point must be " .. point .. ", got " .. tostring(actualPoint))
    expect(relativeTo == parent,
        "AzeriteRespecFrame." .. key .. " relativeTo must be AzeriteRespecFrame")
    expect(relativePoint == point,
        "AzeriteRespecFrame." .. key .. " relativePoint must be " .. point .. ", got " .. tostring(relativePoint))
    expect(actualX == x and actualY == y,
        "AzeriteRespecFrame." .. key .. " offset must be " .. x .. "," .. y ..
        ", got " .. tostring(actualX) .. "," .. tostring(actualY))
end

expect(type(AzeriteRespecFrame) == "table", "AzeriteRespecFrame must exist")
if type(AzeriteRespecFrame) == "table" then
    expect(AzeriteRespecFrame:GetObjectType() == "Frame", "AzeriteRespecFrame must be a Frame")
    expect(AzeriteRespecFrame:GetParent() == UIParent, "AzeriteRespecFrame must be parented to UIParent")
    expect(not AzeriteRespecFrame:IsShown(), "AzeriteRespecFrame must start hidden")

    expectCorner(AzeriteRespecFrame, "CornerBL", "BOTTOMLEFT", -1, 24)
    expectCorner(AzeriteRespecFrame, "CornerBR", "BOTTOMRIGHT", 0, 24)
    expectCorner(AzeriteRespecFrame, "CornerTL", "TOPLEFT", -2, -18)
    expectCorner(AzeriteRespecFrame, "CornerTR", "TOPRIGHT", 0, -18)
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_azerite_respec_ui_frame_inherits_ethereal_template_shape() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(FRAME_TEMPLATE_PROBE_LUA)
                    .expect("AzeriteRespecFrame template probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` frame template shape mismatches:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking frame template shape:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
