#![cfg(feature = "client-mists")]

use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn load_full_game_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    env
}

#[test]
fn mists_pet_journal_and_battle_pet_ui_render_and_interact() {
    let env = load_full_game_ui();
    env.exec(
        r#"
        ToggleCollectionsJournal(COLLECTIONS_JOURNAL_TAB_INDEX_PETS)
        if PetJournal == nil or PetJournal:IsShown() ~= true then
            error("PetJournal did not open")
        end

        local petID = C_PetJournal.GetPetInfoByIndex(1)
        if type(petID) ~= "string" or petID == "" then
            error("PetJournal first petID missing")
        end
        PetJournal_SelectPet(PetJournal, petID)
        if PetJournalPetCard.petID ~= petID then
            error("PetJournal card did not select pet")
        end
        if PetJournalPetCard.PetInfo.name:GetText() == nil then
            error("PetJournal card has no pet name")
        end

        A_Admin.SetPetBattleState(Enum.PetbattleState.WaitingPreBattle)
        PetBattleFrame_Display(PetBattleFrame)
        if PetBattleFrame:IsShown() ~= true then
            error("PetBattleFrame did not show")
        end

        PetBattleFrame_UpdateAssignedUnitFrames(PetBattleFrame)
        if PetBattleFrame.ActiveAlly.Name:GetText() ~= "Arcane Familiar" then
            error("active ally pet name missing")
        end
        if PetBattleFrame.ActiveEnemy.Name:GetText() ~= "Stone Lurker" then
            error("active enemy pet name missing")
        end

        PetBattleFrame_UpdateAllActionButtons(PetBattleFrame)
        if #PetBattleFrame.BottomFrame.abilityButtons < 2 then
            error("ability buttons were not created")
        end
        PetBattleAbilityButton_OnClick(PetBattleFrame.BottomFrame.abilityButtons[2])
        local actionType, actionIndex = C_PetBattles.GetSelectedAction()
        if actionType ~= Enum.BattlePetAction.Ability or actionIndex ~= 2 then
            error("ability click did not select battle action")
        end

        PetBattleCatchButton_OnClick(PetBattleFrame.BottomFrame.CatchButton)
        actionType = C_PetBattles.GetSelectedAction()
        if actionType ~= Enum.BattlePetAction.Trap then
            error("catch button did not select trap action")
        end
        "#,
    )
    .expect("Mists pet journal/battle pet probe should run");

    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "Mists pet journal/battle pet probe emitted Lua errors: {errors:#?}"
    );
}
