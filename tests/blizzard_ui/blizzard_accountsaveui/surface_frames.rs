//! Frame surface for `Blizzard_AccountSaveUI.xml`.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.xml`, lines 4-83):
//!
//! ```xml
//! <Frame name="AccountSaveFrame" mixin="AccountSaveFrameMixin"
//!        parent="CharacterSelect" frameStrata="DIALOG" toplevel="true"
//!        enableMouse="true">
//!     <Frames>
//!         <Frame      parentKey="Border"        inherits="DialogBorderOpaqueTemplate"/>
//!         <Frame      parentKey="ContentInsets" .../>
//!         <Frame      parentKey="AlertIcon"     .../>
//!         <SimpleHTML parentKey="Text"          .../>
//!         <EditBox    parentKey="LockEditBox"   letters="32" .../>
//!         <Button     parentKey="SaveButton"    inherits="UIPanelButtonTemplate" .../>
//!     </Frames>
//! ```
//!
//! The frame is parsed during XML load — these assertions catch
//! regressions in the XML loader (parent resolution, frameStrata
//! propagation, parentKey wiring) without exercising any
//! `AccountSaveFrameMixin` method. Method behavior is pinned by the
//! dedicated behavior fixtures.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";
const PARENT_KEYS: &[&str] = &[
    "Border",
    "ContentInsets",
    "AlertIcon",
    "Text",
    "LockEditBox",
    "SaveButton",
];

#[test]
fn account_save_frame_parent_and_strata() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (frame_type, parent_name, strata) = env
            .eval::<(String, String, String)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after XML load")
                local parent = AccountSaveFrame:GetParent()
                return type(AccountSaveFrame),
                       parent and parent:GetName() or "<nil>",
                       AccountSaveFrame:GetFrameStrata()
                "#,
            )
            .expect("AccountSaveFrame must expose GetParent / GetFrameStrata after XML load");
        assert_eq!(
            frame_type, "table",
            "AccountSaveFrame must register as a table-typed handle (FrameRef reports as \
             `table` via __metatable). Got type = `{frame_type}`."
        );
        assert_eq!(
            parent_name, "CharacterSelect",
            "AccountSaveFrame's `parent=\"CharacterSelect\"` XML attribute must resolve \
             to the CharacterSelect frame at parse time. If this regresses, either the \
             XML attribute parser dropped the parent reference or CharacterSelect failed \
             to materialise from Blizzard_GlueXML's closure. Got parent: `{parent_name}`."
        );
        assert_eq!(
            strata, "DIALOG",
            "AccountSaveFrame's `frameStrata=\"DIALOG\"` XML attribute must propagate to \
             the runtime strata string returned by GetFrameStrata. Got: `{strata}`."
        );
    });
}

#[test]
fn account_save_frame_exposes_all_parent_keys() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for key in PARENT_KEYS {
            let probe = format!("return type(AccountSaveFrame.{key})");
            let kind = env
                .eval::<String>(&probe)
                .unwrap_or_else(|err| panic!("AccountSaveFrame.{key} probe raised: {err}"));
            assert_eq!(
                kind, "table",
                "AccountSaveFrame.{key} must be wired as a child frame via parentKey. \
                 The XML declares `<... parentKey=\"{key}\">` inside <Frames>; if this \
                 regresses, either the parentKey attribute was dropped during XML parse \
                 or the child element type stopped being recognised by the loader. Got \
                 type = `{kind}` for key `{key}`."
            );
        }
    });
}
