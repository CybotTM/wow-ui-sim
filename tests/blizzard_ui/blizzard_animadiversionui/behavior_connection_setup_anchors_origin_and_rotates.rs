//! `AnimaDiversionConnectionMixin:Setup` geometry and visual-state probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const CONNECTION_SETUP_PROBE: &str = r#"
local originalDistance = RegionUtil.CalculateDistanceBetween
local originalAngle = RegionUtil.CalculateAngleBetween
local distanceCalls = {}
local angleCalls = {}

RegionUtil.CalculateDistanceBetween = function(origin, pin)
    table.insert(distanceCalls, { origin = origin, pin = pin })
    return 30
end
RegionUtil.CalculateAngleBetween = function(origin, pin)
    table.insert(angleCalls, { origin = origin, pin = pin })
    return math.pi
end

local function countPlayingAnimationGroups(connection)
    local groupCount = 0
    local playingCount = 0
    for _, animationGroup in ipairs(connection.animationGroups or {}) do
        groupCount = groupCount + 1
        if animationGroup:IsPlaying() then
            playingCount = playingCount + 1
        end
    end
    return groupCount, playingCount
end

local function buildPin(name, state)
    local pin = CreateFrame("Frame", name, UIParent)
    pin.nodeData = {
        state = state,
    }
    return pin
end

local function setupConnection(name, textureKit, state)
    local origin = CreateFrame("Frame", name.."Origin", UIParent)
    origin:SetScale(1.5)

    local pin = buildPin(name.."Pin", state)
    local connection = CreateFrame("Frame", name, UIParent, "AnimaDiversionConnectionTemplate")
    connection:Setup(textureKit, origin, pin)
    return connection, origin, pin
end

local temporaryConnection, temporaryOrigin, temporaryPin = setupConnection(
    "AnimaConnectionTemporaryProbe",
    "Venthyr",
    Enum.AnimaDiversionNodeState.SelectedTemporary
)
local point, relativeTo, relativePoint = temporaryConnection:GetPoint(1)
local startPoint, startTarget = temporaryConnection.Line:GetStartPoint()
local endPoint, endTarget = temporaryConnection.Line:GetEndPoint()
local groupCount, playingCount = countPlayingAnimationGroups(temporaryConnection)
local expectedAngle = math.pi - (math.pi / 2)

local normalConnection = setupConnection(
    "AnimaConnectionNormalProbe",
    "Kyrian",
    Enum.AnimaDiversionNodeState.Available
)
local necrolordConnection = setupConnection(
    "AnimaConnectionNecrolordProbe",
    "Necrolord",
    Enum.AnimaDiversionNodeState.Available
)

RegionUtil.CalculateDistanceBetween = originalDistance
RegionUtil.CalculateAngleBetween = originalAngle

local geometryMatches = point == "BOTTOM"
    and relativeTo == temporaryOrigin
    and relativePoint == "CENTER"
    and math.abs(temporaryConnection:GetHeight() - 45) < 0.001
local rotationMatches = math.abs(temporaryConnection.AnimaLink1:GetRotation() - expectedAngle) < 0.001
    and math.abs(temporaryConnection.AnimaLink2:GetRotation() - expectedAngle) < 0.001
    and math.abs(temporaryConnection.AnimaLinkBlack:GetRotation() - expectedAngle) < 0.001
local endpointsMatch = startPoint == "CENTER"
    and startTarget == temporaryOrigin
    and endPoint == "CENTER"
    and endTarget == temporaryPin
local blackLinkMatches = temporaryConnection.AnimaLinkBlack:IsShown()
    and not normalConnection.AnimaLinkBlack:IsShown()
    and necrolordConnection.AnimaLinkBlack:IsShown()
local maskAndAnimationsMatch = temporaryConnection.Mask:IsShown()
    and not normalConnection.Mask:IsShown()
    and groupCount >= 4
    and playingCount == groupCount
local regionUtilCallsMatch = #distanceCalls == 3
    and distanceCalls[1].origin == temporaryOrigin
    and distanceCalls[1].pin == temporaryPin
    and #angleCalls == 3
    and angleCalls[1].origin == temporaryOrigin
    and angleCalls[1].pin == temporaryPin

return geometryMatches,
       rotationMatches,
       endpointsMatch,
       temporaryConnection.Line:GetThickness(),
       normalConnection.Line:GetThickness(),
       temporaryConnection.Line:GetAtlas(),
       blackLinkMatches,
       maskAndAnimationsMatch,
       regionUtilCallsMatch
"#;

#[test]
fn connection_setup_anchors_origin_rotates_textures_and_sets_visual_state() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: ConnectionSetupState = env
            .eval(CONNECTION_SETUP_PROBE)
            .expect("connection setup probe must run cleanly");

        assert_connection_setup_state(state);
    });
}

type ConnectionSetupState = (bool, bool, bool, f64, f64, String, bool, bool, bool);

fn assert_connection_setup_state(state: ConnectionSetupState) {
    assert_geometry(state.0);
    assert_rotation(state.1);
    assert_line_endpoints(state.2);
    assert_visuals((state.3, state.4, state.5, state.6));
    assert_mask_and_animations(state.7);
    assert_region_util_calls(state.8);
}

fn assert_geometry(geometry_matches: bool) {
    assert!(
        geometry_matches,
        "Connection must anchor BOTTOM to origin CENTER and scale height by origin effective scale"
    );
}

fn assert_rotation(rotation_matches: bool) {
    assert!(
        rotation_matches,
        "Textures must rotate by RegionUtil angle minus pi/2"
    );
}

fn assert_line_endpoints(endpoints_match: bool) {
    assert!(
        endpoints_match,
        "Line endpoints must run from origin CENTER to pin CENTER"
    );
}

fn assert_visuals(state: (f64, f64, String, bool)) {
    let (temporary_thickness, normal_thickness, line_atlas, black_links_match) = state;

    assert_eq!(
        temporary_thickness, 20.0,
        "SelectedTemporary line thickness must be 20"
    );
    assert_eq!(
        normal_thickness, 40.0,
        "Non-temporary line thickness must be 40"
    );
    assert_eq!(
        line_atlas, "_AnimaChannel-Channel-Line-horizontal-Venthyr",
        "Line atlas must be texture-kit formatted"
    );
    assert!(
        black_links_match,
        "Black anima link must show only for Venthyr and Necrolord"
    );
}

fn assert_mask_and_animations(mask_and_animations_match: bool) {
    assert!(
        mask_and_animations_match,
        "Mask must show only for SelectedTemporary and every animation group must play"
    );
}

fn assert_region_util_calls(region_util_calls_match: bool) {
    assert!(
        region_util_calls_match,
        "Setup must measure distance and angle through RegionUtil for each connection"
    );
}
