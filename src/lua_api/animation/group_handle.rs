//! AnimGroupHandle userdata methods and metamethods.

use crate::lua_api::SimState;
use crate::lua_api::frame::frame_ref;
use mlua::{Lua, MultiValue, UserData, UserDataMethods, Value};
use std::cell::RefCell;
use std::rc::Rc;

use super::tick::apply_flipbook_for_group;
use super::{AnimGroupState, AnimHandle, AnimState, AnimationType, LoopType};

/// Parse Play/Restart arguments: (reverse: bool, offset: f64).
fn parse_play_args(args: MultiValue) -> (bool, f64) {
    let args: Vec<Value> = args.into_iter().collect();
    let reverse = args
        .first()
        .and_then(|v| {
            if let Value::Boolean(b) = v {
                Some(*b)
            } else {
                None
            }
        })
        .unwrap_or(false);
    let offset = args
        .get(1)
        .and_then(|v| match v {
            Value::Number(n) => Some(*n),
            Value::Integer(n) => Some(*n as f64),
            _ => None,
        })
        .unwrap_or(0.0);
    (reverse, offset)
}

/// Compute the synced time offset for PlaySynced.
///
/// Reads `syncKey` from the animation group's Lua properties, then uses
/// `SimState.anim_sync_times` to track when each key first started.
/// Returns `(now - start) % duration` so all groups with the same key stay in phase.
pub(crate) fn compute_sync_offset(
    lua: &Lua,
    frame_id: u64,
    state_rc: &Rc<RefCell<SimState>>,
) -> mlua::Result<f64> {
    let sync_key = read_sync_key(lua, frame_id)?;
    let state = state_rc.borrow();
    let now = state.start_time.elapsed();
    let duration = state
        .anim_frame_to_group
        .get(&frame_id)
        .and_then(|gid| state.animation_groups.get(gid))
        .map_or(0.0, |g| g.total_duration());
    let sync_start = state.anim_sync_times.get(&sync_key).copied();
    drop(state);

    let offset = match sync_start {
        Some(start) if duration > 0.0 => {
            let elapsed = (now - start).as_secs_f64();
            elapsed % duration
        }
        None => {
            // First PlaySynced for this key — record start time, offset=0.
            state_rc.borrow_mut().anim_sync_times.insert(sync_key, now);
            0.0
        }
        _ => 0.0,
    };
    Ok(offset)
}

/// Read the `syncKey` property from an animation group's Lua properties.
fn read_sync_key(lua: &Lua, frame_id: u64) -> mlua::Result<String> {
    let self_val = frame_ref(lua, frame_id)?;
    let key_val: Value = lua.load("return select(1,...).syncKey").call(self_val)?;
    match key_val {
        Value::String(s) => Ok(s.to_string_lossy().to_string()),
        _ => Ok("DEFAULT".to_string()),
    }
}

/// Resolve a child_key to a frame ID via the owner's children_keys.
fn resolve_child(state: &SimState, owner_id: u64, child_key: &Option<String>) -> Option<u64> {
    match child_key {
        Some(key) => state
            .widgets
            .get(owner_id)
            .and_then(|owner| owner.children_keys.get(key.as_str()).copied()),
        None => Some(owner_id),
    }
}

/// Start (or restart) playback: reset elapsed, save pre-animation alphas.
/// `offset` allows starting partway through (used by PlaySynced for synchronization).
pub fn start_group_playback(state: &mut SimState, group_id: u64, reverse: bool) {
    start_group_playback_at(state, group_id, reverse, 0.0);
}

/// Start playback at a specific elapsed offset (seconds).
pub fn start_group_playback_at(state: &mut SimState, group_id: u64, reverse: bool, offset: f64) {
    // Collect alpha targets to save before mutating the group.
    let targets: Vec<(u64, f32)> = state
        .animation_groups
        .get(&group_id)
        .map(|group| {
            let owner_id = group.owner_frame_id;
            group
                .animations
                .iter()
                .filter(|a| a.anim_type == AnimationType::Alpha)
                .filter_map(|a| resolve_child(state, owner_id, &a.child_key))
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .filter_map(|id| state.widgets.get(id).map(|f| (id, f.alpha)))
                .collect()
        })
        .unwrap_or_default();

    if let Some(group) = state.animation_groups.get_mut(&group_id) {
        group.playing = true;
        group.paused = false;
        group.done = false;
        group.pending_finish = false;
        group.reverse = reverse;
        group.elapsed = offset;
        group.saved_alphas.clear();
        for (id, alpha) in targets {
            group.saved_alphas.insert(id, alpha);
        }
        for anim in &mut group.animations {
            anim.elapsed = 0.0;
        }
    }
}

