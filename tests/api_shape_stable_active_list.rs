//! Shape-only contract; membership, ordering and empty-state semantics are native-unknown.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

// Retail/PTR: Blizzard_APIDocumentationGenerated/StableInfoDocumentation.lua:15-22,
// :193-214 (PetInfo). Blizzard_StableUI/Blizzard_StableUI.toc:4 selects
// Blizzard_StableUI/Blizzard_StableUI.lua:447 (StableActivePetListMixin:Refresh).
// Forever selects Camelot, which never calls this API; it is deliberately excluded.
#[test]
fn stable_active_list_shape_blocker_contract() {
    crate::common::with_timeout(90, || {
        let ui = crate::common::blizzard_ui_dir();
        let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui);
        env.exec(
            r#"
            assert(type(C_StableInfo.GetActivePetList) == 'function',
                'SHAPE BLOCKER: missing GetActivePetList')
            local function checkList(...)
                assert(select('#', ...) == 1, 'GetActivePetList return arity')
                local pets = ...
                assert(type(pets) == 'table', 'non-nil table<PetInfo> required')
                local fields = {
                    slotID = 'number', icon = 'number', name = 'string', level = 'number',
                    familyName = 'string', specialization = 'string', type = 'string',
                    displayID = 'number', isFavorite = 'boolean', isExotic = 'boolean',
                    uiModelSceneID = 'number', petNumber = 'number', creatureID = 'number',
                    specID = 'number',
                }
                for _, pet in pairs(pets) do
                    assert(type(pet) == 'table', 'PetInfo record required')
                    for field, kind in pairs(fields) do
                        assert(type(pet[field]) == kind, 'PetInfo.' .. field)
                    end
                    for _, field in ipairs({'petAbilities', 'specAbilities'}) do
                        assert(type(pet[field]) == 'table', 'PetInfo.' .. field)
                        for _, ability in pairs(pet[field]) do
                            assert(type(ability) == 'number', 'numeric ability required')
                        end
                    end
                end
            end
            -- Empty is shape-compatible, not a native-confirmed empty-state result.
            -- Default=718 does not require every uiModelSceneID to equal 718.
            checkList(C_StableInfo.GetActivePetList())
            "#,
        )
        .expect("documented active pet list shape (not semantic RED)");
        crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env(
            &env,
            &ui,
            &["Blizzard_FrameXML", "Blizzard_StableUI"],
            &[],
        );
        assert!(
            env.state().borrow().lua_errors.is_empty(),
            "SETUP: original StableUI load errors: {:?}",
            env.state().borrow().lua_errors
        );
        refresh_original_active_list(&env);
    });
}

fn refresh_original_active_list(env: &WowLuaEnv) {
    env.exec(
        r#"
        assert(StableFrame and StableFrame.ActivePetList, 'SETUP: original active list')
        assert(type(StableActivePetListMixin.Refresh) == 'function', 'SETUP: original Refresh')
        StableActivePetListMixin.Refresh(StableFrame.ActivePetList)
        assert(type(StableFrame.ActivePetList.pets) == 'table', 'consumer retained actual list')
        "#,
    )
    .expect("original stable consumer must accept actual API result");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "original stable consumer errors: {:?}",
        env.state().borrow().lua_errors
    );
}
