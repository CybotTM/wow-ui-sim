//! Patch 10.2.5 VertexColor examples plus explicitly modeled lifecycle policies.
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn setup() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function near(actual, expected)
            assert(math.abs(actual - expected) < 0.00001,
                tostring(actual) .. " differs from " .. tostring(expected))
        end
        function rgba(object, r, g, b, a)
            local ar, ag, ab, aa = object:GetRGBA()
            near(ar, r); near(ag, g); near(ab, b); near(aa, a)
        end
        function vertex(texture, r, g, b, a)
            local ar, ag, ab, aa = texture:GetVertexColor()
            near(ar, r); near(ag, g); near(ab, b); near(aa, a)
        end
        texture = UIParent:CreateTexture()
        texture:SetVertexColor(0.2, 0.3, 0.4, 0.5)
        group = texture:CreateAnimationGroup()
        animation = group:CreateAnimation("VertexColor")
        "#,
    )
    .unwrap();
    env
}

fn configure(env: &WowLuaEnv) {
    env.exec(
        r#"
        animation:SetStartColor(CreateColor(1, 0.2, 0, 0.25))
        animation:SetEndColor(CreateColor(0, 0.6, 1, 0.75))
        animation:SetDuration(2)
        "#,
    )
    .unwrap();
}

#[test]
fn vertex_color_animation_endpoints_are_independent_color_objects() {
    let env = setup();
    env.exec(
        r#"
        local start = CreateColor(1, 0.2, 0, 0.25)
        local finish = CreateColor(0, 0.6, 1, 0.75)
        assert(select('#', animation:SetStartColor(start)) == 0)
        assert(select('#', animation:SetEndColor(finish)) == 0)
        assert(select('#', animation:GetStartColor()) == 1)
        assert(select('#', animation:GetEndColor()) == 1)
        rgba(animation:GetStartColor(), 1, 0.2, 0, 0.25)
        rgba(animation:GetEndColor(), 0, 0.6, 1, 0.75)
        start.r = 0.3
        animation:GetEndColor().b = 0.1
        rgba(animation:GetStartColor(), 1, 0.2, 0, 0.25)
        rgba(animation:GetEndColor(), 0, 0.6, 1, 0.75)
        vertex(texture, 0.2, 0.3, 0.4, 0.5)
        "#,
    )
    .unwrap();
}

#[test]
fn vertex_color_animation_interpolates_all_channels_and_restores_on_finish() {
    let env = setup();
    configure(&env);
    env.exec("group:Play(); vertex(texture, 1, 0.2, 0, 0.25)")
        .unwrap();
    env.fire_on_update(0.5).unwrap();
    env.exec("vertex(texture, 0.75, 0.3, 0.25, 0.375)").unwrap();
    env.fire_on_update(0.5).unwrap();
    env.exec("vertex(texture, 0.5, 0.4, 0.5, 0.5)").unwrap();
    env.fire_on_update(1.0).unwrap();
    env.exec("assert(group:IsDone()); vertex(texture, 0.2, 0.3, 0.4, 0.5)")
        .unwrap();
}

#[test]
fn vertex_color_animation_source_bounce_and_reverse_use_owner_timeline() {
    let env = setup();
    env.exec(
        r#"
        animation:SetStartColor(CreateColor(1, 0, 0))
        animation:SetEndColor(CreateColor(0, 0, 1))
        animation:SetDuration(2)
        group:SetLooping("BOUNCE")
        group:Play()
        "#,
    )
    .unwrap();
    for (elapsed, red, blue) in [
        (1.0, 0.5, 0.5),
        (1.0, 0.0, 1.0),
        (0.5, 0.25, 0.75),
        (1.5, 1.0, 0.0),
        (0.5, 0.75, 0.25),
    ] {
        env.fire_on_update(elapsed).unwrap();
        env.exec(&format!(
            "vertex(texture, {red}, 0, {blue}, 1); assert(group:IsPlaying())"
        ))
        .unwrap();
    }
    env.exec(
        "group:Stop(); group:SetLooping('NONE'); group:Play(true); vertex(texture, 0, 0, 1, 1)",
    )
    .unwrap();
    env.fire_on_update(0.5).unwrap();
    env.exec("vertex(texture, 0.25, 0, 0.75, 1)").unwrap();
}

