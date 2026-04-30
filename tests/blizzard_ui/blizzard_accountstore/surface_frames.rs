//! Frame-shape surface pins for the `Blizzard_AccountStore` lane.
//!
//! Spec/source mismatch finding (PLAN.md task for `AccountStoreFrame`
//! parentKey children): the plan named `Inset`, `CategoryList`,
//! `ItemDisplay`, `Footer` — but `Blizzard_AccountStore.xml:78-141`
//! declares six Frame children with parentKeys: `LeftInset`,
//! `RightInset`, `LeftDisplay`, `RightDisplay`, `CategoryList`,
//! `StoreDisplay`. The mismatches are:
//!
//! | PLAN name      | Actual XML name(s)                                | Notes |
//! |----------------|---------------------------------------------------|-------|
//! | `Inset`        | `LeftInset` + `RightInset`                        | The PLAN-named singular `Inset` is split into two halves; both inherit `AccountStoreInsetFrameTemplate`. |
//! | `CategoryList` | `CategoryList`                                    | Matches verbatim — inherits `AccountStoreCategoryListTemplate`. |
//! | `ItemDisplay`  | `StoreDisplay`                                    | Renamed to `StoreDisplay` but still inherits `AccountStoreItemDisplayTemplate`, so the underlying surface is the same. |
//! | `Footer`       | (does not exist)                                  | No `Footer` parentKey anywhere in `Blizzard_AccountStore.xml` — the PLAN-named child is genuinely absent. |
//!
//! The `account_store_frame_publishes_expected_parentkey_children`
//! test pins what actually exists (the six real parentKey children
//! plus `frameStrata == "HIGH"` and the `PageText` FontString
//! parentKey on the ARTWORK layer) plus three positive nil assertions
//! for the PLAN-named keys (`Inset`, `ItemDisplay`, `Footer`) — the
//! mismatches surface as tripwires that flip if Blizzard ever renames
//! the children to match the PLAN shape.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AccountStore";
const FRAME_NAME: &str = "AccountStoreFrame";

const ACTUAL_PARENT_KEY_CHILDREN: &[(&str, &str)] = &[
    ("LeftInset", "Blizzard_AccountStore.xml:93"),
    ("RightInset", "Blizzard_AccountStore.xml:100"),
    ("LeftDisplay", "Blizzard_AccountStore.xml:106"),
    ("RightDisplay", "Blizzard_AccountStore.xml:119"),
    ("CategoryList", "Blizzard_AccountStore.xml:128"),
    ("StoreDisplay", "Blizzard_AccountStore.xml:135"),
];

const PLAN_NAMED_BUT_ABSENT: &[(&str, &str)] = &[
    (
        "Inset",
        "split into `LeftInset` + `RightInset` in the actual XML",
    ),
    ("ItemDisplay", "renamed to `StoreDisplay` in the actual XML"),
    (
        "Footer",
        "no Footer parentKey exists anywhere in `Blizzard_AccountStore.xml`",
    ),
];

