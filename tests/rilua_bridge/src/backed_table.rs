use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{Lua, LuaApiMut, LuaResult, Val};

use crate::lua_bridge::create_frame_table;
use crate::{get_string, make_index_self, set_global_table, stack_val, table_set_fn};

// ---------------------------------------------------------------------------
// Backed table behaves as real Lua table
// ---------------------------------------------------------------------------

#[test]
fn test_backed_table_type_is_table() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        let mut t = Table::new();
        t.set_backing(Some((1, 0)));
        let tref = state.gc.alloc_table(t);
        set_global_table(state, "myframe", tref);
    }
    lua.exec("assert(type(myframe) == 'table', 'got ' .. type(myframe))")
        .unwrap();
}

#[test]
fn test_backed_table_rawset_rawget() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        let mut t = Table::new();
        t.set_backing(Some((1, 0)));
        let tref = state.gc.alloc_table(t);
        set_global_table(state, "myframe", tref);
    }
    lua.exec("rawset(myframe, 'foo', 42)").unwrap();
    lua.exec("assert(rawget(myframe, 'foo') == 42)").unwrap();
}

#[test]
fn test_backed_table_pairs() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        let mut t = Table::new();
        t.set_backing(Some((1, 0)));
        let tref = state.gc.alloc_table(t);
        set_global_table(state, "myframe", tref);
    }
    lua.exec(
        r#"
        myframe.a = 1
        myframe.b = 2
        myframe.c = 3
        local count = 0
        for _ in pairs(myframe) do count = count + 1 end
        assert(count == 3, 'expected 3, got ' .. count)
    "#,
    )
    .unwrap();
}

// ---------------------------------------------------------------------------
// Backed table + metatable methods (the frame pattern)
// ---------------------------------------------------------------------------

#[test]
fn test_backed_table_with_rust_methods() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();

        let mt = Table::new();
        let mt_ref = state.gc.alloc_table(mt);

        register_frame_methods(state, mt_ref);
        make_index_self(state, mt_ref);

        let mut ft = Table::new();
        ft.set_backing(Some((7, 1)));
        ft.set_metatable(Some(mt_ref));
        let ft_ref = state.gc.alloc_table(ft);
        set_global_table(state, "myframe", ft_ref);
    }

    lua.exec("myframe:SetName('TestFrame')").unwrap();
    lua.exec("assert(myframe:GetName() == 'TestFrame')")
        .unwrap();
    lua.exec("assert(myframe:IsFrame() == true)").unwrap();
    lua.exec("assert(myframe:GetFrameIndex() == 7)").unwrap();
    lua.exec("myframe:Show()").unwrap();

    lua.exec("myframe.customProp = 123").unwrap();
    lua.exec("assert(myframe.customProp == 123)").unwrap();

    lua.exec("rawset(myframe, 'raw', true)").unwrap();
    lua.exec("assert(rawget(myframe, 'raw') == true)").unwrap();

    lua.exec("assert(myframe:GetName() == 'TestFrame')")
        .unwrap();
}

fn register_frame_methods(state: &mut LuaState, mt_ref: rilua::vm::gc::arena::GcRef<Table>) {
    register_name_methods(state, mt_ref);
    register_frame_state_methods(state, mt_ref);
}

fn register_name_methods(state: &mut LuaState, mt_ref: rilua::vm::gc::arena::GcRef<Table>) {
    fn set_name(state: &mut LuaState) -> LuaResult<u32> {
        let self_val = stack_val(state, 1);
        let name = get_string(state, 2)?;
        if let Val::Table(tref) = self_val {
            let key = state.gc.intern_string(b"__name");
            let name_val = Val::Str(state.gc.intern_string(name.as_bytes()));
            state
                .gc
                .tables
                .get_mut(tref)
                .unwrap()
                .raw_set(Val::Str(key), name_val, &state.gc.string_arena)
                .unwrap();
        }
        Ok(0)
    }

    fn get_name(state: &mut LuaState) -> LuaResult<u32> {
        let self_val = stack_val(state, 1);
        if let Val::Table(tref) = self_val {
            let key = state.gc.intern_string(b"__name");
            let val = state
                .gc
                .tables
                .get(tref)
                .unwrap()
                .get(Val::Str(key), &state.gc.string_arena);
            state.push(val);
        } else {
            state.push(Val::Nil);
        }
        Ok(1)
    }

    table_set_fn(state, mt_ref, "SetName", set_name);
    table_set_fn(state, mt_ref, "GetName", get_name);
}