#[test]
fn vertex_color_animation_smoothing_and_delays_follow_modeled_time_progress() {
    let env = setup();
    configure(&env);
    env.exec("animation:SetStartDelay(0.5); animation:SetEndDelay(0.5); animation:SetSmoothing('IN'); group:Play()").unwrap();
    env.fire_on_update(0.5).unwrap();
    env.exec("vertex(texture, 1, 0.2, 0, 0.25)").unwrap();
    env.fire_on_update(0.5).unwrap();
    env.exec("vertex(texture, 0.9375, 0.225, 0.0625, 0.28125)")
        .unwrap();
    env.fire_on_update(1.5).unwrap();
    env.exec("vertex(texture, 0, 0.6, 1, 0.75)").unwrap();
    for (mode, progress) in [("NONE", 0.25), ("OUT", 0.4375), ("IN_OUT", 0.125)] {
        env.exec(&format!("group:Stop(); animation:SetStartDelay(0); animation:SetEndDelay(0); animation:SetSmoothing('{mode}'); group:Play()")).unwrap();
        env.fire_on_update(0.5).unwrap();
        env.exec(&format!(
            "vertex(texture, {}, {}, {}, {})",
            1.0 - progress,
            0.2 + 0.4 * progress,
            progress,
            0.25 + 0.5 * progress
        ))
        .unwrap();
    }
}

#[test]
fn vertex_color_animation_child_target_is_isolated_and_restored_on_stop_hide_and_replay() {
    let env = setup();
    env.exec(
        r#"
        owner = CreateFrame('Frame', nil, UIParent)
        owner.Icon = owner:CreateTexture()
        owner.Other = owner:CreateTexture()
        owner.Icon:SetVertexColor(0.2, 0.3, 0.4, 0.5)
        group = owner:CreateAnimationGroup()
        animation = group:CreateAnimation('VertexColor')
        animation:SetChildKey('Icon')
        assert(animation:GetTarget() == owner.Icon)
        "#,
    )
    .unwrap();
    configure(&env);
    env.exec("group:Play()").unwrap();
    env.fire_on_update(1.0).unwrap();
    env.exec(
        "vertex(owner.Icon, 0.5, 0.4, 0.5, 0.5); vertex(owner.Other, 1, 1, 1, 1); group:Pause()",
    )
    .unwrap();
    env.fire_on_update(0.5).unwrap();
    env.exec("vertex(owner.Icon, 0.5, 0.4, 0.5, 0.5); group:Play()")
        .unwrap();
    env.fire_on_update(0.5).unwrap();
    env.exec("vertex(owner.Icon, 0.25, 0.5, 0.75, 0.625); group:Stop(); vertex(owner.Icon, 0.2, 0.3, 0.4, 0.5)").unwrap();
    env.exec("owner.Icon:SetVertexColor(0.8, 0.7, 0.6, 0.9); group:Play()")
        .unwrap();
    env.fire_on_update(0.5).unwrap();
    env.exec("owner:Hide(); assert(not group:IsPlaying()); vertex(owner.Icon, 0.8, 0.7, 0.6, 0.9)")
        .unwrap();
    env.exec("owner:Show(); group:Play(); group:SetToFinalAlpha(true); group:Finish()")
        .unwrap();
    env.fire_on_update(0.0).unwrap();
    env.exec("assert(group:IsDone()); vertex(owner.Icon, 0.8, 0.7, 0.6, 0.9)")
        .unwrap();
}

#[test]
fn vertex_color_animation_xml_matches_lua_source_example() {
    let env = setup();
    let addon = tempfile::tempdir().unwrap();
    std::fs::write(
        addon.path().join("VertexColorFixture.toc"),
        "## Title: VertexColorFixture\nVertexColorFixture.xml\n",
    )
    .unwrap();
    std::fs::write(
        addon.path().join("VertexColorFixture.xml"),
        r#"<Ui>
        <Frame name="VertexColorFixture" parent="UIParent">
            <Layers><Layer><Texture parentKey="Icon">
                <Animations><AnimationGroup parentKey="Colors" looping="BOUNCE">
                    <VertexColor parentKey="Color" duration="2">
                        <StartColor r="1" g="0" b="0"/>
                        <EndColor r="0" g="0" b="1"/>
                    </VertexColor>
                </AnimationGroup></Animations>
            </Texture></Layer></Layers>
        </Frame>
    </Ui>"#,
    )
    .unwrap();
    load_addon(
        &env.loader_env(),
        &addon.path().join("VertexColorFixture.toc"),
    )
    .unwrap();
    env.exec(
        r#"
        animation:SetStartColor(CreateColor(1, 0, 0))
        animation:SetEndColor(CreateColor(0, 0, 1))
        animation:SetDuration(2)
        group:SetLooping('BOUNCE')
        local xml = VertexColorFixture.Icon.Colors.Color
        rgba(xml:GetStartColor(), 1, 0, 0, 1)
        rgba(xml:GetEndColor(), 0, 0, 1, 1)
        VertexColorFixture.Icon.Colors:Play(); group:Play()
    "#,
    )
    .unwrap();
    for delta in [0.5, 1.5, 0.5] {
        env.fire_on_update(delta).unwrap();
        env.exec("local r, g, b, a = texture:GetVertexColor(); vertex(VertexColorFixture.Icon, r, g, b, a)").unwrap();
    }
}
