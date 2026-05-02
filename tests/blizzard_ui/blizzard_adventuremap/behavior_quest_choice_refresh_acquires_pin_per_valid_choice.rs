//! Quest-choice refresh adds one choice pin and one fog pin per valid choice.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AdventureMapZoneChoice;

const ROOT: &str = "Blizzard_AdventureMap";
const VALID_A_QUEST_ID: i64 = 40519;
const COMPLETED_QUEST_ID: i64 = 40520;
const VALID_B_QUEST_ID: i64 = 40521;
const NIL_COORDS_QUEST_ID: i64 = 40522;
const EXPECTED_VALID_CHOICE_COUNT: i64 = 2;

#[test]
fn quest_choice_refresh_acquires_choice_and_fog_pin_per_valid_choice() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_zone_choices(env);

        let surface: QuestChoiceRefreshSurface = env
            .eval(
                r#"
                local originalGetZoneChoiceInfo = C_AdventureMap.GetZoneChoiceInfo
                C_AdventureMap.GetZoneChoiceInfo = function(choiceIndex)
                    if choiceIndex == 4 then
                        local questID, textureKit, name, zoneDescription, normalizedX =
                            originalGetZoneChoiceInfo(choiceIndex)
                        return questID, textureKit, name, zoneDescription, normalizedX, nil
                    end
                    return originalGetZoneChoiceInfo(choiceIndex)
                end

                local acquired = {}
                local mapCanvas = {
                    removedChoicePins = 0,
                    removedFogPins = 0,
                    RemoveAllPinsByTemplate = function(self, template)
                        if template == "AdventureMap_QuestChoicePinTemplate" then
                            self.removedChoicePins = self.removedChoicePins + 1
                        elseif template == "AdventureMap_FogPinTemplate" then
                            self.removedFogPins = self.removedFogPins + 1
                        end
                    end,
                    AcquirePin = function(self, template, playRevealAnims)
                        local pin = { template = template, playRevealAnims = playRevealAnims }
                        pin.Text = { SetText = function(_, text) pin.text = text end }
                        pin.Icon = {
                            SetAtlas = function(_, atlas) pin.iconAtlas = atlas end,
                            SetSize = function() end,
                        }
                        pin.IconHighlight = {
                            SetAtlas = function() end,
                            SetSize = function() end,
                        }
                        pin.SetPosition = function(self, normalizedX, normalizedY)
                            self.normalizedX = normalizedX
                            self.normalizedY = normalizedY
                        end
                        table.insert(acquired, pin)
                        return pin
                    end,
                    ZoomOut = function(self)
                        self.zoomOutCount = (self.zoomOutCount or 0) + 1
                    end,
                }
                local provider = CreateFromMixins(AdventureMap_QuestChoiceDataProviderMixin)
                provider.owningMap = mapCanvas

                provider:RefreshAllData(false)

                C_AdventureMap.GetZoneChoiceInfo = originalGetZoneChoiceInfo

                local choicePinCount = 0
                local fogPinCount = 0
                local validAChoicePin = false
                local completedChoicePin = false
                local validBChoicePin = false
                local nilCoordsChoicePin = false
                local firstChoiceX = nil
                local firstChoiceY = nil

                for _, pin in ipairs(acquired) do
                    if pin.template == "AdventureMap_QuestChoicePinTemplate" then
                        choicePinCount = choicePinCount + 1
                        if pin.questID == 40519 then
                            validAChoicePin = true
                            firstChoiceX = pin.normalizedX
                            firstChoiceY = pin.normalizedY
                        elseif pin.questID == 40520 then
                            completedChoicePin = true
                        elseif pin.questID == 40521 then
                            validBChoicePin = true
                        elseif pin.questID == 40522 then
                            nilCoordsChoicePin = true
                        end
                    elseif pin.template == "AdventureMap_FogPinTemplate" then
                        fogPinCount = fogPinCount + 1
                    end
                end

                return choicePinCount,
                       fogPinCount,
                       validAChoicePin,
                       completedChoicePin,
                       validBChoicePin,
                       nilCoordsChoicePin,
                       firstChoiceX,
                       firstChoiceY,
                       mapCanvas.removedChoicePins,
                       mapCanvas.removedFogPins,
                       provider.playRevealAnims == false
                "#,
            )
            .expect("AdventureMap quest-choice RefreshAllData probe must run cleanly");

        assert_quest_choice_refresh_surface(surface);
    });
}

