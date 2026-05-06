//! Tests for rilua fork features (table backing, app_data) and the
//! lua_bridge ergonomic layer.
//!
//! These tests use rilua directly — they don't depend on the full
//! wow-ui-sim crate, so they compile even when mlua-sys fails.

mod backed_table;
mod benchmark;

#[path = "../../../src/lua_bridge/mod.rs"]
mod lua_bridge;

use rilua::vm::closure::{Closure, RustClosure, RustFn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{Lua, LuaApiMut, LuaResult, Val};

use crate::lua_bridge::{
    FrameArena, FrameObject, FrameRef as BridgeFrameRef, FromStack, IntoStack,
    MultiValue as BridgeMultiValue, TableBuilder, create_frame_table,
};
pub use benchmark::{FieldAccessBenchResult, benchmark_table_field_access};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

pub(crate) fn abs_index(state: &LuaState, index: i32) -> usize {
    if index > 0 {
        state.base + (index as usize) - 1
    } else {
        (state.top as isize + index as isize) as usize
    }
}

pub(crate) fn stack_val(state: &LuaState, index: i32) -> Val {
    let abs = abs_index(state, index);
    if abs < state.stack.len() && abs < state.top {
        state.stack[abs]
    } else {
        Val::Nil
    }
}

pub(crate) fn get_string(state: &LuaState, index: i32) -> LuaResult<String> {
    match stack_val(state, index) {
        Val::Str(r) => {
            let s = state.gc.string_arena.get(r).unwrap();
            Ok(String::from_utf8_lossy(s.data()).into_owned())
        }
        _ => Err(rilua::runtime_error("expected string")),
    }
}

pub(crate) fn table_set_fn(state: &mut LuaState, table: GcRef<Table>, name: &str, func: RustFn) {
    let key = state.gc.intern_string(name.as_bytes());
    let closure = Closure::Rust(RustClosure::new(func, name));
    let closure_ref = state.gc.alloc_closure(closure);
    let t = state.gc.tables.get_mut(table).unwrap();
    t.raw_set(
        Val::Str(key),
        Val::Function(closure_ref),
        &state.gc.string_arena,
    )
    .unwrap();
}

pub(crate) fn set_global_table(state: &mut LuaState, name: &str, table_ref: GcRef<Table>) {
    set_global(state, name, Val::Table(table_ref));
}

pub(crate) fn set_global_val(state: &mut LuaState, name: &str, value: Val) {
    set_global(state, name, value);
}

fn set_global(state: &mut LuaState, name: &str, value: Val) {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    state
        .gc
        .tables
        .get_mut(global)
        .unwrap()
        .raw_set(Val::Str(key), value, &state.gc.string_arena)
        .unwrap();
}

pub(crate) fn make_index_self(state: &mut LuaState, mt_ref: GcRef<Table>) {
    let idx_key = state.gc.intern_string(b"__index");
    state
        .gc
        .tables
        .get_mut(mt_ref)
        .unwrap()
        .raw_set(
            Val::Str(idx_key),
            Val::Table(mt_ref),
            &state.gc.string_arena,
        )
        .unwrap();
}

#[test]
fn test_global_helpers_set_table_and_value_globals() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();

        let table_ref = state.gc.alloc_table(Table::new());
        let table_key = state.gc.intern_string(b"name");
        let table_value = state.gc.intern_string(b"BridgeTable");
        let table = state.gc.tables.get_mut(table_ref).unwrap();
        table
            .raw_set(
                Val::Str(table_key),
                Val::Str(table_value),
                &state.gc.string_arena,
            )
            .unwrap();

        set_global_table(state, "BridgeTableGlobal", table_ref);
        set_global_val(state, "BridgeValueGlobal", Val::Num(42.0));
    }

    lua.exec("assert(BridgeTableGlobal.name == 'BridgeTable')")
        .unwrap();
    lua.exec("assert(BridgeValueGlobal == 42)").unwrap();
}