/// Stop a group: restore pre-animation alphas (unless setToFinalAlpha),
/// clear translation offsets, mark finished.
pub fn stop_group(state: &mut SimState, group_id: u64) {
    if let Some(restore) = collect_group_restore_data(state, group_id) {
        restore_group_visual_state(state, &restore);
    }

    if let Some(group) = state.animation_groups.get_mut(&group_id) {
        group.playing = false;
        group.paused = false;
        group.done = true;
        group.pending_finish = false;
    }
}

struct GroupRestoreData {
    keep_alpha: bool,
    saved_alphas: Vec<(u64, f32)>,
    translation_targets: Vec<u64>,
}

fn collect_group_restore_data(state: &SimState, group_id: u64) -> Option<GroupRestoreData> {
    state.animation_groups.get(&group_id).map(|group| {
        let keep_alpha = group.set_to_final_alpha;
        let saved_alphas = group.saved_alphas.iter().map(|(&id, &a)| (id, a)).collect();
        let translation_targets = translation_target_ids(state, group);
        GroupRestoreData {
            keep_alpha,
            saved_alphas,
            translation_targets,
        }
    })
}

fn translation_target_ids(
    state: &SimState,
    group: &crate::lua_api::animation::AnimGroupState,
) -> Vec<u64> {
    let owner_id = group.owner_frame_id;
    group
        .animations
        .iter()
        .filter(|anim| anim.anim_type == AnimationType::Translation)
        .filter_map(|anim| resolve_child(state, owner_id, &anim.child_key))
        .collect()
}

fn restore_group_visual_state(state: &mut SimState, restore: &GroupRestoreData) {
    if !restore.keep_alpha {
        restore_saved_alphas(state, &restore.saved_alphas);
    }
    clear_translation_offsets(state, &restore.translation_targets);
}

fn restore_saved_alphas(state: &mut SimState, saved_alphas: &[(u64, f32)]) {
    for (id, alpha) in saved_alphas {
        if let Some(frame) = state.widgets.get_mut_visual(*id) {
            frame.alpha = *alpha;
        }
    }
}

fn clear_translation_offsets(state: &mut SimState, translation_targets: &[u64]) {
    for id in translation_targets {
        let had_offset = state
            .widgets
            .get(*id)
            .is_some_and(|f| f.anim_offset_x != 0.0 || f.anim_offset_y != 0.0);
        if let Some(frame) = state.widgets.get_mut_visual(*id) {
            frame.anim_offset_x = 0.0;
            frame.anim_offset_y = 0.0;
        }
        if had_offset {
            state.invalidate_layout(*id);
        }
    }
}

/// Userdata handle for an AnimationGroup.
#[derive(Clone)]
pub struct AnimGroupHandle {
    pub group_id: u64,
    pub state: Rc<RefCell<SimState>>,
}

impl AnimGroupHandle {
    /// Register Play, Restart methods.
    fn add_play_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("Play", |_, this, args: MultiValue| {
            let (reverse, offset) = parse_play_args(args);
            let mut state = this.state.borrow_mut();
            let already = state
                .animation_groups
                .get(&this.group_id)
                .is_some_and(|g| g.playing && !g.done);
            if !already {
                start_group_playback_at(&mut state, this.group_id, reverse, offset);
            }
            Ok(())
        });

        methods.add_method("Restart", |_, this, args: MultiValue| {
            let (reverse, offset) = parse_play_args(args);
            let mut state = this.state.borrow_mut();
            start_group_playback_at(&mut state, this.group_id, reverse, offset);
            Ok(())
        });