type QuestChoiceRefreshSurface = (i64, i64, bool, bool, bool, bool, f64, f64, i64, i64, bool);

fn seed_zone_choices(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state
        .quest_log_entries
        .completed_quest_ids
        .insert(COMPLETED_QUEST_ID as i32);
    state.adventure_map.zone_choices = vec![
        zone_choice(VALID_A_QUEST_ID, "Azsuna", 0.31, 0.55),
        zone_choice(COMPLETED_QUEST_ID, "Completed", 0.42, 0.66),
        zone_choice(VALID_B_QUEST_ID, "Highmountain", 0.73, 0.28),
        zone_choice(NIL_COORDS_QUEST_ID, "Missing", 0.88, 0.91),
    ];
}

fn zone_choice(
    quest_id: i64,
    name: &str,
    normalized_x: f64,
    normalized_y: f64,
) -> AdventureMapZoneChoice {
    AdventureMapZoneChoice {
        quest_id,
        texture_kit: "alliance".to_string(),
        name: name.to_string(),
        zone_description: format!("{name} description"),
        normalized_x,
        normalized_y,
    }
}

fn assert_quest_choice_refresh_surface(surface: QuestChoiceRefreshSurface) {
    let (
        choice_pin_count,
        fog_pin_count,
        valid_a_choice_pin,
        completed_choice_pin,
        valid_b_choice_pin,
        nil_coords_choice_pin,
        first_choice_x,
        first_choice_y,
        removed_choice_pins,
        removed_fog_pins,
        play_reveal_anims_cleared,
    ) = surface;

    assert_valid_pin_counts(choice_pin_count, fog_pin_count);
    assert_valid_choice_filters(
        valid_a_choice_pin,
        completed_choice_pin,
        valid_b_choice_pin,
        nil_coords_choice_pin,
    );
    assert_first_choice_position(first_choice_x, first_choice_y);
    assert_refresh_cleanup(
        removed_choice_pins,
        removed_fog_pins,
        play_reveal_anims_cleared,
    );
}

fn assert_valid_pin_counts(choice_pin_count: i64, fog_pin_count: i64) {
    assert_eq!(
        choice_pin_count, EXPECTED_VALID_CHOICE_COUNT,
        "`RefreshAllData` must acquire one choice pin per valid zone choice"
    );
    assert_eq!(
        fog_pin_count, EXPECTED_VALID_CHOICE_COUNT,
        "`RefreshAllData` must acquire one fog pin per valid zone choice"
    );
}

fn assert_valid_choice_filters(
    valid_a_choice_pin: bool,
    completed_choice_pin: bool,
    valid_b_choice_pin: bool,
    nil_coords_choice_pin: bool,
) {
    assert!(
        valid_a_choice_pin,
        "first valid choice must acquire a choice pin"
    );
    assert!(
        !completed_choice_pin,
        "completed quest choices must not acquire pins"
    );
    assert!(
        valid_b_choice_pin,
        "second valid choice must acquire a choice pin"
    );
    assert!(
        !nil_coords_choice_pin,
        "choices with nil coordinates must not acquire pins"
    );
}

fn assert_first_choice_position(first_choice_x: f64, first_choice_y: f64) {
    assert_approx_eq(first_choice_x, 0.31, "choice pin must keep normalized X");
    assert_approx_eq(first_choice_y, 0.55, "choice pin must keep normalized Y");
}

fn assert_refresh_cleanup(
    removed_choice_pins: i64,
    removed_fog_pins: i64,
    play_reveal_anims_cleared: bool,
) {
    assert_eq!(
        removed_choice_pins, 1,
        "`RefreshAllData` must clear old choice pins before refreshing"
    );
    assert_eq!(
        removed_fog_pins, 1,
        "`RefreshAllData` must clear old fog pins before refreshing"
    );
    assert!(
        play_reveal_anims_cleared,
        "non-OnShow refresh must clear reveal-animation mode after refreshing"
    );
}

fn assert_approx_eq(actual: f64, expected: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}
