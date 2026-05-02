//! Quest-choice data provider registers its exact event set when added.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const EXPECTED_EVENT_COUNT: i64 = 3;
const QUEST_CHOICE_EVENTS: &[&str] = &[
    "ADVENTURE_MAP_UPDATE_POIS",
    "ADVENTURE_MAP_QUEST_UPDATE",
    "QUEST_ACCEPTED",
];

#[test]
fn quest_choice_provider_on_added_registers_exact_events() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface: ProviderEventSurface = env
            .eval(
                r#"
                local mapCanvas = {
                    dataProviders = {},
                    mapEventRegistrations = {},
                    AddDataProvider = MapCanvasMixin.AddDataProvider,
                    AddDataProviderEvent = function(self, event)
                        self.mapEventRegistrations[event] = (self.mapEventRegistrations[event] or 0) + 1
                    end,
                }
                local provider = CreateFromMixins(AdventureMap_QuestChoiceDataProviderMixin)

                mapCanvas:AddDataProvider(provider)

                local providerEventCount = 0
                local unexpectedProviderEvent = nil
                for event in pairs(provider.registeredEvents or {}) do
                    providerEventCount = providerEventCount + 1
                    if event ~= "ADVENTURE_MAP_UPDATE_POIS"
                        and event ~= "ADVENTURE_MAP_QUEST_UPDATE"
                        and event ~= "QUEST_ACCEPTED" then
                        unexpectedProviderEvent = event
                    end
                end

                local mapRegistrationCount = 0
                for _, count in pairs(mapCanvas.mapEventRegistrations) do
                    mapRegistrationCount = mapRegistrationCount + count
                end

                return providerEventCount,
                       provider.registeredEvents["ADVENTURE_MAP_UPDATE_POIS"] == true,
                       provider.registeredEvents["ADVENTURE_MAP_QUEST_UPDATE"] == true,
                       provider.registeredEvents["QUEST_ACCEPTED"] == true,
                       unexpectedProviderEvent,
                       provider.owningMap == mapCanvas,
                       mapRegistrationCount
                "#,
            )
            .expect("AdventureMap quest-choice provider event probe must run cleanly");

        assert_provider_event_surface(surface);
    });
}

type ProviderEventSurface = (i64, bool, bool, bool, Option<String>, bool, i64);

fn assert_provider_event_surface(surface: ProviderEventSurface) {
    let (
        provider_event_count,
        update_pois_registered,
        quest_update_registered,
        quest_accepted_registered,
        unexpected_provider_event,
        owning_map_set,
        map_registration_count,
    ) = surface;

    assert_exact_provider_events(
        provider_event_count,
        update_pois_registered,
        quest_update_registered,
        quest_accepted_registered,
        unexpected_provider_event,
    );
    assert_provider_added_to_map(owning_map_set, map_registration_count);
}

fn assert_exact_provider_events(
    provider_event_count: i64,
    update_pois_registered: bool,
    quest_update_registered: bool,
    quest_accepted_registered: bool,
    unexpected_provider_event: Option<String>,
) {
    assert_eq!(
        provider_event_count, EXPECTED_EVENT_COUNT,
        "`AdventureMap_QuestChoiceDataProviderMixin:OnAdded` must register exactly three events"
    );
    assert!(
        update_pois_registered,
        "`AdventureMap_QuestChoiceDataProviderMixin:OnAdded` must register `{}`",
        QUEST_CHOICE_EVENTS[0]
    );
    assert!(
        quest_update_registered,
        "`AdventureMap_QuestChoiceDataProviderMixin:OnAdded` must register `{}`",
        QUEST_CHOICE_EVENTS[1]
    );
    assert!(
        quest_accepted_registered,
        "`AdventureMap_QuestChoiceDataProviderMixin:OnAdded` must register `{}`",
        QUEST_CHOICE_EVENTS[2]
    );
    assert_eq!(
        unexpected_provider_event, None,
        "`AdventureMap_QuestChoiceDataProviderMixin:OnAdded` must not register extra events"
    );
}

fn assert_provider_added_to_map(owning_map_set: bool, map_registration_count: i64) {
    assert!(
        owning_map_set,
        "`MapCanvasMixin:AddDataProvider` must call `OnAdded` with the owning map"
    );
    assert_eq!(
        map_registration_count, EXPECTED_EVENT_COUNT,
        "`RegisterEvent` must forward each provider event to the owning map"
    );
}