// ---------------------------------------------------------------------------
// Phase 0: Table backing
// ---------------------------------------------------------------------------

#[test]
fn test_table_backing_roundtrip() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();

    let mut table = Table::new();
    assert!(table.backing().is_none());

    table.set_backing(Some((42, 7)));
    assert_eq!(table.backing(), Some((42, 7)));

    let table_ref = state.gc.alloc_table(table);
    let t = state.gc.tables.get(table_ref).unwrap();
    assert_eq!(t.backing(), Some((42, 7)));
}

#[test]
fn test_table_backing_clear() {
    let mut table = Table::new();
    table.set_backing(Some((1, 2)));
    table.set_backing(None);
    assert!(table.backing().is_none());
}

// ---------------------------------------------------------------------------
// Phase 0: app_data
// ---------------------------------------------------------------------------

#[test]
fn test_app_data_set_get() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();

    struct MyData {
        value: i32,
    }

    assert!(state.app_data::<MyData>().is_none());

    state.set_app_data(MyData { value: 99 });
    assert_eq!(state.app_data::<MyData>().unwrap().value, 99);

    state.app_data_mut::<MyData>().unwrap().value = 200;
    assert_eq!(state.app_data::<MyData>().unwrap().value, 200);
}

#[test]
fn test_app_data_wrong_type() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();
    state.set_app_data(42_i32);
    assert!(state.app_data::<String>().is_none());
}

// ---------------------------------------------------------------------------
// Phase 1: wow-ui-sim bridge
// ---------------------------------------------------------------------------

#[test]
fn test_bridge_from_stack_extracts_primitives() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();

    state.push(Val::Num(42.0));
    let hello = state.gc.intern_string(b"hello");
    state.push(Val::Str(hello));
    state.push(Val::Bool(true));

    assert_eq!(i32::from_stack(state, 1).unwrap(), 42);
    assert_eq!(String::from_stack(state, 2).unwrap(), "hello");
    assert!(bool::from_stack(state, 3).unwrap());
    assert_eq!(Option::<u32>::from_stack(state, 4).unwrap(), None);
}

#[test]
fn test_bridge_from_stack_extracts_numbers_and_negative_indices() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();

    state.push(Val::Num(42.5));
    state.push(Val::Num(99.0));
    state.push(Val::Num(7.0));

    assert_eq!(f64::from_stack(state, 1).unwrap(), 42.5);
    assert_eq!(i64::from_stack(state, 2).unwrap(), 99);
    assert_eq!(u32::from_stack(state, -1).unwrap(), 7);
    assert_eq!(Option::<u32>::from_stack(state, -1).unwrap(), Some(7));
    assert_eq!(Val::from_stack(state, 1).unwrap(), Val::Num(42.5));
}

#[test]
fn test_bridge_from_stack_bool_matches_lua_truthiness() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();

    state.push(Val::Nil);
    state.push(Val::Bool(false));
    state.push(Val::Bool(true));
    state.push(Val::Num(0.0));
    let text = state.gc.intern_string(b"hi");
    state.push(Val::Str(text));

    assert!(!bool::from_stack(state, 1).unwrap());
    assert!(!bool::from_stack(state, 2).unwrap());
    assert!(bool::from_stack(state, 3).unwrap());
    assert!(bool::from_stack(state, 4).unwrap());
    assert!(bool::from_stack(state, 5).unwrap());
}

#[test]
fn test_bridge_from_stack_reports_type_errors() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();

    state.push(Val::Bool(true));
    state.push(Val::Num(1.5));
    state.push(Val::Num(-1.0));

    let string_err = String::from_stack(state, 1).unwrap_err().to_string();
    assert!(string_err.contains("expected string, got boolean at argument 1"));

    let int_err = i32::from_stack(state, 2).unwrap_err().to_string();
    assert!(int_err.contains("expected integer, got non-integer number at argument 2"));

    let uint_err = u32::from_stack(state, 3).unwrap_err().to_string();
    assert!(uint_err.contains("expected u32, value -1 out of range at argument 3"));
}

