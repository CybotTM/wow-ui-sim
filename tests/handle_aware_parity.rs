//! Parity tests for the handle-aware / `_static` lookup helpers landed
//! in Track 2.
//!
//! The internal helpers (`table_set_static`, `table_get_static`,
//! `get_global_by_key`, `set_global_raw_by_key`, `create_string_static`,
//! `font_f64_static`, `font_str_static`, `table_set_rust_fn_static`)
//! live inside `pub(crate) mod methods` / `pub(super) mod helpers`, so
//! they can't be imported directly from an integration test. The
//! parity guarantee we want to lock down is Lua-visible: a value
//! written via one path must be readable through the other path, and
//! vice versa. That's exactly what normal Lua semantics demand, so we
//! exercise it via `WowLuaEnv::exec` / `eval` and assert round-trip
//! equality.
//!
//! The underlying implementation (in `methods.rs` +
//! `globals/create_frame/helpers.rs`) is trivially short per helper —
//! each `_static` variant routes `intern_string_static(key.as_bytes())`
//! where the dynamic variant calls `intern_string(key.as_bytes())`.
//! Rilua's static intern cache is content-dedupe compatible with the
//! regular intern table, so keys interned through either path land in
//! the same arena slot. These tests pin that invariant at the
//! wow-ui-sim boundary: if rilua ever breaks it, `_G.foo` vs
//! `GetFontHeight(font)` would disagree between the two paths and
//! these tests would fail.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn global_write_and_read_round_trip() {
    // Writes through `_G.Foo = bar` flow through `set_global_raw` →
    // `set_global_raw_by_key` → `intern_string` on the key. Reads
    // through `return Foo` flow through `get_global` →
    // `get_global_by_key` on the same key. Both paths must see the
    // same value regardless of which one allocated the key first.
    let env = WowLuaEnv::new().expect("fresh wow lua env");
    env.exec("_G.ParityTestGlobalA = 1234").expect("set global");
    env.exec("_G.ParityTestGlobalB = 'hello'")
        .expect("set global");
    let a: f64 = env.eval("return ParityTestGlobalA").expect("read A");
    let b: String = env.eval("return ParityTestGlobalB").expect("read B");
    assert_eq!(a, 1234.0);
    assert_eq!(b, "hello");
}

#[test]
fn font_string_field_accesses_round_trip_through_static_keys() {
    // FontString field access in `fonts.rs` now uses
    // `table_set_static` / `table_get_static` / `font_f64_static` /
    // `font_str_static` for the 19 default-field keys
    // (`__fontHeight`, `__fontFlags`, `__textColor*`, etc.). Writing
    // via the public font API (`SetFontHeight`, `SetFont`,
    // `SetTextColor`) and reading via the matching getter exercises
    // the static-intern path end-to-end; any divergence between the
    // static and dynamic keys would show up as a mis-read here.
    let env = WowLuaEnv::new().expect("fresh wow lua env");
    let out: (f64, String, f64, f64, f64, f64) = env
        .eval(
            r#"
            local f = CreateFont("ParityTestFont")
            f:SetFont("Fonts\\FRIZQT__.TTF", 14, "OUTLINE")
            f:SetTextColor(0.25, 0.5, 0.75, 1.0)
            local _, height, flags = f:GetFont()
            local r, g, b, a = f:GetTextColor()
            return height, flags or "", r, g, b, a
        "#,
        )
        .expect("font round-trip should succeed");
    let (height, flags, r, g, b, a) = out;
    assert_eq!(height, 14.0, "GetFont height mismatch");
    assert_eq!(flags, "OUTLINE", "GetFont flags mismatch");
    assert!((r - 0.25).abs() < 1e-9, "r channel drifted: {r}");
    assert!((g - 0.5).abs() < 1e-9, "g channel drifted: {g}");
    assert!((b - 0.75).abs() < 1e-9, "b channel drifted: {b}");
    assert!((a - 1.0).abs() < 1e-9, "a channel drifted: {a}");
}

#[test]
fn registered_c_namespace_methods_dispatch_through_static_keys() {
    // `ensure_namespace` / `table_set_rust_fn_static` register C_*
    // namespace method names via `intern_string_static`. A Lua call
    // like `C_TradeSkillUI.GetAllRecipeIDs()` routes through the
    // global table → C_TradeSkillUI table → method table, all three
    // lookups needing the same key the registration used. If the
    // static-intern cache returned a distinct `GcRef` from the
    // dynamic path, the method would dispatch to nil.
    let env = WowLuaEnv::new().expect("fresh wow lua env");
    let dispatched: bool = env
        .eval(
            r#"
            return type(C_TradeSkillUI) == "table"
               and type(C_TradeSkillUI.GetAllRecipeIDs) == "function"
               and type(C_AuctionHouse.HasFullBrowseResults) == "function"
        "#,
        )
        .expect("namespace dispatch query should succeed");
    assert!(
        dispatched,
        "C_* namespace methods registered via table_set_rust_fn_static should dispatch",
    );
}
