//! A `<Font>` element's `<Color>` child is the text colour every FontString
//! inheriting the font starts with. It never reached the renderer: the XML
//! loader dropped the element, so `GameFontNormalSmall` and
//! `GameNormalNumberFont` answered (1, 1, 1) and FontStrings inheriting them
//! rendered white where the client draws them gold.

use crate::common::blizzard_addon_harness::with_blizzard_addon_closure;

fn assert_rgb(actual: (f64, f64, f64), expected: (f64, f64, f64), what: &str) {
    let close = |a: f64, b: f64| (a - b).abs() < 0.001;
    assert!(
        close(actual.0, expected.0) && close(actual.1, expected.1) && close(actual.2, expected.2),
        "{what}: expected {expected:?}, got {actual:?}"
    );
}

#[test]
fn blizzard_gold_fonts_carry_their_colour_into_font_strings() {
    with_blizzard_addon_closure(
        &["Blizzard_Colors", "Blizzard_Fonts_Shared"],
        &[],
        |env, _| {
            let (sr, sg, sb, nr, ng, nb, fr, fg, fb, hr, hg, hb, lr, lg, lb): (
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
            ) = env
                .eval(
                    r#"
                local sr, sg, sb = GameFontNormalSmall:GetTextColor()
                local nr, ng, nb = GameNormalNumberFont:GetTextColor()
                local fs = UIParent:CreateFontString(nil, "OVERLAY", "GameFontNormalSmall")
                local fr, fg, fb = fs:GetTextColor()
                local hr, hg, hb = GameFontHighlightSmall:GetTextColor()
                local lr, lg, lb = GameFontNormalLarge:GetTextColor()
                return sr, sg, sb, nr, ng, nb, fr, fg, fb, hr, hg, hb, lr, lg, lb
                "#,
                )
                .expect("font colours");
            assert_rgb(
                (sr, sg, sb),
                (1.0, 0.82, 0.0),
                "GameFontNormalSmall <Color color=NORMAL_FONT_COLOR>",
            );
            assert_rgb(
                (nr, ng, nb),
                (1.0, 0.82, 0.0),
                "GameNormalNumberFont <Color r=1 g=.82 b=0>",
            );
            assert_rgb(
                (fr, fg, fb),
                (1.0, 0.82, 0.0),
                "a FontString inheriting GameFontNormalSmall",
            );
            assert_rgb(
                (hr, hg, hb),
                (1.0, 1.0, 1.0),
                "GameFontHighlightSmall <Color color=WHITE_FONT_COLOR>",
            );
            assert_rgb(
                (lr, lg, lb),
                (1.0, 0.82, 0.0),
                "GameFontNormalLarge inherits GameFontNormal's colour",
            );
        },
    );
}
