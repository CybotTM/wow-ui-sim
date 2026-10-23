//! Shape-only contracts; native semantics unknown. Source predictions are not runtime proof.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]

#[path = "common/cf2_shape.rs"]
mod shape;

#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]
// retail/ptr/wowforever: Blizzard_APIDocumentationGenerated/DiscordConstantsDocumentation.lua:42
// (DiscordChatInfo); Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1843 declares optional discordInfo.
const DISCORD_SCHEMA: &str = r#"local function checkDiscord(value)

if value.discordInfo ~= nil then
    fields(value.discordInfo, {
{'userID', 'opaque', false},
{'globalName', 'string', false},
{'type', 'opaque', false},
{'lastOnlineGUID', 'string', false},
{'lastOnlineName', 'string', false},
{'hasAttachment', 'boolean', false},
{'hasPoll', 'boolean', false},
{'hasEmbed', 'boolean', false},
{'hasSticker', 'boolean', false},
{'hasEmoji', 'boolean', false},
{'hasError', 'boolean', false},
{'hasForwardedMessage', 'boolean', false},
{'forwardedMessage', 'string', false},
{'fromDiscord', 'boolean', false}
})
    member(value.discordInfo.type, {0, 1, 2})
end

end"#;
#[cfg(any(
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]
const DISCORD_SCHEMA: &str = "local function checkDiscord(value) end";
// retail: Blizzard_APIDocumentationGenerated/SocialRestrictionsDocumentation.lua:96 (IsSquelched).
// ptr: Blizzard_APIDocumentationGenerated/SocialRestrictionsDocumentation.lua:71 (IsSquelched).
// mists: Blizzard_APIDocumentationGenerated/SocialRestrictionsDocumentation.lua:62 (IsSquelched).
// Source-predicted FAIL: IsSquelched missing; only boolean shape, no button state; not executed; native semantics unknown.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-mists"
))]
#[test]
fn original_navigation_squelch_shape_blocker() {
    let env = shape::load(&["Blizzard_GroupFinder"]);
    shape::check(
        &env,
        r#"

scalar('boolean', C_SocialRestrictions.IsSquelched())
local source = debug.getinfo(LFGListCategorySelection_UpdateNavButtons, 'S').source:gsub('\\', '/')
assert(source:match('Blizzard_GroupFinder/Mainline/LFGList.lua$') or source:match('Blizzard_GroupFinder/Classic/LFGList.lua$'))
-- Default fixture's unsilenced path reaches the getter; no silence/squelch policy asserted.
LFGListCategorySelection_UpdateNavButtons(LFGListFrame.CategorySelection)

"#,
    );
}

// mists: Blizzard_APIDocumentationGenerated/LFGInfoDocumentation.lua:179 (IsLFREnabled).
// Source-predicted FAIL: IsLFREnabled missing; no visibility or eligibility policy; not executed; native semantics unknown.
#[cfg(any(feature = "client-mists"))]
#[test]
fn original_lfr_visibility_shape_blocker() {
    let env = shape::load(&["Blizzard_GroupFinder"]);
    shape::check(
        &env,
        r#"

scalar('boolean', C_LFGInfo.IsLFREnabled())
original(GroupFinderFrame_EvaluateButtonVisibility, 'Blizzard_GroupFinder/Classic/PVEFrame.lua')
GroupFinderFrame_EvaluateButtonVisibility(GroupFinderFrame)

"#,
    );
}

