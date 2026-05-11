use crate::lua_api::methods::{create_string, create_table, table_get, table_set};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

struct MistsTalentInfo {
    talent_id: u32,
    name: &'static str,
    icon: u32,
    spell_id: u32,
    tier: u8,
    column: u8,
}

const MISTS_TALENTS: &[MistsTalentInfo] = &[
    MistsTalentInfo {
        talent_id: 17565,
        name: "Speed of Light",
        icon: 571558,
        spell_id: 85499,
        tier: 1,
        column: 1,
    },
    MistsTalentInfo {
        talent_id: 17567,
        name: "Long Arm of the Law",
        icon: 571557,
        spell_id: 87172,
        tier: 1,
        column: 2,
    },
    MistsTalentInfo {
        talent_id: 17569,
        name: "Pursuit of Justice",
        icon: 571559,
        spell_id: 26023,
        tier: 1,
        column: 3,
    },
    MistsTalentInfo {
        talent_id: 17573,
        name: "Fist of Justice",
        icon: 135906,
        spell_id: 105593,
        tier: 2,
        column: 1,
    },
    MistsTalentInfo {
        talent_id: 17575,
        name: "Repentance",
        icon: 135942,
        spell_id: 20066,
        tier: 2,
        column: 2,
    },
    MistsTalentInfo {
        talent_id: 17577,
        name: "Blinding Light",
        icon: 571553,
        spell_id: 115750,
        tier: 2,
        column: 3,
    },
    MistsTalentInfo {
        talent_id: 17581,
        name: "Selfless Healer",
        icon: 135964,
        spell_id: 85804,
        tier: 3,
        column: 1,
    },
    MistsTalentInfo {
        talent_id: 17583,
        name: "Eternal Flame",
        icon: 135433,
        spell_id: 114163,
        tier: 3,
        column: 2,
    },
    MistsTalentInfo {
        talent_id: 17585,
        name: "Sacred Shield",
        icon: 236249,
        spell_id: 20925,
        tier: 3,
        column: 3,
    },
    MistsTalentInfo {
        talent_id: 17589,
        name: "Hand of Purity",
        icon: 135970,
        spell_id: 114039,
        tier: 4,
        column: 1,
    },
    MistsTalentInfo {
        talent_id: 17591,
        name: "Unbreakable Spirit",
        icon: 135984,
        spell_id: 114154,
        tier: 4,
        column: 2,
    },
    MistsTalentInfo {
        talent_id: 17593,
        name: "Clemency",
        icon: 135863,
        spell_id: 105622,
        tier: 4,
        column: 3,
    },
    MistsTalentInfo {
        talent_id: 17597,
        name: "Holy Avenger",
        icon: 571555,
        spell_id: 105809,
        tier: 5,
        column: 1,
    },
    MistsTalentInfo {
        talent_id: 17599,
        name: "Sanctified Wrath",
        icon: 236262,
        spell_id: 53376,
        tier: 5,
        column: 2,
    },
    MistsTalentInfo {
        talent_id: 17601,
        name: "Divine Purpose",
        icon: 135897,
        spell_id: 86172,
        tier: 5,
        column: 3,
    },
    MistsTalentInfo {
        talent_id: 17605,
        name: "Holy Prism",
        icon: 613408,
        spell_id: 114165,
        tier: 6,
        column: 1,
    },
    MistsTalentInfo {
        talent_id: 17607,
        name: "Light's Hammer",
        icon: 613955,
        spell_id: 114158,
        tier: 6,
        column: 2,
    },
    MistsTalentInfo {
        talent_id: 17609,
        name: "Execution Sentence",
        icon: 613954,
        spell_id: 114157,
        tier: 6,
        column: 3,
    },
];

pub fn get_talent_info(state: &mut LuaState) -> LuaResult<u32> {
    let query = stack_val(state, 1);
    let tier = table_number(state, query, "tier");
    let column = table_number(state, query, "column");
    let Some(talent) = mists_talent_by_position(tier, column) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = talent_info_table(state, talent);
    state.push(info);
    Ok(1)
}

fn table_number(state: &mut LuaState, table: Val, key: &str) -> u8 {
    match table_get(state, table, key) {
        Val::Num(value) => value as u8,
        _ => 0,
    }
}

fn mists_talent_by_position(tier: u8, column: u8) -> Option<&'static MistsTalentInfo> {
    MISTS_TALENTS
        .iter()
        .find(|talent| talent.tier == tier && talent.column == column)
}

fn talent_info_table(state: &mut LuaState, talent: &MistsTalentInfo) -> Val {
    let info = create_table(state);
    let name = create_string(state, talent.name);
    table_set(state, info, "talentID", number(talent.talent_id));
    table_set(state, info, "name", name);
    table_set(state, info, "icon", number(talent.icon));
    table_set(state, info, "selected", Val::Bool(false));
    table_set(state, info, "available", Val::Bool(true));
    table_set(state, info, "spellID", number(talent.spell_id));
    table_set(state, info, "pvpTalentID", Val::Nil);
    table_set(state, info, "tier", number(talent.tier));
    table_set(state, info, "column", number(talent.column));
    table_set(state, info, "isKnown", Val::Bool(false));
    table_set(state, info, "grantedByAura", Val::Bool(false));
    info
}

fn number(value: impl Into<f64>) -> Val {
    Val::Num(value.into())
}
