//! Simulator-defined enUS school-mask labels, not a native catalog/order baseline.
#![cfg(feature = "client-retail")]

use crate::common;

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn adventures_combat_log_colors_bootstrap_with_mists_row_rgb_palette() {
    let env = WowLuaEnv::new().expect("create combat-log color bootstrap environment");
    env.exec(
        r#"
        local expected = {
            ADVENTURES_COMBAT_LOG_GREY = {185 / 255, 185 / 255, 185 / 255},
            ADVENTURES_COMBAT_LOG_BLUE = {78 / 255, 173 / 255, 215 / 255},
            ADVENTURES_COMBAT_LOG_ORANGE = {198 / 255, 100 / 255, 75 / 255},
            ADVENTURES_COMBAT_LOG_YELLOW = {1, 210 / 255, 0},
        }
        for name, channels in pairs(expected) do
            local color = _G[name]
            assert(color ~= nil, name .. ' must exist in a fresh environment')
            assert(select('#', color:GetRGB()) == 3,
                name .. ':GetRGB() must return exactly three channels')
            local actual = {color:GetRGB()}
            for index = 1, 3 do
                assert(type(actual[index]) == 'number', name .. ' channel must be numeric')
                assert(math.abs(actual[index] - channels[index]) < 1e-12,
                    name .. ' channel mismatch at ' .. index)
            end
        end
        "#,
    )
    .expect("fresh bootstrap exposes combat-log colors with the simulator Mists row RGB palette");
}

#[test]
fn garrison_damage_classes_returns_one_table_with_en_us_pure_school_masks() {
    let env = WowLuaEnv::new().expect("create damage-class query environment");
    env.exec(
        r#"
        assert(select('#', C_Garrison.GetAutoCombatDamageClassValues()) == 1,
            'damage-class query must return exactly one value')
        local rows = C_Garrison.GetAutoCombatDamageClassValues()
        assert(type(rows) == 'table', 'damage-class query must return a table')

        -- These labels are simulator enUS policy, not claims of native parity.
        local expected = {
            [1] = 'Physical', [2] = 'Holy', [4] = 'Fire', [8] = 'Nature',
            [16] = 'Frost', [32] = 'Shadow', [64] = 'Arcane',
        }
        local labels = {}
        for _, row in ipairs(rows) do
            assert(type(row) == 'table', 'damage-class entries must be tables')
            assert(type(row.damageClassValue) == 'number', 'school mask must be numeric')
            assert(type(row.locString) == 'string' and row.locString:match('%S'),
                'school label must be a nonempty readable string')
            assert(labels[row.damageClassValue] == nil, 'school masks must be unambiguous')
            labels[row.damageClassValue] = row.locString
        end
        for mask, label in pairs(expected) do
            assert(labels[mask] == label,
                'simulator enUS school label mismatch for mask ' .. mask)
        end
        "#,
    )
    .expect("damage-class query exposes readable pure-school mask labels");
}

