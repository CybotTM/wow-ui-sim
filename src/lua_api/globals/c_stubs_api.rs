//! Additional C_* namespace stubs.
//!
//! Contains stub implementations for C_* namespaces that are referenced by
//! Blizzard UI code but don't need real logic in the simulator:
//! - C_AchievementInfo - Achievement data
//! - C_ClassTalents - Class talent configuration
//! - C_DelvesUI - Delves companion data
//! - C_Guild - Guild membership
//! - C_GuildInfo - Guild display info
//! - C_LFGList - Looking for Group listings
//! - C_LossOfControl - Loss of control effects
//! - C_Mail - Mailbox system
//! - C_StableInfo - Hunter pet stables
//! - C_Tutorial - Tutorial flags
//! - C_ActionBar - Action bar queries
//! - C_ZoneAbility - Zone ability data

use mlua::{Lua, MultiValue, Result, Value};

#[derive(Clone, Copy)]
struct GlueCharacterDef {
    guid: &'static str,
    name: &'static str,
    class_name: &'static str,
    class_filename: &'static str,
    experience_level: i32,
    area_name: &'static str,
    faction: &'static str,
    realm_name: &'static str,
    realm_address: i32,
    race_name: &'static str,
    last_active_time: i64,
}

const GLUE_CHARACTERS: &[GlueCharacterDef] = &[
    GlueCharacterDef {
        guid: "Player-1-00000001",
        name: "Player",
        class_name: "Warrior",
        class_filename: "WARRIOR",
        experience_level: 70,
        area_name: "Stormwind City",
        faction: "Alliance",
        realm_name: "Burning Blade",
        realm_address: 1,
        race_name: "Human",
        last_active_time: 2,
    },
    GlueCharacterDef {
        guid: "Player-1-00000002",
        name: "Secondhero",
        class_name: "Mage",
        class_filename: "MAGE",
        experience_level: 70,
        area_name: "Orgrimmar",
        faction: "Horde",
        realm_name: "Burning Blade",
        realm_address: 1,
        race_name: "Orc",
        last_active_time: 1,
    },
];

const GLUE_SELECTED_CHARACTER_KEY: &str = "__wow_ui_sim_glue_selected_character";
const GLUE_SELECT_CHARACTER_DISPATCH_KEY: &str = "__wow_ui_sim_glue_select_character_dispatch";
const GLUE_CHARACTER_CREATE_TYPE_KEY: &str = "__glue_character_create_type";
const GLUE_CHARACTER_CREATE_RACE_ID_KEY: &str = "__wow_ui_sim_glue_character_create_race_id";
const GLUE_CHARACTER_CREATE_CLASS_ID_KEY: &str = "__wow_ui_sim_glue_character_create_class_id";
const GLUE_CHARACTER_CREATE_SEX_ID_KEY: &str = "__wow_ui_sim_glue_character_create_sex_id";
const GLUE_CHARACTER_CREATE_FACING_KEY: &str = "__wow_ui_sim_glue_character_create_facing";
const GLUE_CHARACTER_CREATE_MODEL_ALPHA_KEY: &str =
    "__wow_ui_sim_glue_character_create_model_alpha";
const GLUE_CHARACTER_CREATE_CAMERA_ZOOM_KEY: &str =
    "__wow_ui_sim_glue_character_create_camera_zoom";
const GLUE_CHARACTER_CREATE_VIEWING_ALTERED_FORM_KEY: &str =
    "__wow_ui_sim_glue_character_create_viewing_altered_form";
const GLUE_CHARACTER_CREATE_SELECTED_PREVIEW_GEAR_KEY: &str =
    "__wow_ui_sim_glue_character_create_selected_preview_gear";
const GLUE_CHARACTER_CREATE_MODEL_DRESSED_KEY: &str =
    "__wow_ui_sim_glue_character_create_model_dressed";
const GLUE_CHARACTER_CREATE_MODEL_HIDDEN_KEY: &str =
    "__wow_ui_sim_glue_character_create_model_hidden";
const GLUE_CHARACTER_CREATE_BLUR_ENABLED_KEY: &str =
    "__wow_ui_sim_glue_character_create_blur_enabled";
const GLUE_CHARACTER_CREATE_CUSTOMIZATION_CHOICES_KEY: &str =
    "__wow_ui_sim_glue_character_create_customization_choices";
const GLUE_CHARACTER_CREATE_CUSTOMIZATION_PREVIEW_CHOICES_KEY: &str =
    "__wow_ui_sim_glue_character_create_customization_preview_choices";

#[derive(Clone, Copy)]
struct GlueRaceDef {
    race_id: i32,
    name: &'static str,
    client_file_string: &'static str,
    file_name: &'static str,
    faction_internal_name: &'static str,
    create_screen_icon_atlas: &'static str,
    lore_description: &'static str,
    is_allied_race: bool,
    is_neutral_race: bool,
    has_heritage_armor: bool,
    alternate_form: Option<GlueAlternateFormDef>,
}

#[derive(Clone, Copy)]
struct GlueAlternateFormDef {
    name: &'static str,
    create_screen_icon_atlas: &'static str,
}

#[derive(Clone, Copy)]
struct GlueClassDef {
    class_id: i32,
    name: &'static str,
    file_name: &'static str,
    description: &'static str,
    role_info: &'static str,
}

#[derive(Clone, Copy)]
struct GlueCustomizationCategoryDef {
    id: i32,
    name: &'static str,
    icon: &'static str,
    selected_icon: &'static str,
    order_index: i32,
    camera_zoom_level: i32,
    camera_distance_offset: f32,
    options: &'static [GlueCustomizationOptionDef],
}

#[derive(Clone, Copy)]
struct GlueCustomizationOptionDef {
    id: i32,
    name: &'static str,
    option_type: i32,
    order_index: i32,
    choices: &'static [GlueCustomizationChoiceDef],
}

#[derive(Clone, Copy)]
struct GlueCustomizationChoiceDef {
    id: i32,
    name: &'static str,
}

const GLUE_RACES: &[GlueRaceDef] = &[
    GlueRaceDef {
        race_id: 1,
        name: "Human",
        client_file_string: "Human",
        file_name: "Human",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-human-male",
        lore_description: "Versatile and resilient survivors of the Eastern Kingdoms.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 2,
        name: "Orc",
        client_file_string: "Orc",
        file_name: "Orc",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-orc-male",
        lore_description: "Fierce warriors who forged a new destiny on Azeroth.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 3,
        name: "Dwarf",
        client_file_string: "Dwarf",
        file_name: "Dwarf",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-dwarf-male",
        lore_description: "Stout defenders with ancient titan-forged roots.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 4,
        name: "Night Elf",
        client_file_string: "NightElf",
        file_name: "NightElf",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-nightelf-male",
        lore_description: "Ancient guardians of nature and the kaldorei legacy.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 5,
        name: "Undead",
        client_file_string: "Scourge",
        file_name: "Scourge",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-scourge-male",
        lore_description: "Forsaken who seized free will from the Lich King.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 6,
        name: "Tauren",
        client_file_string: "Tauren",
        file_name: "Tauren",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-tauren-male",
        lore_description: "Honorable nomads guided by the Earth Mother.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 7,
        name: "Gnome",
        client_file_string: "Gnome",
        file_name: "Gnome",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-gnome-male",
        lore_description: "Inventive tinkerers with a talent for improbable solutions.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 8,
        name: "Troll",
        client_file_string: "Troll",
        file_name: "Troll",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-troll-male",
        lore_description: "Savage hunters and priests with proud ancient empires.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 9,
        name: "Goblin",
        client_file_string: "Goblin",
        file_name: "Goblin",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-goblin-male",
        lore_description: "Profit-driven masterminds with dangerous ingenuity.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 10,
        name: "Blood Elf",
        client_file_string: "BloodElf",
        file_name: "BloodElf",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-bloodelf-male",
        lore_description: "Arcane masters rebuilding Quel'Thalas with resolve.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 11,
        name: "Draenei",
        client_file_string: "Draenei",
        file_name: "Draenei",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-draenei-male",
        lore_description: "Exiles of Argus strengthened by faith and perseverance.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 22,
        name: "Worgen",
        client_file_string: "Worgen",
        file_name: "Worgen",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-worgen-male",
        lore_description: "Cursed Gilneans who balance feral fury with discipline.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: Some(GlueAlternateFormDef {
            name: "Human Form",
            create_screen_icon_atlas: "raceicon128-human-male",
        }),
    },
    GlueRaceDef {
        race_id: 24,
        name: "Pandaren",
        client_file_string: "Pandaren",
        file_name: "Pandaren",
        faction_internal_name: "Neutral",
        create_screen_icon_atlas: "raceicon128-pandaren-male",
        lore_description: "Wanderers from Pandaria who choose their own path.",
        is_allied_race: false,
        is_neutral_race: true,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 27,
        name: "Nightborne",
        client_file_string: "Nightborne",
        file_name: "Nightborne",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-nightborne-male",
        lore_description: "Arcwine-fueled survivors of Suramar's long isolation.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 28,
        name: "Highmountain Tauren",
        client_file_string: "HighmountainTauren",
        file_name: "HighmountainTauren",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-highmountaintauren-male",
        lore_description: "Tauren tribes united by Huln's enduring legacy.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 29,
        name: "Void Elf",
        client_file_string: "VoidElf",
        file_name: "VoidElf",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-voidelf-male",
        lore_description: "Ren'dorei who wield the whispers of the Void.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 30,
        name: "Lightforged Draenei",
        client_file_string: "LightforgedDraenei",
        file_name: "LightforgedDraenei",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-lightforgeddraenei-male",
        lore_description: "Veterans of the Army of the Light, marked by holy fire.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 31,
        name: "Zandalari Troll",
        client_file_string: "ZandalariTroll",
        file_name: "ZandalariTroll",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-zandalaritroll-male",
        lore_description: "Imperial trolls descended from Azeroth's oldest empire.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 32,
        name: "Kul Tiran",
        client_file_string: "KulTiran",
        file_name: "KulTiran",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-kultiran-male",
        lore_description: "Seasoned mariners hardened by storms and witchcraft.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 34,
        name: "Dark Iron Dwarf",
        client_file_string: "DarkIronDwarf",
        file_name: "DarkIronDwarf",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-darkirondwarf-male",
        lore_description: "Fire-tempered dwarves from Blackrock's shadowed halls.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 35,
        name: "Vulpera",
        client_file_string: "Vulpera",
        file_name: "Vulpera",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-vulpera-male",
        lore_description: "Resourceful nomads who thrive through speed and wit.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 36,
        name: "Mag'har Orc",
        client_file_string: "MagharOrc",
        file_name: "MagharOrc",
        faction_internal_name: "Horde",
        create_screen_icon_atlas: "raceicon128-magharorc-male",
        lore_description: "Uncorrupted orc clans drawn from alternate Draenor.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 37,
        name: "Mechagnome",
        client_file_string: "Mechagnome",
        file_name: "Mechagnome",
        faction_internal_name: "Alliance",
        create_screen_icon_atlas: "raceicon128-mechagnome-male",
        lore_description: "Augmented gnomes pursuing perfection through engineering.",
        is_allied_race: true,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: None,
    },
    GlueRaceDef {
        race_id: 52,
        name: "Dracthyr",
        client_file_string: "Dracthyr",
        file_name: "Dracthyr",
        faction_internal_name: "Neutral",
        create_screen_icon_atlas: "raceicon128-dracthyr-male",
        lore_description: "Dragonkin soldiers awakened to a transformed world.",
        is_allied_race: false,
        is_neutral_race: true,
        has_heritage_armor: false,
        alternate_form: Some(GlueAlternateFormDef {
            name: "Visage",
            create_screen_icon_atlas: "raceicon128-human-male",
        }),
    },
    GlueRaceDef {
        race_id: 84,
        name: "Earthen",
        client_file_string: "Earthen",
        file_name: "Earthen",
        faction_internal_name: "Neutral",
        create_screen_icon_atlas: "raceicon128-earthen-male",
        lore_description: "Titan-forged people emerging from the depths of Khaz Algar.",
        is_allied_race: true,
        is_neutral_race: true,
        has_heritage_armor: true,
        alternate_form: None,
    },
];

