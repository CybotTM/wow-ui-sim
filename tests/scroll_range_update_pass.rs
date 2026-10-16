//! ScrollFrame range refresh at the real frame update boundary.
//! User-supplied historical ordering, not modern native timing proof.

use wow_ui_sim::lua_api::WowLuaEnv;

fn range_update_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(800.0, 600.0);
    env.exec(
        r#"
        C9Viewport = CreateFrame('ScrollFrame', nil, UIParent)
        C9Viewport:SetSize(100, 100)
        C9Viewport:SetPoint('TOPLEFT', UIParent, 'TOPLEFT', 20, -20)
        C9Viewport:Show()
        C9Child = CreateFrame('Frame', nil, C9Viewport)
        C9Child:SetSize(100, 100)
        C9Child:SetPoint('TOPLEFT', C9Viewport, 'TOPLEFT')
        C9Viewport:SetScrollChild(C9Child)
        C9Content = CreateFrame('Frame', nil, C9Child)
        C9Content:SetSize(100, 100)
        C9Content:SetPoint('TOPLEFT', C9Child, 'TOPLEFT')
        C9Viewport:UpdateScrollChildRect()
        "#,
    )
    .unwrap();
    // Establish a committed baseline through the real environment update seam.
    env.fire_on_update(0.016).unwrap();
    assert_eq!(ranges(&env), (0.0, 0.0));
    env.exec(
        r#"
        C9Trace = {}
        C9Viewport:SetScript('OnScrollRangeChanged', function(self, ...)
            -- Record entry before any protected assertion can abort this handler.
            C9Trace[#C9Trace + 1] = 'range-enter'
            local x, y = ...
            C9Trace[#C9Trace + 1] = 'range:' .. tostring(x) .. ':' .. tostring(y)
            assert(self == C9Viewport)
            assert(select('#', ...) == 2)
            assert(x == 80 and y == 120)
            assert(self:GetHorizontalScrollRange() == x)
            assert(self:GetVerticalScrollRange() == y)
            assert(self:GetScrollChild() == C9Child)
            assert(C9Child:GetParent() == self and C9Content:GetParent() == C9Child)
            C9Trace[#C9Trace + 1] = 'range-validated'
        end)
        "#,
    )
    .unwrap();
    assert_no_lua_errors(&env);
    env
}

fn ranges(env: &WowLuaEnv) -> (f64, f64) {
    env.eval("return C9Viewport:GetHorizontalScrollRange(), C9Viewport:GetVerticalScrollRange()")
        .unwrap()
}

fn assert_trace(env: &WowLuaEnv, expected: &str) {
    let actual: String = env.eval("return table.concat(C9Trace, '|')").unwrap();
    assert_eq!(actual, expected);
}

fn assert_no_lua_errors(env: &WowLuaEnv) {
    let state = env.state();
    let state = state.borrow();
    assert!(state.lua_errors.is_empty(), "{:?}", state.lua_errors);
}

fn assert_fixture_preserved(env: &WowLuaEnv) {
    // Outside protected callbacks: ownership and logical layout are controls,
    // not permission to change the existing presentation/range arithmetic.
    env.exec(
        r#"
        assert(C9Viewport:IsVisible())
        assert(C9Viewport:GetWidth() == 100 and C9Viewport:GetHeight() == 100)
        assert(C9Child:GetWidth() == 100 and C9Child:GetHeight() == 100)
        assert(C9Content:GetWidth() == 180 and C9Content:GetHeight() == 220)
        assert(C9Viewport:GetScrollChild() == C9Child)
        assert(C9Child:GetParent() == C9Viewport and C9Content:GetParent() == C9Child)
        local point, relative, relativePoint, x, y = C9Child:GetPoint(1)
        assert(point == 'TOPLEFT' and relative == C9Viewport and relativePoint == 'TOPLEFT')
        assert(x == 0 and y == 0)
        assert(C9Viewport:GetHorizontalScroll() == 0 and C9Viewport:GetVerticalScroll() == 0)
        "#,
    )
    .unwrap();
}

#[test]
fn c9_outside_request_returns_before_range_update_and_unchanged_request_is_silent() {
    let env = range_update_env();
    env.exec(
        r#"
        C9Viewport:SetScript('OnUpdate', function(self, elapsed)
            C9Trace[#C9Trace + 1] = 'update-enter'
            assert(self == C9Viewport and elapsed == 0.016)
            C9Trace[#C9Trace + 1] = 'update-end'
        end)
        -- Resize a descendant, retaining the designated child's 100x100 size.
        C9Content:SetSize(180, 220)
        C9Trace[#C9Trace + 1] = 'outside-request'
        C9Viewport:UpdateScrollChildRect()
        C9Trace[#C9Trace + 1] = 'outside-return'
        "#,
    )
    .unwrap();
    assert_trace(&env, "outside-request|outside-return");
    assert_eq!(ranges(&env), (0.0, 0.0));

    env.fire_on_update(0.016).unwrap();
    assert_trace(
        &env,
        "outside-request|outside-return|update-enter|update-end|range-enter|range:80:120|range-validated",
    );
    assert_eq!(ranges(&env), (80.0, 120.0));
    assert_fixture_preserved(&env);
    assert_no_lua_errors(&env);

    env.exec(
        r#"
        C9Trace = {}
        C9Trace[#C9Trace + 1] = 'outside-request'
        C9Viewport:UpdateScrollChildRect()
        C9Trace[#C9Trace + 1] = 'outside-return'
        "#,
    )
    .unwrap();
    assert_trace(&env, "outside-request|outside-return");
    env.fire_on_update(0.016).unwrap();
    assert_trace(
        &env,
        "outside-request|outside-return|update-enter|update-end",
    );
    assert_eq!(ranges(&env), (80.0, 120.0));
    assert_fixture_preserved(&env);
    assert_no_lua_errors(&env);
}

#[test]
fn c9_on_update_request_notifies_after_callback_in_that_same_frame_only_if_changed() {
    let env = range_update_env();
    env.exec(
        r#"
        C9Viewport:SetScript('OnUpdate', function(self, elapsed)
            C9Trace[#C9Trace + 1] = 'update-enter'
            assert(self == C9Viewport and elapsed == 0.016)
            C9Content:SetSize(180, 220)
            local oldX, oldY = self:GetHorizontalScrollRange(), self:GetVerticalScrollRange()
            C9Trace[#C9Trace + 1] = 'inside-request'
            self:UpdateScrollChildRect()
            C9Trace[#C9Trace + 1] = 'inside-return'
            assert(self:GetHorizontalScrollRange() == oldX)
            assert(self:GetVerticalScrollRange() == oldY)
            C9Trace[#C9Trace + 1] = 'update-end'
        end)
        "#,
    )
    .unwrap();
    assert_trace(&env, "");
    assert_eq!(ranges(&env), (0.0, 0.0));

    // Exactly one real update invocation: no extra tick or timer drain allowed.
    env.fire_on_update(0.016).unwrap();
    assert_trace(
        &env,
        "update-enter|inside-request|inside-return|update-end|range-enter|range:80:120|range-validated",
    );
    assert_eq!(ranges(&env), (80.0, 120.0));
    assert_fixture_preserved(&env);
    assert_no_lua_errors(&env);

    env.exec("C9Trace = {}").unwrap();
    // The same OnUpdate requests again with unchanged content this frame.
    env.fire_on_update(0.016).unwrap();
    assert_trace(&env, "update-enter|inside-request|inside-return|update-end");
    assert_eq!(ranges(&env), (80.0, 120.0));
    assert_fixture_preserved(&env);
    assert_no_lua_errors(&env);
}

#[test]
fn c9_animation_request_is_processed_after_animation_in_the_same_frame() {
    let env = range_update_env();
    env.exec(
        r#"
        C9Viewport:SetScript('OnUpdate', function()
            C9Trace[#C9Trace + 1] = 'update'
        end)
        C9AnimationGroup = C9Viewport:CreateAnimationGroup()
        local animation = C9AnimationGroup:CreateAnimation('Alpha')
        animation:SetFromAlpha(1)
        animation:SetToAlpha(1)
        animation:SetDuration(0.25)
        C9AnimationGroup:SetScript('OnFinished', function(self)
            C9Trace[#C9Trace + 1] = 'animation-enter'
            assert(self == C9AnimationGroup)
            C9Content:SetSize(180, 220)
            C9Viewport:UpdateScrollChildRect()
            C9Trace[#C9Trace + 1] = 'animation-return'
            assert(C9Viewport:GetHorizontalScrollRange() == 0)
            assert(C9Viewport:GetVerticalScrollRange() == 0)
            C9Trace[#C9Trace + 1] = 'animation-end'
        end)
        C9AnimationGroup:Play()
        "#,
    )
    .unwrap();
    assert_trace(&env, "");
    env.fire_on_update(0.25).unwrap();
    assert_trace(
        &env,
        "update|animation-enter|animation-return|animation-end|range-enter|range:80:120|range-validated",
    );
    assert_eq!(ranges(&env), (80.0, 120.0));
    assert_fixture_preserved(&env);
    assert_no_lua_errors(&env);
}