#[test]
fn garrison_damage_classes_returns_fresh_tables_and_rows_without_mutation_leaks() {
    let env = WowLuaEnv::new().expect("create damage-class isolation environment");
    env.exec(
        r#"
        local first = C_Garrison.GetAutoCombatDamageClassValues()
        local second = C_Garrison.GetAutoCombatDamageClassValues()
        assert(type(first) == 'table' and type(second) == 'table')
        assert(first ~= second, 'each query must return a fresh outer table')
        assert(#first > 0 and #first == #second, 'catalog must be nonempty and stable')

        local originalLabels = {}
        local firstRows = {}
        for _, row in ipairs(first) do
            assert(type(row) == 'table')
            assert(not firstRows[row], 'rows within a result must be independent')
            firstRows[row] = true
            originalLabels[row.damageClassValue] = row.locString
        end
        local secondRows = {}
        for _, row in ipairs(second) do
            assert(type(row) == 'table')
            assert(not firstRows[row], 'queries must not share row tables')
            assert(not secondRows[row], 'rows within a result must be independent')
            secondRows[row] = true
            assert(row.locString == originalLabels[row.damageClassValue])
        end

        for index, row in ipairs(first) do
            row.damageClassValue = -index
            row.locString = 'mutated first result'
            row.extra = true
        end
        for index = #first, 1, -1 do
            first[index] = nil
        end
        for _, row in ipairs(second) do
            assert(row.locString == originalLabels[row.damageClassValue],
                'mutating first result must not change an existing second result')
            assert(row.extra == nil)
            row.locString = 'mutated second result'
        end
        second.extra = true

        local third = C_Garrison.GetAutoCombatDamageClassValues()
        assert(third ~= first and third ~= second and third.extra == nil)
        assert(#third == #second, 'outer table mutation must not change later catalogs')
        local remaining = {}
        for mask, label in pairs(originalLabels) do
            remaining[mask] = label
        end
        for _, row in ipairs(third) do
            assert(not firstRows[row] and not secondRows[row],
                'later queries must also allocate fresh rows')
            assert(row.locString == remaining[row.damageClassValue],
                'later query must preserve original masks and labels')
            assert(row.extra == nil)
            remaining[row.damageClassValue] = nil
        end
        assert(next(remaining) == nil, 'later query must preserve every original school')
        "#,
    )
    .expect("damage-class result mutations remain local to each call");
}

#[test]
fn garrison_adventures_combat_log_consumes_fire_school_mask_not_damage_ordinal() {
    test_timeout! {
        common::blizzard_addon_harness::with_blizzard_addon_closure(
            &["Blizzard_GarrisonUI"],
            &[],
            |env, _loaded| {
                env.exec(
                    r#"
                    assert(type(AdventuresCombatLogMixin) == 'table',
                        'authentic cached combat-log mixin must load')
                    local messages = {}
                    local function ignoreUiSetting() end
                    local messageFrame = {
                        SetMaxLines = ignoreUiSetting,
                        SetFading = ignoreUiSetting,
                        SetFontObject = ignoreUiSetting,
                        SetIndentedWordWrap = ignoreUiSetting,
                        SetJustifyH = ignoreUiSetting,
                        AddMessage = function(self, text)
                            messages[#messages + 1] = text
                        end,
                    }
                    -- Only UI wiring and spell input are collaborators. The tested
                    -- damage-class query and authentic consumer methods stay intact.
                    ScrollUtil.InitScrollingMessageFrameWithScrollBar = ignoreUiSetting
                    local actors = {
                        [Enum.GarrAutoBoardIndex.AllyLeftBack] = {
                            GetName = function() return 'Caster' end,
                        },
                        [Enum.GarrAutoBoardIndex.AllyRightBack] = {
                            GetName = function() return 'Target' end,
                        },
                    }
                    local completeScreen = {
                        GetFrameFromBoardIndex = function(self, index)
                            return actors[index]
                        end,
                    }
                    local log = setmetatable({
                        CombatLogMessageFrame = messageFrame,
                        ScrollBar = {},
                        GetParent = function() return completeScreen end,
                    }, { __index = AdventuresCombatLogMixin })
                    log:OnLoad()
                    assert(log.damageTypeKeys[4] == 'Fire', 'Fire must be keyed by mask 4')
                    assert(log.damageTypeKeys[2] == 'Holy', 'mask 2 is Holy, not ordinal Fire')

                    local fireSpellId, holySpellId = 900001, 900002
                    C_Garrison.GetCombatLogSpellInfo = function(spellId)
                        assert(spellId == fireSpellId or spellId == holySpellId)
                        return { name = 'Fixture Spell', schoolMask = spellId == fireSpellId and 4 or 2 }
                    end
                    local function addSpellEvent(spellId)
                        log:AddCombatEvent({
                            spellID = spellId,
                            type = Enum.GarrAutoMissionEventType.SpellRangeDamage,
                            casterBoardIndex = Enum.GarrAutoBoardIndex.AllyLeftBack,
                            targetInfo = {
                                { boardIndex = Enum.GarrAutoBoardIndex.AllyRightBack, points = 37 },
                            },
                        })
                    end
                    addSpellEvent(fireSpellId)
                    addSpellEvent(holySpellId)
                    assert(#messages == 2, 'each spell event must produce one combat-log entry')
                    local format = COVENANT_MISSIONS_COMBAT_LOG_SPELL_RANGE_DAMAGE
                    assert(messages[1] == format:format('Caster', 'Fixture Spell', 'Target', 37, 'Fire'),
                        'authentic AddCombatEvent must render mask 4 as Fire')
                    assert(messages[2] == format:format('Caster', 'Fixture Spell', 'Target', 37, 'Holy'),
                        'authentic AddCombatEvent must not confuse mask 2 with ordinal Fire')
                    assert(messages[1] ~= messages[2], 'school labels must distinguish combat entries')
                    "#,
                )
                .expect("authentic combat-log OnLoad and AddCombatEvent consume school-mask labels");
            },
        );
    }
}
