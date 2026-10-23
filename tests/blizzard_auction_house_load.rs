use crate::common;

use common::auction_fixtures::with_auction_ui;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

#[test]
fn blizzard_auction_house_ui_lod_loads_without_ingestion_errors() {
    let root =
        if wow_ui_sim::client_profile::ACTIVE == wow_ui_sim::client_profile::ClientProfile::Wrath {
            "Blizzard_AuctionUI"
        } else {
            "Blizzard_AuctionHouseUI"
        };
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_auction_ui(|env| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(&format!(r#"return C_AddOns.LoadAddOn("{root}")"#))
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{root}` should load: {reason:?}");

                let is_loaded: bool = env
                    .eval(&format!(r#"return C_AddOns.IsAddOnLoaded("{root}")"#))
                    .expect("C_AddOns.IsAddOnLoaded should return");
                assert!(
                    is_loaded,
                    "`{root}` should be marked loaded after LoadAddOn"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{root}` emitted Lua errors during load:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
