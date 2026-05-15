//! Spell lookup facade over generated DB2 exports plus simulator-required
//! supplement rows.
//!
//! Blizzard UI does not define spell names in Lua. Retail resolves these
//! through client spell data, while the simulator keeps a compact generated
//! spell table. Supplement rows cover valid client spell IDs that addon Lua
//! references but the compact table does not currently include.

use crate::spells::{self, SpellInfo};

const SUPPLEMENTAL_SPELLS: &[(u32, SpellInfo)] = &[(
    395296,
    SpellInfo {
        name: "Ebon Might",
        subtext: "Black",
        icon_file_data_id: 5061347,
        school_mask: 12,
        implicit_target: 1,
    },
)];

pub fn get_spell(id: u32) -> Option<&'static SpellInfo> {
    spells::get_spell(id).or_else(|| supplemental_spell(id))
}

fn supplemental_spell(id: u32) -> Option<&'static SpellInfo> {
    SUPPLEMENTAL_SPELLS
        .iter()
        .find_map(|(spell_id, spell)| (*spell_id == id).then_some(spell))
}
