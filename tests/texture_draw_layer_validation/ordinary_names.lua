local frame = CreateFrame("Frame", nil, UIParent)
local validLayers = { "BACKGROUND", "BORDER", "ARTWORK", "OVERLAY", "HIGHLIGHT" }

if drawLayerContractMode ~= "valid" and drawLayerContractMode ~= "invalid" then
    error("unknown texture draw-layer contract mode")
end

for _, layer in ipairs(validLayers) do
    local texture = frame:CreateTexture(nil, layer)
    assert(texture:GetDrawLayer() == layer, "valid draw layer must roundtrip: " .. layer)
end
assert(frame:GetNumRegions() == #validLayers, "five valid textures must add five regions")
print("DRAW_LAYER_VALID_BASELINE_COMPLETE")

if drawLayerContractMode == "invalid" then
    local regionsBefore = frame:GetNumRegions()
    local accepted = pcall(function()
        frame:CreateTexture(nil, "INVALID")
    end)
    local regionsAfter = frame:GetNumRegions()
    print("DRAW_LAYER_INVALID_OBSERVATION", accepted, regionsBefore, regionsAfter)
    assert(not accepted, "CreateTexture accepted invalid draw layer")
    assert(regionsAfter == regionsBefore, "invalid draw layer must not add a region")
    print("DRAW_LAYER_INVALID_REJECTION_COMPLETE")
end
print("TEXTURE_DRAW_LAYER_CONTRACT_COMPLETE", drawLayerContractMode)
