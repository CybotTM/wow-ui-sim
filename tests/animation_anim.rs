//! Tests for individual Animation object methods: properties, state queries, targets, scripts.

use wow_ui_sim::lua_api::WowLuaEnv;

fn setup() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// Anim: property methods (SetOffset, SetChange, SetScale, SetDegrees, etc.)
// ============================================================================

#[test]
fn animation_set_offset_no_error() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimOffset", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Translation")
        anim:SetOffset(10, -5)
    "#,
    )
    .unwrap();
}

#[test]
fn animation_set_change_alpha() {
    let env = setup();
    env.exec(r#"
        local f = CreateFrame("Frame", "TestAnimChange", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        anim:SetFromAlpha(0.2)
        anim:SetChange(0.5)
        local to = anim:GetToAlpha()
        assert(math.abs(to - 0.7) < 0.01, "SetChange should set to_alpha = from_alpha + change, got " .. to)
    "#).unwrap();
}

#[test]
fn animation_set_scale_no_error() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimScale", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Scale")
        anim:SetScale(2.0, 3.0)
        anim:SetScaleFrom(0.5, 0.5)
        anim:SetScaleTo(1.5, 1.5)
    "#,
    )
    .unwrap();
}

#[test]
fn animation_set_degrees_no_error() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimDeg", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Rotation")
        anim:SetDegrees(360)
        anim:SetOrigin("CENTER", 0, 0)
    "#,
    )
    .unwrap();
}

#[test]
fn rotation_animation_set_radians_no_error() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimRadians", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Rotation")
        anim:SetRadians(math.pi * 2)
    "#,
    )
    .unwrap();
}

// ============================================================================
// Anim: state query methods (IsStopped, IsDelaying, GetProgress, GetSmoothProgress, GetElapsed)
// ============================================================================

#[test]
fn animation_is_stopped_initially() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimStopped", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        assert(anim:IsStopped() == true, "Should be stopped initially")
    "#,
    )
    .unwrap();
}

#[test]
fn animation_is_delaying_stub() {
    let env = setup();
    let delaying: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestAnimDelaying", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        return anim:IsDelaying()
    "#,
        )
        .unwrap();
    assert!(!delaying, "IsDelaying stub should return false");
}

#[test]
fn animation_get_progress_with_duration() {
    let env = setup();
    let progress: f64 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestAnimProg", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        anim:SetDuration(1.0)
        return anim:GetProgress()
    "#,
        )
        .unwrap();
    assert_eq!(
        progress, 0.0,
        "Progress should be 0.0 at start with duration set"
    );
}

#[test]
fn animation_get_smooth_progress_with_duration() {
    let env = setup();
    let progress: f64 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestAnimSmProg", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        anim:SetDuration(1.0)
        return anim:GetSmoothProgress()
    "#,
        )
        .unwrap();
    assert_eq!(
        progress, 0.0,
        "Smooth progress should be 0.0 at start with duration set"
    );
}

#[test]
fn animation_get_elapsed_default() {
    let env = setup();
    let elapsed: f64 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestAnimElapsed2", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        return anim:GetElapsed()
    "#,
        )
        .unwrap();
    assert_eq!(elapsed, 0.0);
}

// ============================================================================
// Anim: target and accessor methods
// ============================================================================

#[test]
fn animation_set_get_target_no_error() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimTarget", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        anim:SetTarget(f)
        anim:SetChildKey("SomeChild")
        anim:SetTargetKey("SomeKey")
        anim:SetTargetName("SomeName")
        anim:SetTargetParent()
    "#,
    )
    .unwrap();
}

#[test]
fn animation_get_name() {
    let env = setup();
    let name: String = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestAnimName", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha", "FadeIn")
        return anim:GetName()
    "#,
        )
        .unwrap();
    assert_eq!(name, "FadeIn");
}

#[test]
fn animation_playback_stubs_no_error() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimStubs", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        anim:Play()
        anim:Pause()
        anim:Stop()
        anim:Restart()
        anim:Finish()
    "#,
    )
    .unwrap();
}

#[test]
fn animation_pause_uses_group_pause_state() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimPauseAlias", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        ag:Play()
        anim:Pause()
        assert(ag:IsPaused() == true, "animation Pause should pause the owning group")
        assert(ag:IsPlaying() == false, "paused group should stop reporting playing")
    "#,
    )
    .unwrap();
}

// ============================================================================
// Anim: script handlers
// ============================================================================

#[test]
fn animation_set_has_script() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimScript", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        assert(anim:HasScript("OnFinished") == false)
        anim:SetScript("OnFinished", function() end)
        assert(anim:HasScript("OnFinished") == true)
        anim:SetScript("OnFinished", nil)
        assert(anim:HasScript("OnFinished") == false)
    "#,
    )
    .unwrap();
}

#[test]
fn animation_hook_script() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimHookScript", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        anim:HookScript("OnPlay", function() end)
        assert(anim:HasScript("OnPlay") == true)
    "#,
    )
    .unwrap();
}

#[test]
fn animation_delays() {
    let env = setup();
    env.exec(r#"
        local f = CreateFrame("Frame", "TestAnimFrameDelay", UIParent)
        local ag = f:CreateAnimationGroup()
        local anim = ag:CreateAnimation("Alpha")
        anim:SetStartDelay(0.1)
        anim:SetEndDelay(0.2)
        anim:SetDuration(0.5)
        assert(anim:GetStartDelay() == 0.1)
        assert(anim:GetEndDelay() == 0.2)
        -- Total time should contribute to group duration: 0.1 + 0.5 + 0.2 = 0.8
        assert(ag:GetDuration() == 0.8, "Duration with delays should be 0.8, got " .. ag:GetDuration())
    "#).unwrap();
}
