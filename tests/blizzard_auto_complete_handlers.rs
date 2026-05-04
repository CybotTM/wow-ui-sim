mod common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";

#[test]
fn blizzard_auto_complete_core_handlers_are_global_functions() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before handler globals can be checked. \
                     Loaded set: {loaded:?}"
                );

                let (ok, message): (bool, Option<String>) = env
                    .eval(
                        r#"
                        local handlers = {
                            "AutoComplete_OnLoad",
                            "AutoComplete_OnEvent",
                            "AutoComplete_Update",
                            "AutoComplete_HideIfAttachedTo",
                            "AutoComplete_SetSelectedIndex",
                            "AutoComplete_GetSelectedIndex",
                            "AutoComplete_GetNumResults",
                            "AutoComplete_UpdateResults",
                            "AutoComplete_IncrementSelection",
                        }

                        for _, name in ipairs(handlers) do
                            if type(_G[name]) ~= "function" then
                                return false, name.." is not a function"
                            end
                        end

                        return true, nil
                        "#,
                    )
                    .expect("AutoComplete handler-global probe should run");

                assert!(
                    ok,
                    "`Blizzard_AutoComplete` handler surface mismatch: {}",
                    message.unwrap_or_else(|| "unknown mismatch".to_string())
                );
            });
        });
    });
}
