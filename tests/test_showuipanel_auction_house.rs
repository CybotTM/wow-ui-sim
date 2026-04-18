//! Focused Auction House panel load/show coverage.

mod common;

use wow_ui_sim::loader::load_addon;

#[test]
fn auction_house_panel_loads_and_shows_seeded_browse_results() {
    test_timeout! {
        let env = common::panel_fixtures::setup_env();
        let ui = common::panel_fixtures::blizzard_ui_dir();
        let toc_path = ui.join("Blizzard_AuctionHouseUI/Blizzard_AuctionHouseUI_Mainline.toc");
        load_addon(&env.loader_env(), &toc_path)
            .unwrap_or_else(|err| panic!("failed to load Blizzard_AuctionHouseUI: {err}"));

        let result: String = env.eval(
            r#"
            A_Admin.ClearAuctionBrowseResults()
            A_Admin.AddAuctionBrowseResult(210935, 70, 25000, 400, false)

            if not AuctionHouseFrame then
                return "missing_frame"
            end

            ShowUIPanel(AuctionHouseFrame)
            if not AuctionHouseFrame:IsShown() then
                return "frame_hidden"
            end

            AuctionHouseFrame.BrowseResultsFrame:UpdateBrowseResults()

            local rows = AuctionHouseFrame.BrowseResultsFrame.browseResults
            if type(rows) ~= "table" then
                return "missing_rows"
            end
            if #rows ~= 1 then
                return "row_count=" .. tostring(#rows)
            end
            if rows[1].itemKey.itemID ~= 210935 then
                return "row_item=" .. tostring(rows[1].itemKey and rows[1].itemKey.itemID)
            end

            return "ok"
            "#,
        ).unwrap();

        assert_eq!(
            result,
            "ok",
            "Auction House panel should load, show, and expose the seeded browse row: {result}"
        );
    }
}