#[test]
fn test_bridge_from_stack_rejects_invalid_utf8_strings() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();
    let invalid = state.gc.intern_string(&[0xff, 0xfe]);
    state.push(Val::Str(invalid));

    let err = String::from_stack(state, 1).unwrap_err().to_string();
    assert!(err.contains("string at argument 1 is not valid UTF-8"));
}

#[derive(Debug, PartialEq, Eq)]
struct TestFrame {
    label: String,
    visits: u32,
}

impl FrameObject for TestFrame {
    type Arena = TestFrameArena;
}

#[derive(Debug)]
struct TestFrameSlot {
    generation: u32,
    frame: Option<TestFrame>,
}

#[derive(Debug)]
struct TestFrameArena {
    slots: Vec<TestFrameSlot>,
}

impl FrameArena for TestFrameArena {
    type Frame = TestFrame;

    fn frame(&self, index: u32, generation: u32) -> Option<&Self::Frame> {
        let slot = self.slots.get(index as usize)?;
        if slot.generation != generation {
            return None;
        }
        slot.frame.as_ref()
    }

    fn frame_mut(&mut self, index: u32, generation: u32) -> Option<&mut Self::Frame> {
        let slot = self.slots.get_mut(index as usize)?;
        if slot.generation != generation {
            return None;
        }
        slot.frame.as_mut()
    }
}

#[test]
fn test_bridge_from_stack_extracts_backed_frame_refs_from_app_data() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();
    state.set_app_data(TestFrameArena {
        slots: vec![TestFrameSlot {
            generation: 3,
            frame: Some(TestFrame {
                label: "FrameA".to_string(),
                visits: 0,
            }),
        }],
    });

    let table_ref = create_frame_table(state, 0, 3);
    state.push(Val::Table(table_ref));

    let frame_ref = BridgeFrameRef::<TestFrameArena>::from_stack(state, 1).unwrap();
    assert_eq!(frame_ref.get(state).unwrap().label, "FrameA");

    frame_ref.get_mut(state).unwrap().visits += 1;
    assert_eq!(frame_ref.get(state).unwrap().visits, 1);
}

#[test]
fn test_bridge_from_stack_rejects_plain_tables_for_frame_refs() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();
    let table_ref = state.gc.alloc_table(Table::new());
    state.push(Val::Table(table_ref));

    let err = BridgeFrameRef::<TestFrameArena>::from_stack(state, 1)
        .unwrap_err()
        .to_string();
    assert!(err.contains("expected frame-backed table, got table at argument 1"));
}

#[test]
fn test_bridge_from_stack_rejects_missing_or_stale_frame_backing() {
    let mut missing_app_data_lua = Lua::new().unwrap();
    {
        let state = missing_app_data_lua.state_mut();
        let missing_app_data_ref = create_frame_table(state, 0, 3);
        state.push(Val::Table(missing_app_data_ref));
        let missing_app_data_err = BridgeFrameRef::<TestFrameArena>::from_stack(state, 1)
            .unwrap_err()
            .to_string();
        assert!(missing_app_data_err.contains("missing frame arena app_data"));
    }

    let mut stale_lua = Lua::new().unwrap();
    {
        let state = stale_lua.state_mut();
        state.set_app_data(TestFrameArena {
            slots: vec![TestFrameSlot {
                generation: 4,
                frame: Some(TestFrame {
                    label: "FrameA".to_string(),
                    visits: 0,
                }),
            }],
        });

        let stale_ref = create_frame_table(state, 0, 3);
        state.push(Val::Table(stale_ref));
        let stale_err = BridgeFrameRef::<TestFrameArena>::from_stack(state, 1)
            .unwrap_err()
            .to_string();
        assert!(stale_err.contains("missing frame for backing (0, 3)"));
    }
}

