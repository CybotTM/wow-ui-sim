-- Source-backed legacy UI defaults, not a claim about current native C++ defaults.
-- Shared/TextToSpeechFrame.lua: DefaultLegacySettings; TextToSpeechCommands.lua:1-5.
local function default_preferences()
    local chatTypes = {}
    for _, name in ipairs({
        "EMOTE", "SYSTEM", "WHISPER", "SAY", "YELL", "PARTY_LEADER", "PARTY",
        "OFFICER", "GUILD", "GUILD_ACHIEVEMENT", "ACHIEVEMENT", "RAID_LEADER",
        "RAID", "RAID_WARNING", "INSTANCE_CHAT_LEADER", "INSTANCE_CHAT", "BN_WHISPER", "PING",
    }) do
        chatTypes[name] = true
    end
    return {
        rate = 0,
        volume = 100,
        voices = {[Enum.TtsVoiceType.Standard] = {id = 0}, [Enum.TtsVoiceType.Alternate] = {id = 1}},
        settings = {
            [Enum.TtsBoolSetting.PlaySoundSeparatingChatLineBreaks] = true,
            [Enum.TtsBoolSetting.AddCharacterNameToSpeech] = true,
            [Enum.TtsBoolSetting.PlayActivitySoundWhenNotFocused] = true,
            [Enum.TtsBoolSetting.AlternateSystemVoice] = true,
            [Enum.TtsBoolSetting.NarrateMyMessages] = false,
        },
        chatTypes = chatTypes,
        channels = {},
        channelKeys = {},
    }
end

local data = __tts_saved_value
local persist = __tts_persist
__tts_saved_value, __tts_persist = nil, nil
if data == nil then
    -- User-approved non-native model: fresh storage is unsaved, Mark makes it saved.
    data = {account = default_preferences(), character = default_preferences(), saved = false}
end
assert(type(data.account) == "table" and type(data.character) == "table"
    and type(data.saved) == "boolean", "invalid persisted TTS model")
local function save_preferences()
    persist(data)
end
local function preferences()
    if GetCVarBool("TTSUseCharacterSettings") then
        return data.character
    end
    return data.account
end

local function require_number(value, minimum, maximum)
    assert(type(value) == "number", "C_TTSSettings requires a number")
    -- Supported UI input domain only; native out-of-range API behavior is UNKNOWN.
    assert(value >= minimum and value <= maximum, "C_TTSSettings value outside supported UI range")
    return value
end

local function bool_value(value)
    -- API docs: omitted newVal defaults to false. Original migration also supplies nil.
    if value == nil then
        return false
    end
    assert(type(value) == "boolean", "C_TTSSettings requires a boolean")
    return value
end

local function require_string(value)
    assert(type(value) == "string", "C_TTSSettings requires a string")
    return value
end

function C_TTSSettings.GetSpeechRate()
    return preferences().rate
end
function C_TTSSettings.SetSpeechRate(value)
    preferences().rate = require_number(value, -10, 10)
    save_preferences()
end
function C_TTSSettings.GetSpeechVolume()
    return preferences().volume
end
function C_TTSSettings.SetSpeechVolume(value)
    preferences().volume = require_number(value, 0, 100)
    save_preferences()
end

local function voice_selection(voiceType)
    local voice = preferences().voices[voiceType]
    assert(voice, "C_TTSSettings requires a TtsVoiceType")
    return voice
end

local function matching_voice(selection)
    -- Never invent a default voice name or infer an ID from a name.
    for _, voice in ipairs(C_VoiceChat.GetTtsVoices()) do
        local matchesID = selection.id ~= nil and voice.voiceID == selection.id
        local matchesName = selection.name ~= nil and voice.name == selection.name
        if matchesID or matchesName then
            return voice
        end
    end
    error("UNKNOWN: selected TTS voice has no supplied catalogue match")
end

function C_TTSSettings.GetVoiceOptionID(voiceType)
    local voice = voice_selection(voiceType)
    if voice.id ~= nil then
        return voice.id
    end
    return matching_voice(voice).voiceID
end
function C_TTSSettings.GetVoiceOptionName(voiceType)
    local voice = voice_selection(voiceType)
    if voice.name ~= nil then
        return voice.name
    end
    return matching_voice(voice).name
end
function C_TTSSettings.SetVoiceOption(voiceType, voiceID)
    voice_selection(voiceType)
    assert(type(voiceID) == "number", "C_TTSSettings requires a numeric voice ID")
    preferences().voices[voiceType] = {id = voiceID}
    save_preferences()
end
function C_TTSSettings.SetVoiceOptionName(voiceType, voiceName)
    voice_selection(voiceType)
    preferences().voices[voiceType] = {name = require_string(voiceName)}
    save_preferences()
end

function C_TTSSettings.GetSetting(setting)
    local value = preferences().settings[setting]
    assert(value ~= nil, "C_TTSSettings requires a TtsBoolSetting")
    return value
end
function C_TTSSettings.SetSetting(setting, value)
    C_TTSSettings.GetSetting(setting)
    preferences().settings[setting] = bool_value(value)
    save_preferences()
end
function C_TTSSettings.GetChatTypeEnabled(chatName)
    return preferences().chatTypes[require_string(chatName)] == true
end
function C_TTSSettings.SetChatTypeEnabled(chatName, value)
    preferences().chatTypes[require_string(chatName)] = bool_value(value)
    save_preferences()
end

-- ChatConstantsDocumentation.lua: ChatChannelInfo. Round-trip a supplied record by value.
-- Native identity across differing metadata and channelKey->record mapping are UNKNOWN.
local channelFields = {"name", "shortcut", "localID", "instanceID", "zoneChannelID", "channelType"}
local function copy_channel(info)
    assert(type(info) == "table", "C_TTSSettings requires ChatChannelInfo")
    local copy = {}
    for index, field in ipairs(channelFields) do
        local expected = index <= 2 and "string" or "number"
        assert(type(info[field]) == expected, "invalid ChatChannelInfo field: "..field)
        copy[field] = info[field]
    end
    return copy
end
local function same_channel(left, right)
    for _, field in ipairs(channelFields) do
        if left[field] ~= right[field] then
            return false
        end
    end
    return true
end
local function channel_entry(info)
    for _, entry in ipairs(preferences().channels) do
        if same_channel(entry.info, info) then
            return entry
        end
    end
end
function C_TTSSettings.GetChannelEnabled(info)
    local entry = channel_entry(copy_channel(info))
    return entry ~= nil and entry.enabled
end
function C_TTSSettings.SetChannelEnabled(info, value)
    local copy = copy_channel(info)
    local enabled = bool_value(value)
    local entry = channel_entry(copy)
    if entry then
        entry.enabled = enabled
    else
        table.insert(preferences().channels, {info = copy, enabled = enabled})
    end
    save_preferences()
end
function C_TTSSettings.SetChannelKeyEnabled(key, value)
    -- Preserve supplied legacy keys without inventing how C++ encodes channel identity.
    preferences().channelKeys[require_string(key)] = bool_value(value)
    save_preferences()
end

function C_TTSSettings.SetDefaultSettings()
    if GetCVarBool("TTSUseCharacterSettings") then
        data.character = default_preferences()
    else
        data.account = default_preferences()
    end
    save_preferences()
end
function C_TTSSettings.GetCharacterSettingsSaved()
    return data.saved
end
function C_TTSSettings.MarkCharacterSettingsSaved()
    data.saved = true
    save_preferences()
end