        methods.add_method("SetPlaying", |_, this, playing: bool| {
            let mut state = this.state.borrow_mut();
            if playing {
                start_group_playback(&mut state, this.group_id, false);
            } else {
                stop_group(&mut state, this.group_id);
            }
            Ok(())
        });
    }

    /// Register PlaySynced — starts playback with a time offset to sync with other groups.
    fn add_play_synced_method<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("PlaySynced", |lua, this, args: MultiValue| {
            let (reverse, _) = parse_play_args(args);
            let frame_id = this
                .state
                .borrow()
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.frame_id);
            let offset = match frame_id {
                Some(fid) => compute_sync_offset(lua, fid, &this.state)?,
                None => 0.0,
            };
            let mut state = this.state.borrow_mut();
            let already = state
                .animation_groups
                .get(&this.group_id)
                .is_some_and(|g| g.playing && !g.done);
            if !already {
                start_group_playback_at(&mut state, this.group_id, reverse, offset);
            }
            Ok(())
        });
    }

    /// Register Stop, Pause, Finish methods.
    fn add_stop_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("Stop", |_, this, ()| {
            let mut state = this.state.borrow_mut();
            stop_group(&mut state, this.group_id);
            Ok(())
        });

        methods.add_method("Pause", |_, this, ()| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                && group.playing
            {
                group.paused = true;
                group.playing = false;
            }
            // Apply flipbook UV at current progress so paused-at-frame-0 shows correctly
            apply_flipbook_for_group(&mut state, this.group_id);
            Ok(())
        });

        methods.add_method("Finish", |_, this, ()| {
            // WoW-confirmed: Finish() marks the group as pending-finish.
            // IsPlaying() stays true, IsDone() stays false.
            // The group will complete at the end of the current loop iteration.
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id) {
                group.pending_finish = true;
            }
            Ok(())
        });
    }

    /// Register state query methods: IsPlaying, IsPaused, IsDone, IsPendingFinish, IsReverse.
    fn add_state_query_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("IsPlaying", |_, this, ()| {
            Ok(this.group_state_flag(|g| Some(g.playing)).unwrap_or(false))
        });

        methods.add_method("IsPaused", |_, this, ()| {
            Ok(this.group_state_flag(|g| Some(g.paused)).unwrap_or(false))
        });

        methods.add_method("IsDone", |_, this, ()| {
            Ok(this.group_state_flag(|g| Some(g.done)).unwrap_or(true))
        });

        methods.add_method("IsPendingFinish", |_, this, ()| {
            Ok(this
                .group_state_flag(|g| Some(g.pending_finish))
                .unwrap_or(false))
        });

        methods.add_method("IsReverse", |_, this, ()| {
            Ok(this.group_state_flag(|g| Some(g.reverse)).unwrap_or(false))
        });
    }

    fn group_state_flag<F>(&self, read: F) -> Option<bool>
    where
        F: FnOnce(&AnimGroupState) -> Option<bool>,
    {
        let state = self.state.borrow();
        state.animation_groups.get(&self.group_id).and_then(read)
    }

    /// Register looping methods: SetLooping, GetLooping, GetLoopState.
    fn add_looping_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetLooping", |_, this, looping: Option<String>| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id) {
                group.looping = LoopType::from_str(looping.as_deref().unwrap_or("NONE"));
            }
            Ok(())
        });

        methods.add_method("GetLooping", |lua, this, ()| {
            let state = this.state.borrow();
            let s = state
                .animation_groups
                .get(&this.group_id)
                .map_or("NONE", |g| g.looping.as_str());
            Ok(Value::String(lua.create_string(s)?))
        });

        methods.add_method("GetLoopState", |lua, this, ()| {
            let state = this.state.borrow();
            let s = state
                .animation_groups
                .get(&this.group_id)
                .map_or("NONE", |g| g.looping.as_str());
            Ok(Value::String(lua.create_string(s)?))
        });
    }

    /// Register timing methods: GetDuration, GetElapsed, GetProgress, speed multiplier.
    fn add_timing_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetDuration", |_, this, ()| {
            Ok(this.read_group_timing(|group| group.total_duration(), 0.0))
        });

        methods.add_method("GetElapsed", |_, this, ()| {
            Ok(this.read_group_timing(|group| group.elapsed, 0.0))
        });

        methods.add_method("GetProgress", |_, this, ()| {
            Ok(this.read_group_timing(progress_from_group, 0.0))
        });

        methods.add_method("SetAnimationSpeedMultiplier", |_, this, mult: f64| {
            this.update_group_timing(|group| group.speed_multiplier = mult);
            Ok(())
        });

        methods.add_method("GetAnimationSpeedMultiplier", |_, this, ()| {
            Ok(this.read_group_timing(|group| group.speed_multiplier, 1.0))
        });
    }

    fn read_group_timing<F>(&self, read: F, default: f64) -> f64
    where
        F: FnOnce(&AnimGroupState) -> f64,
    {
        let state = self.state.borrow();
        state
            .animation_groups
            .get(&self.group_id)
            .map_or(default, read)
    }

    fn update_group_timing<F>(&self, update: F)
    where
        F: FnOnce(&mut AnimGroupState),
    {
        let mut state = self.state.borrow_mut();
        if let Some(group) = state.animation_groups.get_mut(&self.group_id) {
            update(group);
        }
    }

    /// Register alpha methods.
    fn add_alpha_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetToFinalAlpha", |_, this, val: bool| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id) {
                group.set_to_final_alpha = val;
            }
            Ok(())
        });

        methods.add_method("IsSetToFinalAlpha", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .is_some_and(|g| g.set_to_final_alpha))
        });

        methods.add_method("GetToFinalAlpha", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .is_some_and(|g| g.set_to_final_alpha))
        });

        methods.add_method("SetAlpha", |_, _this, _alpha: f64| Ok(()));
        methods.add_method("GetAlpha", |_, _this, ()| Ok(1.0_f64));
    }

    /// Register script handler methods: SetScript, GetScript, HasScript, HookScript.
    fn add_script_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "SetScript",
            |lua, this, (event, handler): (String, Option<mlua::Function>)| {
                let mut state = this.state.borrow_mut();
                if let Some(group) = state.animation_groups.get_mut(&this.group_id) {
                    if let Some(old_key) = group.scripts.remove(&event) {
                        lua.remove_registry_value(old_key).ok();
                    }
                    if let Some(func) = handler {
                        let key = lua.create_registry_value(func)?;
                        group.scripts.insert(event, key);
                    }
                }
                Ok(())
            },
        );

        methods.add_method("GetScript", |lua, this, event: String| {
            let state = this.state.borrow();
            if let Some(group) = state.animation_groups.get(&this.group_id)
                && let Some(key) = group.scripts.get(&event)
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
                .is_some_and(|g| g.scripts.contains_key(&event)))
        });

        methods.add_method(
            "HookScript",
            |lua, this, (event, handler): (String, Option<mlua::Function>)| {
                let mut state = this.state.borrow_mut();
                if let Some(group) = state.animation_groups.get_mut(&this.group_id)
                    && let Some(func) = handler
                {
                    if let Some(old_key) = group.scripts.remove(&event) {
                        lua.remove_registry_value(old_key).ok();
                    }
                    let key = lua.create_registry_value(func)?;
                    group.scripts.insert(event, key);
                }
                Ok(())
            },
        );
    }

    /// Register identity/hierarchy methods: GetObjectType, GetName, GetParent.
    fn add_identity_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetObjectType", |_, _this, ()| Ok("AnimationGroup"));

        methods.add_method("IsObjectType", |_, _this, type_name: String| {
            Ok(matches!(
                type_name.as_str(),
                "AnimationGroup" | "Animation" | "UIObject" | "ParallelAnimation"
            ))
        });

        methods.add_method("GetName", |_, this, ()| {
            let state = this.state.borrow();
            Ok(state
                .animation_groups
                .get(&this.group_id)
                .and_then(|g| g.name.clone()))
        });

        methods.add_method("GetParent", |lua, this, ()| {
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
    }

    /// Register animation management methods.
    fn add_animation_management_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetAnimations", |lua, this, ()| {
            let state = this.state.borrow();
            if let Some(group) = state.animation_groups.get(&this.group_id) {
                let mut values = Vec::new();
                for i in 0..group.animations.len() {
                    let handle = AnimHandle {
                        group_id: this.group_id,
                        anim_index: i,
                        state: Rc::clone(&this.state),
                    };
                    values.push(Value::UserData(lua.create_userdata(handle)?));
                }
                return Ok(MultiValue::from_vec(values));
            }
            Ok(MultiValue::new())
        });

        methods.add_method("CreateAnimation", |lua, this, args: MultiValue| {
            create_animation_impl(lua, this, args)
        });

        methods.add_method("RemoveAnimations", |_, this, ()| {
            let mut state = this.state.borrow_mut();
            if let Some(group) = state.animation_groups.get_mut(&this.group_id) {
                group.animations.clear();
            }
            Ok(())
        });
    }
}

