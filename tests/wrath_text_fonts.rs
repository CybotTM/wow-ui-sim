//! Wrath 12340 text contracts. Exact font-generation acceptance remains held.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_lua_contract(script: &str) {
    let env = WowLuaEnv::new().expect("create Lua environment");
    env.exec(
        r#"
        function capture(...) return { n = select('#', ...), ... } end
        function assert_same_font(before, after)
            assert(after.n == before.n, 'GetFont arity changed')
            for index = 1, before.n do
                assert(after[index] == before[index], 'font field mutated: ' .. index)
            end
        end
        function assert_color(object, r, g, b, a)
            local values = capture(object:GetTextColor())
            assert(values.n == 4)
            local expected = {r, g, b, a}
            for index = 1, 4 do
                assert(math.abs(values[index] - expected[index]) < 0.00001,
                    'color component mismatch: ' .. index)
            end
        end
        "#,
    )
    .expect("install local assertion helpers");
    env.exec(script).expect("Wrath text/font contract");
}

#[test]
fn t5_fontstring_empty_has_zero_returns_and_retains_font() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        -- Precondition: this standard WoW font must be generation-capable.
        -- Do not assert a setup return: Font success arity is not supplied.
        object:SetFont('Fonts\\FRIZQT__.TTF', 18, 'OUTLINE')
        local before = capture(object:GetFont())
        assert(before.n == 3 and before[1] == 'Fonts\\FRIZQT__.TTF')
        assert(before[2] == 18 and before[3] == 'OUTLINE')

        local result = capture(object:SetFont('', 24, 'THICKOUTLINE'))
        assert_same_font(before, capture(object:GetFont()))
        assert(result.n == 0, 'empty FontString filename returns no values')
        "#,
    );
}

#[test]
fn t5_fontstring_zero_height_errors_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        -- Precondition: this standard WoW font must be generation-capable.
        -- Do not assert a setup return: Font success arity is not supplied.
        object:SetFont('Fonts\\FRIZQT__.TTF', 18, 'OUTLINE')
        local before = capture(object:GetFont())
        assert(before.n == 3 and before[1] == 'Fonts\\FRIZQT__.TTF')
        assert(before[2] == 18 and before[3] == 'OUTLINE')

        local ok, error = pcall(function()
            object:SetFont('Fonts\\ARIALN.TTF', 0, 'THICKOUTLINE')
        end)
        assert_same_font(before, capture(object:GetFont()))
        assert(not ok and type(error) == 'string' and #error > 0)
        "#,
    );
}

#[test]
fn t5_fontstring_negative_height_errors_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        -- Precondition: this standard WoW font must be generation-capable.
        -- Do not assert a setup return: Font success arity is not supplied.
        object:SetFont('Fonts\\FRIZQT__.TTF', 18, 'OUTLINE')
        local before = capture(object:GetFont())
        assert(before.n == 3 and before[1] == 'Fonts\\FRIZQT__.TTF')
        assert(before[2] == 18 and before[3] == 'OUTLINE')

        local ok, error = pcall(function()
            object:SetFont('Fonts\\ARIALN.TTF', -2, 'THICKOUTLINE')
        end)
        assert_same_font(before, capture(object:GetFont()))
        assert(not ok and type(error) == 'string' and #error > 0)
        "#,
    );
}

#[test]
fn t5_fontstring_missing_height_errors_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        -- Precondition: this standard WoW font must be generation-capable.
        -- Do not assert a setup return: Font success arity is not supplied.
        object:SetFont('Fonts\\FRIZQT__.TTF', 18, 'OUTLINE')
        local before = capture(object:GetFont())
        assert(before.n == 3 and before[1] == 'Fonts\\FRIZQT__.TTF')
        assert(before[2] == 18 and before[3] == 'OUTLINE')

        local ok, error = pcall(function() object:SetFont('Fonts\\ARIALN.TTF') end)
        assert_same_font(before, capture(object:GetFont()))
        assert(not ok and type(error) == 'string' and #error > 0)
        "#,
    );
}

#[test]
fn t5_fontstring_nil_filename_errors_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        -- Precondition: this standard WoW font must be generation-capable.
        -- Do not assert a setup return: Font success arity is not supplied.
        object:SetFont('Fonts\\FRIZQT__.TTF', 18, 'OUTLINE')
        local before = capture(object:GetFont())
        assert(before.n == 3 and before[1] == 'Fonts\\FRIZQT__.TTF')
        assert(before[2] == 18 and before[3] == 'OUTLINE')

        local ok, error = pcall(function() object:SetFont(nil, 24, 'THICKOUTLINE') end)
        assert_same_font(before, capture(object:GetFont()))
        assert(not ok and type(error) == 'string' and #error > 0)
        "#,
    );
}

