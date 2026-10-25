//! Shared generated-catalog lookup contracts, not original-consumer RED proof.
use std::collections::BTreeSet;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::{spells, traits};

#[test]
fn dangling_70391_trait_spells_return_nil_info_and_no_texture() {
    let missing: BTreeSet<u32> = traits::TRAIT_DEFINITION_DB
        .values()
        .flat_map(|definition| {
            [
                definition.spell_id,
                definition.overrides_spell_id,
                definition.visible_spell_id,
            ]
        })
        .filter(|id| *id != 0 && spells::get_spell(*id).is_none())
        .collect();
    // Concrete export census prevents an empty/vacuous unknown-lookup test.
    assert_eq!(missing.len(), 184);
    assert!(missing.contains(&316099));
    let env = WowLuaEnv::new().unwrap();
    for id in missing {
        env.exec(&format!(
            "assert(C_Spell.GetSpellInfo({id}) == nil); assert(select('#', C_Spell.GetSpellTexture({id})) == 0)"
        ))
        .unwrap_or_else(|error| panic!("dangling trait spell {id}: {error}"));
    }
}

#[test]
fn same_build_trait_definitions_retain_missing_and_revised_spell_ids() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local missing = C_Traits.GetDefinitionInfo(121056)
        assert(missing.spellID == 316099)
        assert(C_Spell.GetSpellInfo(missing.spellID) == nil)
        assert(C_Spell.GetSpellDescription(missing.spellID) == "")
        local revised = C_Traits.GetDefinitionInfo(108697)
        assert(revised.spellID == 1252217)
        local spell = C_Spell.GetSpellInfo(revised.spellID)
        assert(spell and spell.spellID == 1252217)
        "#,
    )
    .unwrap();
}