const GLUE_CLASSES: &[GlueClassDef] = &[
    GlueClassDef {
        class_id: 1,
        name: "Warrior",
        file_name: "WARRIOR",
        description: "Battle-hardened combatants who master arms, rage, and resilience.",
        role_info: "Tank, Damage",
    },
    GlueClassDef {
        class_id: 2,
        name: "Paladin",
        file_name: "PALADIN",
        description: "Holy champions who bring heavy armor, auras, and healing.",
        role_info: "Tank, Healer, Damage",
    },
    GlueClassDef {
        class_id: 3,
        name: "Hunter",
        file_name: "HUNTER",
        description: "Ranged trackers who rely on pets, marksmanship, and survival skills.",
        role_info: "Damage",
    },
    GlueClassDef {
        class_id: 4,
        name: "Rogue",
        file_name: "ROGUE",
        description: "Agile assassins who strike from stealth with precision.",
        role_info: "Damage",
    },
    GlueClassDef {
        class_id: 5,
        name: "Priest",
        file_name: "PRIEST",
        description: "Devotees of Light and Shadow with potent healing and spellcasting.",
        role_info: "Healer, Damage",
    },
    GlueClassDef {
        class_id: 6,
        name: "Death Knight",
        file_name: "DEATHKNIGHT",
        description: "Runeblade-wielding heroes of undeath who command frost and blood.",
        role_info: "Tank, Damage",
    },
    GlueClassDef {
        class_id: 7,
        name: "Shaman",
        file_name: "SHAMAN",
        description: "Elemental spiritualists who answer the call of earth, air, fire, and water.",
        role_info: "Healer, Damage",
    },
    GlueClassDef {
        class_id: 8,
        name: "Mage",
        file_name: "MAGE",
        description: "Pure spellcasters who bend arcane, frost, and fire to their will.",
        role_info: "Damage",
    },
    GlueClassDef {
        class_id: 9,
        name: "Warlock",
        file_name: "WARLOCK",
        description: "Fel-fueled casters who command curses, demons, and draining magic.",
        role_info: "Damage",
    },
    GlueClassDef {
        class_id: 10,
        name: "Monk",
        file_name: "MONK",
        description: "Pandaren martial artists who channel chi into strikes and healing.",
        role_info: "Tank, Healer, Damage",
    },
    GlueClassDef {
        class_id: 11,
        name: "Druid",
        file_name: "DRUID",
        description: "Shape-shifters who protect nature through versatility and forms.",
        role_info: "Tank, Healer, Damage",
    },
    GlueClassDef {
        class_id: 12,
        name: "Demon Hunter",
        file_name: "DEMONHUNTER",
        description: "Illidari vengeance seekers with fel mobility and metamorphosis.",
        role_info: "Tank, Damage",
    },
    GlueClassDef {
        class_id: 13,
        name: "Evoker",
        file_name: "EVOKER",
        description: "Dracthyr spellcasters who channel all five dragonflights.",
        role_info: "Healer, Damage",
    },
];

const FACE_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 1001,
        name: "Face 1",
    },
    GlueCustomizationChoiceDef {
        id: 1002,
        name: "Face 2",
    },
    GlueCustomizationChoiceDef {
        id: 1003,
        name: "Face 3",
    },
];
const SKIN_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 1101,
        name: "Tone 1",
    },
    GlueCustomizationChoiceDef {
        id: 1102,
        name: "Tone 2",
    },
    GlueCustomizationChoiceDef {
        id: 1103,
        name: "Tone 3",
    },
    GlueCustomizationChoiceDef {
        id: 1104,
        name: "Tone 4",
    },
    GlueCustomizationChoiceDef {
        id: 1105,
        name: "Tone 5",
    },
];
const HAIR_STYLE_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 2001,
        name: "Style 1",
    },
    GlueCustomizationChoiceDef {
        id: 2002,
        name: "Style 2",
    },
    GlueCustomizationChoiceDef {
        id: 2003,
        name: "Style 3",
    },
    GlueCustomizationChoiceDef {
        id: 2004,
        name: "Style 4",
    },
];
const HAIR_COLOR_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 2101,
        name: "Color 1",
    },
    GlueCustomizationChoiceDef {
        id: 2102,
        name: "Color 2",
    },
    GlueCustomizationChoiceDef {
        id: 2103,
        name: "Color 3",
    },
    GlueCustomizationChoiceDef {
        id: 2104,
        name: "Color 4",
    },
    GlueCustomizationChoiceDef {
        id: 2105,
        name: "Color 5",
    },
];
const FACIAL_HAIR_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 3001,
        name: "Off",
    },
    GlueCustomizationChoiceDef {
        id: 3002,
        name: "On",
    },
];
const SCAR_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 3101,
        name: "Off",
    },
    GlueCustomizationChoiceDef {
        id: 3102,
        name: "On",
    },
];
const HORN_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 3201,
        name: "Horn 1",
    },
    GlueCustomizationChoiceDef {
        id: 3202,
        name: "Horn 2",
    },
    GlueCustomizationChoiceDef {
        id: 3203,
        name: "Horn 3",
    },
];

const BODY_OPTIONS: &[GlueCustomizationOptionDef] = &[
    GlueCustomizationOptionDef {
        id: 100,
        name: "Face",
        option_type: 0,
        order_index: 1,
        choices: FACE_CHOICES,
    },
    GlueCustomizationOptionDef {
        id: 101,
        name: "Skin Tone",
        option_type: 2,
        order_index: 2,
        choices: SKIN_CHOICES,
    },
];
const HAIR_OPTIONS: &[GlueCustomizationOptionDef] = &[
    GlueCustomizationOptionDef {
        id: 200,
        name: "Hair Style",
        option_type: 0,
        order_index: 1,
        choices: HAIR_STYLE_CHOICES,
    },
    GlueCustomizationOptionDef {
        id: 201,
        name: "Hair Color",
        option_type: 0,
        order_index: 2,
        choices: HAIR_COLOR_CHOICES,
    },
];
const FEATURE_OPTIONS: &[GlueCustomizationOptionDef] = &[
    GlueCustomizationOptionDef {
        id: 300,
        name: "Facial Hair",
        option_type: 1,
        order_index: 1,
        choices: FACIAL_HAIR_CHOICES,
    },
    GlueCustomizationOptionDef {
        id: 301,
        name: "Scars",
        option_type: 1,
        order_index: 2,
        choices: SCAR_CHOICES,
    },
    GlueCustomizationOptionDef {
        id: 302,
        name: "Horn Style",
        option_type: 0,
        order_index: 3,
        choices: HORN_CHOICES,
    },
];

const GLUE_CUSTOMIZATION_CATEGORIES: &[GlueCustomizationCategoryDef] = &[
    GlueCustomizationCategoryDef {
        id: 1,
        name: "Body",
        icon: "classicon-warrior",
        selected_icon: "classicon-warrior",
        order_index: 1,
        camera_zoom_level: 15,
        camera_distance_offset: 0.0,
        options: BODY_OPTIONS,
    },
    GlueCustomizationCategoryDef {
        id: 2,
        name: "Hair",
        icon: "classicon-mage",
        selected_icon: "classicon-mage",
        order_index: 2,
        camera_zoom_level: 35,
        camera_distance_offset: 0.0,
        options: HAIR_OPTIONS,
    },
    GlueCustomizationCategoryDef {
        id: 3,
        name: "Features",
        icon: "classicon-priest",
        selected_icon: "classicon-priest",
        order_index: 3,
        camera_zoom_level: 50,
        camera_distance_offset: 0.0,
        options: FEATURE_OPTIONS,
    },
];

fn get_sim_state_rc(
    lua: &Lua,
) -> Option<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>> {
    lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>>()
        .map(|state| (*state).clone())
}

fn find_glue_race(race_id: i32) -> Option<&'static GlueRaceDef> {
    GLUE_RACES.iter().find(|race| race.race_id == race_id)
}

fn find_glue_race_by_name(name: &str) -> Option<&'static GlueRaceDef> {
    GLUE_RACES.iter().find(|race| {
        race.name.eq_ignore_ascii_case(name)
            || race.client_file_string.eq_ignore_ascii_case(name)
            || race.file_name.eq_ignore_ascii_case(name)
    })
}

fn find_glue_class(class_id: i32) -> Option<&'static GlueClassDef> {
    GLUE_CLASSES
        .iter()
        .find(|class_info| class_info.class_id == class_id)
}

fn faction_group_for_name(faction: &str) -> i32 {
    match faction {
        "Alliance" => 1,
        "Horde" => 0,
        _ => -1,
    }
}

fn default_race_id() -> i32 {
    GLUE_RACES.first().map(|race| race.race_id).unwrap_or(1)
}

fn default_class_id() -> i32 {
    GLUE_CLASSES
        .first()
        .map(|class_info| class_info.class_id)
        .unwrap_or(1)
}

fn default_sex_id() -> i32 {
    0
}

fn has_glue_character(lua: &Lua) -> bool {
    let Some(state) = get_sim_state_rc(lua) else {
        return false;
    };

    matches!(
        state.borrow().screen_kind,
        crate::screen::ScreenKind::CharacterSelect
    )
}

fn glue_character(index: i32) -> Option<&'static GlueCharacterDef> {
    let index = usize::try_from(index.checked_sub(1)?).ok()?;
    GLUE_CHARACTERS.get(index)
}

fn glue_character_by_guid(guid: &str) -> Option<&'static GlueCharacterDef> {
    GLUE_CHARACTERS.iter().find(|character| character.guid == guid)
}

fn glue_character_count(lua: &Lua) -> i32 {
    if has_glue_character(lua) {
        GLUE_CHARACTERS.len() as i32
    } else {
        0
    }
}

fn glue_character_guid(lua: &Lua, index: i32) -> Option<String> {
    has_glue_character(lua)
        .then(|| glue_character(index).map(|character| character.guid.to_string()))
        .flatten()
}

fn glue_selected_character(lua: &Lua) -> i32 {
    lua.globals()
        .get::<Option<i32>>(GLUE_SELECTED_CHARACTER_KEY)
        .ok()
        .flatten()
        .filter(|index| *index >= 0 && *index <= glue_character_count(lua))
        .unwrap_or(0)
}

fn set_glue_selected_character(lua: &Lua, index: i32) -> Result<()> {
    let selected = if glue_character_count(lua) == 0 {
        0
    } else {
        index.clamp(1, glue_character_count(lua))
    };
    lua.globals().set(GLUE_SELECTED_CHARACTER_KEY, selected)
}

fn get_glue_character_create_type(lua: &Lua) -> Result<i32> {
    let value: Value = lua.globals().raw_get(GLUE_CHARACTER_CREATE_TYPE_KEY)?;
    Ok(match value {
        Value::Integer(n) => n as i32,
        Value::Number(n) => n as i32,
        _ => 0,
    })
}

fn set_glue_character_create_type(lua: &Lua, value: &Value) -> Result<()> {
    let create_type = match value {
        Value::Integer(n) => *n as i32,
        Value::Number(n) => *n as i32,
        _ => 0,
    };
    lua.globals()
        .raw_set(GLUE_CHARACTER_CREATE_TYPE_KEY, create_type)?;
    Ok(())
}

fn get_glue_i32(lua: &Lua, key: &str, default: i32) -> Result<i32> {
    let value: Value = lua.globals().raw_get(key)?;
    Ok(match value {
        Value::Integer(n) => n as i32,
        Value::Number(n) => n as i32,
        _ => default,
    })
}

fn set_glue_i32(lua: &Lua, key: &str, value: i32) -> Result<()> {
    lua.globals().raw_set(key, value)
}

fn get_glue_f32(lua: &Lua, key: &str, default: f32) -> Result<f32> {
    let value: Value = lua.globals().raw_get(key)?;
    Ok(match value {
        Value::Integer(n) => n as f32,
        Value::Number(n) => n as f32,
        _ => default,
    })
}

fn set_glue_f32(lua: &Lua, key: &str, value: f32) -> Result<()> {
    lua.globals().raw_set(key, value)
}

fn reset_glue_character_create_state(lua: &Lua) -> Result<()> {
    set_glue_i32(lua, GLUE_CHARACTER_CREATE_RACE_ID_KEY, default_race_id())?;
    set_glue_i32(lua, GLUE_CHARACTER_CREATE_CLASS_ID_KEY, default_class_id())?;
    set_glue_i32(lua, GLUE_CHARACTER_CREATE_SEX_ID_KEY, default_sex_id())?;
    set_glue_f32(lua, GLUE_CHARACTER_CREATE_FACING_KEY, 0.0)?;
    set_glue_f32(lua, GLUE_CHARACTER_CREATE_MODEL_ALPHA_KEY, 1.0)?;
    set_glue_i32(lua, GLUE_CHARACTER_CREATE_CAMERA_ZOOM_KEY, 0)?;
    lua.globals()
        .raw_set(GLUE_CHARACTER_CREATE_VIEWING_ALTERED_FORM_KEY, false)?;
    lua.globals()
        .raw_set(GLUE_CHARACTER_CREATE_MODEL_DRESSED_KEY, true)?;
    lua.globals()
        .raw_set(GLUE_CHARACTER_CREATE_MODEL_HIDDEN_KEY, false)?;
    lua.globals()
        .raw_set(GLUE_CHARACTER_CREATE_BLUR_ENABLED_KEY, false)?;
    lua.globals()
        .raw_set(GLUE_CHARACTER_CREATE_SELECTED_PREVIEW_GEAR_KEY, Value::Nil)?;
    lua.globals().raw_set(
        GLUE_CHARACTER_CREATE_CUSTOMIZATION_CHOICES_KEY,
        lua.create_table()?,
    )?;
    lua.globals().raw_set(
        GLUE_CHARACTER_CREATE_CUSTOMIZATION_PREVIEW_CHOICES_KEY,
        lua.create_table()?,
    )?;
    Ok(())
}

fn get_glue_selected_race_id(lua: &Lua) -> Result<i32> {
    let race_id = get_glue_i32(lua, GLUE_CHARACTER_CREATE_RACE_ID_KEY, default_race_id())?;
    Ok(find_glue_race(race_id)
        .map(|race| race.race_id)
        .unwrap_or_else(default_race_id))
}

fn get_glue_selected_class_id(lua: &Lua) -> Result<i32> {
    let class_id = get_glue_i32(lua, GLUE_CHARACTER_CREATE_CLASS_ID_KEY, default_class_id())?;
    Ok(find_glue_class(class_id)
        .map(|class_info| class_info.class_id)
        .unwrap_or_else(default_class_id))
}

fn get_glue_selected_sex_id(lua: &Lua) -> Result<i32> {
    Ok(get_glue_i32(lua, GLUE_CHARACTER_CREATE_SEX_ID_KEY, default_sex_id())?.clamp(0, 1))
}

