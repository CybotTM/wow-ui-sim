//! Tests for AnimationGroup state/getter helpers beyond the core lifecycle.

use wow_ui_sim::lua_api::WowLuaEnv;

fn setup() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn play_reverse() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimReverse", UIParent)
        local ag = f:CreateAnimationGroup()
        ag:Play(true)
        assert(ag:IsReverse() == true, "Should be reverse after Play(true)")
    "#,
    )
    .unwrap();
}

#[test]
fn is_pending_finish_after_finish() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimPending", UIParent)
        local ag = f:CreateAnimationGroup()
        assert(ag:IsPendingFinish() == false, "Not pending initially")
        ag:Play()
        ag:Finish()
        assert(ag:IsPendingFinish() == true, "Pending finish after Finish()")
    "#,
    )
    .unwrap();
}

#[test]
fn loop_state_matches_looping() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimLoopState", UIParent)
        local ag = f:CreateAnimationGroup()
        ag:SetLooping("BOUNCE")
        assert(ag:GetLoopState() == "BOUNCE", "GetLoopState should match SetLooping")
    "#,
    )
    .unwrap();
}

#[test]
fn elapsed_increases_after_tick() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimElapsed", UIParent)
        _G.testAG = f:CreateAnimationGroup()
        local anim = _G.testAG:CreateAnimation("Alpha")
        anim:SetDuration(1.0)
        _G.testAG:Play()
    "#,
    )
    .unwrap();

    env.fire_on_update(0.3).unwrap();

    let elapsed: f64 = env.eval("return _G.testAG:GetElapsed()").unwrap();
    assert!(
        (elapsed - 0.3).abs() < 0.05,
        "GetElapsed should be ~0.3, got {}",
        elapsed
    );
}

#[test]
fn progress_is_halfway_after_half_tick() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimProgress", UIParent)
        _G.testAGP = f:CreateAnimationGroup()
        local anim = _G.testAGP:CreateAnimation("Alpha")
        anim:SetDuration(1.0)
        _G.testAGP:Play()
    "#,
    )
    .unwrap();

    env.fire_on_update(0.5).unwrap();

    let progress: f64 = env.eval("return _G.testAGP:GetProgress()").unwrap();
    assert!(
        (progress - 0.5).abs() < 0.05,
        "GetProgress should be ~0.5, got {}",
        progress
    );
}

#[test]
fn to_final_alpha_getter_matches_setter() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimGTFA", UIParent)
        local ag = f:CreateAnimationGroup()
        ag:SetToFinalAlpha(true)
        assert(ag:GetToFinalAlpha() == true, "GetToFinalAlpha should return true")
    "#,
    )
    .unwrap();
}

#[test]
fn group_get_alpha_stub() {
    let env = setup();
    let alpha: f64 = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestAnimGA", UIParent)
        local ag = f:CreateAnimationGroup()
        return ag:GetAlpha()
    "#,
        )
        .unwrap();
    assert_eq!(alpha, 1.0, "GetAlpha stub should return 1.0");
}

#[test]
fn hook_script_stores_handler() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimHook", UIParent)
        local ag = f:CreateAnimationGroup()
        ag:HookScript("OnFinished", function() end)
        assert(ag:HasScript("OnFinished") == true, "HookScript should register handler")
    "#,
    )
    .unwrap();
}

#[test]
fn play_synced_no_error() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimSync", UIParent)
        local ag = f:CreateAnimationGroup()
        ag:PlaySynced()
    "#,
    )
    .unwrap();
}

#[test]
fn remove_animations_clears_list() {
    let env = setup();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAnimRemove", UIParent)
        local ag = f:CreateAnimationGroup()
        ag:CreateAnimation("Alpha")
        ag:CreateAnimation("Translation")
        local a1, a2 = ag:GetAnimations()
        assert(a1 ~= nil and a2 ~= nil, "Should have 2 animations")
        ag:RemoveAnimations()
        local b1 = ag:GetAnimations()
        assert(b1 == nil, "Should have no animations after RemoveAnimations")
    "#,
    )
    .unwrap();
}

#[test]
fn script_lookup_returns_function() {
    let env = setup();
    let is_func: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestAnimGetScript", UIParent)
        local ag = f:CreateAnimationGroup()
        ag:SetScript("OnPlay", function() end)
        return type(ag:GetScript("OnPlay")) == "function"
    "#,
        )
        .unwrap();
    assert!(is_func, "GetScript should return the function");
}

#[test]
fn script_lookup_is_nil_when_absent() {
    let env = setup();
    let is_nil: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame", "TestAnimGetScriptNil", UIParent)
        local ag = f:CreateAnimationGroup()
        return ag:GetScript("OnPlay") == nil
    "#,
        )
        .unwrap();
    assert!(is_nil, "GetScript should return nil for unset handlers");
}
