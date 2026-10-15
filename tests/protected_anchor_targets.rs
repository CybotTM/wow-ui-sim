//! Shared clean-room protected-recipient anchor target-kind contract.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, parse_xml, register_template};

// Simulator-owned fixture, not a Blizzard/vendor/native template catalog entry.
const PROTECTED_TEMPLATE_XML: &str = r#"
<Ui>
    <Frame name="L1CandidateProtectedFrameTemplate" virtual="true" protected="true"/>
</Ui>
"#;

fn register_protected_template() {
    let ui = parse_xml(PROTECTED_TEMPLATE_XML).expect("parse simulator-owned protected XML");
    let XmlElement::Frame(frame) = &ui.elements[0] else {
        panic!("expected simulator-owned Frame template");
    };
    register_template("L1CandidateProtectedFrameTemplate", "Frame", frame.clone());
}

fn create_anchor_test_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("create minimal Lua environment without vendor loading");
    register_protected_template();
    env.exec(ANCHOR_FIXTURE_LUA)
        .expect("install public-API anchor fixture");
    env
}

const ANCHOR_FIXTURE_LUA: &str = r#"
function AssertTrustedOutOfCombat()
    assert(issecure() == true, 'fixture requires a trusted caller')
    assert(not InCombatLockdown(), 'fixture requires out-of-combat execution')
end

function AssertProtection(frame, expected)
    local protected, explicit = frame:IsProtected()
    assert(protected == expected, 'unexpected protection state')
    assert(explicit == expected, 'unexpected explicit protection state')
end

AssertTrustedOutOfCombat()
AnchorRoot = CreateFrame('Frame', nil, UIParent)
AnchorRoot:SetSize(800, 600)
AnchorRoot:SetPoint('BOTTOMLEFT', UIParent, 'BOTTOMLEFT', 0, 0)
AssertProtection(AnchorRoot, false)

function PackResults(...)
    return { n = select('#', ...), ... }
end

function AssertTupleEqual(actual, expected, label)
    assert(actual.n == expected.n, label .. ': result count changed')
    for index = 1, expected.n do
        assert(actual[index] == expected[index], label .. ': field ' .. index .. ' changed')
    end
end

function ReadResolvedRect(frame)
    local rect = PackResults(frame:GetRect())
    assert(rect.n == 4, 'GetRect must return four resolved values')
    for index = 1, 4 do
        assert(type(rect[index]) == 'number', 'GetRect must resolve numeric geometry')
    end
    assert(rect[3] > 0 and rect[4] > 0, 'fixture geometry must be nondegenerate')
    return rect
end

function CaptureAnchors(frame)
    local snapshot = { count = frame:GetNumPoints(), points = {}, rect = ReadResolvedRect(frame) }
    assert(snapshot.count == 2, 'recipient baseline must have two active points')
    for index = 1, snapshot.count do
        local point = PackResults(frame:GetPoint(index))
        assert(point.n == 5, 'GetPoint must return the complete five-field tuple')
        snapshot.points[index] = point
    end
    return snapshot
end

function ReadNamedPoint(frame, name)
    for index = 1, frame:GetNumPoints() do
        local point = PackResults(frame:GetPoint(index))
        if point[1] == name then
            return point
        end
    end
    error('missing active anchor ' .. name)
end

function AssertSnapshotUnchanged(frame, baseline)
    assert(frame:GetNumPoints() == baseline.count, 'rejection changed point count')
    for index = 1, baseline.count do
        AssertTupleEqual(PackResults(frame:GetPoint(index)), baseline.points[index], 'GetPoint')
    end
    AssertTupleEqual(ReadResolvedRect(frame), baseline.rect, 'GetRect')
end

function ResetRecipientAnchors(frame)
    frame:ClearAllPoints()
    frame:SetSize(580, 370)
    frame:SetPoint('BOTTOMLEFT', AnchorRoot, 'BOTTOMLEFT', 20, 30)
    frame:SetPoint('TOPRIGHT', AnchorRoot, 'BOTTOMLEFT', 600, 400)
