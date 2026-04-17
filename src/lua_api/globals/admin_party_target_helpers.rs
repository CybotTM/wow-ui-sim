//! Private helpers shared between party/target admin setters in
//! `admin.rs`. Split out of `admin.rs` to keep that file under the
//! 750-line cap without disturbing the public `A_Admin.*` surface.
//!
//! NOTE: `src/lua_api/globals/admin_api/units.rs` has its own separate
//! copies of these helpers (pre-dating the rilua migration). Those are
//! untouched here — consolidating the two copies is a separate cleanup.

use crate::lua_api::game_data::{PartyMember, TargetInfo};

/// Build a `TargetInfo` with default stats from admin-supplied unit
/// parameters. GUID is stamped with the subsecond clock so repeated
/// admin calls produce distinguishable GUIDs without requiring the
/// caller to supply one.
pub(super) fn make_target_info(
    unit_id: &str,
    name: &str,
    level: i32,
    class_index: i32,
    is_enemy: bool,
) -> TargetInfo {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let guid = if is_enemy {
        format!("Creature-0-0-0-0-0-{}", nanos % 1_000_000)
    } else {
        format!("Player-0000-{:08}", nanos % 100_000_000)
    };
    TargetInfo {
        unit_id: unit_id.to_string(),
        name: name.to_string(),
        class_index,
        level,
        health: 100_000,
        health_max: 100_000,
        power: 50_000,
        power_max: 100_000,
        power_type: 0,
        power_type_name: "MANA".to_string(),
        is_player: !is_enemy,
        is_enemy,
        guid,
        classification: "normal".to_string(),
        creature_type: "Humanoid".to_string(),
        reaction: if is_enemy { 2 } else { 5 },
    }
}

/// Build a default `PartyMember` for padding when admin grows the
/// party beyond the seeded roster.
pub(super) fn default_party_member() -> PartyMember {
    PartyMember {
        name: "Unknown".to_string(),
        class_index: 1,
        level: 80,
        health: 100_000,
        health_max: 100_000,
        power: 0,
        power_max: 100,
        power_type: 1,
        power_type_name: "RAGE".to_string(),
        is_leader: false,
        dead_since: None,
    }
}
