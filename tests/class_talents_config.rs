//! Tests for `C_ClassTalents` config / trait-tree probes that read the
//! `TalentState` module:
//!
//! - `C_ClassTalents.GetActiveConfigID()`
//! - `C_ClassTalents.GetConfigIDsBySpecID(specID)`
//! - `C_ClassTalents.GetHeroTalentSpecsForClassSpec(classID, specID)`
//! - `C_ClassTalents.GetTraitTreeForSpec(specID)`
//!
//! These existed as dead `stub_nil` entries in `NAMESPACE_NIL_STUBS`
//! even though the real implementations already ran. The task is to
//! drop the dead entries after pinning the behavior with tests.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_active_config_id_returns_protection_default() {
    let env = env();
    let active: i32 = env
        .eval("return C_ClassTalents.GetActiveConfigID()")
        .unwrap();
    assert_eq!(
        active, 201,
        "seeded Protection Paladin starts on config 201"
    );
}

#[test]
fn get_config_ids_by_spec_id_returns_two_configs_per_paladin_spec() {
    let env = env();
    let (holy, prot, ret): (Vec<i32>, Vec<i32>, Vec<i32>) = env
        .eval(
            r#"
            local function array(tbl)
                local out = {}
                for i = 1, #tbl do out[i] = tbl[i] end
                return out
            end
            return array(C_ClassTalents.GetConfigIDsBySpecID(65)),
                   array(C_ClassTalents.GetConfigIDsBySpecID(66)),
                   array(C_ClassTalents.GetConfigIDsBySpecID(70))
            "#,
        )
        .unwrap();
    assert_eq!(holy, vec![101, 102]);
    assert_eq!(prot, vec![201, 202]);
    assert_eq!(ret, vec![301, 302]);
}

#[test]
fn get_config_ids_by_spec_id_returns_empty_for_unknown_spec() {
    let env = env();
    let n: i32 = env
        .eval("return #C_ClassTalents.GetConfigIDsBySpecID(999)")
        .unwrap();
    assert_eq!(n, 0);
}

#[test]
fn get_hero_talent_specs_for_class_spec_returns_array_and_level() {
    let env = env();
    let (specs_a, specs_b, specs_c, level_for_b): (Vec<i32>, Vec<i32>, Vec<i32>, i32) = env
        .eval(
            r#"
            local function array(tbl)
                local out = {}
                for i = 1, #tbl do out[i] = tbl[i] end
                return out
            end
            local holy, _ = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 65)
            local prot, level = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 66)
            local ret, _ = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 70)
            return array(holy), array(prot), array(ret), level
            "#,
        )
        .unwrap();

    assert_eq!(specs_a, vec![49, 50], "Holy hero specs");
    assert_eq!(specs_b, vec![48, 49], "Protection hero specs");
    assert_eq!(specs_c, vec![48, 50], "Retribution hero specs");
    assert_eq!(level_for_b, 71, "second return is the unlock level");
}

#[test]
fn get_trait_tree_for_spec_returns_tree_id() {
    let env = env();
    let tree_id: i32 = env
        .eval("return C_ClassTalents.GetTraitTreeForSpec(66)")
        .unwrap();
    assert_eq!(tree_id, 790, "Paladin class talent tree");
}