end

function CreateRecipient(protected)
    AssertTrustedOutOfCombat()
    local template = protected and 'L1CandidateProtectedFrameTemplate' or nil
    local frame = CreateFrame('Frame', nil, AnchorRoot, template)
    AssertProtection(frame, protected)
    ResetRecipientAnchors(frame)
    CaptureAnchors(frame)
    return frame
end

function CreateRegion(kind)
    local region
    if kind == 'Texture' then
        region = AnchorRoot:CreateTexture(nil, 'ARTWORK')
    else
        assert(kind == 'FontString', 'unsupported fixture region kind')
        region = AnchorRoot:CreateFontString(nil, 'ARTWORK')
    end
    region:SetSize(90, 50)
    region:SetPoint('BOTTOMLEFT', AnchorRoot, 'BOTTOMLEFT', 240, 180)
    assert(region:GetObjectType() == kind, 'fixture must use the requested real region kind')
    AssertProtection(region, false)
    ReadResolvedRect(region)
    return region
end

function CreateBridge(target)
    local bridge = CreateFrame('Frame', nil, AnchorRoot)
    bridge:SetSize(70, 45)
    bridge:SetPoint('BOTTOMLEFT', target, 'BOTTOMLEFT', 7, 9)
    AssertProtection(bridge, false)
    assert(bridge:GetObjectType() == 'Frame', 'chain target must be an ordinary Frame')
    assert(bridge:GetNumPoints() == 1, 'bridge must have one active anchor')
    AssertTupleEqual(PackResults(bridge:GetPoint(1)),
        PackResults('BOTTOMLEFT', target, 'BOTTOMLEFT', 7, 9), 'bridge active anchor')
    ReadResolvedRect(bridge)
    return bridge
end

function ApplyCandidateAnchor(frame, target)
    frame:SetPoint('BOTTOMLEFT', target, 'BOTTOMLEFT', 11, 13)
end

function ResolveMovingWitness(target, baseline)
    local witness = CreateRecipient(false)
    ApplyCandidateAnchor(witness, target)
    AssertTupleEqual(ReadNamedPoint(witness, 'BOTTOMLEFT'),
        PackResults('BOTTOMLEFT', target, 'BOTTOMLEFT', 11, 13), 'witness applied anchor')
    local moved = ReadResolvedRect(witness)
    assert(moved[1] ~= baseline.rect[1] or moved[2] ~= baseline.rect[2],
        'candidate input must actually move an equivalent unprotected recipient')
    return moved
end

function AssertRejectedAnchor(recipient, target)
    AssertTrustedOutOfCombat()
    AssertProtection(recipient, true)
    local baseline = CaptureAnchors(recipient)
    ResolveMovingWitness(target, baseline)
    -- Decline may be a no-op or an error; no return/error/event contract is imposed.
    pcall(ApplyCandidateAnchor, recipient, target)
    AssertSnapshotUnchanged(recipient, baseline)
    AssertProtection(recipient, true)
    AssertTrustedOutOfCombat()
end

function AssertAcceptedAnchor(recipient, target)
    AssertTrustedOutOfCombat()
    local baseline = CaptureAnchors(recipient)
    local expected_rect = ResolveMovingWitness(target, baseline)
    local untouched = ReadNamedPoint(recipient, 'TOPRIGHT')
    ApplyCandidateAnchor(recipient, target)
    assert(recipient:GetNumPoints() == baseline.count, 'accepted replacement changed point count')
    AssertTupleEqual(ReadNamedPoint(recipient, 'BOTTOMLEFT'),
        PackResults('BOTTOMLEFT', target, 'BOTTOMLEFT', 11, 13), 'accepted anchor')
    AssertTupleEqual(ReadNamedPoint(recipient, 'TOPRIGHT'), untouched, 'untouched anchor')
    AssertTupleEqual(ReadResolvedRect(recipient), expected_rect, 'accepted geometry')
    AssertTrustedOutOfCombat()
