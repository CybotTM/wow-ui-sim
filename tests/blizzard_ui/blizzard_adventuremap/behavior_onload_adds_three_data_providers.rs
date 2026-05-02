//! `AdventureMapMixin:OnLoad` data-provider setup behavior.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";

#[test]
fn adventure_map_onload_adds_standard_data_providers_in_order() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface: ProviderOrderSurface = env
            .eval(
                r#"
                local originalMapCanvasOnLoad = MapCanvasMixin.OnLoad
                local originalCreateFramePool = CreateFramePool
                MapCanvasMixin.OnLoad = function(self)
                    self.dataProviders = {}
                end
                CreateFramePool = function()
                    return {}
                end

                local receiver = {
                    registeredEvents = {},
                    dataProviders = {},
                    BorderFrame = {
                        TitleText = { SetText = function() end },
                        Bg = {
                            SetColorTexture = function() end,
                            SetParent = function() end,
                        },
                        TopTileStreaks = { Hide = function() end },
                        SetPortraitToAsset = function() end,
                    },
                    GetCanvas = function()
                        return {}
                    end,
                    SetMapInsetPool = function(self, pool)
                        self.mapInsetPool = pool
                    end,
                    RegisterEvent = function(self, event)
                        self.registeredEvents[event] = true
                    end,
                    AddDataProvider = function(self, provider)
                        table.insert(self.dataProviders, provider)
                    end,
                }

                Mixin(receiver, AdventureMapMixin)
                AdventureMapMixin.OnLoad(receiver)

                MapCanvasMixin.OnLoad = originalMapCanvasOnLoad
                CreateFramePool = originalCreateFramePool

                local providers = receiver.dataProviders
                return #providers,
                       providers[1] and providers[1].OnAdded == AdventureMap_QuestChoiceDataProviderMixin.OnAdded,
                       providers[2] and providers[2].OnAdded == AdventureMap_QuestOfferDataProviderMixin.OnAdded,
                       providers[3] and providers[3].OnAdded == QuestSessionDataProviderMixin.OnAdded,
                       providers[4] == nil
                "#,
            )
            .expect("AdventureMap OnLoad data-provider probe must run cleanly");

        assert_provider_order(surface);
    });
}

type ProviderOrderSurface = (i64, bool, bool, bool, bool);

fn assert_provider_order(surface: ProviderOrderSurface) {
    let (
        provider_count,
        first_is_quest_choice,
        second_is_quest_offer,
        third_is_quest_session,
        no_extra_provider,
    ) = surface;

    assert_eq!(
        provider_count, 3,
        "`AdventureMapMixin:OnLoad` must add exactly three standard data providers"
    );
    assert!(
        first_is_quest_choice,
        "first data provider must use `AdventureMap_QuestChoiceDataProviderMixin`"
    );
    assert!(
        second_is_quest_offer,
        "second data provider must use `AdventureMap_QuestOfferDataProviderMixin`"
    );
    assert!(
        third_is_quest_session,
        "third data provider must use `QuestSessionDataProviderMixin`"
    );
    assert!(
        no_extra_provider,
        "`AdventureMapMixin:OnLoad` must not add extra standard data providers"
    );
}
