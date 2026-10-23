//! Original cached pet-battle display consumer; populated identity fixtures only.

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::PetBattlePet;

const MOUSE_SPECIES: i32 = 700_001;
const BEETLE_SPECIES: i32 = 700_002;

struct DisplayIdentity<'a> {
    owner: i32,
    slot: i32,
    name: &'a str,
    species: &'a str,
}

fn load_original_pet_battle_consumer() -> Option<WowLuaEnv> {
    match ACTIVE {
        ClientProfile::Retail
        | ClientProfile::Ptr
        | ClientProfile::WowForever
        | ClientProfile::Mists => {}
        ClientProfile::Wrath | ClientProfile::Era | ClientProfile::Anniversary => {
            eprintln!("CF-2 pet-battle consumer is not applicable to {ACTIVE:?}");
            return None;
        }
    }
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui);
    seed_pet_identities(&env);
    let loaded = crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env(
        &env,
        &ui,
        &["Blizzard_PetBattleUI"],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_PetBattleUI"));
    eprintln!("CF-2 original consumer cache: {}", ui.display());
    create_identity_display_fixture(&env);
    Some(env)
}

fn seed_pet_identities(env: &WowLuaEnv) {
    let mut sim = env.state().borrow_mut();
    let mut mouse = sim
        .world
        .pets
        .first()
        .expect("journal fixture entry")
        .clone();
    mouse.species_id = MOUSE_SPECIES as u32;
    mouse.name = "Oak Mouse".into();
    mouse.custom_name = None;
    let mut beetle = mouse.clone();
    beetle.species_id = BEETLE_SPECIES as u32;
    beetle.name = "Copper Beetle".into();
    sim.world.pets = vec![mouse, beetle];
    sim.pet_battles.player_pets = vec![
        named_battle_pet("Pip", MOUSE_SPECIES),
        named_battle_pet("Gear", BEETLE_SPECIES),
    ];
    sim.pet_battles.enemy_pets = vec![named_battle_pet("Thistle", MOUSE_SPECIES)];
    sim.pet_battles.num_pets_player = 2;
    sim.pet_battles.num_pets_enemy = 1;
}

fn named_battle_pet(name: &str, species_id: i32) -> PetBattlePet {
    PetBattlePet {
        name: name.into(),
        species_id,
        ..PetBattlePet::default_player()
    }
}

fn create_identity_display_fixture(env: &WowLuaEnv) {
    env.exec(
        r#"
        local info = debug.getinfo(PetBattleUnitFrame_UpdateDisplay, 'S')
        local suffix = 'Blizzard_PetBattleUI/Shared/Blizzard_PetBattleUI.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must invoke the original cached Blizzard display function')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)

        -- Real widgets are the output fixture, not a replacement consumer.
        -- Optional icon/model/rarity/XP children are absent in this identity-only frame.
        Cf2PetIdentityDisplay = CreateFrame('Frame', nil, UIParent)
        Cf2PetIdentityDisplay.Name = Cf2PetIdentityDisplay:CreateFontString(nil, 'OVERLAY')
        Cf2PetIdentityDisplay.SpeciesName = Cf2PetIdentityDisplay:CreateFontString(nil, 'OVERLAY')
        Cf2PetIdentityDisplay.Name:SetText('stale display name')
        Cf2PetIdentityDisplay.SpeciesName:SetText('stale species name')
        Cf2PetIdentityDisplay.SpeciesName:Hide()
        "#,
    )
    .expect("load original consumer provenance and real identity output widgets");
}

fn assert_original_display(env: &WowLuaEnv, expected: DisplayIdentity<'_>) {
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(&format!(
        "Cf2PetIdentityDisplay.petOwner = {}; Cf2PetIdentityDisplay.petIndex = {}; \
         PetBattleUnitFrame_UpdateDisplay(Cf2PetIdentityDisplay)",
        expected.owner, expected.slot,
    ))
    .expect("original cached display must consume GetName without an error");
    let (name, species, species_shown): (String, String, bool) = env
        .eval(
            "return Cf2PetIdentityDisplay.Name:GetText(), \
             Cf2PetIdentityDisplay.SpeciesName:GetText(), \
             Cf2PetIdentityDisplay.SpeciesName:IsShown()",
        )
        .unwrap();
    assert_eq!(name, expected.name, "display name for selected owner/slot");
    let custom_name_differs = expected.name != expected.species;
    assert_eq!(
        species_shown, custom_name_differs,
        "species label visibility"
    );
    if custom_name_differs {
        assert_eq!(species, expected.species, "separate species label text");
    }
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        errors_before,
        "original display call must not add recorded Lua errors"
    );
}

#[test]
fn original_display_reads_names_by_owner_and_one_based_slot() {
    let Some(env) = load_original_pet_battle_consumer() else {
        return;
    };
    for expected in [
        DisplayIdentity {
            owner: 1,
            slot: 1,
            name: "Pip",
            species: "Oak Mouse",
        },
        DisplayIdentity {
            owner: 1,
            slot: 2,
            name: "Gear",
            species: "Copper Beetle",
        },
        DisplayIdentity {
            owner: 2,
            slot: 1,
            name: "Thistle",
            species: "Oak Mouse",
        },
        DisplayIdentity {
            owner: 1,
            slot: 1,
            name: "Pip",
            species: "Oak Mouse",
        },
    ] {
        assert_original_display(&env, expected);
    }
}

#[test]
fn original_display_tracks_rename_and_species_label_transitions() {
    let Some(env) = load_original_pet_battle_consumer() else {
        return;
    };
    for name in ["Pip", "Oak Mouse", "Pip Again"] {
        env.state().borrow_mut().pet_battles.player_pets[0].name = name.into();
        assert_original_display(
            &env,
            DisplayIdentity {
                owner: 1,
                slot: 1,
                name,
                species: "Oak Mouse",
            },
        );
    }
    // Species identity changes independently of the custom name on the same slot.
    env.state().borrow_mut().pet_battles.player_pets[0].species_id = BEETLE_SPECIES;
    assert_original_display(
        &env,
        DisplayIdentity {
            owner: 1,
            slot: 1,
            name: "Pip Again",
            species: "Copper Beetle",
        },
    );
}
