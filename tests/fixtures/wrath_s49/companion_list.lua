-- A5 = laptop-labels-answers-20261018/laptop-answers-req-5.md.
-- Host catalog inputs are not native character defaults or mount eligibility IDs.
-- No producer/consumer replacements: these are the cache's original real frames.
function S49_AssertCompanionRows(expected)
    -- Native-fact A5:11: selected companion-array count; explicit host fixture size.
    assert(GetNumCompanions("MOUNT") == #expected, "S49 supplied companion count")
    for index, row in ipairs(expected) do
        local button = _G["CompanionButton" .. index]
        -- Cache-consumer fact A5:33 (cache Lua:385-394): reject nil/no-op info
        -- at the ORIGINAL real button before probing primitive tuple contracts.
        assert(button:IsEnabled(), "S49 real populated button enabled")
        assert(button.spellID == row[1], "S49 independent original button spell")
        -- Native-fact A5:13-21: five slots, including nil slots, and positional types.
        assert(select("#", GetCompanionInfo("MOUNT", index)) == 5, "S49 tuple arity")
        local creature, name, spell, icon, active = GetCompanionInfo("MOUNT", index)
        assert(type(creature) == "number", "S49 creature identity is numeric")
        -- Native-fact A5:15-19,26: resolved names sort case-insensitively.
        -- Names/spells are explicit host inputs, not a native learned-mount census.
        assert(name == row[2] and spell == row[1], "S49 sorted name/spell projection")
        assert(icon == nil or type(icon) == "string", "S49 optional texture path")
        assert(active == nil or active == 1, "S49 active is numeric one or nil")
        -- Cache-consumer fact A5:33 (cache Lua:382-405): five-slot projection.
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
    -- Single-category setup: no pet UI or eligible critters are supplied.
    assert(not HasPetUI(), "S49 no pet category supplied")
    assert(GetNumCompanions("CRITTER") == 0, "S49 no critter category supplied")
    -- Cached PetPaperDollFrame.lua:73-104 counts categories, not mounts.
    -- Lines 123-127 hide ALL subtabs when only the mount category is available;
    -- lines 84-104 also hide each unavailable category when the list is empty.
    for index = 1, 3 do
        assert(not _G["PetPaperDollFrameTab" .. index]:IsShown(), "S49 single-category subtabs hidden")
    end
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
