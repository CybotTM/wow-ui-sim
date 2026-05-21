#[test]
fn test_scripts_disallowed_for_beta_defaults_false() {
    let env = super::env_with_addons();
    let disallowed: bool = env
        .eval("return C_AddOns.GetScriptsDisallowedForBeta()")
        .unwrap();
    assert!(!disallowed);
}
