use super::*;

use std::collections::HashSet;

/// `WorldState::default()` returns a *fully empty* state (zero
/// fields, empty collections). Seeded defaults come from
/// `seeded_world_state()` — pin both contracts so callers that
/// want "vanilla WoW-like" state reach for the seeded helper and
/// anyone wiring a fresh sub-state can rely on Default being inert.
#[test]
fn world_default_is_empty_and_zeroed() {
    let world = WorldState::default();
    assert!(world.transmog_appearances.is_empty());
    assert!(world.heirlooms.is_empty());
    assert!(world.collected_heirlooms.is_empty());
    assert_eq!(world.zone_id, 0);
    assert!(world.zone_name.is_empty());
    assert!(world.guild_name.is_none());
    assert!(!world.guild_can_speak_in_chat);
}

#[test]
fn seeded_world_populates_collections_and_seed_fields() {
    let world = seeded_world_state();
    assert!(!world.transmog_appearances.is_empty());
    assert!(!world.mounts.is_empty());
    assert!(!world.pets.is_empty());
    assert!(!world.toys.is_empty());
    assert!(!world.warband_scenes.is_empty());
    assert!(!world.heirlooms.is_empty());
    assert!(!world.premade_listings.is_empty());
    assert_eq!(world.collected_heirlooms.len(), world.heirlooms.len());
    assert_eq!(world.zone_id, 1519);
    assert_eq!(world.pvp_type, "contested");
    assert!(world.guild_can_speak_in_chat);
    assert_seeded_guild_event_log(&world);
    assert_eq!(world.world_pvp_areas.len(), 2);
    assert_eq!(world.world_pvp_areas[0].name, "Wintergrasp");
    assert_eq!(world.world_pvp_areas[1].name, "Tol Barad");
    assert_eq!(
        world
            .holiday_bg_info
            .as_ref()
            .expect("holiday bg info should be seeded")
            .name,
        "Warsong Scramble"
    );
    assert!(world.locklist_maps.is_empty());
}

fn assert_seeded_guild_event_log(world: &WorldState) {
    assert_eq!(world.guild_events.len(), 6);
    let promotion = &world.guild_events[3];
    assert_eq!(promotion.event_type, "promote");
    assert_eq!(promotion.player1, "Uther");
    assert_eq!(promotion.player2.as_deref(), Some("Jaina"));
    assert_eq!(promotion.rank_name.as_deref(), Some("Officer"));

    let removal = world.guild_events.last().expect("guild event should exist");
    assert_eq!(removal.event_type, "remove");
    assert_eq!(removal.player1, "Uther");
    assert_eq!(removal.player2.as_deref(), Some("Sylvanas"));
    assert_eq!(
        (removal.year, removal.month, removal.day, removal.hour),
        (25, 2, 18, 23)
    );
}

#[test]
fn transmog_default_appearances_populated() {
    let world = seeded_world_state();
    // 12 slots × 5 appearances each, plus 2 shirts and 1 tabard.
    assert_eq!(world.transmog_appearances.len(), 63);

    // Each armor slot has 4 collected + 1 uncollected
    let head: Vec<_> = world
        .transmog_appearances
        .iter()
        .filter(|a| a.category_id == 1)
        .collect();
    assert_eq!(head.len(), 5, "Head slot should have 5 appearances");
    assert_eq!(head.iter().filter(|a| a.is_collected).count(), 4);
    assert_eq!(head.iter().filter(|a| !a.is_collected).count(), 1);

    // Source IDs are unique and sequential
    let source_ids: HashSet<i32> = world
        .transmog_appearances
        .iter()
        .map(|a| a.source_id)
        .collect();
    assert_eq!(source_ids.len(), 63, "All source IDs should be unique");
}

#[test]
fn heirloom_defaults_populated() {
    let world = seeded_world_state();
    assert_eq!(
        world.heirlooms.len(),
        11,
        "should have 11 default heirlooms"
    );
    assert_eq!(world.heirlooms[0].name, "Burnished Helm of Might");
    assert_eq!(world.heirlooms[0].equip_loc, "INVTYPE_HEAD");

    let ids: HashSet<u32> = world.heirlooms.iter().map(|h| h.item_id).collect();
    assert_eq!(ids.len(), 11, "all item IDs should be unique");
    assert_eq!(
        world.collected_heirlooms.len(),
        11,
        "all default heirlooms collected"
    );
    assert!(world.collected_heirlooms.contains(&122245));
}
