//! Load smoke for `Blizzard_ActionStatus`.
//!
//! TOC reference (`Interface/BlizzardUI/Blizzard_ActionStatus/
//! Blizzard_ActionStatus.toc`):
//!
//! ```text
//! ## Title: Blizzard_ActionStatus
//! ## DefaultState: enabled
//! ## OptionalDep: Blizzard_FrameXML, Blizzard_GlueXML
//! ## AllowLoad: Both
//! ```

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::{blizzard_ui_dir, recorded_lua_errors};
use wow_ui_sim::toc::TocFile;

const ROOT: &str = "Blizzard_ActionStatus";
const ROOT_TOC_FILE: &str = "Blizzard_ActionStatus.toc";
const RAW_OPTIONAL_DEP_LINE: &str = "## OptionalDep: Blizzard_FrameXML, Blizzard_GlueXML";

#[test]
fn action_status_loads_with_no_required_deps_and_no_lua_errors() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
        assert!(
            loaded.iter().any(|name| name == ROOT),
            "Smoke-shape harness MUST load `{ROOT}` itself when it is the closure root. Loaded \
             set: {loaded:?}"
        );

        assert_toc_declares_no_required_deps_and_only_raw_optional_deps();

        let errors = recorded_lua_errors(env);
        assert!(
            errors.is_empty(),
            "Blizzard_ActionStatus smoke-shape load must emit zero recorded Lua errors after \
             the panel baseline is cleared. Got:\n  {}",
            errors.join("\n  ")
        );
    });
}

fn assert_toc_declares_no_required_deps_and_only_raw_optional_deps() {
    let toc_path = blizzard_ui_dir().join(ROOT).join(ROOT_TOC_FILE);
    let toc = TocFile::from_file(&toc_path).unwrap_or_else(|err| {
        panic!(
            "TOC at `{}` MUST parse cleanly before the load contract can be checked: {err}",
            toc_path.display()
        )
    });

    assert!(
        toc.dependencies().is_empty(),
        "`{ROOT}` must declare no required dependencies. The TOC only lists the singular \
         optional dependency line `{RAW_OPTIONAL_DEP_LINE}`."
    );

    let raw_toc = std::fs::read_to_string(&toc_path).unwrap_or_else(|err| {
        panic!(
            "TOC at `{}` MUST be readable for raw OptionalDep verification: {err}",
            toc_path.display()
        )
    });

    assert!(
        raw_toc.contains(RAW_OPTIONAL_DEP_LINE),
        "`{ROOT}` must keep its only listed dependencies optional via the raw singular TOC line \
         `{RAW_OPTIONAL_DEP_LINE}`. Raw TOC:\n{raw_toc}"
    );
}