#[test]
fn t5_font_zero_height_returns_nil_without_error() {
    assert_lua_contract(
        r#"
        local object = CreateFont('StageTextFont')

        -- Precondition: this standard WoW font must be generation-capable.
        -- Do not assert a setup return: Font success arity is not supplied.
        object:SetFont('Fonts\\FRIZQT__.TTF', 18, 'OUTLINE')
        local before = capture(object:GetFont())
        assert(before.n == 3 and before[1] == 'Fonts\\FRIZQT__.TTF')
        assert(before[2] == 18 and before[3] == 'OUTLINE')

        local ok, result = pcall(function()
            return capture(object:SetFont('Fonts\\ARIALN.TTF', 0, 'THICKOUTLINE'))
        end)
        assert_same_font(before, capture(object:GetFont()))
        assert(ok, 'Font/EditBox rejected height does not use FontString height error')
        assert(result.n == 1 and result[1] == nil)
        "#,
    );
}

#[test]
fn t5_editbox_zero_height_returns_nil_without_error() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('EditBox')

        -- Precondition: this standard WoW font must be generation-capable.
        -- Do not assert a setup return: Font success arity is not supplied.
        object:SetFont('Fonts\\FRIZQT__.TTF', 18, 'OUTLINE')
        local before = capture(object:GetFont())
        assert(before.n == 3 and before[1] == 'Fonts\\FRIZQT__.TTF')
        assert(before[2] == 18 and before[3] == 'OUTLINE')

        local ok, result = pcall(function()
            return capture(object:SetFont('Fonts\\ARIALN.TTF', 0, 'THICKOUTLINE'))
        end)
        assert_same_font(before, capture(object:GetFont()))
        assert(ok, 'Font/EditBox rejected height does not use FontString height error')
        assert(result.n == 1 and result[1] == nil)
        "#,
    );
}

#[test]
fn t7_fontstring_canonicalizes_case_on_both_axes() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        for _, entry in ipairs({{'left','LEFT'}, {'cEnTeR','CENTER'}, {'rIgHt','RIGHT'}}) do
            object:SetJustifyH(entry[1])
            assert(select('#', object:GetJustifyH()) == 1)
            assert(object:GetJustifyH() == entry[2])
        end
        for _, entry in ipairs({{'top','TOP'}, {'mIdDlE','MIDDLE'}, {'bOtToM','BOTTOM'}}) do
            object:SetJustifyV(entry[1])
            assert(select('#', object:GetJustifyV()) == 1)
            assert(object:GetJustifyV() == entry[2])
        end
        "#,
    );
}

#[test]
fn t7_fontstring_wrong_axis_is_accepted_and_reads_unknown() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        for _, token in ipairs({'TOP', 'mIdDlE', 'bottom'}) do
            object:SetJustifyH('RIGHT')
            object:SetJustifyV('BOTTOM')
            local ok = pcall(function() object:SetJustifyH(token) end)
            assert(ok, 'recognized wrong-axis token must not error')
            assert(object:GetJustifyH() == 'UNKNOWN')
            assert(object:GetJustifyV() == 'BOTTOM', 'other axis retained')
        end
        for _, token in ipairs({'LEFT', 'cEnTeR', 'right'}) do
            object:SetJustifyH('RIGHT')
            object:SetJustifyV('BOTTOM')
            local ok = pcall(function() object:SetJustifyV(token) end)
            assert(ok)
            assert(object:GetJustifyV() == 'UNKNOWN')
            assert(object:GetJustifyH() == 'RIGHT', 'other axis retained')
        end
        "#,
    );
}

#[test]
fn t7_fontstring_h_invalid_errors_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        object:SetJustifyH('RIGHT')
        object:SetJustifyV('BOTTOM')
        local calls = {
            function() object:SetJustifyH('DIAGONAL') end,
            function() object:SetJustifyH(nil) end,
            function() object:SetJustifyH() end,
            function() object:SetJustifyH(42) end,
            function() object:SetJustifyH({}) end,
        }
        for _, call in ipairs(calls) do
            local ok, error = pcall(call)
            assert(object:GetJustifyH() == 'RIGHT')
            assert(object:GetJustifyV() == 'BOTTOM')
            assert(not ok and type(error) == 'string' and #error > 0,
                'unknown/nil tokens require usage errors')
        end
        "#,
    );
}

