//! A1 supplied transform state and Translation presentation offsets.
//! Geometry, easing and restoration are simulator policy, not native parity.
use wow_ui_sim::LayoutRect;
use wow_ui_sim::layout::compute_frame_rect;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn new_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function near(actual, expected)
            assert(type(actual) == 'number', 'expected numeric value, got ' .. tostring(actual))
            assert(math.abs(actual - expected) < 0.00001,
                tostring(actual) .. ' differs from ' .. tostring(expected))
        end
        function pair(x, y, expectedX, expectedY)
            near(x, expectedX)
            near(y, expectedY)
        end
        function origin(animation, point, x, y)
            assert(select('#', animation:GetOrigin()) == 3)
            local actualPoint, actualX, actualY = animation:GetOrigin()
            assert(actualPoint == point)
            pair(actualX, actualY, x, y)
        end
        owner = CreateFrame('Frame', 'A1Owner', UIParent)
        owner:SetSize(120, 80)
        owner:SetPoint('TOPLEFT', UIParent, 'TOPLEFT', 100, -100)
        owner.Icon = owner:CreateTexture('A1Icon')
        owner.Icon:SetSize(20, 30)
        owner.Icon:SetPoint('TOPLEFT', owner, 'TOPLEFT', 40, -30)
        owner.Other = owner:CreateTexture('A1Other')
        owner.Other:SetSize(10, 15)
        owner.Other:SetPoint('TOPLEFT', owner, 'TOPLEFT', 70, -30)
        group = owner:CreateAnimationGroup()
        group:SetLooping('NONE')
        "#,
    )
    .unwrap();
    env
}

fn frame_rect(env: &WowLuaEnv, name: &str) -> LayoutRect {
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(name).unwrap();
    compute_frame_rect(&state.widgets, id, 1024.0, 768.0)
}

fn assert_shift(env: &WowLuaEnv, name: &str, baseline: LayoutRect, dx: f32, dy: f32) {
    let actual = frame_rect(env, name);
    assert!(
        (actual.x - baseline.x - dx).abs() < 0.0001,
        "{name}: x={}",
        actual.x
    );
    assert!(
        (actual.y - baseline.y - dy).abs() < 0.0001,
        "{name}: y={}",
        actual.y
    );
    assert_eq!(actual.width, baseline.width, "{name}: width");
    assert_eq!(actual.height, baseline.height, "{name}: height");
}

// Fixture injection, as in Path tests: pre-play presentation state, not a new Lua API.
fn seed_icon_offset(env: &WowLuaEnv, offset: [f32; 2]) {
    let mut state = env.state().borrow_mut();
    let id = state.widgets.get_id_by_name("A1Icon").unwrap();
    let icon = state.widgets.get_mut(id).unwrap();
    [icon.anim_offset_x, icon.anim_offset_y] = offset;
}

fn load_xml(env: &WowLuaEnv, xml: &str) {
    let addon = tempfile::tempdir().unwrap();
    std::fs::write(
        addon.path().join("A1Stage.toc"),
        "## Title: A1Stage\nA1Stage.xml\n",
    )
    .unwrap();
    std::fs::write(addon.path().join("A1Stage.xml"), xml).unwrap();
    load_addon(&env.loader_env(), &addon.path().join("A1Stage.toc")).unwrap();
}

#[test]
fn a1_translation_offset_round_trips() {
    let env = new_env();
    env.exec(
        r#"
        local move = group:CreateAnimation('Translation')
        local other = group:CreateAnimation('Translation')
        move:SetOffset(-350, 12)
        other:SetOffset(9, -4)
        assert(select('#', move:GetOffset()) == 2)
        local x, y = move:GetOffset()
        pair(x, y, -350, 12)
        move:SetOffset(7.5, -18.25)
        x, y = move:GetOffset()
        pair(x, y, 7.5, -18.25)
        x, y = other:GetOffset()
        pair(x, y, 9, -4)
        "#,
    )
    .unwrap();
}

