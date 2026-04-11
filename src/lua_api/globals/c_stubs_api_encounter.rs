//! C_EncounterTimeline stubs and CombatLog global alias fixup.
//!
//! Split from c_stubs_api_combat.rs to keep file sizes manageable.

use mlua::{Lua, MultiValue, Result};

/// C_EncounterTimeline - encounter timeline UI data (boss ability timers).
pub(super) fn register_c_encounter_timeline(lua: &Lua, g: &mlua::Table) -> Result<()> {
    lua.load(ENCOUNTER_TIMELINE_LUA).exec()?;
    g.get::<mlua::Table>("C_EncounterTimeline")
        .and_then(|timeline| g.set("C_EncounterTimeline", timeline))?;
    Ok(())
}

const ENCOUNTER_TIMELINE_LUA: &str = r#"
    C_EncounterTimeline = C_EncounterTimeline or {}
    local api = C_EncounterTimeline

    local function make_color(r, g, b, a)
        local color = { r = r, g = g, b = b, a = a or 1.0 }
        function color:GetRGB()
            return self.r, self.g, self.b
        end
        function color:GetRGBA()
            return self.r, self.g, self.b, self.a
        end
        return color
    end

    local function create_duration(total_duration, elapsed_duration)
        local duration = {
            totalDuration = total_duration,
            elapsedDuration = elapsed_duration,
        }

        function duration:GetElapsedDuration()
            return self.elapsedDuration
        end

        function duration:GetRemainingDuration()
            return math.max(0, self.totalDuration - self.elapsedDuration)
        end

        function duration:GetTotalDuration()
            return self.totalDuration
        end

        function duration:GetElapsedPercent()
            if self.totalDuration <= 0 then
                return 0
            end
            return self:GetElapsedDuration() / self.totalDuration
        end

        function duration:GetRemainingPercent()
            if self.totalDuration <= 0 then
                return 0
            end
            return self:GetRemainingDuration() / self.totalDuration
        end

        function duration:GetStartTime()
            return 0
        end

        function duration:GetEndTime()
            return self.totalDuration
        end

        return duration
    end

    local function default_track_list()
        return {
            {
                id = Enum.EncounterTimelineTrack.Queued,
                type = Enum.EncounterTimelineTrackType.Sorted,
                minimumDuration = 0,
                maximumDuration = 0,
                maximumEventCount = 3,
            },
            {
                id = Enum.EncounterTimelineTrack.Short,
                type = Enum.EncounterTimelineTrackType.Linear,
                minimumDuration = 0,
                maximumDuration = 10,
            },
            {
                id = Enum.EncounterTimelineTrack.Medium,
                type = Enum.EncounterTimelineTrackType.Linear,
                minimumDuration = 10,
                maximumDuration = 30,
            },
            {
                id = Enum.EncounterTimelineTrack.Long,
                type = Enum.EncounterTimelineTrackType.Sorted,
                minimumDuration = 30,
                maximumDuration = 120,
                maximumEventCount = 3,
            },
        }
    end

    local function copy_track_list(track_list)
        local copy = {}
        for index, track in ipairs(track_list) do
            local track_copy = {}
            for key, value in pairs(track) do
                track_copy[key] = value
            end
            copy[index] = track_copy
        end
        return copy
    end

    local function fire_event(event_name, ...)
        if type(FireEvent) == "function" then
            pcall(FireEvent, event_name, ...)
        end
    end

    api._state = api._state or {}
    local state = api._state

    state.featureAvailable = state.featureAvailable ~= false
    state.featureEnabled = state.featureEnabled ~= false
    state.currentTime = tonumber(state.currentTime) or 2.0
    state.highlightTime = tonumber(state.highlightTime) or 5.0
    state.viewType = tonumber(state.viewType) or Enum.EncounterTimelineViewType.Timeline
    state.trackList = type(state.trackList) == "table" and state.trackList or default_track_list()
    state.eventList = type(state.eventList) == "table" and state.eventList or { 1 }
    state.eventInfo = type(state.eventInfo) == "table" and state.eventInfo or {
        [1] = {
            id = 1,
            spellID = 19750,
            spellName = "Flash of Light",
            iconFileID = "Interface\\Icons\\Spell_Holy_FlashHeal",
            icons = 0,
            severity = Enum.EncounterEventSeverity.Medium,
            color = make_color(1.0, 0.82, 0.0, 1.0),
        },
    }
    state.eventState = type(state.eventState) == "table" and state.eventState or {
        [1] = Enum.EncounterTimelineEventState.Active,
    }
    state.eventTrack = type(state.eventTrack) == "table" and state.eventTrack or {
        [1] = { Enum.EncounterTimelineTrack.Short, 1 },
    }
    state.eventBlocked = type(state.eventBlocked) == "table" and state.eventBlocked or {
        [1] = false,
    }
    state.eventTimers = type(state.eventTimers) == "table" and state.eventTimers or {
        [1] = create_duration(12.0, state.currentTime),
    }

    local function get_track_info(track_id)
        for _, track in ipairs(state.trackList) do
            if track.id == track_id then
                return track
            end
        end
        return nil
    end

    local function is_terminal_state(event_state)
        return event_state == Enum.EncounterTimelineEventState.Finished
            or event_state == Enum.EncounterTimelineEventState.Canceled
    end

    local function has_visible_event()
        for _, event_id in ipairs(state.eventList) do
            local event_state = state.eventState[event_id]
            local track_data = get_track_info((state.eventTrack[event_id] or {})[1])
            if not is_terminal_state(event_state)
                and track_data
                and track_data.type ~= Enum.EncounterTimelineTrackType.Hidden
            then
                return true
            end
        end
        return false
    end

    function api.IsFeatureAvailable()
        return state.featureAvailable
    end

    function api.IsFeatureEnabled()
        return state.featureEnabled
    end

    function api.GetEventList()
        local event_list = {}
        for index, event_id in ipairs(state.eventList) do
            event_list[index] = event_id
        end
        return event_list
    end

    function api.GetEventInfo(event_id)
        return state.eventInfo[event_id]
    end

    function api.GetEventState(event_id)
        return state.eventState[event_id]
    end

    function api.GetEventTimer(event_id)
        return state.eventTimers[event_id]
    end

    function api.GetEventTrack(event_id)
        local track = state.eventTrack[event_id]
        if not track then
            return Enum.EncounterTimelineTrack.Indeterminate, 0
        end
        return track[1], track[2]
    end

    function api.GetEventHighlightTime()
        return state.highlightTime
    end

    function api.GetEventTimeRemaining(event_id)
        local timer = api.GetEventTimer(event_id)
        return timer and timer:GetRemainingDuration() or nil
    end

    function api.IsEventBlocked(event_id)
        return state.eventBlocked[event_id] == true
    end

    function api.HasActiveEvents()
        for _, event_id in ipairs(state.eventList) do
            if state.eventState[event_id] == Enum.EncounterTimelineEventState.Active then
                return true
            end
        end
        return false
    end

    function api.HasPausedEvents()
        for _, event_id in ipairs(state.eventList) do
            if state.eventState[event_id] == Enum.EncounterTimelineEventState.Paused then
                return true
            end
        end
        return false
    end

    function api.HasVisibleEvents()
        return has_visible_event()
    end

    function api.GetTrackList()
        return copy_track_list(state.trackList)
    end

    function api.GetTrackType(track_id)
        local track = get_track_info(track_id)
        if not track then
            return Enum.EncounterTimelineTrackType.Hidden
        end
        return track.type
    end

    function api.GetViewType()
        return state.viewType
    end

    function api.SetViewType(view_type)
        local next_view_type = tonumber(view_type) or Enum.EncounterTimelineViewType.None
        local previous_view_type = state.viewType
        if previous_view_type == next_view_type then
            return
        end

        if previous_view_type and previous_view_type ~= Enum.EncounterTimelineViewType.None then
            fire_event("ENCOUNTER_TIMELINE_VIEW_DEACTIVATED", previous_view_type)
        end

        state.viewType = next_view_type

        if next_view_type ~= Enum.EncounterTimelineViewType.None then
            fire_event("ENCOUNTER_TIMELINE_VIEW_ACTIVATED", next_view_type)
        end
    end

    function api.GetCurrentTime()
        return state.currentTime
    end

    function api.AddEditModeEvents()
        return 30.0
    end

    function api.CancelEditModeEvents()
    end

    function api.SetEventIconTextures(event_id, icon_mask, textures)
        local event_info = api.GetEventInfo(event_id)
        local should_show = type(event_info) == "table"
            and bit.band(event_info.icons or 0, icon_mask or 0) ~= 0

        if type(textures) ~= "table" then
            return
        end

        for _, texture in ipairs(textures) do
            if texture and texture.SetShown then
                texture:SetShown(should_show)
            end
            if should_show and texture and texture.SetTexture then
                texture:SetTexture("Interface\\RaidFrame\\ReadyCheck-Ready")
            end
        end
    end
