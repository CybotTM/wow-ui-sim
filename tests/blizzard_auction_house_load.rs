use crate::common;

use common::auction_fixtures::with_auction_ui;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = if cfg!(feature = "client-wrath") {
    "Blizzard_AuctionUI"
} else {
    "Blizzard_AuctionHouseUI"
};

#[test]
fn blizzard_auction_house_ui_lod_loads_without_ingestion_errors() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_auction_ui(|env| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(&format!(r#"return C_AddOns.LoadAddOn("{ROOT}")"#))
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let is_loaded: bool = env
                    .eval(&format!(r#"return C_AddOns.IsAddOnLoaded("{ROOT}")"#))
                    .expect("C_AddOns.IsAddOnLoaded should return");
                assert!(
                    is_loaded,
                    "`{ROOT}` should be marked loaded after LoadAddOn"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors during load:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