#[test]
fn a1_scale_pairs_and_origin_round_trip() {
    let env = new_env();
    env.exec(
        r#"
        local scale = group:CreateAnimation('Scale')
        local other = group:CreateAnimation('Scale')
        scale:SetScaleFrom(0.15, 1)
        scale:SetScaleTo(0.2, 2)
        scale:SetOrigin('LEFT', -3, 4)
        other:SetScaleFrom(3, 4)
        other:SetOrigin('RIGHT', 5, -6)
        assert(select('#', scale:GetScaleFrom()) == 2)
        local x, y = scale:GetScaleFrom()
        pair(x, y, 0.15, 1)
        assert(select('#', scale:GetScaleTo()) == 2)
        x, y = scale:GetScaleTo()
        pair(x, y, 0.2, 2)
        origin(scale, 'LEFT', -3, 4)
        scale:SetScaleFrom(0.5, 0.75)
        scale:SetScaleTo(1.25, 1.5)
        x, y = scale:GetScaleFrom()
        pair(x, y, 0.5, 0.75)
        x, y = scale:GetScaleTo()
        pair(x, y, 1.25, 1.5)
        x, y = other:GetScaleFrom()
        pair(x, y, 3, 4)
        origin(other, 'RIGHT', 5, -6)

        -- Legacy pair is independent supplied state for this simulator stage.
        -- It must not invoke the scalar Frame:SetScale on the animation wrapper.
        scale:SetScale(2, 3)
        assert(select('#', scale:GetScale()) == 2)
        x, y = scale:GetScale()
        pair(x, y, 2, 3)
        near(owner:GetScale(), 1)
        x, y = scale:GetScaleFrom()
        pair(x, y, 0.5, 0.75)
        x, y = scale:GetScaleTo()
        pair(x, y, 1.25, 1.5)
        "#,
    )
    .unwrap();
}

#[test]
fn a1_rotation_units_and_origin_round_trip() {
    let env = new_env();
    env.exec(
        r#"
        local turn = group:CreateAnimation('Rotation')
        local other = group:CreateAnimation('Rotation')
        turn:SetDegrees(-270)
        turn:SetOrigin('BOTTOMRIGHT', -8, 6)
        other:SetDegrees(45)
        other:SetOrigin('CENTER', 1, 2)
        near(turn:GetDegrees(), -270)
        near(turn:GetRadians(), -1.5 * math.pi)
        origin(turn, 'BOTTOMRIGHT', -8, 6)
        turn:SetRadians(2.5 * math.pi)
        near(turn:GetRadians(), 2.5 * math.pi)
        near(turn:GetDegrees(), 450)
        turn:SetOrigin('TOPLEFT', 3, -4)
        origin(turn, 'TOPLEFT', 3, -4)
        near(other:GetDegrees(), 45)
        near(other:GetRadians(), math.pi / 4)
        origin(other, 'CENTER', 1, 2)
        "#,
    )
    .unwrap();
}

#[test]
fn a1_xml_transform_values_reach_queries() {
    let env = new_env();
    load_xml(
        &env,
        r#"<Ui><Frame name="A1XmlOwner" parent="UIParent">
        <Animations><AnimationGroup parentKey="Motion">
            <Translation parentKey="Move" offsetX="-350" offsetY="12" duration="2"/>
            <Scale parentKey="Grow" scaleX="2" scaleY="3" fromScaleX="0.15"
                fromScaleY="1" toScaleX="0.2" toScaleY="2" duration="2">
                <Origin point="LEFT" x="-3" y="4"/>
            </Scale>
            <Rotation parentKey="Turn" degrees="-180" duration="2">
                <Origin point="BOTTOMRIGHT" x="5" y="-6"/>
            </Rotation>
        </AnimationGroup></Animations></Frame></Ui>"#,
    );
    env.exec(
        r#"
        local motion = A1XmlOwner.Motion
        local x, y = motion.Move:GetOffset()
        pair(x, y, -350, 12)
        x, y = motion.Grow:GetScaleFrom()
        pair(x, y, 0.15, 1)
        x, y = motion.Grow:GetScaleTo()
        pair(x, y, 0.2, 2)
        x, y = motion.Grow:GetScale()
        pair(x, y, 2, 3)
        origin(motion.Grow, 'LEFT', -3, 4)
        near(motion.Turn:GetDegrees(), -180)
        near(motion.Turn:GetRadians(), -math.pi)
        origin(motion.Turn, 'BOTTOMRIGHT', 5, -6)
        "#,
    )
    .unwrap();
}

