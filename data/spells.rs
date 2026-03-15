//! Minimal spell data for the simulator's Paladin flow and tests.
//! This keeps the public API shape without carrying the full generated dump.

#[derive(Debug, Clone)]
pub struct SpellInfo {
    pub name: &'static str,
    pub subtext: &'static str,
    pub icon_file_data_id: u32,
    pub school_mask: u32,
    pub implicit_target: u8,
}

pub static SPELL_DB: &[(u32, SpellInfo)] = &[
    (
        100,
        SpellInfo {
            name: "Charge",
            subtext: "",
            icon_file_data_id: 132337,
            school_mask: 1,
            implicit_target: 6,
        },
    ),
    (
        116,
        SpellInfo {
            name: "Frostbolt",
            subtext: "",
            icon_file_data_id: 135846,
            school_mask: 16,
            implicit_target: 6,
        },
    ),
    (
        465,
        SpellInfo {
            name: "Devotion Aura",
            subtext: "",
            icon_file_data_id: 135893,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        498,
        SpellInfo {
            name: "Divine Protection",
            subtext: "",
            icon_file_data_id: 524353,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        633,
        SpellInfo {
            name: "Lay on Hands",
            subtext: "",
            icon_file_data_id: 135928,
            school_mask: 2,
            implicit_target: 21,
        },
    ),
    (
        642,
        SpellInfo {
            name: "Divine Shield",
            subtext: "",
            icon_file_data_id: 524354,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        853,
        SpellInfo {
            name: "Hammer of Justice",
            subtext: "",
            icon_file_data_id: 135963,
            school_mask: 2,
            implicit_target: 6,
        },
    ),
    (
        1022,
        SpellInfo {
            name: "Blessing of Protection",
            subtext: "",
            icon_file_data_id: 135964,
            school_mask: 2,
            implicit_target: 57,
        },
    ),
    (
        1044,
        SpellInfo {
            name: "Blessing of Freedom",
            subtext: "",
            icon_file_data_id: 135968,
            school_mask: 2,
            implicit_target: 21,
        },
    ),
    (
        4987,
        SpellInfo {
            name: "Cleanse",
            subtext: "",
            icon_file_data_id: 135949,
            school_mask: 2,
            implicit_target: 21,
        },
    ),
    (
        5502,
        SpellInfo {
            name: "Sense Undead",
            subtext: "",
            icon_file_data_id: 135974,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        6603,
        SpellInfo {
            name: "Auto Attack",
            subtext: "",
            icon_file_data_id: 135274,
            school_mask: 1,
            implicit_target: 0,
        },
    ),
    (
        6940,
        SpellInfo {
            name: "Blessing of Sacrifice",
            subtext: "",
            icon_file_data_id: 135966,
            school_mask: 2,
            implicit_target: 57,
        },
    ),
    (
        7328,
        SpellInfo {
            name: "Redemption",
            subtext: "",
            icon_file_data_id: 135955,
            school_mask: 2,
            implicit_target: 0,
        },
    ),
    (
        8690,
        SpellInfo {
            name: "Hearthstone",
            subtext: "",
            icon_file_data_id: 134414,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        10326,
        SpellInfo {
            name: "Turn Evil",
            subtext: "",
            icon_file_data_id: 571559,
            school_mask: 2,
            implicit_target: 6,
        },
    ),
    (
        19750,
        SpellInfo {
            name: "Flash of Light",
            subtext: "",
            icon_file_data_id: 135907,
            school_mask: 2,
            implicit_target: 21,
        },
    ),
    (
        20473,
        SpellInfo {
            name: "Holy Shock",
            subtext: "",
            icon_file_data_id: 135972,
            school_mask: 2,
            implicit_target: 25,
        },
    ),
    (
        31850,
        SpellInfo {
            name: "Ardent Defender",
            subtext: "",
            icon_file_data_id: 135870,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        31884,
        SpellInfo {
            name: "Avenging Wrath",
            subtext: "",
            icon_file_data_id: 135875,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        31935,
        SpellInfo {
            name: "Avenger's Shield",
            subtext: "",
            icon_file_data_id: 135874,
            school_mask: 2,
            implicit_target: 6,
        },
    ),
    (
        32223,
        SpellInfo {
            name: "Crusader Aura",
            subtext: "",
            icon_file_data_id: 135890,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        35395,
        SpellInfo {
            name: "Crusader Strike",
            subtext: "",
            icon_file_data_id: 135891,
            school_mask: 1,
            implicit_target: 6,
        },
    ),
    (
        53563,
        SpellInfo {
            name: "Beacon of Light",
            subtext: "",
            icon_file_data_id: 236247,
            school_mask: 2,
            implicit_target: 21,
        },
    ),
    (
        53576,
        SpellInfo {
            name: "Infusion of Light",
            subtext: "",
            icon_file_data_id: 236254,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        53595,
        SpellInfo {
            name: "Hammer of the Righteous",
            subtext: "",
            icon_file_data_id: 236253,
            school_mask: 1,
            implicit_target: 6,
        },
    ),
    (
        53600,
        SpellInfo {
            name: "Shield of the Righteous",
            subtext: "",
            icon_file_data_id: 236265,
            school_mask: 2,
            implicit_target: 24,
        },
    ),
    (
        62124,
        SpellInfo {
            name: "Hand of Reckoning",
            subtext: "",
            icon_file_data_id: 135984,
            school_mask: 2,
            implicit_target: 6,
        },
    ),
    (
        76671,
        SpellInfo {
            name: "Mastery: Divine Bulwark",
            subtext: "",
            icon_file_data_id: 135923,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        82326,
        SpellInfo {
            name: "Holy Light",
            subtext: "",
            icon_file_data_id: 135981,
            school_mask: 2,
            implicit_target: 21,
        },
    ),
    (
        85043,
        SpellInfo {
            name: "Grand Crusader",
            subtext: "",
            icon_file_data_id: 133176,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        85222,
        SpellInfo {
            name: "Light of Dawn",
            subtext: "",
            icon_file_data_id: 461859,
            school_mask: 2,
            implicit_target: 22,
        },
    ),
    (
        85256,
        SpellInfo {
            name: "Templar's Verdict",
            subtext: "",
            icon_file_data_id: 461860,
            school_mask: 2,
            implicit_target: 6,
        },
    ),
    (
        85673,
        SpellInfo {
            name: "Word of Glory",
            subtext: "",
            icon_file_data_id: 133192,
            school_mask: 2,
            implicit_target: 21,
        },
    ),
    (
        86659,
        SpellInfo {
            name: "Guardian of Ancient Kings",
            subtext: "",
            icon_file_data_id: 135919,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        96231,
        SpellInfo {
            name: "Rebuke",
            subtext: "",
            icon_file_data_id: 523893,
            school_mask: 1,
            implicit_target: 6,
        },
    ),
    (
        105809,
        SpellInfo {
            name: "Holy Avenger",
            subtext: "",
            icon_file_data_id: 571555,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        115750,
        SpellInfo {
            name: "Blinding Light",
            subtext: "",
            icon_file_data_id: 571553,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        121183,
        SpellInfo {
            name: "Contemplation",
            subtext: "",
            icon_file_data_id: 134916,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        137026,
        SpellInfo {
            name: "Paladin",
            subtext: "",
            icon_file_data_id: 236216,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        152261,
        SpellInfo {
            name: "Holy Shield",
            subtext: "",
            icon_file_data_id: 1526019,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        183435,
        SpellInfo {
            name: "Retribution Aura",
            subtext: "",
            icon_file_data_id: 135889,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        183997,
        SpellInfo {
            name: "Mastery: Lightbringer",
            subtext: "",
            icon_file_data_id: 133041,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        184575,
        SpellInfo {
            name: "Blade of Justice",
            subtext: "",
            icon_file_data_id: 1360757,
            school_mask: 2,
            implicit_target: 6,
        },
    ),
    (
        184662,
        SpellInfo {
            name: "Shield of Vengeance",
            subtext: "",
            icon_file_data_id: 236264,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        190784,
        SpellInfo {
            name: "Divine Steed",
            subtext: "",
            icon_file_data_id: 1360759,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        200652,
        SpellInfo {
            name: "Tyr's Deliverance",
            subtext: "",
            icon_file_data_id: 1122562,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        204019,
        SpellInfo {
            name: "Blessed Hammer",
            subtext: "",
            icon_file_data_id: 535595,
            school_mask: 2,
            implicit_target: 18,
        },
    ),
    (
        213644,
        SpellInfo {
            name: "Cleanse Toxins",
            subtext: "",
            icon_file_data_id: 135953,
            school_mask: 2,
            implicit_target: 21,
        },
    ),
    (
        231832,
        SpellInfo {
            name: "Blade of Wrath",
            subtext: "",
            icon_file_data_id: 1360757,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        255937,
        SpellInfo {
            name: "Wake of Ashes",
            subtext: "",
            icon_file_data_id: 1112939,
            school_mask: 6,
            implicit_target: 104,
        },
    ),
    (
        267344,
        SpellInfo {
            name: "Art of War",
            subtext: "",
            icon_file_data_id: 236246,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        269569,
        SpellInfo {
            name: "Zeal",
            subtext: "",
            icon_file_data_id: 135961,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        275779,
        SpellInfo {
            name: "Judgment",
            subtext: "",
            icon_file_data_id: 135959,
            school_mask: 2,
            implicit_target: 6,
        },
    ),
    (
        280373,
        SpellInfo {
            name: "Redoubt",
            subtext: "",
            icon_file_data_id: 132359,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        317920,
        SpellInfo {
            name: "Concentration Aura",
            subtext: "",
            icon_file_data_id: 135933,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        327193,
        SpellInfo {
            name: "Moment of Glory",
            subtext: "",
            icon_file_data_id: 237537,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        343527,
        SpellInfo {
            name: "Execution Sentence",
            subtext: "",
            icon_file_data_id: 613954,
            school_mask: 2,
            implicit_target: 6,
        },
    ),
    (
        343721,
        SpellInfo {
            name: "Final Reckoning",
            subtext: "",
            icon_file_data_id: 135878,
            school_mask: 2,
            implicit_target: 87,
        },
    ),
    (
        375576,
        SpellInfo {
            name: "Divine Toll",
            subtext: "",
            icon_file_data_id: 6035315,
            school_mask: 2,
            implicit_target: 25,
        },
    ),
    (
        378974,
        SpellInfo {
            name: "Bastion of Light",
            subtext: "",
            icon_file_data_id: 535594,
            school_mask: 2,
            implicit_target: 1,
        },
    ),
    (
        383185,
        SpellInfo {
            name: "Exorcism",
            subtext: "",
            icon_file_data_id: 135903,
            school_mask: 2,
            implicit_target: 53,
        },
    ),
    (
        385125,
        SpellInfo {
            name: "Of Dusk and Dawn",
            subtext: "",
            icon_file_data_id: 461859,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        387174,
        SpellInfo {
            name: "Eye of Tyr",
            subtext: "",
            icon_file_data_id: 1272527,
            school_mask: 2,
            implicit_target: 22,
        },
    ),
    (
        1230084,
        SpellInfo {
            name: "Transcribe: Blood",
            subtext: "",
            icon_file_data_id: 4620676,
            school_mask: 1,
            implicit_target: 0,
        },
    ),
    (
        1232418,
        SpellInfo {
            name: "Deadly Arcanocrystal Cluster",
            subtext: "",
            icon_file_data_id: 4549098,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        1232421,
        SpellInfo {
            name: "Energized Ley Crystal",
            subtext: "",
            icon_file_data_id: 1033908,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        1234430,
        SpellInfo {
            name: "Souls of the Caw",
            subtext: "Uncommon",
            icon_file_data_id: 442733,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        1242031,
        SpellInfo {
            name: "Twin Echoes",
            subtext: "Bronze",
            icon_file_data_id: 6118850,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        1247534,
        SpellInfo {
            name: "Soul Glutton",
            subtext: "",
            icon_file_data_id: 1120184,
            school_mask: 106,
            implicit_target: 1,
        },
    ),
    (
        1272143,
        SpellInfo {
            name: "Broken Spirit",
            subtext: "",
            icon_file_data_id: 1778228,
            school_mask: 1,
            implicit_target: 1,
        },
    ),
    (
        1279510,
        SpellInfo {
            name: "Niskaran Methods",
            subtext: "",
            icon_file_data_id: 1717019,
            school_mask: 32,
            implicit_target: 1,
        },
    ),
];

pub fn get_spell(id: u32) -> Option<&'static SpellInfo> {
    SPELL_DB
        .iter()
        .find_map(|(spell_id, spell)| (*spell_id == id).then_some(spell))
}
