//! Shape-only contracts; native semantics unknown. Source predictions are not runtime proof.
#![cfg(any(feature = "client-wowforever"))]

#[path = "common/cf2_shape.rs"]
mod shape;

// C1 (retail/ptr) remains blocked: no normal current-entry ingest path is established.
// Blizzard_APIDocumentationGenerated/CombatLogSecureDocumentation.lua:102 declares
// ShouldShowCurrentEntry's boolean; :11 declares no AddEventFilter argument schema.
// A forwarding-only consumer call cannot claim the missing boolean shape coverage.

// wowforever: Blizzard_APIDocumentationGenerated/UnitAuraDocumentation.lua:377 (GetGroupBuffVisualAlerts).
// wowforever: Blizzard_APIDocumentationGenerated/UnitAuraDocumentation.lua:555 (SetGroupBuffVisualAlerts).
// wowforever: Blizzard_APIDocumentationGenerated/UnitConstantsDocumentation.lua:23 (GroupBuffVisualAlertInfo).
// wowforever: Blizzard_APIDocumentationGenerated/VisualAlertConstantsDocumentation.lua:6 (VisualAlertType).
// Source-predicted FAIL: Forever getter and setter missing; actual records only, exact enum, no required list contents; not executed; native semantics unknown.
#[cfg(any(feature = "client-wowforever"))]
#[test]
fn original_group_buff_records_shape_blocker() {
    let env = shape::load(&["Blizzard_FrameXMLUtil"]);
    shape::check(
        &env,
        r#"

local declared = {MarchingAnts=1, MarchingAntsCyan=2, MarchingAntsRed=3,
    MarchingAntsGreen=4, MarchingAntsBlue=5, Flash=6, FlashCyan=7,
    FlashRed=8, FlashGreen=9, FlashBlue=10}
enum(Enum.VisualAlertType, declared)
local alerts = scalar('table', C_UnitAuras.GetGroupBuffVisualAlerts())
for _, alert in ipairs(alerts) do
    fields(alert, {{'spellID','number',false}, {'visualValue','number',false}})
    member(alert.visualValue, declared)
end
original(GroupBuffMixin.RefreshGroupBuffVisualAlerts, 'Blizzard_FrameXMLUtil/AuraUtil.lua')
local manager = CreateFromMixins(GroupBuffMixin)
manager:RefreshGroupBuffVisualAlerts()
-- The setter receives only actual documented records. No setter/event timing assertion.
assert(select('#', C_UnitAuras.SetGroupBuffVisualAlerts(alerts)) == 0)
manager:RefreshGroupBuffVisualAlerts()

"#,
    );
}
