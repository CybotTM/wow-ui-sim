//! Surface-level globals pinned by `Blizzard_AccountStore.lua`.
//!
//! Three tables are published at file scope by the lane's primary Lua
//! body (`Blizzard_AccountStore.lua`):
//!
//! | Global                                    | Defining line                          |
//! |-------------------------------------------|----------------------------------------|
//! | `AccountStoreMixin`                       | `Blizzard_AccountStore.lua:16` (`= {}`) |
//! | `FullscreenAccountStoreContainerMixin`    | `Blizzard_AccountStore.lua:66` (`= {}`) |
//! | `FullscreenLeaveAccountStoreButtonMixin`  | `Blizzard_AccountStore.lua:102` (`= {}`)|
//!
//! Why these three deserve a dedicated surface pin separate from the
//! load-smoke's general "lane file-scope globals exist" check: each
//! mixin is the entry point for a distinct frame in the addon's XML
//! (the root `AccountStoreFrame`, the fullscreen container, and the
//! fullscreen leave button respectively). The XML side wires
//! `mixin="AccountStoreMixin"` etc. on the frame definitions, so any
//! regression that drops a mixin at file scope would surface as a
//! template-resolution failure when the XML loads — but pinning the
//! table type at the global level catches the regression closer to the
//! source. A nil reading here means `Blizzard_AccountStore.lua` either
//! failed to execute (a regression in the load pipeline) or executed
//! but stopped before reaching the relevant assignment (a regression
//! in upstream globals like `Mixin` that the file uses indirectly).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AccountStore";

const PUBLISHED_TABLES: &[&str] = &[
    "AccountStoreMixin",
    "FullscreenAccountStoreContainerMixin",
    "FullscreenLeaveAccountStoreButtonMixin",
];

const CATEGORY_LIST_TABLES: &[&str] =
    &["AccountStoreCategoryMixin", "AccountStoreCategoryListMixin"];

const CARD_TEMPLATE_TABLES: &[&str] = &[
    "AccountStoreBaseCardMixin",
    "AccountStoreCreatureCardMixin",
    "AccountStoreIconCardMixin",
    "AccountStoreTransmogSetCardMixin",
    "AccountStoreMountCardMixin",
];

const ITEM_VIEW_TABLES: &[(&str, &str)] = &[
    (
        "AccountStoreItemDisplayMixin",
        "Blizzard_AccountStoreItemDisplay.lua:2",
    ),
    (
        "AccountStoreItemRackMixin",
        "Blizzard_AccountStoreItemRack.lua:18",
    ),
];

#[test]
fn account_store_publishes_expected_global_mixin_tables() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for global in PUBLISHED_TABLES {
            let actual_type: String = env
                .eval(&format!("return type(_G[{global:?}])"))
                .unwrap_or_else(|error| panic!("failed to probe `{global}` type: {error}"));

            assert_eq!(
                actual_type, "table",
                "Expected `{global}` to publish as a table after `{ROOT}` loads, got \
                 `{actual_type}`. The mixin is declared at file scope in \
                 `Blizzard_AccountStore.lua` and consumed via `mixin=\"{global}\"` on the \
                 corresponding frame in `Blizzard_AccountStore.xml`. A nil here means either \
                 the Lua file did not execute (load-pipeline regression) or it stopped before \
                 the relevant `{global} = {{}}` assignment (upstream-global regression). \
                 Either way, the XML's mixin reference would break and the frame would not \
                 inherit any of its declared methods."
            );
        }
    });
}