#[test]
fn test_bridge_into_stack_pushes_multiple_values() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();

    assert_eq!((123_i32, "ok", false).into_stack(state).unwrap(), 3);

    assert_eq!(i32::from_stack(state, 1).unwrap(), 123);
    assert_eq!(String::from_stack(state, 2).unwrap(), "ok");
    assert!(!bool::from_stack(state, 3).unwrap());
}

#[test]
fn test_bridge_into_stack_pushes_u64_ids_losslessly() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();
    let id = 4_294_967_297_u64;

    assert_eq!(id.into_stack(state).unwrap(), 1);
    assert_eq!(i64::from_stack(state, 1).unwrap(), id as i64);
}

#[test]
fn test_bridge_multivalue_roundtrips_variable_values() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();
    let save_top = state.top;

    assert_eq!(("alpha", 42_i32, true).into_stack(state).unwrap(), 3);

    let values = BridgeMultiValue::from_stack(state, 1).unwrap();
    assert_eq!(values.len(), 3);

    state.top = save_top;
    assert_eq!(values.into_stack(state).unwrap(), 3);
    assert_eq!(String::from_stack(state, 1).unwrap(), "alpha");
    assert_eq!(i32::from_stack(state, 2).unwrap(), 42);
    assert!(bool::from_stack(state, 3).unwrap());
}

#[test]
fn test_table_builder_sets_values_and_functions() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();

        fn ping(state: &mut LuaState) -> LuaResult<u32> {
            "pong".into_stack(state)
        }

        let table = TableBuilder::new(state)
            .set("answer", 42_i32)
            .unwrap()
            .set("enabled", true)
            .unwrap()
            .set_function("Ping", ping)
            .unwrap()
            .build();

        set_global_val(state, "Bridge", table);
    }

    lua.exec("assert(Bridge.answer == 42)").unwrap();
    lua.exec("assert(Bridge.enabled == true)").unwrap();
    lua.exec("assert(Bridge.Ping() == 'pong')").unwrap();
}

#[test]
fn test_table_builder_rejects_multi_value_entries() {
    let mut lua = Lua::new().unwrap();
    let state = lua.state_mut();
    let save_top = state.top;

    let err = match TableBuilder::new(state).set("bad", ("left", "right")) {
        Ok(_) => panic!("table builder unexpectedly accepted a multi-value entry"),
        Err(err) => err.to_string(),
    };

    assert!(err.contains("table builder values must push exactly 0 or 1 Lua values, got 2"));
    assert_eq!(state.top, save_top);
}

#[test]
fn test_define_functions_registers_typed_wrappers() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        let table_ref = TableBuilder::new(state).build();
        let table_ref = match table_ref {
            Val::Table(table_ref) => table_ref,
            _ => unreachable!(),
        };

        define_functions!(state, table_ref, {
            "ConcatCount" => |name: String, count: Option<u32>| -> (String, u32) {
                Ok((format!("{name}!"), count.unwrap_or(0)))
            },
            "IsTruthy" => |value: bool| -> bool {
                Ok(value)
            },
            "GetLargeId" => || -> u64 {
                Ok(4_294_967_297_u64)
            },
            "Echo" => |name: String| {
                Ok(name)
            },
            "Touch" => || {
                Ok(())
            },
            "CountArgs" => |values: BridgeMultiValue| -> u32 {
                Ok(values.len() as u32)
            },
            "EchoTail" => |prefix: String, values: BridgeMultiValue| -> BridgeMultiValue {
                assert_eq!(prefix, "tag");
                Ok(values)
            },
            "Preset" => || -> BridgeMultiValue {
                Ok(BridgeMultiValue::from_vec(vec![Val::Num(7.0), Val::Bool(true)]))
            },
        })
        .unwrap();

        set_global_table(state, "BridgeFns", table_ref);
    }

    lua.exec("local text, count = BridgeFns.ConcatCount('mage', 7); assert(text == 'mage!' and count == 7)")
        .unwrap();
    lua.exec("local text, count = BridgeFns.ConcatCount('rogue'); assert(text == 'rogue!' and count == 0)")
        .unwrap();
    lua.exec("assert(BridgeFns.IsTruthy(1) == true)").unwrap();
    lua.exec("assert(BridgeFns.IsTruthy(nil) == false)")
        .unwrap();
    lua.exec("assert(BridgeFns.GetLargeId() == 4294967297)")
        .unwrap();
    lua.exec("assert(BridgeFns.Echo('priest') == 'priest')")
        .unwrap();
    lua.exec("assert(BridgeFns.Touch() == nil)").unwrap();
    lua.exec("assert(BridgeFns.CountArgs() == 0)").unwrap();
    lua.exec("assert(BridgeFns.CountArgs('x', 2, true) == 3)")
        .unwrap();
    lua.exec("local a, b = BridgeFns.EchoTail('tag', 9, false); assert(a == 9 and b == false)")
        .unwrap();
    lua.exec("local a, b = BridgeFns.Preset(); assert(a == 7 and b == true)")
        .unwrap();
}