fn push_kv_str(lua: &Lua, table: &mlua::Table, key: &str, value: &str) -> Result<()> {
    table.set(key, lua.create_string(value)?)?;
    Ok(())
}

fn glue_racial_abilities(lua: &Lua, race: &GlueRaceDef) -> Result<mlua::Table> {
    let abilities = lua.create_table()?;

    let first = lua.create_table()?;
    push_kv_str(
        lua,
        &first,
        "icon",
        "Interface\\Icons\\INV_Misc_QuestionMark",
    )?;
    push_kv_str(
        lua,
        &first,
        "description",
        &format!(
            "{} heritage is represented in the character create flow.",
            race.name
        ),
    )?;
    abilities.set(1, first)?;

    let second = lua.create_table()?;
    push_kv_str(lua, &second, "icon", "Interface\\Icons\\Ability_DualWield")?;
    push_kv_str(
        lua,
        &second,
        "description",
        &format!("{} can pair with every simulator class option.", race.name),
    )?;
    abilities.set(2, second)?;

    Ok(abilities)
}

fn glue_race_data(lua: &Lua, race_id: i32) -> Result<mlua::Table> {
    let race = find_glue_race(race_id).unwrap_or_else(|| &GLUE_RACES[0]);
    let t = lua.create_table()?;
    t.set("raceID", race.race_id)?;
    push_kv_str(lua, &t, "name", race.name)?;
    push_kv_str(lua, &t, "clientFileString", race.client_file_string)?;
    push_kv_str(lua, &t, "fileName", race.file_name)?;
    push_kv_str(lua, &t, "factionInternalName", race.faction_internal_name)?;
    push_kv_str(
        lua,
        &t,
        "createScreenIconAtlas",
        race.create_screen_icon_atlas,
    )?;
    push_kv_str(lua, &t, "loreDescription", race.lore_description)?;
    t.set(
        "factionGroup",
        faction_group_for_name(race.faction_internal_name),
    )?;
    t.set("isAlliedRace", race.is_allied_race)?;
    t.set("isNeutralRace", race.is_neutral_race)?;
    t.set("enabled", true)?;
    t.set("hasHeritageArmor", race.has_heritage_armor)?;
    t.set("racialAbilities", glue_racial_abilities(lua, race)?)?;
    if let Some(alternate_form) = race.alternate_form {
        let alternate = lua.create_table()?;
        push_kv_str(lua, &alternate, "name", alternate_form.name)?;
        push_kv_str(
            lua,
            &alternate,
            "createScreenIconAtlas",
            alternate_form.create_screen_icon_atlas,
        )?;
        t.set("alternateFormRaceData", alternate)?;
    }
    Ok(t)
}

fn glue_class_data(lua: &Lua, class_id: i32) -> Result<mlua::Table> {
    let class_info = find_glue_class(class_id).unwrap_or_else(|| &GLUE_CLASSES[0]);
    let t = lua.create_table()?;
    t.set("classID", class_info.class_id)?;
    push_kv_str(lua, &t, "name", class_info.name)?;
    push_kv_str(lua, &t, "maleName", class_info.name)?;
    push_kv_str(lua, &t, "femaleName", class_info.name)?;
    push_kv_str(lua, &t, "fileString", class_info.file_name)?;
    push_kv_str(lua, &t, "fileName", class_info.file_name)?;
    push_kv_str(lua, &t, "description", class_info.description)?;
    push_kv_str(lua, &t, "roleInfo", class_info.role_info)?;
    t.set("enabled", true)?;
    Ok(t)
}

fn choice_id_for_index(option: &GlueCustomizationOptionDef, choice_index: usize) -> i32 {
    option
        .choices
        .get(choice_index)
        .or_else(|| option.choices.first())
        .map(|choice| choice.id)
        .unwrap_or(0)
}

fn default_choice_id_for_option(
    option: &GlueCustomizationOptionDef,
    race_id: i32,
    sex_id: i32,
) -> i32 {
    if option.choices.is_empty() {
        return 0;
    }
    let base = (race_id as usize + sex_id as usize + option.id as usize) % option.choices.len();
    choice_id_for_index(option, base)
}

fn customization_choices_table(lua: &Lua, key: &str) -> Result<mlua::Table> {
    let value: Value = lua.globals().raw_get(key)?;
    match value {
        Value::Table(table) => Ok(table),
        _ => {
            let table = lua.create_table()?;
            lua.globals().raw_set(key, table.clone())?;
            Ok(table)
        }
    }
}

fn glue_selected_choice_id(lua: &Lua, option: &GlueCustomizationOptionDef) -> Result<i32> {
    let preview =
        customization_choices_table(lua, GLUE_CHARACTER_CREATE_CUSTOMIZATION_PREVIEW_CHOICES_KEY)?;
    if let Some(choice_id) = preview.get::<Option<i32>>(option.id).ok().flatten() {
        return Ok(choice_id);
    }

    let choices =
        customization_choices_table(lua, GLUE_CHARACTER_CREATE_CUSTOMIZATION_CHOICES_KEY)?;
    if let Some(choice_id) = choices.get::<Option<i32>>(option.id).ok().flatten() {
        return Ok(choice_id);
    }

    Ok(default_choice_id_for_option(
        option,
        get_glue_selected_race_id(lua)?,
        get_glue_selected_sex_id(lua)?,
    ))
}

fn glue_customization_option_table(
    lua: &Lua,
    option: &GlueCustomizationOptionDef,
) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    table.set("id", option.id)?;
    push_kv_str(lua, &table, "name", option.name)?;
    table.set("optionType", option.option_type)?;
    table.set("orderIndex", option.order_index)?;
    table.set("hasNewChoices", false)?;

    let selected_choice_id = glue_selected_choice_id(lua, option)?;
    let mut current_choice_index = 1i32;
    let choices = lua.create_table()?;
    for (index, choice) in option.choices.iter().enumerate() {
        if choice.id == selected_choice_id {
            current_choice_index = (index + 1) as i32;
        }

        let choice_table = lua.create_table()?;
        choice_table.set("id", choice.id)?;
        push_kv_str(lua, &choice_table, "name", choice.name)?;
        choice_table.set("isNew", false)?;
        choice_table.set("disabled", false)?;
        choice_table.set("isLocked", false)?;
        choice_table.set("ineligibleChoice", false)?;
        choices.set(index + 1, choice_table)?;
    }

    table.set("currentChoiceIndex", current_choice_index)?;
    table.set("choices", choices)?;
    Ok(table)
}

fn glue_customization_category_table(
    lua: &Lua,
    category: &GlueCustomizationCategoryDef,
) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    table.set("id", category.id)?;
    push_kv_str(lua, &table, "name", category.name)?;
    push_kv_str(lua, &table, "icon", category.icon)?;
    push_kv_str(lua, &table, "selectedIcon", category.selected_icon)?;
    table.set("orderIndex", category.order_index)?;
    table.set("cameraZoomLevel", category.camera_zoom_level)?;
    table.set("cameraDistanceOffset", category.camera_distance_offset)?;
    table.set("hasNewChoices", false)?;

    let options = lua.create_table()?;
    for (index, option) in category.options.iter().enumerate() {
        options.set(index + 1, glue_customization_option_table(lua, option)?)?;
    }
    table.set("options", options)?;
    Ok(table)
}

fn glue_available_customizations(lua: &Lua) -> Result<mlua::Table> {
    let categories = lua.create_table()?;
    for (index, category) in GLUE_CUSTOMIZATION_CATEGORIES.iter().enumerate() {
        categories.set(index + 1, glue_customization_category_table(lua, category)?)?;
    }
    Ok(categories)
}

fn glue_fire_character_create_event(lua: &Lua, event: &str, args: &[Value]) -> Result<()> {
    let Some(state) = get_sim_state_rc(lua) else {
        return Ok(());
    };
    crate::lua_api::LoaderEnv::new(lua, state)
        .fire_event_with_args(event, args)
        .map_err(mlua::Error::external)
}

fn glue_random_name(lua: &Lua) -> Result<String> {
    let race = find_glue_race(get_glue_selected_race_id(lua)?).unwrap_or(&GLUE_RACES[0]);
    let class_info = find_glue_class(get_glue_selected_class_id(lua)?).unwrap_or(&GLUE_CLASSES[0]);
    let suffix = if get_glue_selected_sex_id(lua)? == 0 {
        "ar"
    } else {
        "ia"
    };
    Ok(format!(
        "{}{}{}",
        race.name
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .take(4)
            .collect::<String>(),
        class_info
            .name
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .take(3)
            .collect::<String>(),
        suffix
    ))
}

fn apply_selected_character_to_player_state(lua: &Lua, name: Option<&str>) {
    let Some(state) = get_sim_state_rc(lua) else {
        return;
    };
    let mut state = state.borrow_mut();
    if let Some(trimmed) = name.map(str::trim).filter(|trimmed| !trimmed.is_empty()) {
        state.player.name = trimmed.to_string();
    }
    state.player.class_index = get_glue_selected_class_id(lua).unwrap_or(default_class_id());
    state.player.sex = match get_glue_selected_sex_id(lua).unwrap_or(default_sex_id()) {
        1 => 3,
        _ => 2,
    };
    state.player.race_index = GLUE_RACES
        .iter()
        .position(|race| {
            race.race_id == get_glue_selected_race_id(lua).unwrap_or(default_race_id())
        })
        .unwrap_or(0)
        .min(crate::lua_api::state::RACE_DATA.len().saturating_sub(1));
}

fn glue_basic_character_info(lua: &Lua, guid: &str) -> Result<Value> {
    let Some(character) = glue_character_by_guid(guid) else {
        return Ok(Value::Nil);
    };
    if !has_glue_character(lua) {
        return Ok(Value::Nil);
    }

    let table = lua.create_table()?;
    if let Some(state) = get_sim_state_rc(lua)
        && character.guid == GLUE_CHARACTERS[0].guid
    {
        let state = state.borrow();
        let class_info = find_glue_class(state.player.class_index).unwrap_or(&GLUE_CLASSES[0]);
        table.set("guid", guid)?;
        table.set("name", state.player.name.clone())?;
        table.set("className", class_info.name)?;
        table.set("classFilename", class_info.file_name)?;
        table.set("experienceLevel", state.player.level)?;
        table.set("areaName", state.world.zone_name.clone())?;
    } else {
        table.set("guid", guid)?;
        table.set("name", character.name)?;
        table.set("className", character.class_name)?;
        table.set("classFilename", character.class_filename)?;
        table.set("experienceLevel", character.experience_level)?;
        table.set("areaName", character.area_name)?;
    }

    table.set("faction", character.faction)?;
    table.set("realmName", character.realm_name)?;
    table.set("realmAddress", character.realm_address)?;
    table.set("lastLoginBuild", 110205i32)?;
    table.set("lastActiveTime", character.last_active_time)?;
    table.set("isLocked", false)?;
    table.set("isGhost", false)?;
    table.set("isTrialBoost", false)?;
    table.set("isTrialBoostCompleted", false)?;
    table.set("isExpansionTrialCharacter", false)?;
    table.set("isLockedByExpansion", false)?;
    table.set("isRevokedCharacterUpgrade", false)?;
    table.set("revokedCharacterUpgrade", false)?;
    table.set("mailSenders", lua.create_table()?)?;
    Ok(Value::Table(table))
}

fn glue_service_character_info(lua: &Lua, guid: &str) -> Result<Value> {
    if glue_character_by_guid(guid).is_none() || !has_glue_character(lua) {
        return Ok(Value::Nil);
    }

    let table = lua.create_table()?;
    table.set("boostInProgress", false)?;
    table.set("hasFactionChange", false)?;
    table.set("hasRaceChange", false)?;
    table.set("hasCustomize", false)?;
    table.set("customizeDisabled", false)?;
    table.set("isTrialBoostCompleted", false)?;
    table.set("isRevokedCharacterUpgrade", false)?;
    table.set("revokedCharacterUpgrade", false)?;
    table.set("hasNameChange", false)?;
    table.set("rpeArathiAvailable", false)?;
    Ok(Value::Table(table))
}

/// Register all additional C_* namespace stubs.
pub fn register_c_stubs_api(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_core_namespaces(lua, std::rc::Rc::clone(&state))?;
    register_ui_and_chat_stubs(lua, state)?;
    register_missing_globals(lua)?;
    register_missing_namespaces(lua)?;
    register_c_perks_activities(lua)?;
    register_game_state_stubs(lua)?;
    register_c_incoming_summon(lua)?;
    super::c_stubs_api_extra::register_extra_stubs(lua)?;
    super::c_stubs_api_combat::register_combat_stubs(lua)?;
    super::c_stubs_api_professions::register_profession_stubs(lua)?;
    Ok(())
}

fn register_core_namespaces(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_c_achievement_info(lua)?;
    super::hero_talents::register_c_class_talents(lua, std::rc::Rc::clone(&state))?;
    register_c_guild(lua, std::rc::Rc::clone(&state))?;
    register_c_guild_info(lua)?;
    register_c_lfg_list(lua)?;
    register_c_loss_of_control(lua)?;
    register_c_mail(lua)?;
    register_c_stable_info(lua)?;
    register_c_tutorial(lua)?;
    super::action_bar_api::register_c_action_bar_namespace(lua, state.clone())?;
    register_unit_frame_global_stubs(lua, std::rc::Rc::clone(&state))?;
    register_powerbar_prediction_colors(lua)?;
    super::c_stubs_achievement::register_achievement_stubs(lua)?;
    super::c_stubs_achievement::register_tracking_stubs(lua)?;
    Ok(())
}

