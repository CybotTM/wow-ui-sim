//! Tests for social / character-sheet probe globals backed by SimState:
//!
//! - `GetNumTitles` / `GetTitleName(index)`
//! - `GetNumClasses`
//! - `GetNumShapeshiftForms`

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_num_titles_reports_sim_state_titles_len() {
    let env = env();
    let before: i32 = env.eval("return GetNumTitles()").unwrap();
    assert_eq!(before, 0);

    {
        let mut state = env.state().borrow_mut();
        state.titles.push("the Patient".to_string());
        state.titles.push("Jenkins".to_string());
        state.titles.push("of the Nightfall".to_string());
    }

    let after: i32 = env.eval("return GetNumTitles()").unwrap();
    assert_eq!(after, 3);
}

#[test]
fn get_title_name_indexes_one_based_and_nils_out_of_range() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.titles.push("the Patient".to_string());
        state.titles.push("Jenkins".to_string());
    }

    let (first, second, below, above, zero): (String, String, bool, bool, bool) = env
        .eval(
            r#"
            return GetTitleName(1),
                   GetTitleName(2),
                   GetTitleName(-1) == nil,
                   GetTitleName(99) == nil,
                   GetTitleName(0) == nil
            "#,
        )
        .unwrap();

    assert_eq!(first, "the Patient");
    assert_eq!(second, "Jenkins");
    assert!(below, "negative index should return nil");
    assert!(above, "out-of-range index should return nil");
    assert!(zero, "zero index should return nil (1-based)");
}

#[test]
fn get_num_classes_returns_thirteen() {
    let env = env();
    let n: i32 = env.eval("return GetNumClasses()").unwrap();
    assert_eq!(n, 13, "retail has 13 classes (includes Evoker)");
}

#[test]
fn get_num_shapeshift_forms_reports_sim_state_len() {
    let env = env();
    let before: i32 = env.eval("return GetNumShapeshiftForms()").unwrap();
    assert_eq!(before, 0, "seeded Paladin has no shapeshift forms");

    {
        use wow_ui_sim::lua_api::state::ShapeshiftForm;
        let mut state = env.state().borrow_mut();
        state.shapeshift_forms.push(ShapeshiftForm {
            name: "Bear Form".to_string(),
            texture: "Interface/Icons/Ability_Racial_BearForm".to_string(),
            spell_id: 5487,
            is_active: false,
            is_castable: true,
        });
        state.shapeshift_forms.push(ShapeshiftForm {
            name: "Cat Form".to_string(),
            texture: "Interface/Icons/Ability_Druid_CatForm".to_string(),
            spell_id: 768,
            is_active: false,
            is_castable: true,
        });
    }

    let after: i32 = env.eval("return GetNumShapeshiftForms()").unwrap();
    assert_eq!(after, 2);
}

#[test]
fn shapeshift_form_id_defaults_to_nil() {
    let env = env();
    let is_nil: bool = env.eval("return GetShapeshiftFormID() == nil").unwrap();
    assert!(is_nil);
}
