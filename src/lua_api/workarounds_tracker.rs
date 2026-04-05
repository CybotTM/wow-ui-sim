use super::WowLuaEnv;

/// Pre-event objective tracker setup: hide empty frames and configure the
/// tracker frame container.
///
/// Module registration happens automatically when PLAYER_ENTERING_WORLD and
/// VARIABLES_LOADED fire (via EventUtil.ContinueAfterAllEvents → Init).
/// Post-event work (quest title callbacks, height fix) runs in
/// `finish_objective_tracker`.
pub(crate) fn init_objective_tracker(env: &WowLuaEnv) {
    hide_empty_managed_frames(env);
    setup_tracker_frame(env);
}

/// Post-event objective tracker setup: ensure modules are registered,
/// populate quest titles, update the quest module, and force height.
///
/// By this point, PLAYER_ENTERING_WORLD and VARIABLES_LOADED have fired,
/// triggering ObjectiveTrackerManager:Init() via ContinueAfterAllEvents.
/// If Init didn't run (e.g. EventRegistry dispatch failed), we call it here.
pub(crate) fn finish_objective_tracker(env: &WowLuaEnv) {
    ensure_tracker_initialized(env);
    populate_quest_titles(env);
}

/// Show ChatFrame1 and set DEFAULT_CHAT_FRAME after addon loading.
pub(crate) fn show_chat_frame(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if ChatFrame1 then
            ChatFrame1:Show()
            DEFAULT_CHAT_FRAME = ChatFrame1
            ChatFrame1:ClearAllPoints()
            ChatFrame1:SetPoint("BOTTOMLEFT", UIParent, "BOTTOMLEFT", 32, 32)
            ChatFrame1:SetSize(430, 120)
            ChatFrame1.oldAlpha = ChatFrame1.oldAlpha or DEFAULT_CHATFRAME_ALPHA or 0.3
        end
    "#,
    );
    start_fake_chat(env);
}

/// Initialize r,g,b on ChatTypeInfo entries.
pub(crate) fn init_chat_type_colors(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not ChatTypeInfo then return end
        local defaults = {
            SYSTEM={1,1,0}, SAY={1,1,1}, PARTY={.67,.67,1}, RAID={1,.5,0},
            GUILD={.25,1,.25}, OFFICER={.25,.75,.25}, YELL={1,.25,.25},
            WHISPER={1,.5,1}, WHISPER_INFORM={1,.5,1}, EMOTE={1,.5,.25},
            TEXT_EMOTE={1,.5,.25}, CHANNEL={1,.75,.5}, LOOT={0,.67,0},
            MONEY={1,1,0}, SKILL={.33,.33,1}, ACHIEVEMENT={1,1,0},
            GUILD_ACHIEVEMENT={.25,1,.25}, BN_WHISPER={0,.8,1},
            BN_WHISPER_INFORM={0,.8,1}, INSTANCE_CHAT={1,.5,0},
            INSTANCE_CHAT_LEADER={1,.5,0},
        }
        for key, info in pairs(ChatTypeInfo) do
            if not info.r then
                local d = defaults[key] or {1, 1, 1}
                info.r, info.g, info.b = d[1], d[2], d[3]
            end
        end
    "#,
    );
}

fn hide_empty_managed_frames(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local frames = { "BossTargetFrameContainer", "DurabilityFrame" }
        for _, name in ipairs(frames) do
            local f = _G[name]
            if f and f.Hide then
                f:Hide()
                -- Prevent OnShow from re-showing during events
                f.ignoreInLayout = true
            end
        end
    "#,
    );
}

fn setup_tracker_frame(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local otf = ObjectiveTrackerFrame
        if not otf then return end
        -- Ensure layoutIndex is set (should come from XML KeyValue but may need fallback)
        if not otf.layoutIndex then otf.layoutIndex = 50 end
        -- AddManagedFrame checks IsInDefaultPosition() and skips frames not in
        -- default position. Since EditMode isn't initialized, the mixin's
        -- IsInDefaultPosition() returns false. Override so the container accepts it.
        otf.IsInDefaultPosition = function() return true end
        otf:Show()
        -- Explicitly add to the managed frame container. The OnShow handler
        -- may not fire correctly, so call AddManagedFrame directly.
        -- This reparents OTF into the container and calls Layout() to set anchors.
        local lp = otf.layoutParent
        if lp and lp.AddManagedFrame then
            pcall(lp.AddManagedFrame, lp, otf)
        end
        -- Compute height from container height minus OTF's vertical offset.
        -- UpdateHeight() does parentHeight + offsetY, but calling it triggers
        -- layout cycles. Compute it directly instead.
        local _, _, _, _, offsetY = otf:GetPoint(1)
        if offsetY and lp then
            local h = lp:GetHeight() + offsetY
            if h < 100 then h = 400 end
            otf:SetHeight(h)
        end
    "#,
    );
}

fn ensure_tracker_initialized(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not ObjectiveTrackerManager or not ObjectiveTrackerManager.Init then
            return
        end
        -- Only call Init if it hasn't run (modules not registered)
        local qt = QuestObjectiveTracker
        if qt and not qt.parentContainer then
            pcall(ObjectiveTrackerManager.Init, ObjectiveTrackerManager)
        end
    "#,
    );
}

