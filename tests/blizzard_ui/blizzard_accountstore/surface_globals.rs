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
