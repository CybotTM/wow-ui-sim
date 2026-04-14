use std::time::{Duration, Instant};

use rilua::vm::gc::arena::GcRef;
use rilua::vm::table::Table;
use rilua::{Lua, LuaApiMut, LuaResult, Val};

use crate::lua_bridge::create_frame_table;

const FIELD_NAME: &[u8] = b"field";
const FIELD_VALUE: f64 = 1.0;

pub struct FieldAccessBenchResult {
    pub iterations_per_round: u32,
    pub rounds: u32,
    pub plain_elapsed: Duration,
    pub backed_elapsed: Duration,
}

impl FieldAccessBenchResult {
    pub fn total_iterations(&self) -> u64 {
        u64::from(self.iterations_per_round) * u64::from(self.rounds)
    }

    pub fn plain_ns_per_access(&self) -> f64 {
        self.plain_elapsed.as_nanos() as f64 / self.total_iterations() as f64
    }

    pub fn backed_ns_per_access(&self) -> f64 {
        self.backed_elapsed.as_nanos() as f64 / self.total_iterations() as f64
    }

    pub fn backed_over_plain_ratio(&self) -> f64 {
        self.backed_elapsed.as_nanos() as f64 / self.plain_elapsed.as_nanos() as f64
    }
}

pub fn benchmark_table_field_access(
    iterations_per_round: u32,
    rounds: u32,
) -> LuaResult<FieldAccessBenchResult> {
    let mut lua = Lua::new()?;
    let key = intern_field_key(&mut lua);
    let plain_table = create_plain_table(&mut lua, key);
    let backed_table = create_backed_table(&mut lua, key);
    let sum_field = load_sum_field(&mut lua)?;
    let expected = FIELD_VALUE * f64::from(iterations_per_round);
    warm_up_access(
        &mut lua,
        &sum_field,
        plain_table,
        backed_table,
        iterations_per_round,
        expected,
    )?;
    let (plain_elapsed, backed_elapsed) = measure_access_rounds(
        &mut lua,
        &sum_field,
        plain_table,
        backed_table,
        iterations_per_round,
        rounds,
        expected,
    )?;

    Ok(FieldAccessBenchResult {
        iterations_per_round,
        rounds,
        plain_elapsed,
        backed_elapsed,
    })
}

fn intern_field_key(lua: &mut Lua) -> rilua::vm::gc::arena::GcRef<rilua::vm::string::LuaString> {
    let state = lua.state_mut();
    state.gc.intern_string(FIELD_NAME)
}

fn create_plain_table(
    lua: &mut Lua,
    key: rilua::vm::gc::arena::GcRef<rilua::vm::string::LuaString>,
) -> GcRef<Table> {
    let state = lua.state_mut();
    let table_ref = state.gc.alloc_table(Table::new());
    set_field(state, table_ref, key, FIELD_VALUE);
    table_ref
}

fn create_backed_table(
    lua: &mut Lua,
    key: rilua::vm::gc::arena::GcRef<rilua::vm::string::LuaString>,
) -> GcRef<Table> {
    let state = lua.state_mut();
    let table_ref = create_frame_table(state, 7, 1);
    set_field(state, table_ref, key, FIELD_VALUE);
    table_ref
}

fn load_sum_field(lua: &mut Lua) -> LuaResult<rilua::Function> {
    lua.load(
        r#"
            local tbl, count = ...
            local total = 0
            for _ = 1, count do
                total = total + tbl.field
            end
            return total
        "#,
    )
}

fn warm_up_access(
    lua: &mut Lua,
    sum_field: &rilua::Function,
    plain_table: GcRef<Table>,
    backed_table: GcRef<Table>,
    iterations_per_round: u32,
    expected: f64,
) -> LuaResult<()> {
    run_sum_field(lua, sum_field, plain_table, iterations_per_round, expected)?;
    run_sum_field(lua, sum_field, backed_table, iterations_per_round, expected)?;
    Ok(())
}

fn measure_access_rounds(
    lua: &mut Lua,
    sum_field: &rilua::Function,
    plain_table: GcRef<Table>,
    backed_table: GcRef<Table>,
    iterations_per_round: u32,
    rounds: u32,
    expected: f64,
) -> LuaResult<(Duration, Duration)> {
    let mut plain_elapsed = Duration::ZERO;
    let mut backed_elapsed = Duration::ZERO;

    for round in 0..rounds {
        if round % 2 == 0 {
            plain_elapsed +=
                time_one_call(lua, sum_field, plain_table, iterations_per_round, expected)?;
            backed_elapsed +=
                time_one_call(lua, sum_field, backed_table, iterations_per_round, expected)?;
        } else {
            backed_elapsed +=
                time_one_call(lua, sum_field, backed_table, iterations_per_round, expected)?;
            plain_elapsed +=
                time_one_call(lua, sum_field, plain_table, iterations_per_round, expected)?;
        }
    }

    Ok((plain_elapsed, backed_elapsed))
}

fn time_one_call(
    lua: &mut Lua,
    sum_field: &rilua::Function,
    table_ref: GcRef<Table>,
    iterations_per_round: u32,
    expected: f64,
) -> LuaResult<Duration> {
    let started = Instant::now();
    run_sum_field(lua, sum_field, table_ref, iterations_per_round, expected)?;
    Ok(started.elapsed())
}

fn run_sum_field(
    lua: &mut Lua,
    sum_field: &rilua::Function,
    table_ref: GcRef<Table>,
    iterations_per_round: u32,
    expected: f64,
) -> LuaResult<()> {
    let results = lua.call_function(
        sum_field,
        &[
            Val::Table(table_ref),
            Val::Num(f64::from(iterations_per_round)),
        ],
    )?;
    assert_eq!(results, vec![Val::Num(expected)]);
    Ok(())
}

fn set_field(
    state: &mut rilua::vm::state::LuaState,
    table_ref: GcRef<Table>,
    key: rilua::vm::gc::arena::GcRef<rilua::vm::string::LuaString>,
    value: f64,
) {
    let table = state.gc.tables.get_mut(table_ref).unwrap();
    table
        .raw_set(Val::Str(key), Val::Num(value), &state.gc.string_arena)
        .unwrap();
}
