//! Profession and recipe data for the WoW UI simulator.
//!
//! Provides static profession info and Blacksmithing recipes for the crafting panel UI.

/// A profession known by the player.
pub struct ProfessionInfo {
    pub profession_id: i32,
    pub skill_line_id: i32,
    pub name: &'static str,
    pub parent_profession_name: &'static str,
    pub skill_level: i32,
    pub max_skill_level: i32,
    pub skill_modifier: i32,
    pub icon: i32,
}

/// A recipe category.
pub struct RecipeCategory {
    pub category_id: i32,
    pub name: &'static str,
    pub parent_category_id: i32,
    pub ui_order: i32,
}

/// A reagent requirement for a recipe.
pub struct ReagentSlot {
    pub item_id: u32,
    pub quantity: i32,
}

/// A crafting recipe.
pub struct RecipeEntry {
    pub recipe_id: i32,
    pub name: &'static str,
    pub learned: bool,
    pub craftable: bool,
    pub difficulty: i32,
    pub category_id: i32,
    pub item_level: i32,
    pub output_item_id: u32,
    pub output_quantity: i32,
    pub reagents: &'static [ReagentSlot],
}

pub static PROFESSIONS: &[ProfessionInfo] = &[
    ProfessionInfo {
        profession_id: 164,
        skill_line_id: 164,
        name: "Blacksmithing",
        parent_profession_name: "Blacksmithing",
        skill_level: 80,
        max_skill_level: 100,
        skill_modifier: 0,
        icon: 136241,
    },
    ProfessionInfo {
        profession_id: 186,
        skill_line_id: 186,
        name: "Mining",
        parent_profession_name: "Mining",
        skill_level: 90,
        max_skill_level: 100,
        skill_modifier: 0,
        icon: 136248,
    },
];

pub static RECIPE_CATEGORIES: &[RecipeCategory] = &[
    RecipeCategory { category_id: 1, name: "Armor", parent_category_id: 0, ui_order: 1 },
    RecipeCategory { category_id: 2, name: "Weapons", parent_category_id: 0, ui_order: 2 },
    RecipeCategory { category_id: 3, name: "Reagents", parent_category_id: 0, ui_order: 3 },
    RecipeCategory { category_id: 4, name: "Miscellaneous", parent_category_id: 0, ui_order: 4 },
];

static HELM_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210934, quantity: 12 },
    ReagentSlot { item_id: 210937, quantity: 2 },
];
static CHEST_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210934, quantity: 16 },
    ReagentSlot { item_id: 210937, quantity: 3 },
];
static GAUNTLETS_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210934, quantity: 8 },
    ReagentSlot { item_id: 210937, quantity: 2 },
];
static GREAVES_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210934, quantity: 14 },
    ReagentSlot { item_id: 210937, quantity: 2 },
];
static GREATSWORD_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210934, quantity: 20 },
    ReagentSlot { item_id: 210937, quantity: 4 },
    ReagentSlot { item_id: 210935, quantity: 2 },
];
static MACE_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210934, quantity: 10 },
    ReagentSlot { item_id: 210937, quantity: 2 },
];
static INGOT_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210930, quantity: 3 },
];
static AQIRITE_INGOT_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210931, quantity: 3 },
];
static SPIKE_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210934, quantity: 6 },
    ReagentSlot { item_id: 210937, quantity: 1 },
];
static CHAIN_REAGENTS: &[ReagentSlot] = &[
    ReagentSlot { item_id: 210934, quantity: 8 },
    ReagentSlot { item_id: 210937, quantity: 1 },
];

pub static BLACKSMITHING_RECIPES: &[RecipeEntry] = &[
    RecipeEntry {
        recipe_id: 100001,
        name: "Khaz Algar Helm",
        learned: true,
        craftable: true,
        difficulty: 3,
        category_id: 1,
        item_level: 590,
        output_item_id: 221096,
        output_quantity: 1,
        reagents: HELM_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100002,
        name: "Khaz Algar Breastplate",
        learned: true,
        craftable: true,
        difficulty: 3,
        category_id: 1,
        item_level: 590,
        output_item_id: 221091,
        output_quantity: 1,
        reagents: CHEST_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100003,
        name: "Khaz Algar Gauntlets",
        learned: true,
        craftable: true,
        difficulty: 2,
        category_id: 1,
        item_level: 580,
        output_item_id: 221092,
        output_quantity: 1,
        reagents: GAUNTLETS_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100004,
        name: "Khaz Algar Greaves",
        learned: true,
        craftable: true,
        difficulty: 3,
        category_id: 1,
        item_level: 590,
        output_item_id: 221095,
        output_quantity: 1,
        reagents: GREAVES_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100005,
        name: "Greatsword of Radiant Dawn",
        learned: true,
        craftable: true,
        difficulty: 4,
        category_id: 2,
        item_level: 610,
        output_item_id: 225583,
        output_quantity: 1,
        reagents: GREATSWORD_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100006,
        name: "Khaz Algar Mace",
        learned: true,
        craftable: true,
        difficulty: 2,
        category_id: 2,
        item_level: 580,
        output_item_id: 0,
        output_quantity: 1,
        reagents: MACE_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100007,
        name: "Khaz Algar Ingot",
        learned: true,
        craftable: true,
        difficulty: 1,
        category_id: 3,
        item_level: 1,
        output_item_id: 210934,
        output_quantity: 1,
        reagents: INGOT_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100008,
        name: "Aqirite Ingot",
        learned: true,
        craftable: true,
        difficulty: 2,
        category_id: 3,
        item_level: 1,
        output_item_id: 210935,
        output_quantity: 1,
        reagents: AQIRITE_INGOT_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100009,
        name: "Khaz Algar Shield Spike",
        learned: true,
        craftable: true,
        difficulty: 2,
        category_id: 4,
        item_level: 1,
        output_item_id: 0,
        output_quantity: 1,
        reagents: SPIKE_REAGENTS,
    },
    RecipeEntry {
        recipe_id: 100010,
        name: "Khaz Algar Weapon Chain",
        learned: true,
        craftable: true,
        difficulty: 2,
        category_id: 4,
        item_level: 1,
        output_item_id: 0,
        output_quantity: 1,
        reagents: CHAIN_REAGENTS,
    },
];

pub fn get_profession(profession_id: i32) -> Option<&'static ProfessionInfo> {
    PROFESSIONS.iter().find(|p| p.profession_id == profession_id)
}

pub fn get_profession_by_index(index: usize) -> Option<&'static ProfessionInfo> {
    PROFESSIONS.get(index)
}

pub fn get_recipe(recipe_id: i32) -> Option<&'static RecipeEntry> {
    BLACKSMITHING_RECIPES.iter().find(|r| r.recipe_id == recipe_id)
}

pub fn get_all_recipe_ids() -> Vec<i32> {
    BLACKSMITHING_RECIPES.iter().map(|r| r.recipe_id).collect()
}

pub fn get_filtered_recipe_ids() -> Vec<i32> {
    BLACKSMITHING_RECIPES
        .iter()
        .filter(|r| r.learned)
        .map(|r| r.recipe_id)
        .collect()
}

pub fn get_category(category_id: i32) -> Option<&'static RecipeCategory> {
    RECIPE_CATEGORIES.iter().find(|c| c.category_id == category_id)
}
