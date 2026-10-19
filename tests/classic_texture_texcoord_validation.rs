//! Staged, unexecuted behavioural contracts: Wrath 12340 only; Cataclysm 15595 corroborates, Mists/Retail unknown.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn check(body: &str) {
    let env = WowLuaEnv::new().expect("create minimal environment");
    env.exec(FIXTURE).expect("install fixture");
    env.exec(body).expect("native behavioural contract");
}

const FIXTURE: &str = r#"
local parent = CreateFrame("Frame", nil, UIParent)
tex = parent:CreateTexture()
function same_uv(expected)
    local actual = {tex:GetTexCoord()}
    assert(#actual == 8)
    for i = 1, 8 do
        if expected[i] ~= expected[i] then
            assert(actual[i] ~= actual[i], 'NaN storage at ' .. i)
        else
            assert(actual[i] == expected[i], 'UV mismatch at ' .. i)
        end
    end
end
function seed_uv()
    tex:SetTexCoord(0.125, 0.875, 0.25, 0.75)
end
function retained_uv()
    same_uv({0.125,0.25, 0.125,0.75, 0.875,0.25, 0.875,0.75})
end
"#;

#[test]
fn t1_rejects_wrong_arity_without_mutation() {
    check(
        r#"
        for _, n in ipairs({0,1,2,3,5,6,7,9,10}) do
            seed_uv()
            local args = {}
            for i = 1, n do args[i] = 0.5 end
            local ok, err = pcall(function() tex:SetTexCoord(unpack(args, 1, n)) end)
            assert(not ok and type(err) == 'string', 'must reject arity ' .. n)
            retained_uv()
        end
        seed_uv()
        assert(not pcall(function() tex:SetTexCoord(0,1,0,1,nil) end))
        retained_uv()
        "#,
    );
}

#[test]
fn t1_numeric_strings_convert_in_both_forms() {
    check(
        r#"
        assert(select('#', tex:SetTexCoord('0.125','0.875','0.25','0.75')) == 0)
        retained_uv()
        assert(select('#', tex:SetTexCoord('0','0.125','0.25','0.375','0.5','0.625','0.75','0.875')) == 0)
        same_uv({0,0.125,0.25,0.375,0.5,0.625,0.75,0.875})
        "#,
    );
}

#[test]
fn t1_non_numeric_values_convert_to_zero_without_compacting() {
    check(
        r#"
        local values = {'not-a-number', false, true, {}, nil}
        for i = 1, 5 do
            local value = values[i]
            seed_uv()
            assert(select('#', tex:SetTexCoord(value, 0.75, 0.25, 1)) == 0)
            same_uv({0,0.25,0,1,0.75,0.25,0.75,1})
            seed_uv()
            assert(select('#', tex:SetTexCoord(0,0.125,0.25,0.375,0.5,0.625,0.75,value)) == 0)
            same_uv({0,0.125,0.25,0.375,0.5,0.625,0.75,0})
        end
        "#,
    );
}

#[test]
fn t1_bounds_and_infinity_reject_atomically_in_every_slot() {
    check(
        r#"
        for _, n in ipairs({4,8}) do
            for slot = 1, n do
                for _, bad in ipairs({10001,-10001,math.huge,-math.huge}) do
                    seed_uv()
                    local args = {}
                    for i = 1, n do args[i] = 0.5 end
                    args[slot] = bad
                    local ok, err = pcall(function() tex:SetTexCoord(unpack(args,1,n)) end)
                    assert(not ok and type(err) == 'string', 'must reject slot ' .. slot)
                    retained_uv()
                end
            end
        end
        "#,
    );
}

#[test]
fn t1_inclusive_bounds_and_corner_order_return_no_values() {
    check(
        r#"
        assert(select('#', tex:SetTexCoord(-10000,10000,10000,-10000)) == 0)
        same_uv({-10000,10000,-10000,-10000,10000,10000,10000,-10000})
        assert(select('#', tex:SetTexCoord(-10000,10000,1,2,3,4,10000,-10000)) == 0)
        same_uv({-10000,10000,1,2,3,4,10000,-10000})
        "#,
    );
}

#[test]
fn t1_nan_storage_only_no_layout_or_render() {
    check(
        r#"
        local nan = 0/0
        assert(nan ~= nan)
        assert(select('#', tex:SetTexCoord(nan,1,0,1)) == 0)
        same_uv({nan,0,nan,1,1,0,1,1})
        assert(select('#', tex:SetTexCoord(0,0,nan,1,1,0,1,1)) == 0)
        same_uv({0,0,nan,1,1,0,1,1})
        "#,
    );
}

#[test]
fn t1_rejection_retains_existing_quad() {
    check(
        r#"
        local before = {0,0.125,0.25,0.375,0.5,0.625,0.75,0.875}
        for _, args in ipairs({{0,1,0,10001}, {0,1,0}, {0,0,0,0,0,0,0,math.huge}}) do
            tex:SetTexCoord(unpack(before))
            assert(not pcall(function() tex:SetTexCoord(unpack(args)) end))
            same_uv(before)
        end
        "#,
    );
}