#[test]
fn t7_fontstring_v_invalid_errors_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        object:SetJustifyH('RIGHT')
        object:SetJustifyV('BOTTOM')
        local calls = {
            function() object:SetJustifyV('DIAGONAL') end,
            function() object:SetJustifyV(nil) end,
            function() object:SetJustifyV() end,
            function() object:SetJustifyV(42) end,
            function() object:SetJustifyV({}) end,
        }
        for _, call in ipairs(calls) do
            local ok, error = pcall(call)
            assert(object:GetJustifyH() == 'RIGHT')
            assert(object:GetJustifyV() == 'BOTTOM')
            assert(not ok and type(error) == 'string' and #error > 0,
                'unknown/nil tokens require usage errors')
        end
        "#,
    );
}

#[test]
fn t7_font_canonicalizes_case_on_both_axes() {
    assert_lua_contract(
        r#"
        local object = CreateFont('StageTextFont')

        for _, entry in ipairs({{'left','LEFT'}, {'cEnTeR','CENTER'}, {'rIgHt','RIGHT'}}) do
            object:SetJustifyH(entry[1])
            assert(select('#', object:GetJustifyH()) == 1)
            assert(object:GetJustifyH() == entry[2])
        end
        for _, entry in ipairs({{'top','TOP'}, {'mIdDlE','MIDDLE'}, {'bOtToM','BOTTOM'}}) do
            object:SetJustifyV(entry[1])
            assert(select('#', object:GetJustifyV()) == 1)
            assert(object:GetJustifyV() == entry[2])
        end
        "#,
    );
}

#[test]
fn t7_font_wrong_axis_is_accepted_and_reads_unknown() {
    assert_lua_contract(
        r#"
        local object = CreateFont('StageTextFont')

        for _, token in ipairs({'TOP', 'mIdDlE', 'bottom'}) do
            object:SetJustifyH('RIGHT')
            object:SetJustifyV('BOTTOM')
            local ok = pcall(function() object:SetJustifyH(token) end)
            assert(ok, 'recognized wrong-axis token must not error')
            assert(object:GetJustifyH() == 'UNKNOWN')
            assert(object:GetJustifyV() == 'BOTTOM', 'other axis retained')
        end
        for _, token in ipairs({'LEFT', 'cEnTeR', 'right'}) do
            object:SetJustifyH('RIGHT')
            object:SetJustifyV('BOTTOM')
            local ok = pcall(function() object:SetJustifyV(token) end)
            assert(ok)
            assert(object:GetJustifyV() == 'UNKNOWN')
            assert(object:GetJustifyH() == 'RIGHT', 'other axis retained')
        end
        "#,
    );
}

#[test]
fn t7_font_h_invalid_errors_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFont('StageTextFont')

        object:SetJustifyH('RIGHT')
        object:SetJustifyV('BOTTOM')
        local calls = {
            function() object:SetJustifyH('DIAGONAL') end,
            function() object:SetJustifyH(nil) end,
            function() object:SetJustifyH() end,
            function() object:SetJustifyH(42) end,
            function() object:SetJustifyH({}) end,
        }
        for _, call in ipairs(calls) do
            local ok, error = pcall(call)
            assert(object:GetJustifyH() == 'RIGHT')
            assert(object:GetJustifyV() == 'BOTTOM')
            assert(not ok and type(error) == 'string' and #error > 0,
                'unknown/nil tokens require usage errors')
        end
        "#,
    );
}

#[test]
fn t7_font_v_invalid_errors_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFont('StageTextFont')

        object:SetJustifyH('RIGHT')
        object:SetJustifyV('BOTTOM')
        local calls = {
            function() object:SetJustifyV('DIAGONAL') end,
            function() object:SetJustifyV(nil) end,
            function() object:SetJustifyV() end,
            function() object:SetJustifyV(42) end,
            function() object:SetJustifyV({}) end,
        }
        for _, call in ipairs(calls) do
            local ok, error = pcall(call)
            assert(object:GetJustifyH() == 'RIGHT')
            assert(object:GetJustifyV() == 'BOTTOM')
            assert(not ok and type(error) == 'string' and #error > 0,
                'unknown/nil tokens require usage errors')
        end
        "#,
    );
}

#[test]
fn t8_fontstring_numeric_strings_and_clamps_replace_color() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        object:SetTextColor(0.25, 0.5, 0.75, 0.5)
        local result = capture(object:SetTextColor('-0.25', '0.5', '2', '0.25'))
        assert(result.n == 0)
        assert_color(object, 0, 0.5, 1, 0.25)
        object:SetTextColor(-2, 0.25, 3, -4)
        assert_color(object, 0, 0.25, 1, 0)
        object:SetTextColor(0, 0, 0, 4)
        assert_color(object, 0, 0, 0, 1)
        "#,
    );
}

