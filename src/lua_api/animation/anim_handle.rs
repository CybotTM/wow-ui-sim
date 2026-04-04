//! AnimHandle userdata methods.

use crate::lua_api::SimState;
use crate::lua_api::frame::frame_ref;
use mlua::{MultiValue, UserData, UserDataMethods, Value};
use std::cell::RefCell;
use std::rc::Rc;

use super::{AnimGroupHandle, AnimationType, Smoothing, extract_number};

/// Userdata handle for an individual Animation.
#[derive(Clone)]
pub struct AnimHandle {
    pub group_id: u64,
    pub anim_index: usize,
    pub state: Rc<RefCell<SimState>>,
}

impl AnimHandle {
    /// Register duration methods: SetDuration, GetDuration.
    fn add_duration_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetDuration", |_, this, dur: f64| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.duration = dur;
            }
            Ok(())
        });

        methods.add_method("GetDuration", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(0.0, |a| a.duration))
        });
    }

    /// Register start/end delay methods.
    fn add_start_end_delay_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetStartDelay", |_, this, delay: f64| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.start_delay = delay;
            }
            Ok(())
        });

        methods.add_method("GetStartDelay", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(0.0, |a| a.start_delay))
        });

        methods.add_method("SetEndDelay", |_, this, delay: f64| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.end_delay = delay;
            }
            Ok(())
        });

        methods.add_method("GetEndDelay", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(0.0, |a| a.end_delay))
        });
    }

    /// Register order methods: SetOrder, GetOrder.
    fn add_order_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetOrder", |_, this, order: u32| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.order = order;
            }
            Ok(())
        });

        methods.add_method("GetOrder", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(1_u32, |a| a.order))
        });
    }

    /// Register delay and order methods.
    fn add_delay_and_order_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_start_end_delay_methods(methods);
        Self::add_order_methods(methods);
    }

    /// Register all property methods by delegating to sub-helpers.
    fn add_property_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_smoothing_methods(methods);
        Self::add_alpha_property_methods(methods);
        Self::add_translation_methods(methods);
        Self::add_scale_methods(methods);
        Self::add_rotation_methods(methods);
        Self::add_flipbook_methods(methods);
    }

    /// Register smoothing methods: SetSmoothing, GetSmoothing.
    fn add_smoothing_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetSmoothing", |_, this, smooth: String| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.smoothing = Smoothing::from_str(&smooth);
            }
            Ok(())
        });

        methods.add_method("GetSmoothing", |lua, this, ()| {
            let state = this.state.borrow();
            let s = state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or("NONE", |a| a.smoothing.as_str());
            Ok(Value::String(lua.create_string(s)?))
        });
    }

    /// Register alpha property methods.
    fn add_alpha_property_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetFromAlpha", |_, this, alpha: f64| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.from_alpha = alpha;
            }
            Ok(())
        });

        methods.add_method("GetFromAlpha", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(0.0, |a| a.from_alpha))
        });

        methods.add_method("SetToAlpha", |_, this, alpha: f64| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.to_alpha = alpha;
            }
            Ok(())
        });

        methods.add_method("GetToAlpha", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(1.0, |a| a.to_alpha))
        });
    }

    /// Register translation methods: SetOffset, SetChange.
    fn add_translation_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetOffset", |_, this, args: MultiValue| {
            let args: Vec<Value> = args.into_iter().collect();
            let x = extract_number(&args, 0).unwrap_or(0.0);
            let y = extract_number(&args, 1).unwrap_or(0.0);
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.offset_x = x;
                anim.offset_y = y;
            }
            Ok(())
        });

        methods.add_method("SetChange", |_, this, args: MultiValue| {
            let args: Vec<Value> = args.into_iter().collect();
            let val = extract_number(&args, 0).unwrap_or(0.0);
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
                && anim.anim_type == AnimationType::Alpha
            {
                anim.to_alpha = anim.from_alpha + val;
            }
            Ok(())
        });
    }

    /// Register scale methods: SetScale, SetScaleFrom, SetScaleTo.
    fn add_scale_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetScale", |_, this, args: MultiValue| {
            this.update_scale_values(args, |anim, x, y| {
                anim.scale_x = x;
                anim.scale_y = y;
            });
            Ok(())
        });

        methods.add_method("SetScaleFrom", |_, this, args: MultiValue| {
            this.update_scale_values(args, |anim, x, y| {
                anim.from_scale_x = x;
                anim.from_scale_y = y;
            });
            Ok(())
        });

        methods.add_method("SetScaleTo", |_, this, args: MultiValue| {
            this.update_scale_values(args, |anim, x, y| {
                anim.to_scale_x = x;
                anim.to_scale_y = y;
            });
            Ok(())
        });
    }

    fn update_scale_values<F>(&self, args: MultiValue, mut apply: F)
    where
        F: FnMut(&mut super::AnimState, f64, f64),
    {
        let args: Vec<Value> = args.into_iter().collect();
        let x = extract_number(&args, 0).unwrap_or(1.0);
        let y = extract_number(&args, 1).unwrap_or(1.0);
        let mut state = self.state.borrow_mut();
        if let Some(group) = state.animation_groups.get_mut(&self.group_id)
            && let Some(anim) = group.animations.get_mut(self.anim_index)
        {
            apply(anim, x, y);
        }
    }

    /// Parse origin args from a Lua MultiValue into (point, offset_x, offset_y).
    fn parse_origin_args(args: MultiValue) -> (String, f64, f64) {
        let mut iter = args.into_iter();
        let point = match iter.next() {
            Some(Value::String(s)) => s.to_string_lossy().to_string(),
            _ => "CENTER".to_string(),
        };
        let offset_x = match iter.next() {
            Some(Value::Number(n)) => n,
            Some(Value::Integer(n)) => n as f64,
            _ => 0.0,
        };
        let offset_y = match iter.next() {
            Some(Value::Number(n)) => n,
            Some(Value::Integer(n)) => n as f64,
            _ => 0.0,
        };
        (point, offset_x, offset_y)
    }

    /// Register SetDegrees and SetOrigin rotation methods.
    fn add_set_rotation_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetDegrees", |_, this, degrees: f64| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.degrees = degrees;
            }
            Ok(())
        });

        methods.add_method("SetOrigin", |_, this, args: MultiValue| {
            let (point, offset_x, offset_y) = Self::parse_origin_args(args);
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.origin_point = point;
                anim.origin_offset_x = offset_x;
                anim.origin_offset_y = offset_y;
            }
            Ok(())
        });
    }

    /// Register rotation methods: SetDegrees, SetOrigin, GetOrigin.
    fn add_rotation_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_set_rotation_methods(methods);

        methods.add_method("GetOrigin", |lua, this, _: ()| {
            let state = this.state.borrow();
            if let Some(group) = state.animation_groups.get(&this.group_id)
                && let Some(anim) = group.animations.get(this.anim_index)
            {
                Ok(MultiValue::from_vec(vec![
                    Value::String(lua.create_string(&anim.origin_point)?),
                    Value::Number(anim.origin_offset_x),
                    Value::Number(anim.origin_offset_y),
                ]))
            } else {
                Ok(MultiValue::from_vec(vec![
                    Value::String(lua.create_string("CENTER")?),
                    Value::Number(0.0),
                    Value::Number(0.0),
                ]))
            }
        });
    }

    /// Register flipbook rows and columns methods.
    fn add_flipbook_rows_cols_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetFlipBookRows", |_, this, rows: u32| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.flip_book_rows = rows;
            }
            Ok(())
        });

        methods.add_method("GetFlipBookRows", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(1_u32, |a| a.flip_book_rows))
        });

        methods.add_method("SetFlipBookColumns", |_, this, cols: u32| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.flip_book_columns = cols;
            }
            Ok(())
        });

        methods.add_method("GetFlipBookColumns", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(1_u32, |a| a.flip_book_columns))
        });
    }

    /// Register flipbook frames methods: SetFlipBookFrames, GetFlipBookFrames.
    fn add_flipbook_frames_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetFlipBookFrames", |_, this, frames: u32| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.flip_book_frames = frames;
            }
            Ok(())
        });

        methods.add_method("GetFlipBookFrames", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(1_u32, |a| a.flip_book_frames))
        });
    }

    /// Register flipbook property methods.
    fn add_flipbook_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_flipbook_rows_cols_methods(methods);
        Self::add_flipbook_frames_methods(methods);
    }

    /// Register playback control stubs and state queries.
    fn add_playback_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_playback_stub_methods(methods);

        methods.add_method("IsPlaying", |_, this, ()| {
            Ok(this.group_state_flag(|g| Some(g.playing)).unwrap_or(false))
        });

        methods.add_method("IsPaused", |_, this, ()| {
            Ok(this.group_state_flag(|g| Some(g.paused)).unwrap_or(false))
        });

        methods.add_method("IsDone", |_, this, ()| {
            Ok(this.group_state_flag(|g| Some(g.done)).unwrap_or(true))
        });

        methods.add_method("IsStopped", |_, this, ()| {
            Ok(this
                .group_state_flag(|g| Some(!g.playing && !g.paused))
                .unwrap_or(true))
        });

        methods.add_method("IsDelaying", |_, _this, ()| Ok(false));
    }

    fn add_playback_stub_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        for name in ["Play", "Stop", "Pause", "Restart", "Finish"] {
            methods.add_method(name, |_, _this, ()| Ok(()));
        }
    }

    fn group_state_flag<F>(&self, read: F) -> Option<bool>
    where
        F: FnOnce(&super::AnimGroupState) -> Option<bool>,
    {
        let state = self.state.borrow();
        state.animation_groups.get(&self.group_id).and_then(read)
    }

    /// Register progress query methods.
    fn add_progress_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetProgress", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(0.0, |a| a.raw_progress()))
        });

        methods.add_method("GetSmoothProgress", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(0.0, |a| a.smooth_progress()))
        });

        methods.add_method("GetElapsed", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map_or(0.0, |a| a.elapsed))
        });
    }

    /// Register parent and name accessor methods.
    fn add_parent_name_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetParent", |lua, this, ()| {
            let handle = AnimGroupHandle {
                group_id: this.group_id,
                state: Rc::clone(&this.state),
            };
            lua.create_userdata(handle)
        });

        methods.add_method("GetRegionParent", |lua, this, ()| {
            let owner_id = this
                .state
                .borrow()
                .animation_groups
                .get(&this.group_id)
                .map(|g| g.owner_frame_id);
            match owner_id {
                Some(id) => frame_ref(lua, id),
                None => Ok(Value::Nil),
            }
        });

        methods.add_method("GetName", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .and_then(|a| a.name.clone()))
        });
    }

    /// Register target accessor and key methods.
    fn add_target_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetTarget", |lua, this, ()| {
            let state = this.state.borrow();
            let Some(group) = state.animation_groups.get(&this.group_id) else {
                return Ok(Value::Nil);
            };
            let owner_id = group.owner_frame_id;
            let child_key = group
                .animations
                .get(this.anim_index)
                .and_then(|a| a.child_key.clone());
            let target_id = match &child_key {
                Some(key) => state
                    .widgets
                    .get(owner_id)
                    .and_then(|owner| owner.children_keys.get(key.as_str()).copied()),
                None => Some(owner_id),
            };
            let Some(id) = target_id else {
                return Ok(Value::Nil);
            };
            drop(state);
            frame_ref(lua, id)
        });

        methods.add_method("SetTarget", |_, _this, target: Value| {
            if target.is_nil() || target == Value::NULL {
                return Err(mlua::Error::RuntimeError(
                    "Usage: Animation:SetTarget(target)".into(),
                ));
            }
            Ok(())
        });

        methods.add_method("SetChildKey", |_, this, key: String| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && let Some(anim) = group.animations.get_mut(this.anim_index)
            {
                anim.child_key = Some(key);
            }
            Ok(())
        });

        methods.add_method("SetTargetKey", |_, _this, _key: String| Ok(()));
        methods.add_method("SetTargetName", |_, _this, _name: String| Ok(()));
        methods.add_method("SetTargetParent", |_, _this, ()| Ok(()));
    }

    /// Register parent, name, and target accessor methods.
    fn add_accessor_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_parent_name_methods(methods);
        Self::add_target_methods(methods);
    }

    /// Register SetScript, GetScript, HasScript methods.
    fn add_set_get_script_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "SetScript",
            |lua, this, (event, handler): (String, Option<mlua::Function>)| {
                let mut state = this.state.borrow_mut();
                if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                    && let Some(anim) = group.animations.get_mut(this.anim_index)
                {
                    if let Some(old_key) = anim.scripts.remove(&event) {
                        lua.remove_registry_value(old_key).ok();
                    }
                    if let Some(func) = handler {
                        let key = lua.create_registry_value(func)?;
                        anim.scripts.insert(event, key);
                    }
                }
                Ok(())
            },
        );

        methods.add_method("GetScript", |lua, this, event: String| {
            let state = this.state.borrow();
            if let Some(group) = state.animation_groups.get(&this.group_id)
                && let Some(anim) = group.animations.get(this.anim_index)
                && let Some(key) = anim.scripts.get(&event)
                && let Ok(func) = lua.registry_value::<mlua::Function>(key)
            {
                return Ok(Value::Function(func));
            }
            Ok(Value::Nil)
        });

        methods.add_method("HasScript", |_, this, event: String| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .is_some_and(|a| a.scripts.contains_key(&event)))
        });
    }

    /// Register HookScript method.
    fn add_hook_script_method<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "HookScript",
            |lua, this, (event, handler): (String, Option<mlua::Function>)| {
                let mut state = this.state.borrow_mut();
                if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                    && let Some(anim) = group.animations.get_mut(this.anim_index)
                    && let Some(func) = handler
                {
                    if let Some(old_key) = anim.scripts.remove(&event) {
                        lua.remove_registry_value(old_key).ok();
                    }
                    let key = lua.create_registry_value(func)?;
                    anim.scripts.insert(event, key);
                }
                Ok(())
            },
        );
    }

    /// Register script handler methods.
    fn add_script_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_set_get_script_methods(methods);
        Self::add_hook_script_method(methods);
    }
}

impl UserData for AnimHandle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetObjectType", |_, this, ()| {
            let state = this.state.borrow();
            let anim_type = state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.animations.get(this.anim_index))
                .map(|a| a.anim_type)
                .unwrap_or(AnimationType::Animation);
            Ok(anim_type.as_str())
        });

        Self::add_duration_methods(methods);
        Self::add_delay_and_order_methods(methods);
        Self::add_property_methods(methods);
        Self::add_playback_methods(methods);
        Self::add_progress_methods(methods);
        Self::add_accessor_methods(methods);
        Self::add_script_methods(methods);
    }
}