"#;

/// Global-to-namespace alias pairs: (global_name, C_CombatLog method name).
const COMBAT_LOG_ALIASES: &[(&str, &str)] = &[
    ("CombatLogAddFilter", "AddEventFilter"),
    ("CombatLogGetCurrentEntry", "GetCurrentEntryInfo"),
    ("CombatLogGetCurrentEventInfo", "GetCurrentEventInfo"),
    ("CombatLogGetNumEntries", "GetEntryCount"),
    ("CombatLogAdvanceEntry", "AdvanceEntry"),
    ("CombatLogSetCurrentEntry", "SetCurrentEntry"),
    ("CombatLogShowCurrentEntry", "ShowCurrentEntry"),
    ("CombatLogResetFilter", "ResetFilter"),
    ("CombatLogClearEntries", "ClearEntries"),
    ("CombatLogGetRetentionTime", "GetRetentionTime"),
    ("CombatLogSetRetentionTime", "SetRetentionTime"),
];

/// Re-alias CombatLog* globals to the same function objects stored in C_CombatLog.
///
/// Wowless's cfuncs uniqueChecker requires that alias pairs (e.g. "C_CombatLog.AddEventFilter"
/// and "CombatLogAddFilter") share the same underlying C function pointer. This function
/// overwrites separately-created global stubs with direct references to the namespace functions.
/// Must be called after all CombatLog registrations complete.
pub fn fixup_combat_log_aliases(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let Ok(cl) = g.get::<mlua::Table>("C_CombatLog") else {
        return Ok(());
    };
    apply_aliases(lua, g, &cl, COMBAT_LOG_ALIASES)
}

/// Apply alias pairs from a namespace table to globals, with fallback no-op on missing methods.
fn apply_aliases(
    lua: &Lua,
    g: &mlua::Table,
    ns: &mlua::Table,
    pairs: &[(&str, &str)],
) -> Result<()> {
    for &(global_name, method_name) in pairs {
        let f = match ns.get::<mlua::Function>(method_name) {
            Ok(f) => f,
            Err(_) => lua.create_function(|_, _: MultiValue| Ok(()))?,
        };
        g.set(global_name, f)?;
    }
    Ok(())
}