#[test]
fn test_define_methods_registers_backed_table_methods() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        state.set_app_data(TestFrameArena {
            slots: vec![TestFrameSlot {
                generation: 3,
                frame: Some(TestFrame {
                    label: "FrameA".to_string(),
                    visits: 0,
                }),
            }],
        });

        let mt_ref = state.gc.alloc_table(Table::new());
        define_methods!(state, mt_ref, {
            "Describe" => |frame: &mut TestFrame, label: String, count: u32| -> (String, u32) {
                frame.label = label.clone();
                frame.visits += count;
                Ok((frame.label.clone(), frame.visits))
            },
            "VisitCount" => |frame: &TestFrame| -> u32 {
                Ok(frame.visits)
            },
        })
        .unwrap();
        make_index_self(state, mt_ref);

        let frame_ref = create_frame_table(state, 0, 3);
        state
            .gc
            .tables
            .get_mut(frame_ref)
            .unwrap()
            .set_metatable(Some(mt_ref));
        set_global_table(state, "myframe", frame_ref);
    }

    lua.exec("local label, count = myframe:Describe('frame', 4); assert(label == 'frame' and count == 4)")
        .unwrap();
    lua.exec("assert(myframe:VisitCount() == 4)").unwrap();
}

#[test]
fn test_define_methods_extracts_non_self_args_from_slot_two() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        state.set_app_data(TestFrameArena {
            slots: vec![TestFrameSlot {
                generation: 7,
                frame: Some(TestFrame {
                    label: "seed".to_string(),
                    visits: 2,
                }),
            }],
        });

        let mt_ref = state.gc.alloc_table(Table::new());
        define_methods!(state, mt_ref, {
            "Rewrite" => |frame: &mut TestFrame, label: String, delta: u32| -> (String, u32) {
                frame.label = format!("{}-{label}", frame.label);
                frame.visits += delta;
                Ok((frame.label.clone(), frame.visits))
            },
        })
        .unwrap();
        make_index_self(state, mt_ref);

        let frame_ref = create_frame_table(state, 0, 7);
        state
            .gc
            .tables
            .get_mut(frame_ref)
            .unwrap()
            .set_metatable(Some(mt_ref));
        set_global_table(state, "slot_two_frame", frame_ref);
    }

    lua.exec(
        "local label, visits = slot_two_frame:Rewrite('tail', 5); assert(label == 'seed-tail' and visits == 7)",
    )
    .unwrap();
}