#[test]
fn a1_translation_ticks_restore_and_replay() {
    let env = new_env();
    seed_icon_offset(&env, [3.0, -7.0]);
    env.exec(
        r#"
        move = group:CreateAnimation('Translation')
        move:SetChildKey('Icon')
        move:SetOffset(-40, 20)
        move:SetDuration(2)
        move:SetSmoothing('NONE')
        group:SetToFinalAlpha(true) -- alpha-only flag, not retain-translation
        assert(move:GetTarget() == owner.Icon)
        "#,
    )
    .unwrap();
    let icon = frame_rect(&env, "A1Icon");
    let owner = frame_rect(&env, "A1Owner");
    let other = frame_rect(&env, "A1Other");
    env.exec("group:Play()").unwrap();
    assert_shift(&env, "A1Icon", icon, 0.0, 0.0);
    env.fire_on_update(1.0).unwrap();
    assert_shift(&env, "A1Icon", icon, -20.0, 10.0);
    assert_shift(&env, "A1Owner", owner, 0.0, 0.0);
    assert_shift(&env, "A1Other", other, 0.0, 0.0);
    env.exec("group:Pause()").unwrap();
    env.fire_on_update(0.25).unwrap();
    assert_shift(&env, "A1Icon", icon, -20.0, 10.0);
    env.exec("group:Play()").unwrap();
    env.fire_on_update(0.5).unwrap();
    assert_shift(&env, "A1Icon", icon, -30.0, 15.0);
    env.exec("group:Stop()").unwrap();
    assert_shift(&env, "A1Icon", icon, 0.0, 0.0);
    seed_icon_offset(&env, [-9.0, 11.0]);
    let replay = frame_rect(&env, "A1Icon");
    env.exec("group:Play()").unwrap();
    env.fire_on_update(0.5).unwrap();
    assert_shift(&env, "A1Icon", replay, -10.0, 5.0);
    env.fire_on_update(1.5).unwrap();
    env.exec("assert(group:IsDone())").unwrap();
    assert_shift(&env, "A1Icon", replay, 0.0, 0.0);
}

#[test]
fn a1_translation_xml_out_moves_only_declared_child() {
    let env = new_env();
    // Literal public CatalogShop tag (Retail line335); only its surrounding scene
    // is reduced. This is not a claim that the full vendor template was executed.
    load_xml(
        &env,
        r#"<Ui><Frame name="A1GlowOwner" parent="UIParent">
        <Size x="120" y="80"/>
        <Anchors><Anchor point="TOPLEFT" relativeTo="UIParent" relativePoint="TOPLEFT" x="100" y="-100"/></Anchors>
        <Layers><Layer level="ARTWORK"><Texture name="A1Glow" parentKey="Glow1">
            <Size x="20" y="30"/>
            <Anchors><Anchor point="TOPLEFT" x="40" y="-30"/></Anchors>
        </Texture></Layer></Layers>
        <Animations><AnimationGroup parentKey="Motion" setToFinalAlpha="true">
            <Translation childKey="Glow1" duration="0.75" order="1" smoothing="OUT" offsetX="-350" offsetY="0"/>
        </AnimationGroup></Animations></Frame></Ui>"#,
    );
    let glow = frame_rect(&env, "A1Glow");
    let owner = frame_rect(&env, "A1GlowOwner");
    env.exec("A1GlowOwner.Motion:Play()").unwrap();
    env.fire_on_update(0.375).unwrap();
    // Existing VertexColor OUT policy gives 0.75 progress at half time.
    assert_shift(&env, "A1Glow", glow, -262.5, 0.0);
    assert_shift(&env, "A1GlowOwner", owner, 0.0, 0.0);
    env.fire_on_update(0.375).unwrap();
    env.exec("assert(A1GlowOwner.Motion:IsDone())").unwrap();
    assert_shift(&env, "A1Glow", glow, 0.0, 0.0);
}

#[test]
fn a1_translation_owner_and_hidden_stop() {
    let env = new_env();
    env.exec(
        r#"
        move = group:CreateAnimation('Translation')
        move:SetOffset(24, -16)
        move:SetDuration(2)
        move:SetSmoothing('NONE')
        assert(move:GetTarget() == owner)
        "#,
    )
    .unwrap();
    let owner = frame_rect(&env, "A1Owner");
    env.exec("group:Play()").unwrap();
    env.fire_on_update(1.0).unwrap();
    assert_shift(&env, "A1Owner", owner, 12.0, -8.0);
    env.exec("owner:Hide(); assert(not group:IsPlaying())")
        .unwrap();
    assert_shift(&env, "A1Owner", owner, 0.0, 0.0);
}
