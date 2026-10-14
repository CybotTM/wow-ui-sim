-- Observational assertions only. No providers, vendor methods, or handlers replaced.
function AssertSpacerFrame(frame, width, height, acquiredTag)
    assert(frame:IsVisible(), 'spacer visibility')
    assert(frame.widgetID == 762201 and frame.widgetSetID == 422 and frame.widgetType == 22)
    assert(math.abs(frame:GetWidth() - width) < .0001, 'supplied/clamped width')
    assert(math.abs(frame:GetHeight() - height) < .0001, 'supplied/clamped height')
    assert(frame:GetWidth() >= 1 and frame:GetHeight() >= 1)
    assert(math.abs(frame:GetScale() - .9) < .0001)
    assert(frame.widgetTag == acquiredTag, 'tag belongs to acquisition')
    assert(frame.orderIndex == 37.5 and frame.layoutDirection == 1)
    assert(frame.inAnimType == 0 and frame.outAnimType == 0)
    assert(frame.modelSceneLayer == 0 and frame.effectController == nil)
    assert(frame.hasTimer == false)
    assert(FixtureContainer.numTimers == 0 and FixtureContainer.ticker == nil)
end

function AssertSpacerTags(frame, activeTag, absentTag)
    local active, absent = {}, {}
    FixtureContainer:GatherWidgetsByWidgetTag(active, activeTag)
    FixtureContainer:GatherWidgetsByWidgetTag(absent, absentTag)
    assert(#active == 1 and active[1] == frame)
    assert(#absent == 0)
end

function AssertUpdatedSpacerSnapshot()
    local current = FixtureGetter(762201)
    assert(current.widgetWidth == 206.25 and current.widgetHeight == 73.75)
    assert(current.widgetTag == 'updated-spacer-tag')
    AssertSuppliedSnapshot(SavedSpacerSnapshot)
end
