//! Frame-surface probes for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const PARENT_KEY_CHILDREN: &[&str] = &[
    "summaryPage",
    "completedPage",
    "artifactPage",
    "helpPage",
    "RaceFilterDropdown",
    "bgLeft",
    "bgRight",
    "factionIcon",
    "tab1",
    "tab2",
    "infoButton",
];

#[test]
fn archaeology_frame_matches_xml_surface() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface: ArchaeologyFrameSurface = env
            .eval(
                r#"
                return type(ArchaeologyFrame),
                       ArchaeologyFrame:GetObjectType(),
                       ArchaeologyFrame:GetParent() == UIParent,
                       ArchaeologyFrame.CloseButton ~= nil,
                       ArchaeologyFrame.Bg ~= nil,
                       ArchaeologyFrame.Inset ~= nil
                "#,
            )
            .expect("ArchaeologyFrame surface probe must run cleanly");

        assert_archaeology_frame_surface(surface);

        for child_name in PARENT_KEY_CHILDREN {
            let child_exists = archaeology_frame_child_exists(env, child_name);

            assert!(
                child_exists,
                "`ArchaeologyFrame.{child_name}` must be exposed as an XML parentKey child"
            );
        }
    });
}

type ArchaeologyFrameSurface = (String, String, bool, bool, bool, bool);

fn assert_archaeology_frame_surface(surface: ArchaeologyFrameSurface) {
    let (frame_type, object_type, parent_is_ui_parent, has_close_button, has_bg, has_inset) =
        surface;

    assert_eq!(frame_type, "table", "`ArchaeologyFrame` must exist");
    assert_eq!(
        object_type, "Frame",
        "`ArchaeologyFrame` must be a Frame object"
    );
    assert!(
        parent_is_ui_parent,
        "`ArchaeologyFrame` must be parented to `UIParent`"
    );
    assert!(
        has_close_button && has_bg && has_inset,
        "`ArchaeologyFrame` must inherit `ButtonFrameTemplate` frame parts"
    );
}

fn archaeology_frame_child_exists(env: &wow_ui_sim::lua_api::WowLuaEnv, child_name: &str) -> bool {
    env.eval(&format!("return ArchaeologyFrame[{child_name:?}] ~= nil"))
        .unwrap_or_else(|err| {
            panic!("failed to probe ArchaeologyFrame parentKey child `{child_name}`: {err}")
        })
}
