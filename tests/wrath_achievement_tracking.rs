//! Build-12340 tracking contracts from the laptop's 2026-10-21 static
//! reconstruction. Original AchievementUI event consumers remain installed.

#![cfg(feature = "client-wrath")]

use crate::common::wrath_achievement_fixtures::with_native;

#[test]
fn native_tracking_mutations_deliver_ordered_state_inline() {
    assert!(with_native(|env| {
        env.exec(
            r##"
            local events = {}
            local phase = "outside"
            local observer = CreateFrame("Frame")
            observer:RegisterEvent("TRACKED_ACHIEVEMENT_UPDATE")
            observer:SetScript("OnEvent", function(self, event, ...)
                assert(phase == "inside", "tracking event must precede mutator return")
                assert(select("#", ...) == 1)
                local id = ...
                assert(type(id) == "number")
                local ids = {GetTrackedAchievements()}
                assert(GetNumTrackedAchievements() == #ids)
                events[#events + 1] = id .. ":" .. table.concat(ids, ",")
            end)
            assert(select("#", GetTrackedAchievements()) == 0)
            phase = "inside"
            assert(select("#", AddTrackedAchievement(7)) == 0)
            AddTrackedAchievement(6)
            AddTrackedAchievement(7)
            phase = "outside"
            assert(table.concat(events, "|") == "7:7|6:7,6|7:7,6")
            assert(IsTrackedAchievement(7) and IsTrackedAchievement(6))
            phase = "inside"
            assert(select("#", RemoveTrackedAchievement(7)) == 0)
            RemoveTrackedAchievement(891)
            AddTrackedAchievement(7)
            phase = "outside"
            assert(table.concat(events, "|") == "7:7|6:7,6|7:7,6|7:6|7:6,7")
            -- The original consumer's private cache must already contain both IDs.
            phase = "inside"
            AchievementButton_ToggleTracking(6)
            phase = "outside"
            assert(GetNumTrackedAchievements() == 1 and GetTrackedAchievements() == 7)
            assert(events[#events] == "6:7")
            "##,
        )
        .expect("native tracking should expose ordered state before inline events");
    }));
}

#[test]
fn native_tracking_validates_arguments_and_enforces_ten_entries() {
    assert!(with_native(|env| {
        let fixture = env.state().borrow().achievements[&6].clone();
        for id in 6001..=6011 {
            let mut record = fixture.clone();
            record.achievement_id = id;
            record.name = format!("Supplied tracking fixture {id}");
            env.state().borrow_mut().achievements.insert(id, record);
        }
        env.exec(
            r##"
            local eventCount = 0
            local observer = CreateFrame("Frame")
            observer:RegisterEvent("TRACKED_ACHIEVEMENT_UPDATE")
            observer:SetScript("OnEvent", function() eventCount = eventCount + 1 end)
            A_Admin.SetAchievementEarned(6001, true)
            for id = 6001, 6010 do AddTrackedAchievement(id) end
            assert(GetNumTrackedAchievements() == 10 and eventCount == 10)
            assert(IsTrackedAchievement(6001), "raw add accepts completed achievements")
            AddTrackedAchievement(6011)
            assert(GetNumTrackedAchievements() == 10 and eventCount == 10)
            AddTrackedAchievement("6001.9")
            assert(GetNumTrackedAchievements() == 10 and eventCount == 11)
            RemoveTrackedAchievement(6002)
            AddTrackedAchievement(6011)
            assert(GetNumTrackedAchievements() == 10 and eventCount == 13)
            local ids = {GetTrackedAchievements("ignored")}
            assert(ids[1] == 6001 and ids[2] == 6003 and ids[10] == 6011)
            assert(not IsTrackedAchievement(6002))
            assert(IsTrackedAchievement("6001.9"))
            assert(not IsTrackedAchievement(nil) and not IsTrackedAchievement("bad"))
            assert(not IsTrackedAchievement(0) and not IsTrackedAchievement(999999))
            for _, value in ipairs({0, "bad", 999999}) do
                local ok, message = pcall(AddTrackedAchievement, value)
                assert(not ok and string.find(message, "Usage: AddTrackedAchievement(achievementID)", 1, true))
            end
            local ok, message = pcall(AddTrackedAchievement)
            assert(not ok and string.find(message, "Usage: AddTrackedAchievement(achievementID)", 1, true))
            ok, message = pcall(RemoveTrackedAchievement, 0)
            assert(not ok and string.find(message, "Usage: RemoveTrackedAchievement(achievementID)", 1, true))
            RemoveTrackedAchievement(999999)
            assert(eventCount == 13 and GetNumTrackedAchievements() == 10)
            "##,
        )
        .expect("native tracking should validate IDs and retain its bounded ordered list");
    }));
}