#[test]
fn t8_fontstring_nonconvertible_rgb_zero_alpha_defaults_one() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        object:SetTextColor(0.25, 0.5, 0.75, 0.5)
        local result = capture(object:SetTextColor('not-a-number', {}, false, 'bad-alpha'))
        assert(result.n == 0)
        assert_color(object, 0, 0, 0, 1)
        assert(select('#', object:SetTextColor(nil, nil, nil, nil)) == 0)
        assert_color(object, 0, 0, 0, 1)
        assert(select('#', object:SetTextColor()) == 0)
        assert_color(object, 0, 0, 0, 1)
        "#,
    );
}

#[test]
fn t8_fontstring_valid_values_return_zero_and_alpha_resets() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateFontString()

        assert(select('#', object:SetTextColor(0.25, 0.5, 0.75, 0.5)) == 0)
        assert_color(object, 0.25, 0.5, 0.75, 0.5)
        assert(select('#', object:SetTextColor(0.75, 0.25, 0.5)) == 0)
        assert_color(object, 0.75, 0.25, 0.5, 1)
        "#,
    );
}

#[test]
fn t8_font_numeric_strings_and_clamps_replace_color() {
    assert_lua_contract(
        r#"
        local object = CreateFont('StageTextFont')

        object:SetTextColor(0.25, 0.5, 0.75, 0.5)
        local result = capture(object:SetTextColor('-0.25', '0.5', '2', '0.25'))
        assert(result.n == 0)
        assert_color(object, 0, 0.5, 1, 0.25)
        object:SetTextColor(-2, 0.25, 3, -4)
        assert_color(object, 0, 0.25, 1, 0)
        object:SetTextColor(0, 0, 0, 4)
        assert_color(object, 0, 0, 0, 1)
        "#,
    );
}

#[test]
fn t8_font_nonconvertible_rgb_zero_alpha_defaults_one() {
    assert_lua_contract(
        r#"
        local object = CreateFont('StageTextFont')

        object:SetTextColor(0.25, 0.5, 0.75, 0.5)
        local result = capture(object:SetTextColor('not-a-number', {}, false, 'bad-alpha'))
        assert(result.n == 0)
        assert_color(object, 0, 0, 0, 1)
        assert(select('#', object:SetTextColor(nil, nil, nil, nil)) == 0)
        assert_color(object, 0, 0, 0, 1)
        assert(select('#', object:SetTextColor()) == 0)
        assert_color(object, 0, 0, 0, 1)
        "#,
    );
}

#[test]
fn t8_font_valid_values_return_zero_and_alpha_resets() {
    assert_lua_contract(
        r#"
        local object = CreateFont('StageTextFont')

        assert(select('#', object:SetTextColor(0.25, 0.5, 0.75, 0.5)) == 0)
        assert_color(object, 0.25, 0.5, 0.75, 0.5)
        assert(select('#', object:SetTextColor(0.75, 0.25, 0.5)) == 0)
        assert_color(object, 0.75, 0.25, 0.5, 1)
        "#,
    );
}

#[test]
fn t2_accepted_blend_tokens_roundtrip_case_insensitively() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateTexture()

        for _, entry in ipairs({{'disable','DISABLE'}, {'aLpHaKeY','ALPHAKEY'},
                {'bLeNd','BLEND'}, {'add','ADD'}, {'mOd','MOD'}}) do
            object:SetBlendMode(entry[1])
            assert(select('#', object:GetBlendMode()) == 1)
            assert(object:GetBlendMode() == entry[2])
        end
        "#,
    );
}

#[test]
fn t2_invalid_blend_tokens_error_before_mutation() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateTexture()

        local calls = {
            function() object:SetBlendMode('SCREEN') end,
            function() object:SetBlendMode('') end,
            function() object:SetBlendMode(nil) end,
            function() object:SetBlendMode() end,
            function() object:SetBlendMode(42) end,
            function() object:SetBlendMode({}) end,
            function() object:SetBlendMode(false) end,
        }
        for _, call in ipairs(calls) do
            object:SetBlendMode('ADD')
            local ok, error = pcall(call)
            assert(object:GetBlendMode() == 'ADD', 'invalid input must retain stored token')
            assert(not ok and type(error) == 'string' and #error > 0)
        end
        "#,
    );
}

#[test]
fn t2_t7_existing_zero_setter_arity_preservation_not_native_oracle() {
    assert_lua_contract(
        r#"
        local object = CreateFrame('Frame'):CreateTexture()
        local text = CreateFrame('Frame'):CreateFontString()

        -- Source-based simulator preservation only: batch2 does not report these arities.
        assert(select('#', object:SetBlendMode('BLEND')) == 0)
        assert(select('#', text:SetJustifyH('RIGHT')) == 0)
        assert(select('#', text:SetJustifyV('BOTTOM')) == 0)
        "#,
    );
}
