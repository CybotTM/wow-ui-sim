local parent = CreateFrame("Frame", nil, UIParent)
parent:SetAlpha(0.5)
local frame = CreateFrame("Frame", nil, parent)
local regions = { frame, frame:CreateTexture(), frame:CreateFontString() }

if alphaContractMode == "valid" then
    for _, region in ipairs(regions) do
        for _, alpha in ipairs({ 0, 0.5, 1 }) do
            assert(select("#", region:SetAlpha(alpha)) == 0, "SetAlpha returns no values")
            assert(region:GetAlpha() == alpha, "valid alpha must roundtrip")
        end
    end
elseif alphaContractMode == "invalid" then
    local nearBoundary = 2 ^ -30
    local invalid = { -0.25, 1.25, -nearBoundary, 1 + nearBoundary, -math.huge, math.huge, 0 / 0 }
    for _, region in ipairs(regions) do
        region:SetAlpha(0.625)
        local effectiveBefore = region:GetEffectiveAlpha()
        for _, alpha in ipairs(invalid) do
            local accepted = pcall(function() region:SetAlpha(alpha) end)
            assert(not accepted, "SetAlpha must reject out-of-range numeric alpha: " .. tostring(alpha))
            assert(region:GetAlpha() == 0.625, "rejection must preserve stored alpha")
            assert(region:GetEffectiveAlpha() == effectiveBefore, "rejection must preserve inherited alpha")
        end
    end
else
    error("unknown alpha contract mode")
end
print("REGION_ALPHA_CONTRACT_COMPLETE", alphaContractMode)
