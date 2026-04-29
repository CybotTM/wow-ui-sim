use crate::common;

use std::path::PathBuf;
use wow_ui_sim::iced_app::compute_frame_rect;
use wow_ui_sim::lua_api::WowLuaEnv;

const WORLD_MAP_ROOT_ADDONS: &[&str] = &[
    "Blizzard_FrameEffects",
    "Blizzard_StoreUI",
    "Blizzard_UIPanels_Game",
    "Blizzard_MapCanvasSecureUtil",
    "Blizzard_MapCanvas",
    "Blizzard_SharedMapDataProviders",
    "Blizzard_WorldMap",
    "Blizzard_GameMenu",
    "Blizzard_UIWidgets",
    "Blizzard_AddOnList",
    "Blizzard_TimerunningUtil",
];

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn build_strata_buckets(env: &WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

fn is_descendant_of(
    widgets: &wow_ui_sim::widget::WidgetRegistry,
    mut frame_id: u64,
    ancestor_id: u64,
) -> bool {
    loop {
        if frame_id == ancestor_id {
            return true;
        }
        let Some(parent_id) = widgets.get(frame_id).and_then(|frame| frame.parent_id) else {
            return false;
        };
        frame_id = parent_id;
    }
}

fn env_with_root_addons_ui_with_overrides(
    roots: &[&str],
    overrides: &[wow_ui_sim::loader::BlizzardAddonOverride<'_>],
) -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    let (env, _) =
        common::blizzard_addon_harness::build_blizzard_addon_closure_env(&ui, roots, overrides);
    env.apply_post_load_workarounds();
    wow_ui_sim::startup::settle_headless_startup(&env);
    env
}

fn open_world_map(env: &WowLuaEnv) {
    env.exec("ToggleWorldMap()")
        .expect("failed to toggle world map after startup");
    wow_ui_sim::startup::process_pending_timers(env);
    wow_ui_sim::startup::fire_one_on_update_tick(env);
}

#[test]
fn chat_frame_voice_button_renders_below_overlapping_world_map_widgets() {
    common::with_timeout(120, move || {
        let env = env_with_root_addons_ui_with_overrides(
            WORLD_MAP_ROOT_ADDONS,
            common::blizzard_addon_manifest::WORLD_MAP_VOICE_CHAT_OVERRIDES,
        );
        open_world_map(&env);

        let buckets = build_strata_buckets(&env);
        let flattened: Vec<u64> = buckets.iter().flatten().copied().collect();
        let state = env.state().borrow();

        let world_map_id = state
            .widgets
            .get_id_by_name("WorldMapFrame")
            .expect("world map should exist");
        let voice_button_id = state
            .widgets
            .get_id_by_name("ChatFrameChannelButton")
            .expect("chat voice button should exist");
        let voice_icon_id = state
            .widgets
            .get(voice_button_id)
            .and_then(|frame| frame.children_keys.get("Icon"))
            .copied()
            .expect("chat voice button icon should exist");

        let voice_button_rect = compute_frame_rect(&state.widgets, voice_button_id, 1024.0, 768.0);
        let voice_icon = state.widgets.get(voice_icon_id).unwrap();
        let voice_button = state.widgets.get(voice_button_id).unwrap();

        assert_eq!(
            voice_icon.atlas.as_deref(),
            Some("chatframe-button-icon-voicechat")
        );
        assert!(
            !voice_button.children_keys.contains_key("Background"),
            "chat voice button should not synthesize an extra white background child"
        );

        let voice_button_pos = flattened
            .iter()
            .position(|&id| id == voice_button_id)
            .expect("chat voice button should be in render list");

        let overlapping_world_map_widgets: Vec<(usize, String)> = flattened
            .iter()
            .filter(|&&id| id != voice_button_id && is_descendant_of(&state.widgets, id, world_map_id))
            .filter_map(|&id| {
                let rect = compute_frame_rect(&state.widgets, id, 1024.0, 768.0);
                let overlaps_horizontally = voice_button_rect.x < rect.x + rect.width
                    && voice_button_rect.x + voice_button_rect.width > rect.x;
                let overlaps_vertically = voice_button_rect.y < rect.y + rect.height
                    && voice_button_rect.y + voice_button_rect.height > rect.y;
                if !overlaps_horizontally || !overlaps_vertically {
                    return None;
                }

                let pos = flattened.iter().position(|&bucket_id| bucket_id == id)?;
                let frame = state.widgets.get(id)?;
                Some((
                    pos,
                    format!(
                        "id={id} pos={pos} name={:?} parent_key={:?} type={:?} rect={rect:?} atlas={:?} texture={:?}",
                        frame.name,
                        frame.parent_key,
                        frame.widget_type,
                        frame.atlas,
                        frame.texture,
                    ),
                ))
            })
            .collect();

        assert!(
            !overlapping_world_map_widgets.is_empty(),
            "voice button should overlap at least one world map widget; button={voice_button_rect:?}"
        );
        assert!(
            overlapping_world_map_widgets
                .iter()
                .all(|(pos, _)| voice_button_pos < *pos),
            "chat voice button should render before all overlapping world map widgets; \
             button_pos={voice_button_pos}, button={voice_button_rect:?}, overlaps={overlapping_world_map_widgets:#?}",
        );
    });
}
