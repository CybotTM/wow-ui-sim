//! Static quest data seeded into the simulator.

pub struct Objective {
    pub text: &'static str,
    pub obj_type: &'static str,
    pub finished: bool,
}

pub enum QuestLogEntry {
    Header {
        title: &'static str,
    },
    Quest {
        quest_id: i32,
        title: &'static str,
        description: &'static str,
        objectives: &'static [Objective],
    },
}

pub struct WorldQuest {
    pub quest_id: i32,
    pub map_id: i32,
    pub x: f64,
    pub y: f64,
    pub title: &'static str,
    pub num_objectives: i32,
}

pub static QUEST_LOG: &[QuestLogEntry] = &[
    QuestLogEntry::Header {
        title: "Khaz Algar",
    },
    QuestLogEntry::Quest {
        quest_id: 80000,
        title: "The Lost Expedition",
        description: "An old journal found near the quarry entrance describes an Ironforge expedition that went missing decades ago. Scattered relics hint at their path deeper underground. Collect what remains and piece together what happened.",
        objectives: &[
            Objective {
                text: "Ironforge Relics collected: 3/5",
                obj_type: "item",
                finished: false,
            },
            Objective {
                text: "Explore the Old Quarry",
                obj_type: "event",
                finished: false,
            },
        ],
    },
    QuestLogEntry::Quest {
        quest_id: 80001,
        title: "Defending the Gates",
        description: "The Stormwind gate guards are under constant pressure from the gnoll raiders that have been sighted along the forest road. Lend your strength to the defense until reinforcements arrive from Goldshire.",
        objectives: &[Objective {
            text: "Stormwind Guards defended: 7/10",
            obj_type: "monster",
            finished: false,
        }],
    },
    QuestLogEntry::Quest {
        quest_id: 80002,
        title: "Supply Run",
        description: "The quartermaster at the forward camp is running low on provisions. Gather supplies from the nearby farmsteads and deliver them before nightfall.",
        objectives: &[
            Objective {
                text: "Supplies gathered: 5/5",
                obj_type: "item",
                finished: true,
            },
            Objective {
                text: "Deliver to Quartermaster",
                obj_type: "event",
                finished: false,
            },
        ],
    },
];

pub static WORLD_QUESTS: &[WorldQuest] = &[
    WorldQuest {
        quest_id: 90101,
        map_id: 2248,
        x: 0.45,
        y: 0.35,
        title: "Earthen Relic Recovery",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90102,
        map_id: 2248,
        x: 0.62,
        y: 0.58,
        title: "Arathi Signal Fires",
        num_objectives: 2,
    },
    WorldQuest {
        quest_id: 90103,
        map_id: 2215,
        x: 0.40,
        y: 0.50,
        title: "Crystal Shard Collection",
        num_objectives: 3,
    },
    WorldQuest {
        quest_id: 90104,
        map_id: 2214,
        x: 0.55,
        y: 0.45,
        title: "Kobold Tunnel Collapse",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90001,
        map_id: 2025,
        x: 0.52,
        y: 0.63,
        title: "Glittering Geodes",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90002,
        map_id: 2025,
        x: 0.38,
        y: 0.41,
        title: "Temporal Rift Collapse",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90003,
        map_id: 2024,
        x: 0.47,
        y: 0.55,
        title: "Frozen Tuskarr Supplies",
        num_objectives: 3,
    },
    WorldQuest {
        quest_id: 90004,
        map_id: 2024,
        x: 0.62,
        y: 0.32,
        title: "Brackenhide Gnolls",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90005,
        map_id: 2023,
        x: 0.71,
        y: 0.48,
        title: "Storm-Charged Hunt",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90006,
        map_id: 2023,
        x: 0.35,
        y: 0.62,
        title: "Centaur Caravan Defense",
        num_objectives: 2,
    },
    WorldQuest {
        quest_id: 90007,
        map_id: 2022,
        x: 0.58,
        y: 0.70,
        title: "Lava Surge Containment",
        num_objectives: 1,
    },
    WorldQuest {
        quest_id: 90008,
        map_id: 2022,
        x: 0.44,
        y: 0.35,
        title: "Djaradin Weapon Cache",
        num_objectives: 2,
    },
];

pub const SEEDED_WORLD_QUEST_TIME_LEFT_MINUTES: i32 = 120;