fn populate_quest_titles(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if QuestEventListener and QuestEventListener.FireCallbacks then
            for _, qid in ipairs({80000, 80001, 80002}) do
                pcall(QuestEventListener.FireCallbacks, QuestEventListener, qid)
            end
        end
        -- Update quest module directly (bypass container loop which
        -- crashes on MawBuffs/ScenarioObjectiveTracker stubs)
        local qt = QuestObjectiveTracker
        if not qt then return end
        if not qt.parentContainer then return end
        local c = qt.parentContainer
        local avail = c:GetAvailableHeight()
        pcall(qt.Update, qt, avail, false)
        local h = qt.contentsHeight or 0
        if h > 0 then
            qt:SetHeight(h + (qt.bottomSpacing or 0))
            qt:ClearAllPoints()
            qt:SetPoint("TOP", c, "TOP", 0, -(c.topModulePadding or 0))
            qt:SetPoint("LEFT", c, "LEFT", qt.leftMargin or 0, 0)
            qt:Show()
        end
    "#,
    );
}

fn start_fake_chat(env: &WowLuaEnv) {
    register_fake_chat_data(env);
    schedule_fake_chat_tickers(env);
}

fn register_fake_chat_data(env: &WowLuaEnv) {
    register_fake_chat_messages(env);
    register_fake_chat_names(env);
}

fn register_fake_chat_messages(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not ChatFrame1 then return end
        _FakeChat = { msgs = {}, names = {}, idx = {} }
        _FakeChat.msgs.general = {
            "Anyone know where the portal trainer is?",
            "LFM Deadmines, need healer",
            "WTS [Copper Bar] x20, 5g each",
            "How do I get to Ironforge from here?",
            "Is the Darkmoon Faire up this week?",
            "Just hit level 60!",
            "What's the fastest way to level cooking?",
            "Any good guilds recruiting?",
        }
        _FakeChat.msgs.trade = {
            "WTS [Enchant Weapon - Crusader] your mats + 10g tip",
            "WTB [Large Brilliant Shard] x5, paying 3g each",
            "LF Blacksmith to craft [Arcanite Reaper], have mats",
            "WTS [Flask of the Titans] 45g, cheap!",
            "WTB [Righteous Orb] x2, PST with price",
            "Selling port to Dalaran, 1g",
        }
        _FakeChat.msgs.say = {
            "Anyone else lagging?", "Thanks for the group!",
            "Where did that quest NPC go?",
            "I think I took a wrong turn somewhere",
            "Wow, this place is huge", "Can someone help with this elite?",
        }
        _FakeChat.msgs.guild = {
            "Hey everyone!", "Anyone up for a dungeon run?",
            "Grats on the new gear!", "Guild bank has some free enchanting mats",
            "Raid signup is up on the calendar",
            "I just finished the attunement quest chain",
        }
    "#,
    );
}

fn register_fake_chat_names(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not _FakeChat then return end
        _FakeChat.names.general = {"Thunderfury", "Moonwhisper", "Stabbymcstab", "Healbot", "Tanklord"}
        _FakeChat.names.trade = {"Goldmaker", "Craftypants", "Auctioneer", "Bankalt"}
        _FakeChat.names.say = {"Legolas", "Arthasdklol", "Pwnstar", "Noobslayer"}
        _FakeChat.names.guild = {"Valorheart", "Shieldmaiden", "Firestorm", "Arcanewing"}
        _FakeChat.idx = {general = 1, trade = 1, say = 1, guild = 1}
        function _FakeChat:pick(channel)
            local list = self.msgs[channel]
            local i = self.idx[channel]
            self.idx[channel] = (i % #list) + 1
            return list[i], self.names[channel][math.random(#self.names[channel])]
        end
    "#,
    );
}

fn schedule_fake_chat_tickers(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not _FakeChat then return end
        local fc = _FakeChat
        local function timestamp()
            local fmt = GetCVar and GetCVar("showTimestamps")
            if fmt and fmt ~= "" and fmt ~= "none" then
                return date(fmt, time())
            end
            return ""
        end
        local function post(channel, prefix, r, g, b)
            local msg, name = fc:pick(channel)
            ChatFrame1:_AddMessageSilent(timestamp() .. prefix ..
                "|Hplayer:" .. name .. "|h[" .. name .. "]|h: " .. msg,
                r, g, b)
        end
        -- General (0s offset, light orange)
        C_Timer.After(0, function() C_Timer.NewTicker(40, function()
            post("general", "|Hchannel:General|h[1. General]|h ", 1.0, 0.75, 0.5)
        end) end)
        -- Trade (5s offset, light orange)
        C_Timer.After(5, function() C_Timer.NewTicker(40, function()
            post("trade", "|Hchannel:Trade|h[2. Trade]|h ", 1.0, 0.75, 0.5)
        end) end)
        -- Say (10s offset, white — uses "says:" format)
        C_Timer.After(10, function() C_Timer.NewTicker(40, function()
            local msg, name = fc:pick("say")
            ChatFrame1:_AddMessageSilent(
                timestamp() .. "|Hplayer:" .. name .. "|h[" .. name .. "]|h says: " .. msg,
                1.0, 1.0, 1.0)
        end) end)
        -- Guild (15s offset, green)
        C_Timer.After(15, function() C_Timer.NewTicker(40, function()
            post("guild", "|Hchannel:Guild|h[Guild]|h ", 0.25, 1.0, 0.25)
        end) end)
    "#,
    );
}
