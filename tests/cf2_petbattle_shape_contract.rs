//! Shape-only contracts; native semantics unknown. Source predictions are not runtime proof.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists"
))]

#[path = "common/cf2_shape.rs"]
mod shape;

// retail: Blizzard_APIDocumentationGenerated/PetBattlesDocumentation.lua:44 (GetName).
// retail: Blizzard_APIDocumentationGenerated/BattlePetConstantsDocumentation.lua:37 (BattlePetOwner).
// ptr: Blizzard_APIDocumentationGenerated/PetBattlesDocumentation.lua:44 (GetName).
// ptr: Blizzard_APIDocumentationGenerated/BattlePetConstantsDocumentation.lua:37 (BattlePetOwner).
// wowforever: Blizzard_APIDocumentationGenerated/PetBattlesDocumentation.lua:44 (GetName).
// wowforever: Blizzard_APIDocumentationGenerated/BattlePetConstantsDocumentation.lua:37 (BattlePetOwner).
// mists: Blizzard_APIDocumentationGenerated/PetBattlesDocumentation.lua:42 (GetName).
// mists: Blizzard_APIDocumentationGenerated/BattlePetConstantsDocumentation.lua:37 (BattlePetOwner).
// Source-predicted FAIL: GetName missing; optional two strings and original display acceptance, no fallback names; not executed; native semantics unknown.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists"
))]
#[test]
fn original_pet_display_name_pair_shape_blocker() {
    let env = shape::load(&["Blizzard_PetBattleUI"]);
    {
        let mut sim = env.state().borrow_mut();
        let species = sim
            .world
            .pets
            .first()
            .expect("existing pet species fixture");
        let pet = wow_ui_sim::lua_api::state::PetBattlePet {
            name: "Shape pet".into(),
            species_id: i32::try_from(species.species_id).expect("species ID fits battle model"),
            ..wow_ui_sim::lua_api::state::PetBattlePet::default_player()
        };
        sim.pet_battles.player_pets = vec![pet];
        sim.pet_battles.num_pets_player = 1;
    }
    shape::check(
        &env,
        r#"

enum(Enum.BattlePetOwner, {Weather=0, Ally=1, Enemy=2})
local owner, slot = Enum.BattlePetOwner.Ally, 1
local result = pack(C_PetBattles.GetName(owner, slot))
assert(result.n == 0 or result.n == 2, 'documented optional name pair')
if result.n == 2 then
    assert(type(result[1]) == 'string' and type(result[2]) == 'string')
end
original(PetBattleUnitFrame_UpdateDisplay, 'Blizzard_PetBattleUI/Shared/Blizzard_PetBattleUI.lua')
local frame = CreateFrame('Frame', nil, UIParent)
frame.petOwner, frame.petIndex = owner, slot
frame.Name = frame:CreateFontString(nil, 'OVERLAY')
frame.SpeciesName = frame:CreateFontString(nil, 'OVERLAY')
PetBattleUnitFrame_UpdateDisplay(frame)

"#,
    );
}