fn register_frame_state_methods(state: &mut LuaState, mt_ref: rilua::vm::gc::arena::GcRef<Table>) {
    fn is_frame(state: &mut LuaState) -> LuaResult<u32> {
        let self_val = stack_val(state, 1);
        let result = if let Val::Table(tref) = self_val {
            state.gc.tables.get(tref).unwrap().backing().is_some()
        } else {
            false
        };
        state.push(Val::Bool(result));
        Ok(1)
    }

    fn get_frame_index(state: &mut LuaState) -> LuaResult<u32> {
        let self_val = stack_val(state, 1);
        if let Val::Table(tref) = self_val {
            if let Some((idx, _)) = state.gc.tables.get(tref).unwrap().backing() {
                state.push(Val::Num(idx as f64));
                return Ok(1);
            }
        }
        state.push(Val::Nil);
        Ok(1)
    }

    fn show(state: &mut LuaState) -> LuaResult<u32> {
        let self_val = stack_val(state, 1);
        if let Val::Table(tref) = self_val {
            assert!(state.gc.tables.get(tref).unwrap().backing().is_some());
        }
        Ok(0)
    }

    table_set_fn(state, mt_ref, "IsFrame", is_frame);
    table_set_fn(state, mt_ref, "GetFrameIndex", get_frame_index);
    table_set_fn(state, mt_ref, "Show", show);
}

// ---------------------------------------------------------------------------
// Multiple backed tables share a metatable
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_backed_tables_shared_metatable() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();

        let mt = Table::new();
        let mt_ref = state.gc.alloc_table(mt);

        fn get_id(state: &mut LuaState) -> LuaResult<u32> {
            let self_val = stack_val(state, 1);
            if let Val::Table(tref) = self_val {
                if let Some((idx, _)) = state.gc.tables.get(tref).unwrap().backing() {
                    state.push(Val::Num(idx as f64));
                    return Ok(1);
                }
            }
            state.push(Val::Nil);
            Ok(1)
        }

        table_set_fn(state, mt_ref, "GetID", get_id);
        make_index_self(state, mt_ref);

        for (name, idx) in &[("frame1", 10), ("frame2", 20)] {
            let mut ft = Table::new();
            ft.set_backing(Some((*idx, 0)));
            ft.set_metatable(Some(mt_ref));
            let ft_ref = state.gc.alloc_table(ft);
            set_global_table(state, name, ft_ref);
        }
    }

    lua.exec("assert(frame1:GetID() == 10)").unwrap();
    lua.exec("assert(frame2:GetID() == 20)").unwrap();

    lua.exec("frame1.x = 1; frame2.x = 2").unwrap();
    lua.exec("assert(frame1.x == 1)").unwrap();
    lua.exec("assert(frame2.x == 2)").unwrap();
}

// ---------------------------------------------------------------------------
// create_frame_table helper
// ---------------------------------------------------------------------------

#[test]
fn test_create_frame_table_sets_backing_and_keeps_table_behavior() {
    let mut lua = Lua::new().unwrap();
    {
        let state = lua.state_mut();
        let frame_ref = create_frame_table(state, 17, 9);
        let frame = state.gc.tables.get(frame_ref).unwrap();
        assert_eq!(frame.backing(), Some((17, 9)));
        set_global_table(state, "helper_frame", frame_ref);
    }

    lua.exec("assert(type(helper_frame) == 'table')").unwrap();
    lua.exec("rawset(helper_frame, 'field', 55)").unwrap();
    lua.exec("assert(rawget(helper_frame, 'field') == 55)")
        .unwrap();
}