end
"#;

#[test]
fn protected_recipient_declines_direct_unprotected_texture() {
    let env = create_anchor_test_env();
    env.exec("AssertRejectedAnchor(CreateRecipient(true), CreateRegion('Texture'))")
        .expect("direct Texture decline must preserve all points and resolved geometry");
}

#[test]
fn protected_recipient_declines_direct_unprotected_font_string() {
    let env = create_anchor_test_env();
    env.exec("AssertRejectedAnchor(CreateRecipient(true), CreateRegion('FontString'))")
        .expect("direct FontString decline must preserve all points and resolved geometry");
}

#[test]
fn protected_recipient_declines_ordinary_frames_with_active_region_chains() {
    let env = create_anchor_test_env();
    env.exec(
        r#"
        for _, kind in ipairs({ 'Texture', 'FontString' }) do
            local region = CreateRegion(kind)
            local middle = CreateBridge(region)
            local recipient = CreateRecipient(true)
            AssertRejectedAnchor(recipient, middle)
            local outer = CreateBridge(middle)
            AssertRejectedAnchor(recipient, outer)
        end
        "#,
    )
    .expect("one-hop and transitive region chains must preserve protected recipient state");
}

#[test]
fn allowed_controls_preserve_unprotected_regions_and_ordinary_frame_targets() {
    let env = create_anchor_test_env();
    env.exec(
        r#"
        for _, kind in ipairs({ 'Texture', 'FontString' }) do
            AssertAcceptedAnchor(CreateRecipient(false), CreateRegion(kind))
        end
        for _, protected_target in ipairs({ false, true }) do
            local target = CreateRecipient(protected_target)
            target:ClearAllPoints()
            target:SetSize(90, 50)
            target:SetPoint('BOTTOMLEFT', AnchorRoot, 'BOTTOMLEFT', 240, 180)
            assert(target:GetObjectType() == 'Frame')
            AssertProtection(target, protected_target)
            local recipient = CreateRecipient(true)
            AssertAcceptedAnchor(recipient, target)
            AssertProtection(recipient, true)
        end
        "#,
    )
    .expect("controls must move: unprotected recipients to regions; protected to ordinary Frames");
}

#[test]
fn removing_and_restoring_active_region_chain_changes_target_eligibility() {
    let env = create_anchor_test_env();
    env.exec(
        r#"
        for _, kind in ipairs({ 'Texture', 'FontString' }) do
            local region = CreateRegion(kind)
            local middle = CreateBridge(region)
            local recipient = CreateRecipient(true)
            AssertRejectedAnchor(recipient, middle)

            middle:ClearAllPoints()
            assert(middle:GetNumPoints() == 0, 'remove the active region chain')
            middle:SetPoint('BOTTOMLEFT', AnchorRoot, 'BOTTOMLEFT', 240, 180)
            AssertTupleEqual(PackResults(middle:GetPoint(1)),
                PackResults('BOTTOMLEFT', AnchorRoot, 'BOTTOMLEFT', 240, 180), 'replacement chain')
            AssertProtection(middle, false)
            AssertAcceptedAnchor(recipient, middle)

            ResetRecipientAnchors(recipient)
            middle:ClearAllPoints()
            assert(middle:GetNumPoints() == 0)
            middle:SetPoint('BOTTOMLEFT', region, 'BOTTOMLEFT', 7, 9)
            AssertTupleEqual(PackResults(middle:GetPoint(1)),
                PackResults('BOTTOMLEFT', region, 'BOTTOMLEFT', 7, 9), 'restored region chain')
            AssertProtection(middle, false)
            AssertRejectedAnchor(recipient, middle)
        end
        "#,
    )
    .expect("eligibility must follow active anchors rather than historical region dependencies");
}
