-- Direct original Blizzard consumer: no alternate filtering or dirty processing.
function FixtureCreateContainer(parent, unit, spec)
    local container = CreateFrame('AuraContainer', nil, parent, 'CustomAuraContainerTemplate')
    container:SetPoint(unpack(spec.point))
    container:SetSize(1, 1)
    for _, group in ipairs(spec.groups) do
        container:AddAuraGroup(group.key, table.concat(group.filter, '|'), {
            maxFrameCount = group.maxFrameCount,
            candidateFilters = group.candidateFilters,
            initializeFrame = function(button)
                button:SetSize(32, 32)
                local icon = button:CreateTexture(nil, 'ARTWORK')
                icon:SetAllPoints(button)
                local stack = button:CreateFontString(nil, 'OVERLAY')
                stack:SetFont(STANDARD_TEXT_FONT, 12, 'OUTLINE')
                stack:SetPoint('BOTTOMRIGHT', button, 'BOTTOMRIGHT')
                button:SetIcon(icon)
                button:SetApplicationCount(stack, {})
                group.extraInit(button, {icon = icon, stack = stack})
            end,
        })
    end
    container:SetUnit(unit)
    container:UpdateAllAuras()
    return container
end
