use wow_ui_sim::lua_api::WowLuaEnv;

const WORLD_MAP_CATA_LUA: &str = include_str!(
    "../Interface/BlizzardUI/Mists/AddOns/Blizzard_WorldMap/Cata/Blizzard_WorldMap.lua"
);

#[test]
fn world_map_opacity_default_feeds_set_opacity() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    env.exec(WORLD_MAP_CATA_LUA)
        .expect("Cata/Mists WorldMap Lua should define opacity helpers");

    let (opacity, ok, err, frame_alpha, scroll_alpha, quest_alpha): (
        String,
        bool,
        String,
        f64,
        f64,
        f64,
    ) = env
        .eval(
            r#"
            WorldMapFrame = {
                SetAlpha = function(self, alpha) self.alpha = alpha end,
                ScrollContainer = {
                    SetAlpha = function(self, alpha) self.alpha = alpha end,
                },
            }
            QuestMapFrame = {
                SetAlpha = function(self, alpha) self.alpha = alpha end,
            }

            local opacity = GetCVar("worldMapOpacity")
            local ok, err = pcall(WorldMapFrame_SetOpacity, opacity)
            return opacity,
                ok,
                tostring(err),
                WorldMapFrame.alpha,
                WorldMapFrame.ScrollContainer.alpha,
                QuestMapFrame.alpha
            "#,
        )
        .expect("worldMapOpacity default should drive WorldMapFrame_SetOpacity");

    assert_eq!(
        (opacity, ok, err),
        ("1".to_string(), true, "nil".to_string()),
        "Mists WorldMap opacity should be seeded before minimized map sync"
    );
    assert_eq!((frame_alpha, scroll_alpha, quest_alpha), (0.5, 0.35, 0.45));
}