fn register_ui_and_chat_stubs(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_c_log(lua)?;
    register_c_campaign_info(lua)?;
    register_quest_global_functions(lua, state)?;
    register_chat_stubs(lua)?;
    register_chat_window_stubs(lua)?;
    register_c_macro(lua)?;
    register_c_wowlabs_matchmaking(lua)?;
    super::fading_frame_api::register_fading_frame_stubs(lua)?;
    Ok(())
}

fn register_c_achievement_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetRewardItemID",
        lua.create_function(|_, _achievement_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetAchievementInfo",
        lua.create_function(|_, _achievement_id: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_AchievementInfo", t)?;
    Ok(())
}

fn register_c_guild(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let t = lua.create_table()?;
    let st = std::rc::Rc::clone(&state);
    t.set(
        "GetNumMembers",
        lua.create_function(move |_, ()| Ok(st.borrow().world.guild_num_members))?,
    )?;
    let st = std::rc::Rc::clone(&state);
    t.set(
        "IsInGuild",
        lua.create_function(move |_, ()| Ok(st.borrow().world.guild_name.is_some()))?,
    )?;
    t.set(
        "GetGuildInfo",
        lua.create_function(move |_, _unit: Option<String>| {
            let s = state.borrow();
            match &s.world.guild_name {
                Some(name) => {
                    let rank = s.world.guild_rank.clone().unwrap_or_default();
                    Ok((name.clone(), rank, s.world.guild_num_members, String::new()))
                }
                None => Ok((String::new(), String::new(), 0i32, String::new())),
            }
        })?,
    )?;
    t.set(
        "GetMemberInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_Guild", t)?;
    Ok(())
}

fn register_c_guild_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetGuildTabardInfo",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetGuildNewsInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "AreGuildEventsEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("GuildRoster", lua.create_function(|_, ()| Ok(()))?)?;
    lua.globals().set("C_GuildInfo", t)?;
    Ok(())
}

fn register_c_lfg_list(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetActiveEntryInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "HasActiveEntryInfo",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetSearchResultInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "CanCreateQuestGroup",
        lua.create_function(|_, _quest_id: i32| Ok(false))?,
    )?;
    t.set(
        "GetAvailableRoles",
        lua.create_function(|_, ()| Ok((true, true, true)))?,
    )?;
    t.set(
        "GetApplications",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetNumApplications",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    t.set("IsSquelched", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetAvailableCategories",
        lua.create_function(|lua, _args: mlua::MultiValue| lua.create_table())?,
    )?;
    t.set("HasActivityList", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_LFGList", t)?;
    Ok(())
}

fn register_c_loss_of_control(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetActiveLossOfControlData",
        lua.create_function(|_, _index: Option<i32>| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetActiveLossOfControlDataCount",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    lua.globals().set("C_LossOfControl", t)?;
    Ok(())
}

fn register_c_mail(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetNumItems", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("HasNewMail", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsCommandPending", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_Mail", t)?;
    Ok(())
}

fn register_c_stable_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetNumStablePets", lua.create_function(|_, ()| Ok(0i32))?)?;
    lua.globals().set("C_StableInfo", t)?;
    Ok(())
}

fn register_c_tutorial(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetTutorialStatus",
        lua.create_function(|_, _tutorial_id: Option<i32>| Ok(false))?,
    )?;
    t.set(
        "SetTutorialFlag",
        lua.create_function(|_, _tutorial_id: i32| Ok(()))?,
    )?;
    lua.globals().set("C_Tutorial", t)?;
    Ok(())
}

/// Resolve a texture path or file data ID to a WoW interface path.
fn resolve_texture_path(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => s.to_str().ok().and_then(|s| {
            if let Ok(id) = s.parse::<u32>() {
                let p = crate::manifest_interface_data::get_texture_path(id)?;
                Some(format!("Interface\\{}", p.replace('/', "\\")))
            } else {
                Some(s.to_string())
            }
        }),
        Value::Integer(n) => crate::manifest_interface_data::get_texture_path(*n as u32)
            .map(|p| format!("Interface\\{}", p.replace('/', "\\"))),
        Value::Number(n) => crate::manifest_interface_data::get_texture_path(*n as u32)
            .map(|p| format!("Interface\\{}", p.replace('/', "\\"))),
        _ => None,
    }
}

/// Set texture path on a frame UserData (FrameRef) widget.
fn set_texture_on_handle(lua: &mlua::Lua, tex: &Value, path: Option<String>) {
    if let Some(id) = crate::lua_api::frame::extract_frame_id(tex) {
        let state_rc = crate::lua_api::frame::get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.texture = path;
        }
    }
}

