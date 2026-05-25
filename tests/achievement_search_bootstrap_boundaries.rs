#[test]
fn achievement_search_repairs_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "__wow_ensure_achievement_search_previews",
        "__wow_patch_achievement_search_preview_selection",
        "__wow_patch_achievement_summary_empty_text_overlap",
        "AchievementFrame_SetSearchPreviewSelection = function",
        "AchievementFrameSummary_UpdateAchievements = function",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} must live in the explicit temporary achievement-search workaround boundary, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} must live in the explicit temporary achievement-search workaround boundary, not shared bootstrap"
        );
    }
}
