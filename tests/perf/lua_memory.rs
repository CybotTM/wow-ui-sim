use crate::perf_game_ui::LoadedGameUi;

pub fn measure_settled_lua_memory_kib(loaded_ui: &LoadedGameUi) -> f64 {
    loaded_ui
        .env
        .eval::<f64>(
            r#"
            collectgarbage("collect")
            return collectgarbage("count")
        "#,
        )
        .expect("collectgarbage('count') should report Lua heap usage in KiB")
}
