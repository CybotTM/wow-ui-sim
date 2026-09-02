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
        let (lair, lair_name, quest_tag_atlas, max_arena_teams): (f64, String, String, f64) = env
            .eval(
                r#"
                return LE_LFG_CATEGORY_LAIR, LFG_CATEGORY_NAMES[LE_LFG_CATEGORY_LAIR],
                    type(QUEST_TAG_ATLAS), MAX_ARENA_TEAMS
                "#,
            )
            .expect("Constants.lua globals should be readable");
        assert_eq!(lair, 8.0, "LE_LFG_CATEGORY_LAIR is the ordinal after BATTLEFIELD");
        assert_eq!(lair_name, "Lair", "the LAIR global string is in the 12.1.0 table");
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

#[test]
fn text_to_speech_commands_loads_past_the_combat_start_sound_block() {
    // TextToSpeechCommands.lua:652 formats SLASH_CAA_HELP_SAY_COMBAT_START_SOUND
    // inside a file-scope do-block; the string was missing from the table, so
    // the file aborted there and the 20 command blocks below never registered.
    // The last block (line 1548) registers SLASH_CAA_PLAY_SOUND ("playsound").
    with_blizzard_addon_closure(&["Blizzard_ChatFrame"], &[], |env, _| {
        let (help_text, play_sound, multiline): (String, String, bool) = env
            .eval(
                r#"
                return type(SLASH_CAA_HELP_SAY_COMBAT_START_SOUND),
                    type(CAACommands and CAACommands:GetCommand(SLASH_CAA_PLAY_SOUND)),
                    COMMUNITY_MEMBER_LIST_CROSS_FACTION:find("\n", 1, true) ~= nil
                "#,
            )
            .expect("TextToSpeechCommands globals should be readable");
        assert_eq!(help_text, "string", "SLASH_CAA_HELP_SAY_COMBAT_START_SOUND is in the table");
        assert_eq!(play_sound, "table", "the playsound command from the file's last block is registered");
        assert!(multiline, "a string spanning CSV lines reaches Lua whole");
    });
}
