use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const EXPECTED_FLAG_ALL: f64 = 0xffff_ffff_u32 as f64;

#[test]
fn blizzard_auto_complete_constants_are_global_after_load() {
    if wow_ui_sim::client_profile::ACTIVE == wow_ui_sim::client_profile::ClientProfile::Wrath {
        common::wrath_autocomplete::load_and_assert_native(
            r#"
assert(AUTOCOMPLETE_MAX_BUTTONS == 5)
assert(AUTOCOMPLETE_FLAG_NONE == 0)
assert(AUTOCOMPLETE_FLAG_IN_GROUP == 1)
assert(AUTOCOMPLETE_FLAG_IN_GUILD == 2)
assert(AUTOCOMPLETE_FLAG_FRIEND == 4)
assert(AUTOCOMPLETE_FLAG_BNET == 8)
assert(AUTOCOMPLETE_FLAG_INTERACTED_WITH == 16)
assert(AUTOCOMPLETE_FLAG_ONLINE == 32)
assert(AUTOCOMPLETE_FLAG_ALL == 4294967295)
assert(AUTOCOMPLETE_SIMPLE_REGEX == '(.+)')
assert(AUTOCOMPLETE_SIMPLE_FORMAT_REGEX == '%1$s')
assert(AUTOCOMPLETE_DEFAULT_Y_OFFSET == 3)
"#,
        );
        return;
    }

    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before its file-scope constants can be checked. \
                     Loaded set: {loaded:?}"
                );

                let (
                    max_buttons,
                    flag_none,
                    flag_all,
                    default_y_offset,
                    simple_regex,
                    simple_format_regex,
                ): (f64, f64, f64, f64, String, String) = env
                    .eval(
                        r#"
                        return AUTOCOMPLETE_MAX_BUTTONS,
                            AUTOCOMPLETE_FLAG_NONE,
                            AUTOCOMPLETE_FLAG_ALL,
                            AUTOCOMPLETE_DEFAULT_Y_OFFSET,
                            AUTOCOMPLETE_SIMPLE_REGEX,
                            AUTOCOMPLETE_SIMPLE_FORMAT_REGEX
                        "#,
                    )
                    .expect("AutoComplete global constants should be readable");

                assert_eq!(max_buttons, 5.0);
                assert_eq!(flag_none, 0.0);
                assert_eq!(flag_all, EXPECTED_FLAG_ALL);
                assert_eq!(default_y_offset, 3.0);
                assert_eq!(simple_regex, "(.+)");
                assert_eq!(simple_format_regex, "%1$s");
            });
        });
    });
}
