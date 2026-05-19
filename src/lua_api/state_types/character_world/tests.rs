use super::*;

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
