-- Only acceptance inputs and observations; all region wiring is real AuraKit.
local regions = {}
local function create(candidateFilters)
    local host = CreateFrame('Frame', nil, UIParent)
    host:SetSize(64, 64)
    host:SetPoint('CENTER', UIParent, 'CENTER', 0, 0)
    return FixtureCreateContainer(host, 'player', {
        point = {'CENTER', host, 'CENTER'},
        groups = {{
            key = 'current-package-helpful', filter = {'HELPFUL'},
            maxFrameCount = 1, candidateFilters = candidateFilters,
            style = 'current-package-default',
            extraInit = function(button, data)
                regions[button] = data
            end,
        }},
    })
end
debug.setobjecttaint(create, 'EllesmereFixture')
FixtureArray = create({includeSpellIDs = {19750}})
FixtureKeyed = create({includeSpellIDs = {[19750] = true}})

-- Return identity only. Reading addon-written regions taints this call; widget
-- readouts belong to the Rust observer, like native tree/render inspection.
function FixtureWidgets(container)
    local widgets = {}
    for index = 1, container:GetAuraGroupFrameCount('current-package-helpful') do
        local button = container:GetAuraGroupFrame('current-package-helpful', index)
        local data = assert(regions[button], 'real initializer regions missing')
        widgets[index] = {button, data.icon, data.stack}
    end
    return widgets
end