#[test]
fn test_frame_table_roundtrips_five_registered_methods() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        state.set_app_data(TestFrameArena {
            slots: vec![TestFrameSlot {
                generation: 9,
                frame: Some(TestFrame {
                    label: "init".to_string(),
                    visits: 1,
                }),
            }],
        });

        let mt_ref = state.gc.alloc_table(Table::new());
        define_methods!(state, mt_ref, {
            "SetLabel" => |frame: &mut TestFrame, label: String| {
                frame.label = label;
                Ok(())
            },
            "AppendLabel" => |frame: &mut TestFrame, suffix: String| -> String {
                frame.label.push_str(&suffix);
                Ok(frame.label.clone())
            },
            "AddVisits" => |frame: &mut TestFrame, delta: u32| -> u32 {
                frame.visits += delta;
                Ok(frame.visits)
            },
            "GetLabel" => |frame: &TestFrame| -> String {
                Ok(frame.label.clone())
            },
            "Snapshot" => |frame: &TestFrame| -> (String, u32) {
                Ok((frame.label.clone(), frame.visits))
            },
        })
        .unwrap();
        make_index_self(state, mt_ref);

        let frame_ref = create_frame_table(state, 0, 9);
        state
            .gc
            .tables
            .get_mut(frame_ref)
            .unwrap()
            .set_metatable(Some(mt_ref));
        set_global_table(state, "roundtrip_frame", frame_ref);
    }

    lua.exec("assert(roundtrip_frame:SetLabel('mage') == nil)")
        .unwrap();
    lua.exec("assert(roundtrip_frame:AppendLabel('-healer') == 'mage-healer')")
        .unwrap();
    lua.exec("assert(roundtrip_frame:AddVisits(4) == 5)")
        .unwrap();
    lua.exec("assert(roundtrip_frame:GetLabel() == 'mage-healer')")
        .unwrap();
    lua.exec("local label, visits = roundtrip_frame:Snapshot(); assert(label == 'mage-healer' and visits == 5)")
        .unwrap();

    let state = lua.state_mut();
    let arena = state.app_data::<TestFrameArena>().unwrap();
    let frame = arena.frame(0, 9).unwrap();
    assert_eq!(frame.label, "mage-healer");
    assert_eq!(frame.visits, 5);
}

#[test]
fn test_define_methods_extracts_multiple_args_through_helper() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        state.set_app_data(TestFrameArena {
            slots: vec![TestFrameSlot {
                generation: 11,
                frame: Some(TestFrame {
                    label: "seed".to_string(),
                    visits: 2,
                }),
            }],
        });

        let mt_ref = state.gc.alloc_table(Table::new());
        define_methods!(state, mt_ref, {
            "Compose" => |frame: &mut TestFrame, prefix: String, delta: u32, suffix: String| -> (String, u32) {
                frame.label = format!("{prefix}-{}-{suffix}", frame.label);
                frame.visits += delta;
                Ok((frame.label.clone(), frame.visits))
            }
        })
        .unwrap();
        make_index_self(state, mt_ref);

        let frame_ref = create_frame_table(state, 0, 11);
        state
            .gc
            .tables
            .get_mut(frame_ref)
            .unwrap()
            .set_metatable(Some(mt_ref));
        set_global_table(state, "helper_frame", frame_ref);
    }

    lua.exec(
        "local label, visits = helper_frame:Compose('pre', 5, 'post'); \
         assert(label == 'pre-seed-post' and visits == 7)",
    )
    .unwrap();

    let state = lua.state_mut();
    let arena = state.app_data::<TestFrameArena>().unwrap();
    let frame = arena.frame(0, 11).unwrap();
    assert_eq!(frame.label, "pre-seed-post");
    assert_eq!(frame.visits, 7);
}

#[test]
fn test_benchmark_table_field_access_returns_timings() {
    let result = benchmark_table_field_access(1_000, 5).unwrap();

    assert_eq!(result.total_iterations(), 5_000);
    assert!(result.plain_elapsed.as_nanos() > 0);
    assert!(result.backed_elapsed.as_nanos() > 0);
    assert!(result.plain_ns_per_access().is_finite());
    assert!(result.backed_ns_per_access().is_finite());
    assert!(result.backed_over_plain_ratio().is_finite());
}
