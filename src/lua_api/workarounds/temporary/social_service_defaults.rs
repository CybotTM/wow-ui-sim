//! Named M3 policies for social services without backing capability snapshots.
//!
//! Cached documentation fixes signatures/return types, not native defaults.
//! Retire each policy when its named missing system publishes the query in c_api.

use crate::client_profile::{ACTIVE, ClientProfile};

const SHARED: &str = r#"
local function install(namespace, name, fn)
    if rawget(namespace, name) == nil then namespace[name] = fn end
end
C_PartyInfo = C_PartyInfo or {}
C_SocialRestrictions = C_SocialRestrictions or {}
C_LFGInfo = C_LFGInfo or {}
C_LFGList = C_LFGList or {}
C_GuildBank = C_GuildBank or {}

-- Missing per-member faction/category roster. Existing party members have no
-- faction field; their legacy faction query inherits the player's faction.
install(C_PartyInfo, "IsCrossFactionParty", function(category) return false end)
-- Missing authoritative account silence/reception policy. Match the existing
-- unrestricted IsChatDisabled default, not a claim about live account status.
install(C_SocialRestrictions, "IsSilenced", function() return false end)
install(C_SocialRestrictions, "CanReceiveChat", function() return true end)
-- Missing LFD/GroupFinder service availability and LFG account authentication.
-- Existing dungeon/listing data does not establish either service capability.
install(C_LFGInfo, "IsLFDEnabled", function() return false end)
install(C_LFGInfo, "IsGroupFinderEnabled", function() return false end)
install(C_LFGList, "IsPlayerAuthenticatedForLFG", function(activityCategoryID) return false end)
-- Missing realm guild-bank capability. Preserve Mists' existing storage-backed
-- publication; other profiles have no enabled bank service snapshot.
install(C_GuildBank, "IsGuildBankEnabled", function() return false end)
"#;

const MODERN: &str = r#"
local function install(namespace, name, fn)
    if rawget(namespace, name) == nil then namespace[name] = fn end
end
C_PlayerMentorship = C_PlayerMentorship or {}
C_ChatInfo = C_ChatInfo or {}
C_RecruitAFriend = C_RecruitAFriend or {}
C_FriendList = C_FriendList or {}
-- Missing mentor membership/eligibility service. Documented status None = 0.
install(C_PlayerMentorship, "GetMentorshipStatus", function(playerLocation) return 0 end)
-- Missing learned-language capability model. Use only the explicit legacy
-- language list until that model replaces both the legacy and C_* queries.
install(C_ChatInfo, "CanPlayerSpeakLanguage", function(languageID)
    for index = 1, GetNumLanguages() do
        local _, knownID = GetLanguageByIndex(index)
        if knownID == languageID then return true end
    end
    return false
end)
-- Missing RAF service availability and game-type legacy-friend mode policy.
install(C_RecruitAFriend, "IsSystemEnabled", function() return false end)
install(C_FriendList, "IsLegacyFriendSystemEnabled", function() return false end)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    if ACTIVE == ClientProfile::Wrath {
        return Ok(());
    }
    lua.exec(SHARED)?;
    if matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    ) {
        lua.exec(MODERN)?;
    }
    if ACTIVE == ClientProfile::WowForever {
        // Missing Battle.net friend-tag/game-type capability policy. Only the
        // Forever cache declares this member; existing friend tags are data,
        // not evidence that a tag is supported for this game type.
        lua.exec(r#"
            C_BattleNet = C_BattleNet or {}
            if rawget(C_BattleNet, "IsFriendTagSupportedForCurrentGameType") == nil then
                function C_BattleNet.IsFriendTagSupportedForCurrentGameType(friendTag) return false end
            end
        "#)?;
    }
    Ok(())
}
