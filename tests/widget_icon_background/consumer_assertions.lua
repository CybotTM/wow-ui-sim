-- Observe unchanged cached Setup; no provider, atlas metadata or method replacement.
local function AssertIconBackground(frame, text, textureKit)
    assert(frame:IsVisible() and frame.Text:IsVisible(), 'frame/text visibility')
    assert(frame.Text:GetText() == text, 'supplied text')
    assert(frame.Text:GetStringWidth() > 1, 'nonempty text measurement')
    assert(frame.Icon:GetAtlas() == textureKit .. '-icon', 'Icon kit formatting')
    assert(frame.Glow:GetAtlas() == textureKit .. '-glow', 'Glow kit formatting')
    -- GetAtlas observes requested names even on a simulator lookup miss.
    -- No region visibility, asset existence, successful lookup or pixel claim.
    assert(frame:GetWidth() >= 100, 'XML minimumWidth through actual Layout')
    assert(frame:GetParent() == FixtureContainer, 'cached vertical layout parent')
end

function AssertIconBackgroundInitial(frame)
    AssertIconBackground(frame, 'Background text 731', 'iconbackground-fixture-731')
    assert(math.abs(frame:GetScale() - .9) < .0001, 'base initial scale')
    assert(frame.orderIndex == 37.5 and frame.layoutDirection == 1)
    assert(frame.widgetTag == 'iconbackground-contract-731')
end

function AssertIconBackgroundUpdated(frame, acquiredTag)
    AssertIconBackground(frame, 'Updated background 947', 'iconbackground-fixture-947')
    assert(frame:GetScale() == 1, 'base updated scale')
    assert(frame.orderIndex == -12.25 and frame.layoutDirection == 1)
    assert(frame.widgetTag == acquiredTag, 'cached acquisition tag, not latest record tag')
end
