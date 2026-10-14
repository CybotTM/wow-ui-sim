craftedExpected = {
            quality = 3,
            icon = "crafted-icon",
            iconSmall = "crafted-small",
            iconInventory = "crafted-inventory",
            iconMixed = "crafted-mixed",
            iconAppear = "crafted-appear",
            iconDissolve = "crafted-dissolve",
            barFill = "crafted-fill",
            barBackground = "crafted-background",
            barBackgroundCap = "crafted-cap",
            barHighlight = "crafted-highlight",
            iconChat = "crafted-chat",
            iconQuestObjective = "crafted-quest",
        }
        function assertCraftedSnapshot(actual, expected)
            assert(type(actual) == 'table', 'supplied crafted item must return a table, not auto-nil')
            local count = 0
            for key, value in pairs(actual) do
                count = count + 1
                assert(expected[key] ~= nil, 'unexpected crafted field: ' .. tostring(key))
                assert(value == expected[key], 'wrong crafted field: ' .. tostring(key))
                assert(type(value) == type(expected[key]), 'wrong crafted field type: ' .. tostring(key))
            end
            assert(count == 13, 'crafted result must have exactly thirteen fields')
            for key, value in pairs(expected) do
                assert(actual[key] == value, 'missing/wrong crafted field: ' .. tostring(key))
            end
        end
