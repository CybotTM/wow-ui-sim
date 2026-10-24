-- A5 = laptop-labels-answers-20261018/laptop-answers-req-5.md.
-- Host catalog inputs are not native character defaults or mount eligibility IDs.
-- No producer/consumer replacements: these are the cache's original real frames.
function S49_AssertCompanionRows(expected)
    -- Native-fact A5:11: selected companion-array count; explicit host fixture size.
    assert(GetNumCompanions("MOUNT") == #expected, "S49 supplied companion count")
    for index, row in ipairs(expected) do
        -- Native-fact A5:13-21: five slots, including nil slots, and positional types.
        assert(select("#", GetCompanionInfo("MOUNT", index)) == 5, "S49 tuple arity")
        local creature, name, spell, icon, active = GetCompanionInfo("MOUNT", index)
        assert(type(creature) == "number", "S49 creature identity is numeric")
        -- Native-fact A5:15-19,26: resolved names sort case-insensitively.
        -- Names/spells are explicit host inputs, not a native learned-mount census.
        assert(name == row[2] and spell == row[1], "S49 sorted name/spell projection")
        assert(icon == nil or type(icon) == "string", "S49 optional texture path")
        assert(active == nil or active == 1, "S49 active is numeric one or nil")
        local button = _G["CompanionButton" .. index]
        -- Cache-consumer fact A5:33 (cache Lua:382-405): five-slot projection.
        assert(button:IsEnabled(), "S49 real populated button enabled")
        assert(button.creatureID == creature and button.spellID == row[1], "S49 button identities")
        assert(button.active == active, "S49 button active projection")
        assert(_G[button:GetName() .. "ActiveTexture"]:IsShown() == (active == 1), "S49 active indicator")
    end
    -- Native-fact A5:13,21: first out-of-range index returns zero values, not nil.
    assert(select("#", GetCompanionInfo("MOUNT", #expected + 1)) == 0, "S49 out-of-range arity")
    for index = #expected + 1, NUM_COMPANIONS_PER_PAGE do
        local button = _G["CompanionButton" .. index]
        -- Cache-consumer fact A5:33 (cache Lua:385-394): absent row clears/disables.
        assert(not button:IsEnabled(), "S49 unused button disabled")
        assert(button.creatureID == nil and button.spellID == nil, "S49 unused button cleared")
    end
    -- Cache-consumer fact A5:40 (cache Lua:98-104): tab follows count.
    assert(PetPaperDollFrameTab3:IsShown() == (#expected > 0), "S49 mount tab visibility")
end

-- Cache-consumer fact A5:33: selected PetPaperDollFrame functions on loaded frames.
ToggleCharacter("PetPaperDollFrame")
PetPaperDollFrame_SetTab(3)
assert(CharacterFrame:IsShown() and PetPaperDollFrameCompanionFrame:IsShown(), "S49 original panels shown")
assert(PetPaperDollFrameCompanionFrame.mode == "MOUNT", "S49 original mount mode")
S49_AssertCompanionRows({{S49_FIRST_SPELL, "Zulu S49"}})
-- Cache-consumer A5:33 + native-fact A5:35: assertions after fire_event in Rust
-- inspect original handlers' effects without a tick. Explicit delivery proves the
-- dispatcher/consumer boundary ONLY, not native learning or cache callback emission.
