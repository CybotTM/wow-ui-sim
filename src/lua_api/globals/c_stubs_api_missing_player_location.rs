use mlua::{Lua, Result, Value};

pub(super) fn register_player_location_stub(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if player_location_already_registered(g)? {
        return Ok(());
    }

    install_player_location_bootstrap(lua)?;
    install_player_location_factories(lua)?;
    install_player_location_source_methods(lua)?;
    install_player_location_state_methods(lua)?;
    Ok(())
}

fn player_location_already_registered(g: &mlua::Table) -> Result<bool> {
    Ok(!g.get::<Value>("PlayerLocation")?.is_nil())
}

fn install_player_location_bootstrap(lua: &Lua) -> Result<()> {
    lua.load(PLAYER_LOCATION_BOOTSTRAP_LUA).exec()
}

fn install_player_location_factories(lua: &Lua) -> Result<()> {
    lua.load(PLAYER_LOCATION_FACTORIES_LUA).exec()
}

fn install_player_location_source_methods(lua: &Lua) -> Result<()> {
    lua.load(PLAYER_LOCATION_SOURCE_METHODS_LUA).exec()
}

fn install_player_location_state_methods(lua: &Lua) -> Result<()> {
    lua.load(PLAYER_LOCATION_STATE_METHODS_LUA).exec()
}

const PLAYER_LOCATION_BOOTSTRAP_LUA: &str = r#"
    PlayerLocation = {};
    PlayerLocationMixin = {};
"#;

const PLAYER_LOCATION_FACTORIES_LUA: &str = r#"
    local function CreatePlayerLocation(fieldName, ...)
        local playerLocation = CreateFromMixins(PlayerLocationMixin);
        if fieldName == "guid" then
            playerLocation:SetGUID(...);
        elseif fieldName == "unit" then
            playerLocation:SetUnit(...);
        elseif fieldName == "chatLineID" then
            playerLocation:SetChatLineID(...);
        elseif fieldName == "communityData" then
            playerLocation:SetCommunityData(...);
        elseif fieldName == "communityInvitation" then
            playerLocation:SetCommunityInvitation(...);
        elseif fieldName == "battlefieldScoreIndex" then
            playerLocation:SetBattlefieldScoreIndex(...);
        elseif fieldName == "voiceID" then
            playerLocation:SetVoiceID(...);
        elseif fieldName == "battleNetID" then
            playerLocation:SetBattleNetID(...);
        end
        return playerLocation;
    end

    function PlayerLocation:CreateFromGUID(guid)
        return CreatePlayerLocation("guid", guid);
    end

    function PlayerLocation:CreateFromUnit(unit)
        return CreatePlayerLocation("unit", unit);
    end

    function PlayerLocation:CreateFromChatLineID(lineID)
        return CreatePlayerLocation("chatLineID", lineID);
    end

    function PlayerLocation:CreateFromCommunityChatData(clubID, streamID, epoch, position)
        return CreatePlayerLocation("communityData", clubID, streamID, epoch, position);
    end

    function PlayerLocation:CreateFromCommunityInvitation(clubID, guid)
        return CreatePlayerLocation("communityInvitation", clubID, guid);
    end

    function PlayerLocation:CreateFromBattlefieldScoreIndex(index)
        return CreatePlayerLocation("battlefieldScoreIndex", index);
    end

    function PlayerLocation:CreateFromVoiceID(memberID, channelID)
        return CreatePlayerLocation("voiceID", memberID, channelID);
    end

    function PlayerLocation:CreateFromBattleNetID(battleNetID)
        return CreatePlayerLocation("battleNetID", battleNetID);
    end
"#;

