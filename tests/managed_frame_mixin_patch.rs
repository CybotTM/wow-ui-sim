//! Pin `UIParentManagedFrameMixin:OnShow` / `OnHide` temporary workaround
//! (`src/lua_api/workarounds/temporary/uiparent_managed_frame_mixin.rs`,
//! installed after `Blizzard_UIParent`).
//!
//! Blizzard wires `self.layoutParent` via `<KeyValue type="global">`
//! that resolves eagerly against `_G`. In the sim the container can
//! be nil when a child fires OnHide during initial load — the patch
//! guards both methods. Delete with the patch once the
//! `<KeyValue type="global">` resolution order is fixed.
//!
//! The patch is applied to a fake mixin so the contract can be
//! exercised without loading Blizzard_UIParent.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::workarounds::patch_uiparent_managed_frame_mixin;

fn env_with_patch() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    env.exec(
        r#"
        UIParentManagedFrameMixin = {}
        function UIParentManagedFrameMixin:OnShow()
            -- Unpatched body (ignores nil guard): would error when layoutParent is nil.
            self.layoutParent:AddManagedFrame(self)
        end
        function UIParentManagedFrameMixin:OnHide()
            self.layoutParent:RemoveManagedFrame(self)
        end
        "#,
    )
    .unwrap();
    patch_uiparent_managed_frame_mixin(&env.loader_env()).expect("patch install");
    env
}

#[test]
fn on_show_is_noop_when_layout_parent_is_nil() {
    let env = env_with_patch();
    let ok: bool = env
        .eval(
            r#"
            local inst = setmetatable({}, { __index = UIParentManagedFrameMixin })
            -- inst.layoutParent intentionally nil
            local ran = pcall(function() inst:OnShow() end)
            return ran
            "#,
        )
        .unwrap();
    assert!(ok, "OnShow must not error when layoutParent is nil");
}

#[test]
fn on_hide_is_noop_when_layout_parent_is_nil() {
    let env = env_with_patch();
    let ok: bool = env
        .eval(
            r#"
            local inst = setmetatable({}, { __index = UIParentManagedFrameMixin })
            local ran = pcall(function() inst:OnHide() end)
            return ran
            "#,
        )
        .unwrap();
    assert!(ok, "OnHide must not error when layoutParent is nil");
}

#[test]
fn on_show_calls_add_when_layout_parent_has_method() {
    let env = env_with_patch();
    let (added_self, add_count): (bool, i64) = env
        .eval(
            r#"
            local captured = nil
            local calls = 0
            local parent = {
                AddManagedFrame = function(self, child)
                    calls = calls + 1
                    captured = child
                end,
            }
            local inst = setmetatable({ layoutParent = parent }, { __index = UIParentManagedFrameMixin })
            inst:OnShow()
            return captured == inst, calls
            "#,
        )
        .unwrap();
    assert!(
        added_self,
        "patched OnShow must pass self to AddManagedFrame"
    );
    assert_eq!(add_count, 1);
}

#[test]
fn on_hide_calls_remove_when_layout_parent_has_method() {
    let env = env_with_patch();
    let (removed_self, remove_count): (bool, i64) = env
        .eval(
            r#"
            local captured = nil
            local calls = 0
            local parent = {
                RemoveManagedFrame = function(self, child)
                    calls = calls + 1
                    captured = child
                end,
            }
            local inst = setmetatable({ layoutParent = parent }, { __index = UIParentManagedFrameMixin })
            inst:OnHide()
            return captured == inst, calls
            "#,
        )
        .unwrap();
    assert!(removed_self);
    assert_eq!(remove_count, 1);
}

#[test]
fn on_show_is_noop_when_layout_parent_missing_method() {
    let env = env_with_patch();
    let ok: bool = env
        .eval(
            r#"
            local inst = setmetatable(
                { layoutParent = {} }, -- parent exists but lacks AddManagedFrame
                { __index = UIParentManagedFrameMixin }
            )
            return pcall(function() inst:OnShow() end)
            "#,
        )
        .unwrap();
    assert!(ok, "missing AddManagedFrame must not blow up");
}

#[test]
fn patch_is_inert_when_mixin_global_is_nil() {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    // Do NOT define the mixin; patch must silently no-op.
    patch_uiparent_managed_frame_mixin(&env.loader_env())
        .expect("patch must not error when mixin global is absent");
    let nil_still: bool = env.eval("return UIParentManagedFrameMixin == nil").unwrap();
    assert!(nil_still);
}
