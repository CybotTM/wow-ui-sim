use super::{env, setup_texture};

#[test]
fn test_nine_slice_margins() {
    let env = env();
    let (_, tex) = setup_texture(&env, "NS");
    env.exec(&format!("{tex}:SetTextureSliceMargins(10, 20, 30, 40)"))
        .unwrap();
    let (l, r, t, b): (f64, f64, f64, f64) = env
        .eval(&format!("return {tex}:GetTextureSliceMargins()"))
        .unwrap();
    assert_eq!((l, r, t, b), (10.0, 20.0, 30.0, 40.0));
}

#[test]
fn test_nine_slice_mode() {
    let env = env();
    let (_, tex) = setup_texture(&env, "NSMode");
    env.exec(&format!("{tex}:SetTextureSliceMode(1)")).unwrap();
    let mode: i32 = env
        .eval(&format!("return {tex}:GetTextureSliceMode()"))
        .unwrap();
    assert_eq!(mode, 1);
}

#[test]
fn test_clear_texture_slice_resets_margins_and_mode() {
    let env = env();
    let (_, tex) = setup_texture(&env, "NSClear");
    let (l, r, t, b, mode): (f64, f64, f64, f64, i32) = env
        .eval(&format!(
            r#"
            {tex}:SetTextureSliceMargins(10, 20, 30, 40)
            {tex}:SetTextureSliceMode(1)
            {tex}:ClearTextureSlice()
            local l, r, t, b = {tex}:GetTextureSliceMargins()
            return l, r, t, b, {tex}:GetTextureSliceMode()
        "#
        ))
        .unwrap();
    assert_eq!((l, r, t, b), (0.0, 0.0, 0.0, 0.0));
    assert_eq!(mode, 0);
}