const PLAYER_LOCATION_SOURCE_METHODS_LUA: &str = r#"
    function PlayerLocationMixin:SetGUID(guid)
        self:ClearAndSetField("guid", guid);
    end

    function PlayerLocationMixin:IsGUID()
        return self.guid ~= nil;
    end

    function PlayerLocationMixin:IsBattleNetGUID()
        return false;
    end

    function PlayerLocationMixin:GetGUID()
        return self.guid or self.communityClubInviterGUID;
    end

    function PlayerLocationMixin:SetUnit(unit)
        self:ClearAndSetField("unit", unit);
    end

    function PlayerLocationMixin:IsUnit()
        return self.unit ~= nil;
    end

    function PlayerLocationMixin:GetUnit()
        return self.unit;
    end

    function PlayerLocationMixin:SetChatLineID(lineID)
        self:ClearAndSetField("chatLineID", lineID);
    end

    function PlayerLocationMixin:IsChatLineID()
        return self.chatLineID ~= nil;
    end

    function PlayerLocationMixin:GetChatLineID()
        return self.chatLineID;
    end

    function PlayerLocationMixin:SetBattlefieldScoreIndex(index)
        self:ClearAndSetField("battlefieldScoreIndex", index);
    end

    function PlayerLocationMixin:IsBattlefieldScoreIndex()
        return self.battlefieldScoreIndex ~= nil;
    end

    function PlayerLocationMixin:GetBattlefieldScoreIndex()
        return self.battlefieldScoreIndex;
    end

    function PlayerLocationMixin:SetVoiceID(memberID, channelID)
        self:Clear();
        self.voiceMemberID = memberID;
        self.voiceChannelID = channelID;
    end

    function PlayerLocationMixin:IsVoiceID()
        return self.voiceMemberID ~= nil and self.voiceChannelID ~= nil;
    end

    function PlayerLocationMixin:GetVoiceID()
        return self.voiceMemberID, self.voiceChannelID;
    end

    function PlayerLocationMixin:SetBattleNetID(battleNetID)
        self:Clear();
        self.battleNetID = battleNetID;
    end

    function PlayerLocationMixin:IsBattleNetID()
        return self.battleNetID ~= nil;
    end

    function PlayerLocationMixin:GetBattleNetID()
        return self.battleNetID;
    end

    function PlayerLocationMixin:SetCommunityData(clubID, streamID, epoch, position)
        self:Clear();
        self.communityClubID = clubID;
        self.communityStreamID = streamID;
        self.communityEpoch = epoch;
        self.communityPosition = position;
    end

    function PlayerLocationMixin:IsCommunityData()
        return self.communityClubID ~= nil and self.communityStreamID ~= nil and self.communityEpoch ~= nil and self.communityPosition ~= nil;
    end

    function PlayerLocationMixin:SetCommunityInvitation(clubID, guid)
        self:Clear();
        self.communityClubID = clubID;
        self.communityClubInviterGUID = guid;
    end

    function PlayerLocationMixin:IsCommunityInvitation()
        return self.communityClubID ~= nil and self.communityClubInviterGUID ~= nil;
    end
"#;

const PLAYER_LOCATION_STATE_METHODS_LUA: &str = r#"
    function PlayerLocationMixin:IsValid()
        if self:IsGUID() then
            local guid = self:GetGUID();
            return guid ~= nil and (C_PlayerInfo.GUIDIsPlayer(guid) or C_AccountInfo.IsGUIDBattleNetAccountType(guid));
        elseif self:IsCommunityData() then
            return C_Club.CanResolvePlayerLocationFromClubMessageData(self.communityClubID, self.communityStreamID, self.communityEpoch, self.communityPosition);
        elseif self:IsUnit() then
            local unit = self:GetUnit();
            return unit ~= nil and UnitIsHumanPlayer(unit);
        end

        return self:IsChatLineID() or self:IsBattlefieldScoreIndex() or self:IsVoiceID() or self:IsBattleNetID() or self:IsCommunityInvitation();
    end

    function PlayerLocationMixin:Clear()
        self.guid = nil;
        self.unit = nil;
        self.chatLineID = nil;
        self.battlefieldScoreIndex = nil;
        self.voiceMemberID = nil;
        self.voiceChannelID = nil;
        self.communityClubID = nil;
        self.communityStreamID = nil;
        self.communityEpoch = nil;
        self.communityPosition = nil;
        self.communityClubInviterGUID = nil;
        self.battleNetID = nil;
    end

    function PlayerLocationMixin:ClearAndSetField(fieldName, field)
        self:Clear();
        self[fieldName] = field;
    end
"#;
