//! Behavior pin: main action bar attach/detach preserves its original anchors.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";

#[test]
fn main_action_bar_attach_and_detach_restore_parent_and_points() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        install_region_util_spy(env);
        seed_main_bar_anchor(env);

        attach_main_bar(env);
        assert_attached_to_target(env);
        assert_anchor_snapshot_captured(env);
        assert_attach_cleared_points(env);

        detach_from_unrelated_frame(env);
        assert_attached_to_target(env);
        assert_region_points_not_restored(env);

        detach_from_target(env);
        assert_detached_to_ui_parent(env);
        assert_region_points_restored(env);
        assert_original_anchor_restored(env);
    });
    }
}

fn install_region_util_spy(env: &WowLuaEnv) {
    env.exec(
        r#"
        AttachDetachTarget = CreateFrame("Frame", "AttachDetachTarget", UIParent)
        AttachDetachOther = CreateFrame("Frame", "AttachDetachOther", UIParent)

        AttachDetachSpy = { getPoints = 0, applyPoints = 0 }
        AttachDetachOriginalGetPointsArray = RegionUtil.GetPointsArray
        AttachDetachOriginalApplyRegionPoints = RegionUtil.ApplyRegionPoints

        RegionUtil.GetPointsArray = function(region)
            AttachDetachSpy.getPoints = AttachDetachSpy.getPoints + 1
            return AttachDetachOriginalGetPointsArray(region)
        end

        RegionUtil.ApplyRegionPoints = function(region, points)
            AttachDetachSpy.applyPoints = AttachDetachSpy.applyPoints + 1
            return AttachDetachOriginalApplyRegionPoints(region, points)
        end
        "#,
    )
    .expect("attach/detach spy fixture must install cleanly");
}

fn seed_main_bar_anchor(env: &WowLuaEnv) {
    env.exec(
        r#"
        MainActionBar.attachedFrame = nil
        MainActionBar.preAttachPoints = nil
        MainActionBar:SetParent(UIParent)
        MainActionBar:ClearAllPoints()
        MainActionBar:SetPoint("BOTTOM", UIParent, "BOTTOM", 11, 22)
        "#,
    )
    .expect("main-bar anchor fixture must run cleanly");
}

fn attach_main_bar(env: &WowLuaEnv) {
    env.eval::<()>("MainActionBar:AttachToFrame(AttachDetachTarget)")
        .expect("MainActionBar:AttachToFrame must run cleanly");
}

fn detach_from_unrelated_frame(env: &WowLuaEnv) {
    env.eval::<()>("MainActionBar:DetachFromFrame(AttachDetachOther)")
        .expect("unrelated MainActionBar:DetachFromFrame must run cleanly");
}

fn detach_from_target(env: &WowLuaEnv) {
    env.eval::<()>("MainActionBar:DetachFromFrame(AttachDetachTarget)")
        .expect("matching MainActionBar:DetachFromFrame must run cleanly");
}

fn assert_attached_to_target(env: &WowLuaEnv) {
    let attached: bool = env
        .eval(
            r#"
            return MainActionBar:GetParent() == AttachDetachTarget
                and MainActionBar.attachedFrame == AttachDetachTarget
            "#,
        )
        .expect("attached parent probe must run cleanly");
    assert!(
        attached,
        "MainActionBar must remain attached to the target frame"
    );
}

fn assert_anchor_snapshot_captured(env: &WowLuaEnv) {
    let (get_points_calls, snapshot_count): (i32, i32) = env
        .eval("return AttachDetachSpy.getPoints, #MainActionBar.preAttachPoints")
        .expect("anchor snapshot probe must run cleanly");
    assert_eq!(get_points_calls, 1);
    assert_eq!(snapshot_count, 1);
}

fn assert_attach_cleared_points(env: &WowLuaEnv) {
    let point_count: i32 = env
        .eval("return MainActionBar:GetNumPoints()")
        .expect("attached point count probe must run cleanly");
    assert_eq!(point_count, 0, "AttachToFrame must clear existing anchors");
}

fn assert_region_points_not_restored(env: &WowLuaEnv) {
    let apply_points_calls: i32 = env
        .eval("return AttachDetachSpy.applyPoints")
        .expect("unrelated detach spy probe must run cleanly");
    assert_eq!(
        apply_points_calls, 0,
        "DetachFromFrame must ignore unrelated frames"
    );
}

fn assert_detached_to_ui_parent(env: &WowLuaEnv) {
    let detached: bool = env
        .eval("return MainActionBar:GetParent() == UIParent and MainActionBar.attachedFrame == nil")
        .expect("detached parent probe must run cleanly");
    assert!(detached, "DetachFromFrame must restore UIParent");
}

fn assert_region_points_restored(env: &WowLuaEnv) {
    let restored: bool = env
        .eval("return AttachDetachSpy.applyPoints == 1 and MainActionBar.preAttachPoints == nil")
        .expect("matching detach restore probe must run cleanly");
    assert!(
        restored,
        "DetachFromFrame must apply and clear saved anchors"
    );
}

fn assert_original_anchor_restored(env: &WowLuaEnv) {
    let (point, relative_parent_matches, relative_point, x_offset, y_offset): (
        String,
        bool,
        String,
        f64,
        f64,
    ) = env
        .eval(
            r#"
            local point, relativeTo, relativePoint, xOfs, yOfs = MainActionBar:GetPoint(1)
            return point, relativeTo == UIParent, relativePoint, xOfs, yOfs
            "#,
        )
        .expect("restored anchor probe must run cleanly");
    assert_eq!(point, "BOTTOM");
    assert!(
        relative_parent_matches,
        "restored anchor must target UIParent"
    );
    assert_eq!(relative_point, "BOTTOM");
    assert_eq!(x_offset, 11.0);
    assert_eq!(y_offset, 22.0);
}