fn progress_from_group(group: &AnimGroupState) -> f64 {
    let duration = group.total_duration();
    if duration <= 0.0 {
        0.0
    } else {
        (group.elapsed / duration).clamp(0.0, 1.0)
    }
}

/// CreateAnimation implementation — extracted to keep add_animation_management_methods short.
fn create_animation_impl(
    lua: &mlua::Lua,
    this: &AnimGroupHandle,
    args: MultiValue,
) -> mlua::Result<mlua::AnyUserData> {
    let args: Vec<Value> = args.into_iter().collect();
    let anim_type_str = args.first().and_then(|v| {
        if let Value::String(s) = v {
            Some(s.to_string_lossy().to_string())
        } else {
            None
        }
    });
    let anim_name = args.get(1).and_then(|v| {
        if let Value::String(s) = v {
            Some(s.to_string_lossy().to_string())
        } else {
            None
        }
    });
    let anim_type = AnimationType::from_str(anim_type_str.as_deref().unwrap_or("Animation"));
    let mut anim = AnimState::new(anim_type);
    anim.name = anim_name;
    let anim_index = {
        let mut state = this.state.borrow_mut();
        let group = state
            .animation_groups
            .get_mut(&this.group_id)
            .ok_or_else(|| mlua::Error::runtime("Animation group not found"))?;
        let idx = group.animations.len();
        group.animations.push(anim);
        idx
    };
    lua.create_userdata(AnimHandle {
        group_id: this.group_id,
        anim_index,
        state: Rc::clone(&this.state),
    })
}

