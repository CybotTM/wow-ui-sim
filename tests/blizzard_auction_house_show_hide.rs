//! Show/hide smoke coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";

#[test]
fn auction_house_show_hide_round_trip_has_no_lua_errors() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            let env = common::panel_fixtures::setup_env();
            load_auction_house_through_runtime(&env);
            load_show_hide_prerequisites(&env);
            clear_recorded_lua_errors(&env);

            let result: String = env
                .eval(
                    r#"
                    if not AuctionHouseFrame then
                        return "missing_frame"
                    end

                    ShowUIPanel(AuctionHouseFrame)
                    if not AuctionHouseFrame:IsShown() then
                        return "show_failed"
                    end

                    HideUIPanel(AuctionHouseFrame)
                    if AuctionHouseFrame:IsShown() then
                        return "hide_failed"
                    end

                    return "ok"
                    "#,
                )
                .expect("AuctionHouseFrame ShowUIPanel/HideUIPanel probe should run");
            assert_eq!("ok", result, "`{ROOT}` show/hide result: {result}");

            let errors = recorded_lua_errors(&env);
            assert!(
                errors.is_empty(),
                "`{ROOT}` show/hide emitted Lua errors:\n{}",
                errors.join("\n")
            );
        });
    });
}

fn load_show_hide_prerequisites(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        local tokenLoaded, tokenReason = C_AddOns.LoadAddOn("Blizzard_TokenUI")
        assert(tokenLoaded, "Blizzard_TokenUI should load: " .. tostring(tokenReason))
        if ContainerFrameSettingsManager and not ContainerFrameSettingsManager.TokenTracker then
            ContainerFrameSettingsManager:OnAddonLoaded("Blizzard_TokenUI")
        end

        local wowTokenLoaded, wowTokenReason = C_AddOns.LoadAddOn("Blizzard_WowTokenUI")
        assert(wowTokenLoaded, "Blizzard_WowTokenUI should load: " .. tostring(wowTokenReason))

        if type(WowToken_IsWowTokenAuctionDialogShown) ~= "function" then
            function WowToken_IsWowTokenAuctionDialogShown()
                return false
            end
        end
        "#,
    )
    .expect("show/hide prerequisite addons should load");
    env.apply_post_load_workarounds();
}

fn load_auction_house_through_runtime(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, reason): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
        .expect("C_AddOns.LoadAddOn should return");
    assert!(loaded, "`{ROOT}` should load: {reason:?}");
}
