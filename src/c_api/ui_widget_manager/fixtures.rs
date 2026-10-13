//! Preserve existing demo quest widgets as explicit state, never a runtime ID band.
use super::{IconAndTextRecord, UiWidgetInputs, WidgetId, WidgetSetEntry, WidgetSetRecord};
use crate::lua_api::globals::quest_surface::data::WORLD_QUESTS;

pub(crate) fn seed_demo_world_quest_widgets() -> UiWidgetInputs {
    let mut inputs = UiWidgetInputs::default();
    // Tooltip, BehindIcon, AdventureMapDetails are the existing demo consumers.
    // IDs retain the old fixture values; this is not a native quest association.
    for quest in WORLD_QUESTS {
        for set_type in [0, 1, 2] {
            let set_id = f64::from(1_000_000 + quest.quest_id * 10 + set_type);
            inputs.quest_sets.insert((quest.quest_id, set_type), set_id);
            inputs
                .sets
                .insert(WidgetId::from_number(set_id), demo_set(set_id));
            inputs
                .icon_and_text
                .insert(WidgetId::from_number(set_id), demo_icon());
        }
    }
    inputs
}

fn demo_set(widget_id: f64) -> WidgetSetRecord {
    WidgetSetRecord {
        layout_direction: 2,
        vertical_padding: 0.0,
        widgets: vec![WidgetSetEntry {
            widget_id,
            widget_type: 0,
            unit_token: None,
        }],
    }
}

fn demo_icon() -> IconAndTextRecord {
    IconAndTextRecord {
        state: 1,
        text: String::new(),
        tooltip: String::new(),
        dynamic_tooltip: String::new(),
        tooltip_loc: 0,
        shift_text_type: 0,
        widget_size_setting: 0.0,
        texture_kit: "Worldquest".into(),
        frame_texture_kit: String::new(),
        has_timer: false,
        order_index: 1.0,
        widget_tag: String::new(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 0,
        layout_direction: 0,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}
