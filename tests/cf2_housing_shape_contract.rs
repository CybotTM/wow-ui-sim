//! Shape-only contracts; native semantics unknown. Source predictions are not runtime proof.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]

#[path = "common/cf2_shape.rs"]
mod shape;

// retail: Blizzard_APIDocumentationGenerated/SuperTrackManagerDocumentation.lua:32 (GetSuperTrackedContent).
// retail: Blizzard_APIDocumentationGenerated/ContentTrackingTypesDocumentation.lua:60 (ContentTrackingType).
// ptr: Blizzard_APIDocumentationGenerated/SuperTrackManagerDocumentation.lua:32 (GetSuperTrackedContent).
// ptr: Blizzard_APIDocumentationGenerated/ContentTrackingTypesDocumentation.lua:60 (ContentTrackingType).
// wowforever: Blizzard_APIDocumentationGenerated/SuperTrackManagerDocumentation.lua:32 (GetSuperTrackedContent).
// wowforever: Blizzard_APIDocumentationGenerated/ContentTrackingTypesDocumentation.lua:60 (ContentTrackingType).
// Source-predicted FAIL: GetSuperTrackedContent missing; optional enum-number pair, no demanded content ID; not executed; native semantics unknown.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]
#[test]
fn original_supertracking_pair_shape_blocker() {
    let env = shape::load(&["Blizzard_FrameXMLUtil"]);
    shape::check(
        &env,
        r#"

local declared = {Appearance=0, Mount=1, Achievement=2, Decor=3}
enum(Enum.ContentTrackingType, declared)
original(QuestSuperTracking_GetSuperTrackedContent, 'Blizzard_FrameXMLUtil/Mainline/Blizzard_QuestSuperTracking.lua')
local result = pack(QuestSuperTracking_GetSuperTrackedContent())
assert(result.n == 0 or result.n == 2, 'documented optional pair, not one nil')
if result.n == 2 then
    assert(type(result[1]) == 'number' and type(result[2]) == 'number')
    member(result[1], declared)
end
local frame = EnumerateFrames()
local reached = false
while frame do
    if frame:IsEventRegistered('SUPER_TRACKING_CHANGED') then
        local handler = frame:GetScript('OnEvent')
        local info = handler and debug.getinfo(handler, 'S')
        if info and info.source:match('Blizzard_QuestSuperTracking.lua$') then
            original(handler, 'Blizzard_FrameXMLUtil/Mainline/Blizzard_QuestSuperTracking.lua')
            handler(frame, 'SUPER_TRACKING_CHANGED')
            reached = true
        end
    end
    frame = EnumerateFrames(frame)
end
assert(reached, 'normal original cache/listener prerequisite')

"#,
    );
}

// retail: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:307 (IsInsideHouse).
// retail: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:352 (IsInsidePlot).
// retail: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:325 (IsInsideOwnedHouse).
// retail: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:343 (IsInsideOwnedPlot).
// retail: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:316 (IsInsideHouseOrPlot).
// retail: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:334 (IsInsideOwnedHouseOrPlot).
// ptr: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:307 (IsInsideHouse).
// ptr: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:352 (IsInsidePlot).
// ptr: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:325 (IsInsideOwnedHouse).
// ptr: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:343 (IsInsideOwnedPlot).
// ptr: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:316 (IsInsideHouseOrPlot).
// ptr: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:334 (IsInsideOwnedHouseOrPlot).
// wowforever: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:307 (IsInsideHouse).
// wowforever: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:352 (IsInsidePlot).
// wowforever: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:325 (IsInsideOwnedHouse).
// wowforever: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:343 (IsInsideOwnedPlot).
// wowforever: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:316 (IsInsideHouseOrPlot).
// wowforever: Blizzard_APIDocumentationGenerated/HousingUIDocumentation.lua:334 (IsInsideOwnedHouseOrPlot).
// Source-predicted FAIL: IsInsideHouse missing; boolean predicates, original minimap accepts general interior result; not executed; native semantics unknown.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]
#[test]
fn original_minimap_housing_predicates_shape_blocker() {
    let env = shape::load(&["Blizzard_Minimap"]);
    shape::check(
        &env,
        r#"
scalar('boolean', C_Housing.IsInsideHouse())
scalar('boolean', C_Housing.IsInsidePlot())
scalar('boolean', C_Housing.IsInsideOwnedHouse())
scalar('boolean', C_Housing.IsInsideOwnedPlot())
scalar('boolean', C_Housing.IsInsideHouseOrPlot())
scalar('boolean', C_Housing.IsInsideOwnedHouseOrPlot())
original(MinimapMixin.UpdateStaticOverlayTexture, "Blizzard_Minimap/Mainline/Minimap.lua")
MinimapMixin.UpdateStaticOverlayTexture(Minimap)

"#,
    );
}