#[test]
fn account_store_frame_publishes_expected_parentkey_children() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let frame_type: String = env
            .eval(&format!("return type(_G[{FRAME_NAME:?}])"))
            .expect("AccountStoreFrame global probe must run cleanly");

        assert_eq!(
            frame_type, "table",
            "Expected `_G[{FRAME_NAME:?}]` to be a table after `{ROOT}` loads, got \
             `{frame_type}`. The frame is declared at `Blizzard_AccountStore.xml:78` with \
             `name=\"AccountStoreFrame\"` and `parent=\"UIParent\"`, so the named-frame \
             registration runs at XML load time. A nil reading means either the XML did not \
             execute (a regression in the load pipeline) or the frame failed to register its \
             name (a regression in the named-frame routing inside `CreateFrame`). Either way, \
             every downstream consumer that reaches `AccountStoreFrame.X` would surface a \
             nil-table-index error."
        );

        let frame_strata: String = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:GetFrameStrata()"))
            .expect("`GetFrameStrata` must run cleanly on AccountStoreFrame");

        assert_eq!(
            frame_strata, "HIGH",
            "Expected `AccountStoreFrame:GetFrameStrata()` to return `HIGH` after `{ROOT}` \
             loads, got `{frame_strata}`. The XML at `Blizzard_AccountStore.xml:78` declares \
             `frameStrata=\"HIGH\"` literally. A regression dropping or rewriting this attribute \
             would change the panel's render order — HIGH places it above MEDIUM (the default \
             UIPanel stratum) so the account-store panel appears on top of normal world-frame \
             chrome but below TOOLTIP / DIALOG / FULLSCREEN strata."
        );

        for (parent_key, defining_site) in ACTUAL_PARENT_KEY_CHILDREN {
            let child_type: String = env
                .eval(&format!("return type(_G[{FRAME_NAME:?}][{parent_key:?}])"))
                .unwrap_or_else(|error| {
                    panic!("failed to probe `AccountStoreFrame.{parent_key}`: {error}")
                });

            assert_eq!(
                child_type, "table",
                "Expected `AccountStoreFrame.{parent_key}` to be a table after `{ROOT}` loads \
                 (declared at `{defining_site}`), got `{child_type}`. The XML element registers \
                 the parentKey via `parentKey=\"{parent_key}\"` on a `<Frame>` child, which the \
                 simulator's `__newindex` metamethod syncs to both the Lua-side property and the \
                 Rust-side `children_keys` HashMap. A nil reading means either the XML element \
                 was dropped (a regression in this lane's XML) or the parentKey-sync routing \
                 broke (a regression in `register_new_frame` / `__newindex`). The six real \
                 parentKey children together cover the panel's entire visible chrome: the \
                 LeftInset / RightInset pair frames the two-pane layout, LeftDisplay / \
                 RightDisplay host the inset backgrounds, CategoryList drives the left-pane \
                 category scroll-box, and StoreDisplay drives the right-pane focused-card \
                 detail panel."
            );
        }

        let page_text_type: String = env
            .eval(&format!("return type(_G[{FRAME_NAME:?}].PageText)"))
            .expect("`AccountStoreFrame.PageText` probe must run cleanly");

        assert_eq!(
            page_text_type, "table",
            "Expected `AccountStoreFrame.PageText` to be a table (FontString userdata that \
             type()s as table) after `{ROOT}` loads. Declared at `Blizzard_AccountStore.xml:85` \
             as a `<FontString parentKey=\"PageText\" inherits=\"GameFontHighlight\">` on the \
             ARTWORK layer, anchored BOTTOMRIGHT — used by `AccountStoreMixin` to render the \
             current page indicator (e.g. `2/5`) on the panel's bottom-right corner. A nil \
             reading means the FontString registration was dropped, leaving the panel without \
             page indication."
        );

        for (absent_key, mismatch_reason) in PLAN_NAMED_BUT_ABSENT {
            let absent_type: String = env
                .eval(&format!("return type(_G[{FRAME_NAME:?}][{absent_key:?}])"))
                .unwrap_or_else(|error| {
                    panic!("failed to probe absence of `AccountStoreFrame.{absent_key}`: {error}")
                });

            assert_eq!(
                absent_type, "nil",
                "Expected `AccountStoreFrame.{absent_key}` to be nil after `{ROOT}` loads \
                 (PLAN.md spec/source mismatch tripwire — {mismatch_reason}), got \
                 `{absent_type}`. The PLAN.md task names `{absent_key}` as a parentKey child of \
                 AccountStoreFrame, but the actual XML at `Blizzard_AccountStore.xml:78-141` \
                 does not declare a child with this parentKey. This positive nil assertion \
                 flips if Blizzard ever renames a child to match the PLAN shape, forcing a \
                 re-pin against the new parentKey set."
            );
        }
    });
}
