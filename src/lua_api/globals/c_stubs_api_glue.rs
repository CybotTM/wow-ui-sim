use mlua::{Lua, MultiValue, Result, Table, Value};

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
        lore_description: "Inventive tinkerers fueled by curiosity and wit.",
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
        lore_description: "Ancient hunters and mystics who stand with the Horde.",
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
        lore_description: "Cunning dealmakers with explosive ideas.",
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
        lore_description: "Arcane devotees seeking to preserve their radiant legacy.",
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
        lore_description: "Exiled heroes empowered by the Light and the naaru.",
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
        lore_description: "Cursed Gilneans balancing humanity and savagery.",
        is_allied_race: false,
        is_neutral_race: false,
        has_heritage_armor: true,
        alternate_form: Some(GlueAlternateFormDef {
            name: "Human",
            create_screen_icon_atlas: "raceicon128-human-male",
        }),
    },
];

const GLUE_CLASSES: &[GlueClassDef] = &[
    GlueClassDef {
        class_id: 1,
        name: "Warrior",
        file_name: "WARRIOR",
        description: "Weapon masters who thrive on the front lines.",
        role_info: "Tanks or melee damage dealers.",
    },
    GlueClassDef {
        class_id: 2,
        name: "Paladin",
        file_name: "PALADIN",
        description: "Champions of the Light with holy magic and heavy armor.",
        role_info: "Tank, healer, or melee damage dealer.",
    },
    GlueClassDef {
        class_id: 3,
        name: "Hunter",
        file_name: "HUNTER",
        description: "Ranged survivalists bonded with loyal companions.",
        role_info: "Ranged or melee damage dealer.",
    },
    GlueClassDef {
        class_id: 4,
        name: "Rogue",
        file_name: "ROGUE",
        description: "Stealthy opportunists who strike from the shadows.",
        role_info: "Melee damage dealer.",
    },
    GlueClassDef {
        class_id: 5,
        name: "Priest",
        file_name: "PRIEST",
        description: "Faithful spellcasters wielding holy and shadow magic.",
        role_info: "Healer or ranged damage dealer.",
    },
    GlueClassDef {
        class_id: 6,
        name: "Death Knight",
        file_name: "DEATHKNIGHT",
        description: "Former heroes raised to command runeblades and undeath.",
        role_info: "Tank or melee damage dealer.",
    },
    GlueClassDef {
        class_id: 7,
        name: "Shaman",
        file_name: "SHAMAN",
        description: "Spiritual guides channeling the elements.",
        role_info: "Healer, melee, or ranged damage dealer.",
    },
    GlueClassDef {
        class_id: 8,
        name: "Mage",
        file_name: "MAGE",
        description: "Arcane scholars mastering fire, frost, and sorcery.",
        role_info: "Ranged damage dealer.",
    },
    GlueClassDef {
        class_id: 9,
        name: "Warlock",
        file_name: "WARLOCK",
        description: "Fel casters commanding curses and demons.",
        role_info: "Ranged damage dealer.",
    },
    GlueClassDef {
        class_id: 10,
        name: "Monk",
        file_name: "MONK",
        description: "Agile martial artists powered by chi.",
        role_info: "Tank, healer, or melee damage dealer.",
    },
    GlueClassDef {
        class_id: 11,
        name: "Druid",
        file_name: "DRUID",
        description: "Shapeshifters empowered by nature and the Emerald Dream.",
        role_info: "Tank, healer, melee, or ranged damage dealer.",
    },
    GlueClassDef {
        class_id: 12,
        name: "Demon Hunter",
        file_name: "DEMONHUNTER",
        description: "Illidari wielding fel power and spectral sight.",
        role_info: "Tank or melee damage dealer.",
    },
    GlueClassDef {
        class_id: 13,
        name: "Evoker",
        file_name: "EVOKER",
        description: "Dracthyr spellcasters empowered by dragonflights.",
        role_info: "Healer or ranged damage dealer.",
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

const SKIN_COLOR_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 1101,
        name: "Skin Tone 1",
    },
    GlueCustomizationChoiceDef {
        id: 1102,
        name: "Skin Tone 2",
    },
    GlueCustomizationChoiceDef {
        id: 1103,
        name: "Skin Tone 3",
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
];

const HAIR_COLOR_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 2101,
        name: "Brown",
    },
    GlueCustomizationChoiceDef {
        id: 2102,
        name: "Black",
    },
    GlueCustomizationChoiceDef {
        id: 2103,
        name: "Blonde",
    },
];

const FACIAL_HAIR_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 3001,
        name: "Style 1",
    },
    GlueCustomizationChoiceDef {
        id: 3002,
        name: "Style 2",
    },
    GlueCustomizationChoiceDef {
        id: 3003,
        name: "Style 3",
    },
];

const SCAR_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 3101,
        name: "None",
    },
    GlueCustomizationChoiceDef {
        id: 3102,
        name: "Scar 1",
    },
    GlueCustomizationChoiceDef {
        id: 3103,
        name: "Scar 2",
    },
];