impl AnimGroupHandle {
    /// __index metamethod: look up custom fields (set by Mixin, KeyValues, etc.)
    fn add_index_metamethod<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(
            mlua::MetaMethod::Index,
            |lua: &Lua, (ud, key): (mlua::AnyUserData, Value)| {
                let handle = ud.borrow::<AnimGroupHandle>()?;
                let group_id = handle.group_id;
                drop(handle);

                let key_str = match &key {
                    Value::String(s) => s.to_string_lossy().to_string(),
                    _ => return Ok(Value::Nil),
                };

                if let Ok(fields_table) = lua.globals().get::<mlua::Table>("__anim_group_fields")
                    && let Ok(group_fields) = fields_table.get::<mlua::Table>(group_id)
                {
                    let value: Value = group_fields
                        .get::<Value>(key_str.as_str())
                        .unwrap_or(Value::Nil);
                    if value != Value::Nil {
                        return Ok(value);
                    }
                }

                Ok(Value::Nil)
            },
        );
    }

    /// __newindex metamethod: store custom fields (used by Mixin, KeyValues, etc.)
    fn add_newindex_metamethod<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(
            mlua::MetaMethod::NewIndex,
            |lua: &Lua, (ud, key, value): (mlua::AnyUserData, String, Value)| {
                let handle = ud.borrow::<AnimGroupHandle>()?;
                let group_id = handle.group_id;
                drop(handle);

                let fields_table = get_or_create_table(lua, "__anim_group_fields");
                let group_fields: mlua::Table = fields_table
                    .get::<mlua::Table>(group_id)
                    .unwrap_or_else(|_| {
                        let t = lua.create_table().unwrap();
                        fields_table.set(group_id, t.clone()).unwrap();
                        t
                    });

                group_fields.set(key, value)?;
                Ok(())
            },
        );
    }
}

/// Get or create a named global Lua table.
fn get_or_create_table(lua: &Lua, name: &str) -> mlua::Table {
    lua.globals().get::<mlua::Table>(name).unwrap_or_else(|_| {
        let t = lua.create_table().unwrap();
        lua.globals().set(name, t.clone()).unwrap();
        t
    })
}

impl UserData for AnimGroupHandle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_play_methods(methods);
        Self::add_play_synced_method(methods);
        Self::add_stop_methods(methods);
        Self::add_state_query_methods(methods);
        Self::add_looping_methods(methods);
        Self::add_timing_methods(methods);
        Self::add_alpha_methods(methods);
        Self::add_script_methods(methods);
        Self::add_identity_methods(methods);
        Self::add_animation_management_methods(methods);
        Self::add_index_metamethod(methods);
        Self::add_newindex_metamethod(methods);
    }
}
