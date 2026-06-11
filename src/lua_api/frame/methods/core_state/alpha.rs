//! Alpha methods: SetAlpha, GetAlpha, GetEffectiveAlpha, SetAlphaFromBoolean,
//! SetIgnoreParentAlpha, GetIgnoreParentAlpha, IsIgnoringParentAlpha.

use super::helpers::{arg_bool, frame_id, opt_f32};
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn set_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let alpha = opt_f32(state, 2);
    let clamped = if (0.0..=1.0).contains(&alpha) {
        alpha
    } else {
        alpha.clamp(0.0, 1.0)
    };
    {
        let sim = borrow_state(state)?;
        let Some(frame) = sim.widgets.get(id) else {
            return Ok(0);
        };
        if frame.alpha == clamped {
            return Ok(0);
        }
    }

    let mut sim = borrow_state_mut(state)?;
    let Some(frame) = sim.widgets.get(id) else {
        return Ok(0);
    };
    if frame.alpha == clamped {
        return Ok(0);
    }

    let parent_eff = parent_effective_alpha(&sim, id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.alpha = clamped;
    }
    sim.widgets.propagate_effective_alpha(id, parent_eff);
    Ok(0)
}

pub fn get_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.alpha).unwrap_or(1.0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn get_effective_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.effective_alpha)
        .unwrap_or(1.0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn set_alpha_from_boolean(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let flag = arg_bool(state, 2);
    let alpha_if_true = opt_f32_or(state, 3, 1.0);
    let alpha_if_false = opt_f32_or(state, 4, 0.0);
    let new_alpha = if flag { alpha_if_true } else { alpha_if_false }.clamp(0.0, 1.0);
    let mut sim = borrow_state_mut(state)?;
    let changed = sim
        .widgets
        .get(id)
        .map(|f| f.alpha != new_alpha)
        .unwrap_or(false);
    if !changed {
        return Ok(0);
    }
    let parent_eff = parent_effective_alpha(&sim, id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.alpha = new_alpha;
    }
    sim.widgets.propagate_effective_alpha(id, parent_eff);
    Ok(0)
}

fn opt_f32_or(state: &LuaState, index: i32, default: f32) -> f32 {
    match crate::lua_bridge::stack_val(state, index) {
        Val::Num(n) => n as f32,
        _ => default,
    }
}

pub fn set_ignore_parent_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    let parent_eff = parent_effective_alpha(&sim, id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.ignore_parent_alpha = ignore;
    }
    sim.widgets.propagate_effective_alpha(id, parent_eff);
    Ok(0)
}

pub fn get_ignore_parent_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.ignore_parent_alpha)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_ignoring_parent_alpha(state: &mut LuaState) -> LuaResult<u32> {
    get_ignore_parent_alpha(state)
}

fn parent_effective_alpha(sim: &crate::lua_api::state::SimState, id: u64) -> f32 {
    sim.widgets
        .get(id)
        .and_then(|f| f.parent_id)
        .and_then(|pid| sim.widgets.get(pid))
        .map(|p| p.effective_alpha)
        .unwrap_or(1.0)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn set_alpha_to_zero_preserves_strata_buckets() {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local texture = UIParent:CreateTexture("AlphaBucketTexture")
            texture:SetSize(10, 10)
            texture:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
            texture:SetTexture("Interface/Buttons/UI-Panel-Button-Up")
            "#,
        )
        .unwrap();

        build_strata_buckets(&env);
        env.exec(r#"AlphaBucketTexture:SetAlpha(0)"#).unwrap();

        assert!(
            env.state().borrow().strata_buckets.is_some(),
            "SetAlpha crossing to zero should not invalidate strata buckets",
        );
    }

    #[test]
    fn set_alpha_from_zero_preserves_strata_buckets() {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local texture = UIParent:CreateTexture("AlphaBucketRestoreTexture")
            texture:SetSize(10, 10)
            texture:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 0, 0)
            texture:SetTexture("Interface/Buttons/UI-Panel-Button-Up")
            texture:SetAlpha(0)
            "#,
        )
        .unwrap();

        build_strata_buckets(&env);
        env.exec(r#"AlphaBucketRestoreTexture:SetAlpha(1)"#)
            .unwrap();

        assert!(
            env.state().borrow().strata_buckets.is_some(),
            "SetAlpha crossing from zero should not invalidate strata buckets",
        );
    }

    #[test]
    fn set_alpha_from_boolean_uses_supplied_alpha_values() {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "AlphaBoolCustomFrame", UIParent)
            frame:SetAlphaFromBoolean(true, 0.5, 0.25)
            "#,
        )
        .unwrap();

        let true_alpha: f64 = env.eval("return AlphaBoolCustomFrame:GetAlpha()").unwrap();
        assert!(
            (true_alpha - 0.5).abs() < 0.001,
            "true branch should use alphaIfTrue"
        );

        env.exec(r#"AlphaBoolCustomFrame:SetAlphaFromBoolean(false, 0.5, 0.25)"#)
            .unwrap();
        let false_alpha: f64 = env.eval("return AlphaBoolCustomFrame:GetAlpha()").unwrap();
        assert!(
            (false_alpha - 0.25).abs() < 0.001,
            "false branch should use alphaIfFalse"
        );
    }

    fn build_strata_buckets(env: &WowLuaEnv) {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        assert!(
            state.strata_buckets.is_some(),
            "test setup should build cached strata buckets",
        );
    }
}