/// Global function stubs needed by Blizzard_UnitFrame.
fn register_unit_frame_global_stubs(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let g = lua.globals();
    let s2 = std::rc::Rc::clone(&state);
    g.set(
        "InCombatLockdown",
        lua.create_function(move |_, ()| Ok(s2.borrow().player.in_combat))?,
    )?;
    g.set(
        "IsResting",
        lua.create_function(move |_, ()| Ok(state.borrow().player.is_resting))?,
    )?;
    g.set("IsPVPTimerRunning", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("GetPVPTimer", lua.create_function(|_, ()| Ok(0.0f64))?)?;
    g.set(
        "GetReadyCheckStatus",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;
    g.set(
        "HasLFGRestrictions",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetPartyLFGID",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "RequestGuildPartyState",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetLFGCategoryForID",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "IsEveryoneAssistant",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "WorldLootObjectExists",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set("IsInRaid", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetRaidRosterInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    g.set("PartialPlayTime", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("NoPlayTime", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetBillingTimeRested",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "SetPortraitToTexture",
        lua.create_function(|lua, (tex, path): (Value, Value)| {
            set_texture_on_handle(lua, &tex, resolve_texture_path(&path));
            Ok(())
        })?,
    )?;
    register_unit_frame_global_stubs_2(lua)?;
    Ok(())
}

/// Continuation of unit-frame global stubs (combat, arena, UIParent handlers).
fn register_unit_frame_global_stubs_2(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "GetUnitTotalModifiedMaxHealthPercent",
        lua.create_function(|_, _unit: Option<String>| Ok(0.0f64))?,
    )?;
    g.set(
        "IsThreatWarningEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetThreatStatusColor",
        lua.create_function(|_, _status: i32| Ok((1.0f64, 1.0f64, 1.0f64)))?,
    )?;
    g.set("LE_REALM_RELATION_VIRTUAL", 3i32)?;
    g.set(
        "IsActiveBattlefieldArena",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumArenaOpponents",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetBattlefieldEstimatedWaitTime",
        lua.create_function(|_, _index: Value| Ok(0i32))?,
    )?;
    g.set("PetUsesPetFrame", lua.create_function(|_, ()| Ok(true))?)?;
    g.set(
        "UnitIsPossessed",
        lua.create_function(|_, _unit: Option<String>| Ok(false))?,
    )?;
    g.set(
        "GetReleaseTimeRemaining",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "FCF_OnUpdate",
        lua.create_function(|_, _elapsed: Option<f64>| Ok(()))?,
    )?;
    g.set(
        "HelpOpenWebTicketButton_OnUpdate",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    g.set(
        "GetLootSpecialization",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    // UIParent PLAYER_ENTERING_WORLD handler stubs
    g.set(
        "GetSpellConfirmationPromptsInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "ResurrectGetOfferer",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetActiveLootRollIDs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "GetTutorialsEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "BoostTutorial_AttemptLoad",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "ExpansionTrial_CheckLoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "SubscriptionInterstitial_LoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "ShowResurrectRequest",
        lua.create_function(|_, _offerer: String| Ok(()))?,
    )?;
    g.set(
        "GroupLootContainer_AddRoll",
        lua.create_function(|_, (_id, _dur): (Value, Value)| Ok(()))?,
    )?;
    g.set(
        "RemixArtifactTutorialUI_LoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    Ok(())
}

/// POWERBAR_PREDICTION_COLOR_* globals used by PowerBarColorUtil.lua at parse time.
const POWERBAR_COLORS: &[(&str, f64, f64, f64)] = &[
    ("POWERBAR_PREDICTION_COLOR_MANA", 0.0, 0.0, 1.0),
    ("POWERBAR_PREDICTION_COLOR_RAGE", 1.0, 0.0, 0.0),
    ("POWERBAR_PREDICTION_COLOR_FOCUS", 1.0, 0.5, 0.25),
    ("POWERBAR_PREDICTION_COLOR_ENERGY", 1.0, 1.0, 0.0),
    ("POWERBAR_PREDICTION_COLOR_RUNIC_POWER", 0.0, 0.82, 1.0),
    ("POWERBAR_PREDICTION_COLOR_LUNAR_POWER", 0.3, 0.52, 0.9),
    ("POWERBAR_PREDICTION_COLOR_MAELSTROM", 0.0, 0.5, 1.0),
    ("POWERBAR_PREDICTION_COLOR_INSANITY", 0.4, 0.0, 0.8),
    ("POWERBAR_PREDICTION_COLOR_FURY", 0.788, 0.259, 0.992),
    ("POWERBAR_PREDICTION_COLOR_PAIN", 1.0, 0.612, 0.0),
];

fn build_color_entry(
    lua: &Lua,
    r: f64,
    green: f64,
    b: f64,
    get_rgba: &mlua::Function,
    get_rgb: &mlua::Function,
) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set("r", r)?;
    t.set("g", green)?;
    t.set("b", b)?;
    t.set("a", 0.5f64)?;
    t.set("GetRGBA", get_rgba.clone())?;
    t.set("GetRGB", get_rgb.clone())?;
    Ok(t)
}

fn register_powerbar_prediction_colors(lua: &Lua) -> Result<()> {
    let get_rgba = lua.create_function(|_, this: mlua::Table| {
        Ok((
            this.get::<f64>("r")?,
            this.get::<f64>("g")?,
            this.get::<f64>("b")?,
            this.get::<f64>("a")?,
        ))
    })?;
    let get_rgb = lua.create_function(|_, this: mlua::Table| {
        Ok((
            this.get::<f64>("r")?,
            this.get::<f64>("g")?,
            this.get::<f64>("b")?,
        ))
    })?;
    let g = lua.globals();
    for &(name, r, green, b) in POWERBAR_COLORS {
        g.set(
            name,
            build_color_entry(lua, r, green, b, &get_rgba, &get_rgb)?,
        )?;
    }
    Ok(())
}

fn register_c_log(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("LogMessage", lua.create_function(|_, _msg: Value| Ok(()))?)?;
    t.set(
        "LogErrorMessage",
        lua.create_function(|_, _msg: Value| Ok(()))?,
    )?;
    lua.globals().set("C_Log", t)?;
    Ok(())
}

/// C_CampaignInfo namespace - campaign/war campaign data.
fn register_c_campaign_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetCampaignID",
        lua.create_function(|_, _quest_id: i32| Ok(0i32))?,
    )?;
    t.set(
        "GetCampaignInfo",
        lua.create_function(|_, _campaign_id: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_CampaignInfo", t)?;
    Ok(())
}

/// Quest-related global functions used by ObjectiveTracker.
fn register_quest_global_functions(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let g = lua.globals();
    g.set(
        "IsInInstance",
        lua.create_function(move |_, ()| {
            let s = state.borrow();
            Ok((s.world.in_instance, s.world.instance_type.clone()))
        })?,
    )?;
    g.set(
        "IsQuestSequenced",
        lua.create_function(|_, _quest_id: i32| Ok(false))?,
    )?;
    g.set(
        "GetQuestLogCompletionText",
        lua.create_function(|_, _log_idx: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetQuestProgressBarPercent",
        lua.create_function(|_, _quest_id: i32| Ok(0.0f64))?,
    )?;
    g.set(
        "QuestMapFrame_GetFocusedQuestID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "IsModifiedClick",
        lua.create_function(|_, _action: String| Ok(false))?,
    )?;
    g.set(
        "GetQuestLink",
        lua.create_function(|_, _quest_id: i32| Ok(Value::Nil))?,
    )?;
    g.set("IsInJailersTower", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "IsOnGroundFloorInJailersTower",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumAutoQuestPopUps",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetAutoQuestPopUp",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetQuestLogSpecialItemInfo",
        lua.create_function(|_, _log_idx: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetTasksTable",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "ExpandQuestHeader",
        lua.create_function(|_, (_idx, _no_update): (i32, Option<bool>)| Ok(()))?,
    )?;
    g.set(
        "CollapseQuestHeader",
        lua.create_function(|_, (_idx, _no_update): (i32, Option<bool>)| Ok(()))?,
    )?;
    register_quest_leaderboard_functions(lua, &g)?;
    Ok(())
}

/// GetNumQuestLeaderBoards / GetQuestLogLeaderBoard - quest objective data.
/// Delegates to c_quest_api which owns the single source of truth for quest data.
fn register_quest_leaderboard_functions(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetNumQuestLeaderBoards",
        lua.create_function(|_, log_idx: i32| {
            Ok(super::c_quest_api::num_quest_leaderboards(log_idx))
        })?,
    )?;
    g.set(
        "GetQuestLogLeaderBoard",
        lua.create_function(
            |_, (obj_idx, log_idx, _suppress): (i32, i32, Option<bool>)| {
                Ok(super::c_quest_api::quest_leaderboard_entry(
                    log_idx, obj_idx,
                ))
            },
        )?,
    )?;
    Ok(())
}

/// Chat window management stubs needed by FloatingChatFrame.
fn register_chat_window_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "SetChatWindowLocked",
        lua.create_function(|_, (_id, _locked): (i32, bool)| Ok(()))?,
    )?;
    g.set(
        "SetChatWindowUninteractable",
        lua.create_function(|_, (_id, _flag): (i32, bool)| Ok(()))?,
    )?;
    g.set(
        "GetChatWindowSavedDimensions",
        lua.create_function(|_, _id: i32| Ok((430.0f64, 120.0f64)))?,
    )?;
    g.set(
        "SetChatWindowColor",
        lua.create_function(|_, (_id, _r, _g, _b): (i32, f64, f64, f64)| Ok(()))?,
    )?;
    g.set(
        "SetChatWindowAlpha",
        lua.create_function(|_, (_id, _a): (i32, f64)| Ok(()))?,
    )?;
    g.set(
        "GetChatWindowSavedPosition",
        lua.create_function(|_, _id: i32| {
            // Returns: point, yOffset, xOffset, relativePoint
            Ok(("BOTTOMLEFT", 0.0f64, 0.0f64, "BOTTOMLEFT"))
        })?,
    )?;
    // ChangeChatColor: sets r,g,b on ChatTypeInfo[type]
    g.set(
        "ChangeChatColor",
        lua.create_function(|lua, (ct, r, g, b): (String, f64, f64, f64)| {
            let cti: mlua::Table = lua
                .globals()
                .get::<mlua::Table>("ChatTypeInfo")?
                .get(&*ct)?;
            cti.set("r", r)?;
            cti.set("g", g)?;
            cti.set("b", b)?;
            Ok(())
        })?,
    )?;
    Ok(())
}

/// Chat-related global function stubs needed by Blizzard_ChatFrame.
fn register_chat_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    // GetChatTypeIndex: deterministic integer from chat type name
    g.set(
        "GetChatTypeIndex",
        lua.create_function(|_, name: String| {
            let hash = name
                .bytes()
                .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
            Ok((hash % 50 + 1) as i32)
        })?,
    )?;
    // CreateSecureDelegate: no taint system, return the function as-is
    g.set(
        "CreateSecureDelegate",
        lua.create_function(|_, func: mlua::Function| Ok(func))?,
    )?;
    // GetChatWindowInfo: return defaults (only window 1 is shown)
    // Returns: name, fontSize, r, g, b, alpha, shown, locked, docked, uninteractable
    // Default color is black (0,0,0) at 25% alpha, matching DEFAULT_CHATFRAME_COLOR/ALPHA
    g.set(
        "GetChatWindowInfo",
        lua.create_function(|_, id: i32| {
            let name = format!("ChatFrame{id}");
            let shown = id == 1;
            Ok((
                name, 14.0f64, 0.0f64, 0.0f64, 0.0f64, 0.25f64, shown, false, false, false,
            ))
        })?,
    )?;
    // GetChatWindowMessages/GetChatWindowChannels: return no message types or channels
    g.set(
        "GetChatWindowMessages",
        lua.create_function(|_, _id: i32| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "GetChatWindowChannels",
        lua.create_function(|_, _id: i32| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "GetDefaultLanguage",
        lua.create_function(|_, ()| Ok("Common"))?,
    )?;
    g.set(
        "GetAlternativeDefaultLanguage",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// C_Macro namespace - macro management stubs.
fn register_c_macro(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "SetMacroExecuteLineCallback",
        lua.create_function(|_, _cb: Value| Ok(()))?,
    )?;
    t.set(
        "GetMacroInfo",
        lua.create_function(|_, _id: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetNumMacros",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    lua.globals().set("C_Macro", t)?;
    Ok(())
}

fn register_c_wowlabs_matchmaking(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetCurrentParty",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetPartyPlaylistEntry",
        lua.create_function(|_, ()| Ok(mlua::Value::Nil))?,
    )?;
    t.set("ClearFastLogin", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "SetAutoQueueOnLogout",
        lua.create_function(|_, _flag: bool| Ok(()))?,
    )?;
    lua.globals().set("C_WoWLabsMatchmaking", t)?;

    // C_WowLabsDataManager (note: different casing from C_WoWLabsMatchmaking)
    let dm = lua.create_table()?;
    dm.set("IsInPrematch", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_WowLabsDataManager", dm)?;
    Ok(())
}

/// FadingFrame_* global functions used by ZoneText.lua.

/// Missing global functions referenced during startup events.
fn register_missing_globals(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.raw_set(GLUE_CHARACTER_CREATE_TYPE_KEY, 0i32)?;
    reset_glue_character_create_state(lua)?;
    register_timer_and_bar_globals(lua, &g)?;
    register_lfg_and_guild_stubs(lua, &g)?;
    register_action_button_util(lua, &g)?;
    register_player_location_stub(lua, &g)?;
    g.set(
        "GetSavedAccountName",
        lua.create_function(|lua, ()| {
            let Some(state) =
                lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>>()
            else {
                return Ok(String::new());
            };
            Ok(state.borrow().saved_account_name.clone())
        })?,
    )?;
    g.set(
        "SetSavedAccountName",
        lua.create_function(|lua, account_name: String| {
            if let Some(state) =
                lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>>()
            {
                let mut state = state.borrow_mut();
                state.saved_account_name = account_name.clone();
                if !account_name.is_empty() && state.saved_account_list.is_empty() {
                    state.saved_account_list = account_name;
                }
            }
            Ok(())
        })?,
    )?;
    g.set(
        "GetSavedAccountList",
        lua.create_function(|lua, ()| {
            let Some(state) =
                lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>>()
            else {
                return Ok(String::new());
            };
            Ok(state.borrow().saved_account_list.clone())
        })?,
    )?;
    g.set(
        "SetUsesToken",
        lua.create_function(|lua, uses_token: bool| {
            if let Some(state) =
                lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>>()
            {
                state.borrow_mut().uses_token = uses_token;
            }
            Ok(())
        })?,
    )?;
    g.set(
        "WasScreenFirstDisplayed",
        lua.create_function(|lua, ()| {
            let Some(state) =
                lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>>()
            else {
                return Ok(false);
            };
            let state = state.borrow();
            Ok(state.screen_first_displayed || state.screen_kind.is_glue())
        })?,
    )?;
    g.set(
        "InitializeCharacterScreenData",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "SetInCharacterSelect",
        lua.create_function(|_, _in_character_select: bool| Ok(()))?,
    )?;
    g.set(
        "SetWorldFrameStrata",
        lua.create_function(|_, _frame: Value| Ok(()))?,
    )?;
    g.set(
        "SetCharSelectModelFrame",
        lua.create_function(|_, _frame_name: String| Ok(()))?,
    )?;
    g.set(
        "SetCharSelectMapSceneFrame",
        lua.create_function(|_, _frame_name: String| Ok(()))?,
    )?;
    g.set(
        "MoveCharactersToMapSceneFrame",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "MoveCharactersToModelFFXFrame",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetSelectBackgroundModel",
        lua.create_function(|_, _character_id: i32| Ok(0i32))?,
    )?;
    g.set(
        "SetCharSelectBackground",
        lua.create_function(|_, _background_id: Value| Ok(()))?,
    )?;
    g.set(
        "PlayGlueAmbience",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    g.set(
        "StopGlueAmbience",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    g.set(
        "UpdateSelectionCustomizationScene",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetMaxWarbandGroupCount",
        lua.create_function(|_, ()| Ok(20i32))?,
    )?;
    g.set(
        "GetNumCharacters",
        lua.create_function(|lua, _include_empty_slots: Option<bool>| {
            Ok(glue_character_count(lua))
        })?,
    )?;
    g.set(
        "GetCharacterGUID",
        lua.create_function(|lua, index: i32| match glue_character_guid(lua, index) {
            Some(guid) => Ok(Value::String(lua.create_string(&guid)?)),
            None => Ok(Value::Nil),
        })?,
    )?;
    g.set(
        "GetCharacterRace",
        lua.create_function(|_, index: i32| {
            if let Some(character) = glue_character(index) {
                Ok((index, String::from(character.race_name)))
            } else {
                Ok((0i32, String::new()))
            }
        })?,
    )?;
    g.set(
        "GetBasicCharacterInfo",
        lua.create_function(|lua, guid: String| glue_basic_character_info(lua, &guid))?,
    )?;
    g.set(
        "GetServiceCharacterInfo",
        lua.create_function(|lua, guid: String| glue_service_character_info(lua, &guid))?,
    )?;
    g.set(
        "GetCharacterSelection",
        lua.create_function(|lua, ()| Ok(glue_selected_character(lua)))?,
    )?;
    g.set(
        "SelectCharacter",
        lua.create_function(|lua, character_id: i32| {
            set_glue_selected_character(lua, character_id)?;
            let new_selection = glue_selected_character(lua);
            let is_dispatching = lua
                .globals()
                .get::<Option<bool>>(GLUE_SELECT_CHARACTER_DISPATCH_KEY)
                .ok()
                .flatten()
                .unwrap_or(false);
            if new_selection > 0 && !is_dispatching {
                lua.globals()
                    .set(GLUE_SELECT_CHARACTER_DISPATCH_KEY, true)?;
                let fire_event: mlua::Function = lua.globals().get("FireEvent")?;
                fire_event.call::<()>(("UPDATE_SELECTED_CHARACTER", new_selection))?;
                lua.globals()
                    .set(GLUE_SELECT_CHARACTER_DISPATCH_KEY, false)?;
            }
            Ok(())
        })?,
    )?;
    g.set("CanCreateCharacter", lua.create_function(|_, ()| Ok(true))?)?;
    g.set(
        "GetCharacterListGroupsInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "SaveCharacterOrder",
        lua.create_function(|_, _: Value| Ok(()))?,
    )?;
    g.set(
        "GetCharacterListUpdate",
        lua.create_function(|lua, ()| {
            set_glue_selected_character(lua, glue_selected_character(lua))?;
            let fire_event: mlua::Function = lua.globals().get("FireEvent")?;
            fire_event.call::<()>(("CHARACTER_LIST_UPDATE", glue_character_count(lua)))?;
            Ok(())
        })?,
    )?;
    g.set(
        "CheckCharacterUndeleteCooldown",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetCharacterUndeleteStatus",
        lua.create_function(|_, ()| Ok((true, false, 0i32, 0i32)))?,
    )?;
    g.set(
        "GetServerName",
        lua.create_function(|_, ()| {
            Ok((
                String::from("Burning Blade"),
                String::new(),
                false,
                false,
                1i32,
            ))
        })?,
    )?;
    g.set(
        "IsConnectedToServer",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    g.set(
        "ShouldShowLevelSquishDialog",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetActiveTimerunningSeasonID",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetPlayersOnServer",
        lua.create_function(|_, ()| Ok((false, 0i32, 0i32)))?,
    )?;
    g.set(
        "GetCharacterTimerunningSeasonID",
        lua.create_function(|_, _guid: Value| Ok(Value::Nil))?,
    )?;
    g.set(
        "IsCharacterTimerunning",
        lua.create_function(|_, _guid: Value| Ok(false))?,
    )?;
    g.set(
        "IsCharacterTimerunningConversionAllowed",
        lua.create_function(|_, _guid: Value| Ok(false))?,
    )?;
    g.set(
        "IsTimerunningEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "HasCheckedSystemRequirements",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    g.set(
        "SetCheckedSystemRequirements",
        lua.create_function(|_, _checked: bool| Ok(()))?,
    )?;
    g.set(
        "AlertFrame_SetDuration",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    g.set(
        "UnitGetAvailableRoles",
        lua.create_function(|_, _unit: Value| Ok((true, true, true)))?,
    )?;
    g.set(
        "UnitIsGameObject",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetLFGRoleUpdate",
        lua.create_function(|_, ()| Ok((false, 0i32, 0i32, 0i32, 0i32, false)))?,
    )?;
    register_paperdoll_container_and_misc_stubs(lua, &g)?;
    register_secure_env_globals(lua, &g)?;
    Ok(())
}

fn register_player_location_stub(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if !g.get::<Value>("PlayerLocation")?.is_nil() {
        return Ok(());
    }

    lua.load(
        r#"
        PlayerLocation = {};
        PlayerLocationMixin = {};

        local function CreatePlayerLocation(fieldName, ...)
            local playerLocation = CreateFromMixins(PlayerLocationMixin);
            if fieldName == "guid" then
                playerLocation:SetGUID(...);
            elseif fieldName == "unit" then
                playerLocation:SetUnit(...);
            elseif fieldName == "chatLineID" then
                playerLocation:SetChatLineID(...);
            elseif fieldName == "communityData" then
                playerLocation:SetCommunityData(...);
            elseif fieldName == "communityInvitation" then
                playerLocation:SetCommunityInvitation(...);
            elseif fieldName == "battlefieldScoreIndex" then
                playerLocation:SetBattlefieldScoreIndex(...);
            elseif fieldName == "voiceID" then
                playerLocation:SetVoiceID(...);
            elseif fieldName == "battleNetID" then
                playerLocation:SetBattleNetID(...);
            end
            return playerLocation;
        end

        function PlayerLocation:CreateFromGUID(guid)
            return CreatePlayerLocation("guid", guid);
        end

        function PlayerLocation:CreateFromUnit(unit)
            return CreatePlayerLocation("unit", unit);
        end

        function PlayerLocation:CreateFromChatLineID(lineID)
            return CreatePlayerLocation("chatLineID", lineID);
        end

        function PlayerLocation:CreateFromCommunityChatData(clubID, streamID, epoch, position)
            return CreatePlayerLocation("communityData", clubID, streamID, epoch, position);
        end

        function PlayerLocation:CreateFromCommunityInvitation(clubID, guid)
            return CreatePlayerLocation("communityInvitation", clubID, guid);
        end

        function PlayerLocation:CreateFromBattlefieldScoreIndex(index)
            return CreatePlayerLocation("battlefieldScoreIndex", index);
        end

        function PlayerLocation:CreateFromVoiceID(memberID, channelID)
            return CreatePlayerLocation("voiceID", memberID, channelID);
        end

        function PlayerLocation:CreateFromBattleNetID(battleNetID)
            return CreatePlayerLocation("battleNetID", battleNetID);
        end

        function PlayerLocationMixin:SetGUID(guid)
            self:ClearAndSetField("guid", guid);
        end

        function PlayerLocationMixin:IsGUID()
            return self.guid ~= nil;
        end

        function PlayerLocationMixin:IsBattleNetGUID()
            return false;
        end

        function PlayerLocationMixin:GetGUID()
            return self.guid or self.communityClubInviterGUID;
        end

        function PlayerLocationMixin:SetUnit(unit)
            self:ClearAndSetField("unit", unit);
        end

        function PlayerLocationMixin:IsUnit()
            return self.unit ~= nil;
        end

        function PlayerLocationMixin:GetUnit()
            return self.unit;
        end

        function PlayerLocationMixin:SetChatLineID(lineID)
            self:ClearAndSetField("chatLineID", lineID);
        end

        function PlayerLocationMixin:IsChatLineID()
            return self.chatLineID ~= nil;
        end

        function PlayerLocationMixin:GetChatLineID()
            return self.chatLineID;
        end

        function PlayerLocationMixin:SetBattlefieldScoreIndex(index)
            self:ClearAndSetField("battlefieldScoreIndex", index);
        end

        function PlayerLocationMixin:IsBattlefieldScoreIndex()
            return self.battlefieldScoreIndex ~= nil;
        end

        function PlayerLocationMixin:GetBattlefieldScoreIndex()
            return self.battlefieldScoreIndex;
        end

        function PlayerLocationMixin:SetVoiceID(memberID, channelID)
            self:Clear();
            self.voiceMemberID = memberID;
            self.voiceChannelID = channelID;
        end

        function PlayerLocationMixin:IsVoiceID()
            return self.voiceMemberID ~= nil and self.voiceChannelID ~= nil;
        end

        function PlayerLocationMixin:GetVoiceID()
            return self.voiceMemberID, self.voiceChannelID;
        end

        function PlayerLocationMixin:SetBattleNetID(battleNetID)
            self:Clear();
            self.battleNetID = battleNetID;
        end

        function PlayerLocationMixin:IsBattleNetID()
            return self.battleNetID ~= nil;
        end

        function PlayerLocationMixin:GetBattleNetID()
            return self.battleNetID;
        end

        function PlayerLocationMixin:SetCommunityData(clubID, streamID, epoch, position)
            self:Clear();
            self.communityClubID = clubID;
            self.communityStreamID = streamID;
            self.communityEpoch = epoch;
            self.communityPosition = position;
        end

        function PlayerLocationMixin:IsCommunityData()
            return self.communityClubID ~= nil and self.communityStreamID ~= nil and self.communityEpoch ~= nil and self.communityPosition ~= nil;
        end

        function PlayerLocationMixin:SetCommunityInvitation(clubID, guid)
            self:Clear();
            self.communityClubID = clubID;
            self.communityClubInviterGUID = guid;
        end

        function PlayerLocationMixin:IsCommunityInvitation()
            return self.communityClubID ~= nil and self.communityClubInviterGUID ~= nil;
        end

        function PlayerLocationMixin:IsValid()
            return true;
        end

        function PlayerLocationMixin:Clear()
            self.guid = nil;
            self.unit = nil;
            self.chatLineID = nil;
            self.battlefieldScoreIndex = nil;
            self.voiceMemberID = nil;
            self.voiceChannelID = nil;
            self.communityClubID = nil;
            self.communityStreamID = nil;
            self.communityEpoch = nil;
            self.communityPosition = nil;
            self.communityClubInviterGUID = nil;
            self.battleNetID = nil;
        end

        function PlayerLocationMixin:ClearAndSetField(fieldName, field)
            self:Clear();
            self[fieldName] = field;
        end
        "#,
    )
    .exec()?;

    Ok(())
}

fn register_secure_env_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    // Globals normally set in secure-environment files via SwapToGlobalEnvironment().
    // Our sim doesn't implement secure environments, so these need explicit stubs.
    let combat_log = lua.create_table()?;
    combat_log.set(
        "GenerateMessage",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    g.set("CombatLogInbound", combat_log)?;
    g.set(
        "StoreFrame_CheckForFree",
        lua.create_function(|_, ()| Ok(()))?,
    )?;

    // GetAvailableLocaleInfo: generated stub returns empty table, needs locale data.
    // Set here so the is_nil() check in generated_stubs.rs skips it.
    g.set(
        "GetAvailableLocaleInfo",
        lua.create_function(|lua, _: MultiValue| {
            let entry = lua.create_table()?;
            entry.set("localeName", "enUS")?;
            entry.set("localeId", 1)?;
            let result = lua.create_table()?;
            result.set(1, entry)?;
            Ok(Value::Table(result))
        })?,
    )?;
    Ok(())
}

fn register_timer_and_bar_globals(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set("GetDefaultScale", lua.create_function(|_, ()| Ok(1.0f64))?)?;
    g.set(
        "HasVehicleActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "HasOverrideActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetMaxBattlefieldID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set("RequestRaidInfo", lua.create_function(|_, ()| Ok(()))?)?;
    g.set(
        "RequestLFDPlayerLockInfo",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "RequestLFDPartyLockInfo",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetQuestTimers",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetMirrorTimerInfo",
        lua.create_function(|_, _timer: Value| Ok(("UNKNOWN", 0i32, 0i32, -1i32, false, "")))?,
    )?;
    g.set(
        "GetInventoryAlertStatus",
        lua.create_function(|_, _slot: i32| Ok(0i32))?,
    )?;
    g.set(
        "GetWorldElapsedTimers",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetWorldElapsedTime",
        lua.create_function(|_, _id: i32| Ok((0i32, 0i32, 0i32)))?,
    )?;
    g.set("HasBonusActionBar", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "HasTempShapeshiftActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set("PutItemInBackpack", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("PutItemInBag", lua.create_function(|_, _bag: i32| Ok(()))?)?;
    Ok(())
}

/// PaperDoll, container frame, group roster, and miscellaneous stubs.
fn register_paperdoll_container_and_misc_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "PaperDollItemSlotButton_OnLoad",
        lua.create_function(|_, _frame: Value| Ok(()))?,
    )?;
    g.set(
        "PaperDollItemSlotButton_OnShow",
        lua.create_function(|_, _frame: Value| Ok(()))?,
    )?;
    g.set(
        "ContainerFrame_GetContainerNumSlots",
        lua.create_function(|_, id: Value| {
            let bag = match id {
                Value::Integer(n) => n as i32,
                Value::Number(n) => n as i32,
                _ => -1,
            };
            let count = super::c_container_api::bag_slot_count(bag);
            Ok((count, count))
        })?,
    )?;
    g.set(
        "GetGroupMemberCounts",
        lua.create_function(|lua, ()| {
            let t = lua.create_table()?;
            for key in ["TANK", "HEALER", "DAMAGER", "NOROLE", "ASSIGNEDROLE"] {
                t.set(key, 0i32)?;
            }
            Ok(t)
        })?,
    )?;
    g.set(
        "GetDungeonDifficultyID",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    g.set("GetSendMailPrice", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set(
        "GuildControlSetRank",
        lua.create_function(|_, _rank: Value| Ok(()))?,
    )?;
    g.set(
        "StoreSecureReference",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    // ResetCursor — cursor_api handles ClearCursor; ResetCursor resets visual.
    g.set("ResetCursor", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

/// LFG, dungeon finder, guild, and honor global stubs.
fn register_lfg_and_guild_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetLFGProposal",
        lua.create_function(|_, ()| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "GetLFGQueuedList",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(mlua::MultiValue::new()))?,
    )?;
    // GetActionBarToggles/SetActionBarToggles registered in action_bar_api.rs
    g.set(
        "UnitPowerBarTimerInfo",
        lua.create_function(|_, _unit: Value| Ok(Value::Nil))?,
    )?;
    g.set("GetWebTicket", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    g.set(
        "UnitGroupRolesAssigned",
        lua.create_function(|_, _unit: Option<String>| Ok("NONE"))?,
    )?;
    g.set(
        "GuildControlGetNumRanks",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "RequestGuildChallengeInfo",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetGuildFactionGroup",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    g.set(
        "UnitHonor",
        lua.create_function(|_, _unit: Option<String>| Ok(0i32))?,
    )?;
    g.set(
        "UnitHonorMax",
        lua.create_function(|_, _unit: Option<String>| Ok(100i32))?,
    )?;
    g.set(
        "UnitHonorLevel",
        lua.create_function(|_, _unit: Option<String>| Ok(1i32))?,
    )?;
    g.set(
        "GetLFGInfoServer",
        lua.create_function(|_, (_cat, _id): (Value, Value)| Ok(mlua::MultiValue::new()))?,
    )?;
    Ok(())
}

/// ActionButtonUtil enum tables needed by Blizzard_SpellSearch at load time.
/// Blizzard_ActionBar will overwrite this with the full version when it loads.
fn register_action_button_util(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let abu = lua.create_table()?;
    let status = lua.create_table()?;
    status.set("NotMissing", 1)?;
    status.set("MissingFromAllBars", 2)?;
    status.set("OnInactiveBonusBar", 3)?;
    status.set("OnDisabledActionBar", 4)?;
    abu.set("ActionBarActionStatus", status)?;
    let bar_type = lua.create_table()?;
    bar_type.set("MainActionBar", 1)?;
    bar_type.set("MultiActionBar", 2)?;
    bar_type.set("StanceBar", 3)?;
    bar_type.set("PetBar", 4)?;
    bar_type.set("PossessActionBar", 5)?;
    bar_type.set("BonusBar", 6)?;
    bar_type.set("VehicleBar", 16)?;
    bar_type.set("TempShapeshiftBar", 17)?;
    bar_type.set("OverrideBar", 18)?;
    abu.set("ActionBarType", bar_type)?;
    g.set("ActionButtonUtil", abu)?;
    Ok(())
}

/// C_PerksActivities - Monthly activities / Trading Post tracking.
fn register_c_perks_activities(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetTrackedPerksActivities",
        lua.create_function(|lua, ()| {
            let result = lua.create_table()?;
            result.set("trackedIDs", lua.create_table()?)?;
            Ok(result)
        })?,
    )?;
    t.set(
        "GetPerksActivityInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetPerksActivityChatLink",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "RemoveTrackedPerksActivity",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    lua.globals().set("C_PerksActivities", t)?;
    Ok(())
}

/// Missing C_* namespaces and globals referenced during startup events.
fn register_missing_namespaces(lua: &Lua) -> Result<()> {
    register_social_namespaces(lua)?;
    register_system_namespaces(lua)?;
    Ok(())
}

/// Social, friends, and matchmaking namespace stubs.
fn register_social_namespaces(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_social_status_namespaces(lua, &g)?;
    register_social_queue_namespace(lua, &g)?;
    Ok(())
}

fn register_social_status_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let spectating = lua.create_table()?;
    spectating.set("IsSpectating", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_SpectatingUI", spectating)?;

    let social = lua.create_table()?;
    social.set("IsMuted", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsSilenced", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsSquelched", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsChatDisabled", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("CanReceiveChat", lua.create_function(|_, ()| Ok(true))?)?;
    g.set("C_SocialRestrictions", social)?;

    let lobby = lua.create_table()?;
    lobby.set("IsParticipating", lua.create_function(|_, ()| Ok(false))?)?;
    lobby.set("IsInQueue", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_LobbyMatchmakerInfo", lobby)?;

    let mentorship = lua.create_table()?;
    mentorship.set(
        "GetMentorshipStatus",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    mentorship.set(
        "IsActivePlayerConsideredNewcomer",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set("C_PlayerMentorship", mentorship)?;

    let recent_allies = lua.create_table()?;
    recent_allies.set("IsSystemEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_RecentAllies", recent_allies)?;
    Ok(())
}

fn register_social_queue_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let social_queue = lua.create_table()?;
    social_queue.set(
        "GetAllGroups",
        lua.create_function(|lua, _local_only: Option<bool>| lua.create_table())?,
    )?;
    social_queue.set(
        "GetConfig",
        lua.create_function(|lua, ()| {
            let config = lua.create_table()?;
            config.set("toastDuration", 60.0f64)?;
            config.set("enableToasts", false)?;
            Ok(config)
        })?,
    )?;
    g.set("C_SocialQueue", social_queue)?;
    Ok(())
}

/// System, service, and utility namespace stubs.
fn register_system_namespaces(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_cinematic_login_nameplates(lua, &g)?;
    register_character_services_namespace(lua, &g)?;
    register_social_contract_glue_namespace(lua, &g)?;
    super::c_stubs_api_store::register_c_account_store(lua)?;
    register_c_video_options(lua)?;
    Ok(())
}

fn register_social_contract_glue_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let social_contract = lua.create_table()?;
    social_contract.set(
        "GetShouldShowSocialContract",
        lua.create_function(|lua, ()| {
            let fire_event: mlua::Function = lua.globals().get("FireEvent")?;
            fire_event.call::<()>(("SOCIAL_CONTRACT_STATUS_UPDATE", false))?;
            Ok(false)
        })?,
    )?;
    social_contract.set(
        "TryUpdateSocialContract",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set("C_SocialContractGlue", social_contract)?;
    Ok(())
}

fn register_cinematic_login_nameplates(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let cinematic = lua.create_table()?;
    cinematic.set(
        "GetUICinematicList",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set("C_CinematicList", cinematic)?;

    let login = lua.create_table()?;
    login.set(
        "GetState",
        lua.create_function(|lua, ()| {
            let Some(state) =
                lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>>()
            else {
                return Ok((1i32, false, 0i32, false));
            };
            Ok(state.borrow().screen_kind.login_state())
        })?,
    )?;
    login.set(
        "IsLoginReady",
        lua.create_function(|lua, ()| {
            let Some(state) =
                lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>>()
            else {
                return Ok(false);
            };
            Ok(matches!(
                state.borrow().screen_kind,
                crate::screen::ScreenKind::Login
            ))
        })?,
    )?;
    login.set("IsLauncherLogin", lua.create_function(|_, ()| Ok(false))?)?;
    login.set(
        "WasEverLauncherLogin",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    login.set(
        "AttemptedLauncherLogin",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    login.set(
        "IsReconnectLoginPossible",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    login.set("ReconnectLogin", lua.create_function(|_, ()| Ok(()))?)?;
    login.set("ClearReconnectLogin", lua.create_function(|_, ()| Ok(()))?)?;
    login.set(
        "SelectGameAccount",
        lua.create_function(|_, _: Value| Ok(()))?,
    )?;
    login.set(
        "RequestAutoRealmJoin",
        lua.create_function(|_, _realm_addr: Value| Ok(()))?,
    )?;
    login.set("Login", lua.create_function(|_, _: MultiValue| Ok(()))?)?;
    login.set("ClearLastError", lua.create_function(|_, ()| Ok(()))?)?;
    login.set("GetLastError", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    g.set("C_Login", login)?;

    g.set(
        "DefaultCompactNamePlateEnemyFrameOptions",
        lua.create_table()?,
    )?;
    g.set(
        "DefaultCompactNamePlateFriendlyFrameOptions",
        lua.create_table()?,
    )?;
    g.set(
        "DefaultCompactNamePlatePlayerFrameSetUpOptions",
        lua.create_table()?,
    )?;

    // Register C_FunctionContainers with proper LuaFunctionContainer UserData
    super::function_container::register_c_function_containers(lua)?;

    let spell_overlay = lua.create_table()?;
    spell_overlay.set(
        "IsSpellOverlayed",
        lua.create_function(|_, _spell_id: i32| Ok(false))?,
    )?;
    g.set("C_SpellActivationOverlay", spell_overlay)?;
    Ok(())
}

fn register_character_services_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let char_svc = lua.create_table()?;
    char_svc.set("ApplyLevelUp", lua.create_function(|_, ()| Ok(()))?)?;
    char_svc.set(
        "AssignUpgradeDistribution",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    char_svc.set("HasQueuedUpgrade", lua.create_function(|_, ()| Ok(false))?)?;
    char_svc.set(
        "DoesGUIDHavePendingFactionChange",
        lua.create_function(|_, _guid: Value| Ok(false))?,
    )?;
    char_svc.set(
        "GetActiveClassTrialBoostType",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    char_svc.set(
        "GetAutomaticBoost",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    char_svc.set(
        "GetAutomaticBoostCharacter",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    char_svc.set(
        "GetQueuedUpgradeGUID",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    char_svc.set("ClearQueuedUpgrade", lua.create_function(|_, ()| Ok(()))?)?;
    char_svc.set(
        "HasRequiredBoostForUnrevoke",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    char_svc.set(
        "HasRequiredBoostForClassTrial",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    char_svc.set(
        "IsTrialBoostEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    char_svc.set(
        "GetCharacterServiceDisplayInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    char_svc.set(
        "GetCharacterServiceDisplayDataByVASType",
        lua.create_function(|lua, _vas_type: Value| {
            let popup_info = lua.create_table()?;
            popup_info.set("textureKit", "")?;
            let t = lua.create_table()?;
            t.set("popupInfo", popup_info)?;
            t.set("flowTitle", "")?;
            Ok(Value::Table(t))
        })?,
    )?;
    char_svc.set(
        "GetFactionGroupByIndex",
        lua.create_function(|_, _character_index: i32| Ok("Alliance"))?,
    )?;
    char_svc.set(
        "GetVASDistributions",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    char_svc.set(
        "IsLiveRegionCharacterListEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    char_svc.set(
        "IsLiveRegionCharacterCopyEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    char_svc.set(
        "IsLiveRegionAccountCopyEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    char_svc.set(
        "IsLiveRegionKeyBindingsCopyEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    char_svc.set(
        "RequestManualUnrevoke",
        lua.create_function(|_, _guid: Value| Ok(()))?,
    )?;
    char_svc.set(
        "SetAutomaticBoost",
        lua.create_function(|_, _boost_type: Value| Ok(()))?,
    )?;
    char_svc.set(
        "SetAutomaticBoostCharacter",
        lua.create_function(|_, _guid: Value| Ok(()))?,
    )?;
    char_svc.set(
        "TrialBoostCharacter",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    char_svc.set(
        "GetLiveRegionCharacterCopySourceRegions",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    char_svc.set(
        "GetCharacterServiceDisplayData",
        lua.create_function(|lua, _boost_type: Value| {
            let popup_info = lua.create_table()?;
            popup_info.set("textureKit", "")?;
            let t = lua.create_table()?;
            t.set("popupInfo", popup_info)?;
            t.set("flowTitle", "")?;
            Ok(Value::Table(t))
        })?,
    )?;
    g.set("C_CharacterServices", char_svc)?;
    Ok(())
}

fn register_realm_list_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let realm_list = lua.create_table()?;
    realm_list.set(
        "RequestChangeRealmList",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set("C_RealmList", realm_list)?;
    Ok(())
}

fn register_character_creation_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let character_creation = lua.create_table()?;
    character_creation.set(
        "ClearCharacterTemplate",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    character_creation.set(
        "CreateCharacter",
        lua.create_function(|lua, (name, _use_npe, _faction): (String, Value, Value)| {
            apply_selected_character_to_player_state(lua, Some(&name));
            let guid = Value::String(lua.create_string(GLUE_CHARACTERS[0].guid)?);
            let args = [Value::Boolean(true), Value::Nil, guid];
            glue_fire_character_create_event(lua, "CHARACTER_CREATION_RESULT", &args)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "CreateAuxModel",
        lua.create_function(|_, _: MultiValue| Ok(1i32))?,
    )?;
    character_creation.set(
        "CustomizeExistingCharacter",
        lua.create_function(|_, _character_id: i32| Ok(()))?,
    )?;
    character_creation.set(
        "DestroyAuxModel",
        lua.create_function(|_, _model_index: i32| Ok(()))?,
    )?;
    character_creation.set(
        "EquipWeaponsOnAuxModel",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    character_creation.set(
        "GetCharacterTemplateInfo",
        lua.create_function(|_, _character_index: i32| Ok((String::new(), String::new())))?,
    )?;
    character_creation.set(
        "GetAlliedRaceAchievementRequirements",
        lua.create_function(|lua, _race_id: i32| lua.create_table())?,
    )?;
    character_creation.set(
        "GetCharacterCreateType",
        lua.create_function(|lua, ()| Ok(get_glue_character_create_type(lua)?))?,
    )?;
    character_creation.set(
        "GetNumCharacterTemplates",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    character_creation.set(
        "GetAvailableClasses",
        lua.create_function(|lua, ()| {
            let classes = lua.create_table()?;
            for (index, class_info) in GLUE_CLASSES.iter().enumerate() {
                classes.set(index + 1, glue_class_data(lua, class_info.class_id)?)?;
            }
            Ok(classes)
        })?,
    )?;
    character_creation.set(
        "GetAvailableCustomizations",
        lua.create_function(|lua, ()| glue_available_customizations(lua))?,
    )?;
    character_creation.set(
        "GetAvailableRaces",
        lua.create_function(|lua, ()| {
            let races = lua.create_table()?;
            for (index, race) in GLUE_RACES.iter().enumerate() {
                races.set(index + 1, glue_race_data(lua, race.race_id)?)?;
            }
            Ok(races)
        })?,
    )?;
    character_creation.set(
        "GetBlockedRaces",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    character_creation.set(
        "GetCharacterCreateFacing",
        lua.create_function(|lua, ()| {
            Ok(get_glue_f32(lua, GLUE_CHARACTER_CREATE_FACING_KEY, 0.0)?)
        })?,
    )?;
    character_creation.set(
        "GetRaceDataByID",
        lua.create_function(|lua, race_id: i32| {
            if find_glue_race(race_id).is_some() {
                Ok(Value::Table(glue_race_data(lua, race_id)?))
            } else {
                Ok(Value::Nil)
            }
        })?,
    )?;
    character_creation.set(
        "GetRaceIDFromName",
        lua.create_function(|_, race_name: String| {
            Ok(find_glue_race_by_name(&race_name)
                .map(|race| race.race_id)
                .unwrap_or(default_race_id()))
        })?,
    )?;
    character_creation.set(
        "GetCreateBackgroundModel",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    character_creation.set(
        "GetCurrentCameraZoom",
        lua.create_function(|lua, ()| {
            Ok(get_glue_i32(lua, GLUE_CHARACTER_CREATE_CAMERA_ZOOM_KEY, 0)?)
        })?,
    )?;
    character_creation.set(
        "GetDefaultCharacterCreateFacing",
        lua.create_function(|_, ()| Ok(0.0f32))?,
    )?;
    character_creation.set(
        "GetFactionForRace",
        lua.create_function(|_, race_id: i32| {
            Ok(find_glue_race(race_id)
                .map(|race| race.faction_internal_name.to_string())
                .unwrap_or_else(|| "Alliance".to_string()))
        })?,
    )?;
    character_creation.set(
        "GetClassAchievementRequirements",
        lua.create_function(|lua, _: MultiValue| lua.create_table())?,
    )?;
    character_creation.set(
        "GetModelAlpha",
        lua.create_function(|lua, ()| {
            Ok(get_glue_f32(
                lua,
                GLUE_CHARACTER_CREATE_MODEL_ALPHA_KEY,
                1.0,
            )?)
        })?,
    )?;
    character_creation.set(
        "GetNameForRace",
        lua.create_function(|_, race_id: i32| {
            Ok(find_glue_race(race_id)
                .map(|race| race.name.to_string())
                .unwrap_or_else(|| GLUE_RACES[0].name.to_string()))
        })?,
    )?;
    character_creation.set(
        "GetSelectedClass",
        lua.create_function(|lua, ()| {
            Ok(Value::Table(glue_class_data(
                lua,
                get_glue_selected_class_id(lua)?,
            )?))
        })?,
    )?;
    character_creation.set(
        "GetSelectedRace",
        lua.create_function(|lua, ()| Ok(get_glue_selected_race_id(lua)?))?,
    )?;
    character_creation.set(
        "GetSelectedSex",
        lua.create_function(|lua, ()| Ok(get_glue_selected_sex_id(lua)?))?,
    )?;
    character_creation.set(
        "GetStartingZoneChoices",
        lua.create_function(|lua, ()| {
            let first = lua.create_table()?;
            first.set("zoneName", "Exile's Reach")?;
            first.set("zoneImageAtlas", "charactercreate-startingzone-exilesreach")?;
            first.set("isNPE", true)?;
            let second = lua.create_table()?;
            second.set("zoneName", "Starting Zone")?;
            second.set("zoneImageAtlas", "charactercreate-startingzone-classic")?;
            second.set("isNPE", false)?;
            Ok((Value::Table(first), Value::Table(second)))
        })?,
    )?;
    character_creation.set(
        "GetTrialBoostStartingLevel",
        lua.create_function(|_, ()| Ok(70i32))?,
    )?;
    character_creation.set(
        "GetValidRacesForClass",
        lua.create_function(|lua, class_id: i32| {
            let races = lua.create_table()?;
            if find_glue_class(class_id).is_none() {
                return Ok(races);
            }
            for (index, race) in GLUE_RACES.iter().enumerate() {
                races.set(index + 1, glue_race_data(lua, race.race_id)?)?;
            }
            Ok(races)
        })?,
    )?;
    character_creation.set(
        "IsCharacterNameValid",
        lua.create_function(|lua, name: String| {
            let trimmed = name.trim();
            let valid = !trimmed.is_empty()
                && trimmed.len() <= 12
                && trimmed.chars().all(|ch| ch.is_ascii_alphabetic());
            if valid {
                Ok((true, Value::Nil))
            } else {
                Ok((
                    false,
                    Value::String(lua.create_string(if trimmed.is_empty() {
                        "ERR_NAME_TOO_SHORT"
                    } else if trimmed.len() > 12 {
                        "ERR_NAME_TOO_LONG2"
                    } else {
                        "ERR_NAME_TOO_SHORT"
                    })?),
                ))
            }
        })?,
    )?;
    character_creation.set(
        "IsNewPlayerRestricted",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    character_creation.set(
        "IsForcingCharacterTemplate",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    character_creation.set(
        "IsRaceClassValid",
        lua.create_function(|_, (race_id, class_id): (i32, i32)| {
            Ok(find_glue_race(race_id).is_some() && find_glue_class(class_id).is_some())
        })?,
    )?;
    character_creation.set(
        "IsTimerunningEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    character_creation.set(
        "IsTrialAccountRestricted",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    character_creation.set(
        "IsUsingCharacterTemplate",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    character_creation.set(
        "IsViewingAlteredForm",
        lua.create_function(|lua, ()| {
            Ok(lua
                .globals()
                .raw_get::<Option<bool>>(GLUE_CHARACTER_CREATE_VIEWING_ALTERED_FORM_KEY)?
                .unwrap_or(false))
        })?,
    )?;
    character_creation.set(
        "GenerateRandomName",
        lua.create_function(|lua, ()| glue_random_name(lua))?,
    )?;
    character_creation.set("OnPlayerInteraction", lua.create_function(|_, ()| Ok(()))?)?;
    character_creation.set(
        "PlayClassIdleAnimationOnCharacter",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    character_creation.set(
        "PlayCustomizationIdleAnimationOnCharacter",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    character_creation.set(
        "PlaySpellVisualKitOnAuxModel",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    character_creation.set(
        "PlaySpellVisualKitOnCharacter",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    character_creation.set(
        "PlaySpellVisualKitOnGround",
        lua.create_function(|_, _: MultiValue| Ok(0i32))?,
    )?;
    character_creation.set(
        "PreviewCustomizationChoice",
        lua.create_function(|lua, (option_id, choice_id): (i32, i32)| {
            let preview = customization_choices_table(
                lua,
                GLUE_CHARACTER_CREATE_CUSTOMIZATION_PREVIEW_CHOICES_KEY,
            )?;
            preview.set(option_id, choice_id)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "ClearPreviewChoices",
        lua.create_function(|lua, _: MultiValue| {
            lua.globals().raw_set(
                GLUE_CHARACTER_CREATE_CUSTOMIZATION_PREVIEW_CHOICES_KEY,
                lua.create_table()?,
            )?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "MarkCustomizationChoiceAsSeen",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    character_creation.set(
        "MarkCustomizationOptionAsSeen",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    character_creation.set(
        "RandomizeCharCustomization",
        lua.create_function(|lua, ()| {
            let choices =
                customization_choices_table(lua, GLUE_CHARACTER_CREATE_CUSTOMIZATION_CHOICES_KEY)?;
            let race_id = get_glue_selected_race_id(lua)?;
            let sex_id = get_glue_selected_sex_id(lua)?;
            for category in GLUE_CUSTOMIZATION_CATEGORIES {
                for option in category.options {
                    let choice_id = default_choice_id_for_option(option, race_id + 1, sex_id + 1);
                    choices.set(option.id, choice_id)?;
                }
            }
            lua.globals().raw_set(
                GLUE_CHARACTER_CREATE_CUSTOMIZATION_PREVIEW_CHOICES_KEY,
                lua.create_table()?,
            )?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "RequestCheckNameAvailability",
        lua.create_function(|lua, name: String| {
            let args = [
                Value::Boolean(true),
                Value::String(lua.create_string(&name)?),
                Value::Nil,
            ];
            glue_fire_character_create_event(lua, "CHECK_CHARACTER_NAME_AVAILABILITY_RESULT", &args)
        })?,
    )?;
    character_creation.set(
        "RequestRandomName",
        lua.create_function(|lua, ()| {
            let name = glue_random_name(lua)?;
            let args = [
                Value::Boolean(true),
                Value::String(lua.create_string(&name)?),
            ];
            glue_fire_character_create_event(lua, "RANDOM_CHARACTER_NAME_RESULT", &args)
        })?,
    )?;
    character_creation.set(
        "ResetCharCustomize",
        lua.create_function(|lua, ()| {
            reset_glue_character_create_state(lua)?;
            Ok(())
        })?,
    )?;
    character_creation.set("SaveSeenChoices", lua.create_function(|_, ()| Ok(()))?)?;
    character_creation.set(
        "SetBlurEnabled",
        lua.create_function(|lua, enabled: bool| {
            lua.globals()
                .raw_set(GLUE_CHARACTER_CREATE_BLUR_ENABLED_KEY, enabled)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetCameraZoomLevel",
        lua.create_function(
            |lua, (zoom_level, _keep_custom_zoom): (i32, Option<bool>)| {
                set_glue_i32(
                    lua,
                    GLUE_CHARACTER_CREATE_CAMERA_ZOOM_KEY,
                    zoom_level.clamp(0, 100),
                )?;
                Ok(())
            },
        )?,
    )?;
    character_creation.set(
        "SetCharCustomizeBackground",
        lua.create_function(|_, _background_id: Value| Ok(()))?,
    )?;
    character_creation.set(
        "SetCharCustomizeFrame",
        lua.create_function(|_, _frame_name: String| Ok(()))?,
    )?;
    character_creation.set(
        "SetCharacterCreateFacing",
        lua.create_function(|lua, facing: f32| {
            set_glue_f32(lua, GLUE_CHARACTER_CREATE_FACING_KEY, facing)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetCharacterTemplate",
        lua.create_function(|_, _character_index: i32| Ok(()))?,
    )?;
    character_creation.set(
        "SetCharacterCreateType",
        lua.create_function(|lua, value: Value| {
            set_glue_character_create_type(lua, &value)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetCustomizationChoice",
        lua.create_function(|lua, (option_id, choice_id): (i32, i32)| {
            let choices =
                customization_choices_table(lua, GLUE_CHARACTER_CREATE_CUSTOMIZATION_CHOICES_KEY)?;
            choices.set(option_id, choice_id)?;
            let preview = customization_choices_table(
                lua,
                GLUE_CHARACTER_CREATE_CUSTOMIZATION_PREVIEW_CHOICES_KEY,
            )?;
            preview.raw_set(option_id, Value::Nil)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetInCharacterCreate",
        lua.create_function(|_, _in_character_create: bool| Ok(()))?,
    )?;
    character_creation.set(
        "SetModelAlpha",
        lua.create_function(|lua, alpha: f32| {
            set_glue_f32(lua, GLUE_CHARACTER_CREATE_MODEL_ALPHA_KEY, alpha)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetModelDressState",
        lua.create_function(|lua, dressed: bool| {
            lua.globals()
                .raw_set(GLUE_CHARACTER_CREATE_MODEL_DRESSED_KEY, dressed)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetPaidService",
        lua.create_function(|_, _has_paid_service: bool| Ok(()))?,
    )?;
    character_creation.set(
        "SetPlayerModelHiddenState",
        lua.create_function(|lua, hidden: bool| {
            lua.globals()
                .raw_set(GLUE_CHARACTER_CREATE_MODEL_HIDDEN_KEY, hidden)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetSelectedClass",
        lua.create_function(|lua, class_id: i32| {
            let class_id = find_glue_class(class_id)
                .map(|class_info| class_info.class_id)
                .unwrap_or_else(default_class_id);
            set_glue_i32(lua, GLUE_CHARACTER_CREATE_CLASS_ID_KEY, class_id)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetSelectedPreviewGearType",
        lua.create_function(|lua, gear_type: Value| {
            lua.globals()
                .raw_set(GLUE_CHARACTER_CREATE_SELECTED_PREVIEW_GEAR_KEY, gear_type)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetSelectedRace",
        lua.create_function(|lua, race_id: i32| {
            let race_id = find_glue_race(race_id)
                .map(|race| race.race_id)
                .unwrap_or_else(default_race_id);
            set_glue_i32(lua, GLUE_CHARACTER_CREATE_RACE_ID_KEY, race_id)?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetSelectedSex",
        lua.create_function(|lua, sex_id: i32| {
            set_glue_i32(lua, GLUE_CHARACTER_CREATE_SEX_ID_KEY, sex_id.clamp(0, 1))?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "SetTimerunningSeasonID",
        lua.create_function(|_, _: Value| Ok(()))?,
    )?;
    character_creation.set(
        "SetAuxModelHiddenState",
        lua.create_function(|_, _: MultiValue| Ok(()))?,
    )?;
    character_creation.set(
        "SetViewingAlteredForm",
        lua.create_function(|lua, viewing_altered_form: bool| {
            lua.globals().raw_set(
                GLUE_CHARACTER_CREATE_VIEWING_ALTERED_FORM_KEY,
                viewing_altered_form,
            )?;
            Ok(())
        })?,
    )?;
    character_creation.set(
        "StopAllSpellVisualKitsOnCharacter",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    character_creation.set(
        "StopSpellVisualKit",
        lua.create_function(|_, _: Value| Ok(()))?,
    )?;
    character_creation.set("UseBeginnerMode", lua.create_function(|_, ()| Ok(false))?)?;
    character_creation.set(
        "ZoomCamera",
        lua.create_function(
            |lua, (zoom_amount, _zoom_time, _force): (i32, Option<f32>, Option<bool>)| {
                let current = get_glue_i32(lua, GLUE_CHARACTER_CREATE_CAMERA_ZOOM_KEY, 0)?;
                set_glue_i32(
                    lua,
                    GLUE_CHARACTER_CREATE_CAMERA_ZOOM_KEY,
                    (current + zoom_amount).clamp(0, 100),
                )?;
                Ok(())
            },
        )?,
    )?;
    g.set("C_CharacterCreation", character_creation)?;
    Ok(())
}

fn register_shared_character_services_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let shared_character_services = lua.create_table()?;
    shared_character_services.set(
        "GetUpgradeDistributions",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set("C_SharedCharacterServices", shared_character_services)?;
    Ok(())
}

fn register_configuration_warnings_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let configuration_warnings = lua.create_table()?;
    configuration_warnings.set(
        "GetConfigurationWarnings",
        lua.create_function(|lua, _include_seen_warnings: Option<bool>| lua.create_table())?,
    )?;
    configuration_warnings.set(
        "GetConfigurationWarningString",
        lua.create_function(|_, _warning: Value| Ok(Value::Nil))?,
    )?;
    g.set("C_ConfigurationWarnings", configuration_warnings)?;
    Ok(())
}

fn register_store_glue_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let store_glue = lua.create_table()?;
    store_glue.set(
        "GetDisconnectOnLogout",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    store_glue.set(
        "GetVASProductReady",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    store_glue.set(
        "GetVASPurchaseStateInfo",
        lua.create_function(|_, _guid: Value| Ok((0i32, Value::Nil, Value::Nil)))?,
    )?;
    store_glue.set(
        "RequestCharacterQueueTime",
        lua.create_function(|_, _guid: Value| Ok(()))?,
    )?;
    store_glue.set(
        "UpdateVASPurchaseStates",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set("C_StoreGlue", store_glue)?;
    Ok(())
}

/// C_VideoOptions — screen resolution and graphics queries.
fn register_c_video_options(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    let video = lua.create_table()?;
    video.set(
        "GetDefaultGameWindowSize",
        lua.create_function(|lua, _monitor: i32| {
            let t = lua.create_table()?;
            t.set("x", 1920)?;
            t.set("y", 1080)?;
            Ok(t)
        })?,
    )?;
    video.set(
        "GetCurrentGameWindowSize",
        lua.create_function(|lua, _args: MultiValue| {
            let t = lua.create_table()?;
            t.set("x", 1920)?;
            t.set("y", 1080)?;
            Ok(t)
        })?,
    )?;
    video.set(
        "GetGameWindowSizes",
        lua.create_function(|lua, _args: MultiValue| lua.create_table())?,
    )?;
    video.set(
        "GetGxAdapterInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    video.set(
        "IsSpellVisualDensitySystemSupported",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    video.set(
        "SetGameWindowSize",
        lua.create_function(|_, (_x, _y): (i32, i32)| Ok(()))?,
    )?;
    g.set("C_VideoOptions", video)?;
    Ok(())
}

/// Game-state global stubs for functions referenced during startup events.
fn register_game_state_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_character_creation_namespace(lua, &g)?;
    register_shared_character_services_namespace(lua, &g)?;
    register_configuration_warnings_namespace(lua, &g)?;
    register_store_glue_namespace(lua, &g)?;
    register_realm_list_namespace(lua, &g)?;
    g.set("IsTargetLoose", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("IsPartyLFG", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("IsPartyWorldPVP", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "PlayerGetTimerunningSeasonID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "UnitDistanceSquared",
        lua.create_function(|_, _unit: Value| Ok((0.0f64, true)))?,
    )?;
    g.set(
        "UnitInOtherParty",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "UnitHasIncomingResurrection",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetLFGRoles",
        lua.create_function(|_, ()| Ok((false, false, false)))?,
    )?;
    g.set(
        "GetLFGReadyCheckUpdate",
        lua.create_function(|_, ()| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "CanPartyLFGBackfill",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumArenaOpponentSpecs",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetArenaOpponentSpec",
        lua.create_function(|_, _index: Value| Ok((0i32, 0i32)))?,
    )?;
    g.set(
        "UnitTreatAsPlayerForDisplay",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetLFGDeserterExpiration",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "UnitHasLFGDeserter",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetWorldPVPQueueStatus",
        lua.create_function(|_, _index: Value| Ok(("none", 0i32, 0i32, 0i32)))?,
    )?;
    g.set(
        "CanHearthAndResurrectFromArea",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetChannelList",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "CanBeRaidTarget",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetRaidTargetIndex",
        lua.create_function(|_, _unit: Value| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// C_IncomingSummon namespace stubs.
fn register_c_incoming_summon(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "HasIncomingSummon",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    t.set(
        "IncomingSummonStatus",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    lua.globals().set("C_IncomingSummon", t)?;
    Ok(())
}
