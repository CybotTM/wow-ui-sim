//! `C_LFGList.GetNumApplications` / `GetNumApplicants` — two-value probes.

use wow_ui_sim::lua_api::WowLuaEnv;

fn apps(env: &WowLuaEnv) -> (i64, i64) {
    env.eval(r#"return C_LFGList.GetNumApplications()"#).unwrap()
}

fn applicants(env: &WowLuaEnv) -> (i64, i64) {
    env.eval(r#"return C_LFGList.GetNumApplicants()"#).unwrap()
}

#[test]
fn defaults_zero_zero() {
    let env = WowLuaEnv::new().unwrap();
    assert_eq!(apps(&env), (0, 0));
    assert_eq!(applicants(&env), (0, 0));
}

#[test]
fn both_return_two_values() {
    // The shape matters — LFGListFrame destructures `local total, viewed = ...`.
    let env = WowLuaEnv::new().unwrap();
    let (apps_count, applicants_count): (i64, i64) = env
        .eval(
            r#"
            return select('#', C_LFGList.GetNumApplications()),
                   select('#', C_LFGList.GetNumApplicants())
            "#,
        )
        .unwrap();
    assert_eq!(apps_count, 2);
    assert_eq!(applicants_count, 2);
}

#[test]
fn admin_drives_application_counts() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetLfgApplicationCounts(5, 3)").unwrap();
    assert_eq!(apps(&env), (5, 3));
    assert_eq!(applicants(&env), (0, 0), "applicant counts untouched");
}

#[test]
fn admin_drives_applicant_counts() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetLfgApplicantCounts(12, 8)").unwrap();
    assert_eq!(applicants(&env), (12, 8));
    assert_eq!(apps(&env), (0, 0), "application counts untouched");
}

#[test]
fn missing_args_default_to_zero() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetLfgApplicationCounts(7)").unwrap();
    let (total, viewed) = apps(&env);
    assert_eq!(total, 7);
    assert_eq!(viewed, 0, "missing viewed arg should default to 0");
}

#[test]
fn negative_counts_clamp_to_zero() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetLfgApplicationCounts(-1, -5)").unwrap();
    env.exec("A_Admin.SetLfgApplicantCounts(-3, -2)").unwrap();
    assert_eq!(apps(&env), (0, 0));
    assert_eq!(applicants(&env), (0, 0));
}

#[test]
fn no_arg_resets_to_defaults() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetLfgApplicationCounts(5, 3)").unwrap();
    env.exec("A_Admin.SetLfgApplicationCounts()").unwrap();
    assert_eq!(apps(&env), (0, 0));
}

#[test]
fn applications_and_applicants_are_independent() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetLfgApplicationCounts(10, 4)").unwrap();
    env.exec("A_Admin.SetLfgApplicantCounts(20, 8)").unwrap();
    assert_eq!(apps(&env), (10, 4));
    assert_eq!(applicants(&env), (20, 8));
}