// retail: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:580 (GetInvitationsForSelf).
// retail: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1938 (ClubSelfInvitationInfo).
// retail: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1760 (ClubInfo).
// retail: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1808 (ClubMemberInfo).
// ptr: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:580 (GetInvitationsForSelf).
// ptr: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1938 (ClubSelfInvitationInfo).
// ptr: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1760 (ClubInfo).
// ptr: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1808 (ClubMemberInfo).
// wowforever: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:580 (GetInvitationsForSelf).
// wowforever: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1938 (ClubSelfInvitationInfo).
// wowforever: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1760 (ClubInfo).
// wowforever: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1808 (ClubMemberInfo).
// mists: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:545 (GetInvitationsForSelf).
// mists: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1839 (ClubSelfInvitationInfo).
// mists: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1662 (ClubInfo).
// mists: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1710 (ClubMemberInfo).
// era: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:545 (GetInvitationsForSelf).
// era: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1839 (ClubSelfInvitationInfo).
// era: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1662 (ClubInfo).
// era: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1710 (ClubMemberInfo).
// anniversary: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:545 (GetInvitationsForSelf).
// anniversary: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1839 (ClubSelfInvitationInfo).
// anniversary: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1662 (ClubInfo).
// anniversary: Blizzard_APIDocumentationGenerated/ClubDocumentation.lua:1710 (ClubMemberInfo).
// Source-predicted FAIL: GetInvitationsForSelf missing; recursive actual records, empty list allowed; not executed; native semantics unknown.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]
#[test]
fn original_microbutton_invitation_records_shape_blocker() {
    let env = shape::load(&["Blizzard_MicroMenu"]);
    shape::check(
        &env,
        &format!(
            "{DISCORD_SCHEMA}\n{}",
            r#"

local invitations = scalar('table', C_Club.GetInvitationsForSelf())
local clubSchema = {
{'clubId', 'opaque', false},
{'name', 'string', false},
{'shortName', 'string', true},
{'description', 'string', false},
{'broadcast', 'string', false},
{'clubType', 'opaque', false},
{'avatarId', 'number', false},
{'memberCount', 'number', true},
-- BigUInteger's Lua primitive representation is native-unknown; nullability only.
{'favoriteTimeStamp', 'opaque', true},
{'joinTime', 'opaque', true},
{'socialQueueingEnabled', 'boolean', true},
{'crossFaction', 'boolean', true}
}
local memberSchema = {
{'isSelf', 'boolean', false},
{'memberId', 'opaque', false},
{'name', 'string', true},
{'role', 'opaque', true},
{'presence', 'opaque', false},
{'clubType', 'opaque', true},
{'guid', 'string', true},
{'bnetAccountId', 'number', true},
{'memberNote', 'string', true},
{'officerNote', 'string', true},
{'classID', 'number', true},
{'race', 'number', true},
{'level', 'number', true},
{'zone', 'string', true},
{'achievementPoints', 'number', true},
{'profession1ID', 'number', true},
{'profession1Rank', 'number', true},
{'profession1Name', 'string', true},
{'profession2ID', 'number', true},
{'profession2Rank', 'number', true},
{'profession2Name', 'string', true},
{'lastOnlineYear', 'number', true},
{'lastOnlineMonth', 'number', true},
{'lastOnlineDay', 'number', true},
{'lastOnlineHour', 'number', true},
{'guildRank', 'string', true},
{'guildRankOrder', 'number', true},
{'isRemoteChat', 'boolean', true},
{'overallDungeonScore', 'number', true},
{'faction', 'opaque', true},
{'timerunningSeasonID', 'number', true}
}
local invitationSchema = {
{'invitationId', 'opaque', false},
{'club', 'table', false},
{'inviter', 'table', false},
{'leaders', 'table', false}
}
local function checkMember(value)
    fields(value, memberSchema)
    checkDiscord(value)
    member(value.presence, {0, 1, 2, 3, 4, 5})
    if value.clubType ~= nil then member(value.clubType, {0, 1, 2, 3}) end
    if value.role ~= nil then member(value.role, {1, 2, 3, 4}) end
    if value.faction ~= nil then member(value.faction, {0, 1}) end
end
for _, value in ipairs(invitations) do
    fields(value, invitationSchema)
    fields(value.club, clubSchema)
    member(value.club.clubType, {0, 1, 2, 3})
    checkMember(value.inviter)
    for _, leader in ipairs(value.leaders) do checkMember(leader) end
end
local source = debug.getinfo(GuildMicroButtonMixin.HasUnseenInvitations, 'S').source:gsub('\\', '/')
assert(source:match('Blizzard_MicroMenu/Mainline/MainMenuBarMicroButtons.lua$') or source:match('Blizzard_MicroMenu/Classic/MainMenuBarMicroButtons.lua$'))
assert(type(DISPLAYED_COMMUNITIES_INVITATIONS) == 'table', 'normal display-history prerequisite')
local button = CreateFrame('Button', nil, UIParent)
Mixin(button, GuildMicroButtonMixin)
button:HasUnseenInvitations()

"#
        ),
    );
}