const HORN_CHOICES: &[GlueCustomizationChoiceDef] = &[
    GlueCustomizationChoiceDef {
        id: 3201,
        name: "Horn Style 1",
    },
    GlueCustomizationChoiceDef {
        id: 3202,
        name: "Horn Style 2",
    },
    GlueCustomizationChoiceDef {
        id: 3203,
        name: "Horn Style 3",
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
        name: "Skin Color",
        option_type: 0,
        order_index: 2,
        choices: SKIN_COLOR_CHOICES,
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
    GLUE_CHARACTERS
        .iter()
        .find(|character| character.guid == guid)
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
    table.set("raceName", character.race_name)?;
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

pub fn initialize_globals(lua: &Lua) -> Result<()> {
    lua.globals()
        .raw_set(GLUE_CHARACTER_CREATE_TYPE_KEY, 0i32)?;
    reset_glue_character_create_state(lua)
}

pub fn character_count(lua: &Lua) -> i32 {
    glue_character_count(lua)
}

pub fn character_guid(lua: &Lua, index: i32) -> Option<String> {
    glue_character_guid(lua, index)
}

pub fn character_race_name(index: i32) -> Option<&'static str> {
    glue_character(index).map(|character| character.race_name)
}

pub fn basic_character_info(lua: &Lua, guid: &str) -> Result<Value> {
    glue_basic_character_info(lua, guid)
}

pub fn service_character_info(lua: &Lua, guid: &str) -> Result<Value> {
    glue_service_character_info(lua, guid)
}

pub fn selected_character(lua: &Lua) -> i32 {
    glue_selected_character(lua)
}

pub fn set_selected_character(lua: &Lua, index: i32) -> Result<()> {
    set_glue_selected_character(lua, index)
}

pub fn dispatch_select_character(lua: &Lua, character_id: i32) -> Result<()> {
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
}

pub fn refresh_character_list(lua: &Lua) -> Result<()> {
    set_glue_selected_character(lua, glue_selected_character(lua))?;
    let fire_event: mlua::Function = lua.globals().get("FireEvent")?;
    fire_event.call::<()>(("CHARACTER_LIST_UPDATE", glue_character_count(lua)))?;
    Ok(())
}

pub fn register_character_select_globals(lua: &Lua, g: &Table) -> Result<()> {
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
        "MapSceneCharacterHighlightStart",
        lua.create_function(|_, _guid: String| Ok(()))?,
    )?;
    g.set(
        "MapSceneCharacterHighlightEnd",
        lua.create_function(|_, _guid: String| Ok(()))?,
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
        lua.create_function(|lua, _include_empty_slots: Option<bool>| Ok(character_count(lua)))?,
    )?;
    g.set(
        "GetCharacterGUID",
        lua.create_function(|lua, index: i32| match character_guid(lua, index) {
            Some(guid) => Ok(Value::String(lua.create_string(&guid)?)),
            None => Ok(Value::Nil),
        })?,
    )?;
    g.set(
        "GetCharacterRace",
        lua.create_function(|_, index: i32| {
            if let Some(race_name) = character_race_name(index) {
                Ok((index, String::from(race_name)))
            } else {
                Ok((0i32, String::new()))
            }
        })?,
    )?;
    g.set(
        "GetBasicCharacterInfo",
        lua.create_function(|lua, guid: String| basic_character_info(lua, &guid))?,
    )?;
    g.set(
        "GetServiceCharacterInfo",
        lua.create_function(|lua, guid: String| service_character_info(lua, &guid))?,
    )?;
    g.set(
        "GetCharacterSelection",
        lua.create_function(|lua, ()| Ok(selected_character(lua)))?,
    )?;
    g.set(
        "SelectCharacter",
        lua.create_function(|lua, character_id: i32| dispatch_select_character(lua, character_id))?,
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
        lua.create_function(|lua, ()| refresh_character_list(lua))?,
    )?;
    Ok(())
}

pub fn register_login_state_globals(lua: &Lua, g: &Table) -> Result<()> {
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
    Ok(())
}

pub fn register_system_namespaces(lua: &Lua, g: &Table) -> Result<()> {
    register_cinematic_login_nameplates(lua, g)?;
    register_character_services_namespace(lua, g)?;
    register_social_contract_glue_namespace(lua, g)?;
    Ok(())
}

pub fn register_game_state_namespaces(lua: &Lua, g: &Table) -> Result<()> {
    register_character_creation_namespace(lua, g)?;
    register_realm_list_namespace(lua, g)?;
    Ok(())
}

fn register_social_contract_glue_namespace(lua: &Lua, g: &Table) -> Result<()> {
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

fn register_cinematic_login_nameplates(lua: &Lua, g: &Table) -> Result<()> {
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

    super::function_container::register_c_function_containers(lua)?;

    let spell_overlay = lua.create_table()?;
    spell_overlay.set(
        "IsSpellOverlayed",
        lua.create_function(|_, _spell_id: i32| Ok(false))?,
    )?;
    g.set("C_SpellActivationOverlay", spell_overlay)?;
    Ok(())
}

fn register_character_services_namespace(lua: &Lua, g: &Table) -> Result<()> {
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

fn register_realm_list_namespace(lua: &Lua, g: &Table) -> Result<()> {
    let realm_list = lua.create_table()?;
    realm_list.set(
        "RequestChangeRealmList",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set("C_RealmList", realm_list)?;
    Ok(())
}

fn register_character_creation_namespace(lua: &Lua, g: &Table) -> Result<()> {
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