#[test]
fn account_store_category_list_publishes_expected_global_mixin_tables() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for global in CATEGORY_LIST_TABLES {
            let actual_type: String = env
                .eval(&format!("return type(_G[{global:?}])"))
                .unwrap_or_else(|error| panic!("failed to probe `{global}` type: {error}"));

            assert_eq!(
                actual_type, "table",
                "Expected `{global}` to publish as a table after `{ROOT}` loads, got \
                 `{actual_type}`. Both globals are declared at file scope in \
                 `Blizzard_AccountStoreCategoryList.lua` (lines 2 and 18 — `{global} = {{}}`) \
                 and consumed by the corresponding XML on the category-list scroll-box rows \
                 and the list container respectively. The TOC orders \
                 `Blizzard_AccountStoreCategoryList.lua` second (right after \
                 `Blizzard_AccountStoreUtil.lua`); a regression that drops this file from the \
                 TOC, or that fails to reach the assignment, would surface as a nil here. \
                 The category-list mixins are also a load-order canary for the rest of the \
                 lane: `Blizzard_AccountStore.lua:OnStoreFrontSet` reaches into \
                 `AccountStoreCategoryListMixin` indirectly via the XML's parentKey routing, \
                 so any regression dropping these globals would silently break the whole \
                 category-selection flow."
            );
        }
    });
}

#[test]
fn account_store_card_templates_publishes_expected_global_mixin_tables() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for global in CARD_TEMPLATE_TABLES {
            let actual_type: String = env
                .eval(&format!("return type(_G[{global:?}])"))
                .unwrap_or_else(|error| panic!("failed to probe `{global}` type: {error}"));

            assert_eq!(
                actual_type, "table",
                "Expected `{global}` to publish as a table after `{ROOT}` loads, got \
                 `{actual_type}`. The five card mixins are declared at file scope in \
                 `Blizzard_AccountStoreCardTemplates.lua` (lines 13, 230, 252, 263, 367) and \
                 wired via `mixin=\"...\"` on the corresponding card frame templates in \
                 `Blizzard_AccountStoreCardTemplates.xml`. The XML's per-card-type templates \
                 build on `AccountStoreBaseCardMixin` for shared OnLoad/OnEnter/OnLeave/OnEvent \
                 handlers and override `UpdateCardDisplay` per type — a nil reading here means \
                 either the file did not execute or it stopped before the relevant assignment, \
                 which would leave every card frame in the lane without its display-update \
                 method."
            );
        }

        let mount_aliases_creature: bool = env
            .eval("return rawequal(AccountStoreMountCardMixin, AccountStoreCreatureCardMixin)")
            .expect("Mount/Creature alias-identity probe must run cleanly");

        assert!(
            mount_aliases_creature,
            "`AccountStoreMountCardMixin` MUST be the same table reference as \
             `AccountStoreCreatureCardMixin` (Blizzard_AccountStoreCardTemplates.lua:367 — \
             `AccountStoreMountCardMixin = AccountStoreCreatureCardMixin;`). The two card \
             types share their entire behavior surface (the mount card is rendered with the \
             same creature-display path as the creature card), so the addon expresses this by \
             aliasing the table directly rather than wrapping with `CreateFromMixins`. A \
             regression that converts the alias into a fresh empty table or a `CreateFromMixins` \
             wrap would break the shared-behavior contract: the mount card would lose access \
             to `UpdateCardDisplay` and any future Creature-mixin additions would silently \
             fail to propagate. The general type-only assertion above would pass in that \
             regression scenario (Mount is still a table), so this identity check is the only \
             thing keeping the alias contract honest."
        );
    });
}

#[test]
fn account_store_item_view_publishes_expected_global_mixin_tables() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for (global, defining_site) in ITEM_VIEW_TABLES {
            let actual_type: String = env
                .eval(&format!("return type(_G[{global:?}])"))
                .unwrap_or_else(|error| panic!("failed to probe `{global}` type: {error}"));

            assert_eq!(
                actual_type, "table",
                "Expected `{global}` to publish as a table after `{ROOT}` loads, got \
                 `{actual_type}`. Declared at `{defining_site}` (`{global} = {{}}`) and \
                 consumed via `mixin=\"{global}\"` on the corresponding XML frame template. \
                 The two globals come from sibling Lua files \
                 (`Blizzard_AccountStoreItemRack.lua` runs third, \
                 `Blizzard_AccountStoreItemDisplay.lua` runs fourth in the TOC order); a \
                 regression that drops either file from the TOC, or that fails to reach the \
                 assignment, would surface as a nil here. The item-rack mixin renders the \
                 grid of available cards inside the selected category, and the item-display \
                 mixin renders the focused-card detail panel — both are required for the lane \
                 to function past the category-list selection."
            );
        }
    });
}
