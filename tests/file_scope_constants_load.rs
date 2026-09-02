//! Three Blizzard files aborted at file scope on a nil table index, losing
//! every definition below the line. Each test loads the addon the way startup
//! does and reads a symbol that sits BELOW the former abort, so a regression
//! shows as the symbol going nil rather than as a startup error to be counted.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_closure;

#[test]
fn constants_lua_loads_past_the_lfg_category_names_table() {
    // Constants.lua:497 indexes LFG_CATEGORY_NAMES by LE_LFG_CATEGORY_LAIR;
    // QUEST_TAG_ATLAS (line 529) and MAX_ARENA_TEAMS (501) sit below it.
    with_blizzard_addon_closure(&["Blizzard_FrameXMLBase"], &[], |env, _| {
        let (lair, quest_tag_atlas, max_arena_teams): (f64, String, f64) = env
            .eval(
                r#"
                return LE_LFG_CATEGORY_LAIR, type(QUEST_TAG_ATLAS), MAX_ARENA_TEAMS
                "#,
            )
            .expect("Constants.lua globals should be readable");
        assert_eq!(lair, 8.0, "LE_LFG_CATEGORY_LAIR is the ordinal after BATTLEFIELD");
        // The Lair entry's VALUE is the LAIR global string, which the simulator
        // does not carry; a nil value is fine, only a nil KEY aborts the file.
        assert_eq!(quest_tag_atlas, "table", "QUEST_TAG_ATLAS is defined below the LFG table");
        assert_eq!(max_arena_teams, 2.0, "Constants.lua ran to its PVP section");
    });
}

#[test]
fn recent_allies_util_loads_past_the_legacy_friend_entry() {
    // Blizzard_RecentAlliesUtil.lua:112 indexes a table by
    // Enum.RolodexType.LegacyFriend; the function at line 119 sits below it.
    with_blizzard_addon_closure(&["Blizzard_RecentAllies"], &[], |env, _| {
        let (legacy_friend, generate): (f64, String) = env
            .eval(
                r#"
                return Enum.RolodexType.LegacyFriend,
                    type(RecentAlliesUtil and RecentAlliesUtil.GenerateContextStringForInteraction)
                "#,
            )
            .expect("RecentAlliesUtil globals should be readable");
        assert_eq!(legacy_friend, 23.0, "RolodexConstantsDocumentation.lua gives LegacyFriend = 23");
        assert_eq!(generate, "function", "RecentAlliesUtil.lua ran past the LegacyFriend entry");
    });
}

#[test]
fn shared_xml_mainline_toc_still_defines_list_header_mixin() {
    // Blizzard_SharedXML_Mainline.toc omits ListTemplates.lua/.xml; the loader
    // appends them. Ten Blizzard files inherit ListHeaderThreeSliceTemplate and
    // QuestMapFrame.lua:417 indexes ListHeaderMixin when the world map opens.
    let env = common::env_with_shared_xml();
    let (mixin, set_click_handler): (String, String) = env
        .eval(
            r#"
            return type(ListHeaderMixin),
                type(ListHeaderMixin and ListHeaderMixin.SetClickHandler)
            "#,
        )
        .expect("ListHeaderMixin should be readable");
    assert_eq!(mixin, "table", "ListTemplates.lua should load with the Mainline TOC");
    assert_eq!(set_click_handler, "function");
}
