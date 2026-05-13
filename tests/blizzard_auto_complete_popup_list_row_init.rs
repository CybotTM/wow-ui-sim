use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const ROW_INIT_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListRowInitOwner", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  popup:SetWidth(222)

  local row = CreateFrame("Button", "TestPopupListRowInitButton", popup,
                          "AutoCompletePopupListResultTemplate")
  expect(row ~= nil, "AutoCompletePopupListResultTemplate must instantiate")

  if row ~= nil then
    row.HighlightTexture:Show()
    local resultInfo = { text = "Result" }
    row:Init({
      resultInfo = resultInfo,
      index = 2,
      owner = popup,
      displayText = "Hello",
      subtext = nil,
      displayTexture = nil,
    })

    expect(row:GetWidth() == popup:GetWidth(),
           "row width must match popup width")
    expect(row:GetIndex() == 2, "row index must be 2")
    expect(row:GetResultInfo() == resultInfo,
           "row resultInfo must be the original table")
    expect(not row.Icon:IsShown(), "row Icon must be hidden")
    expect(not row.IconFrame:IsShown(), "row IconFrame must be hidden")
    expect(row.Name:GetText() == "Hello",
           "row Name text must be Hello, got " .. tostring(row.Name:GetText()))
    expect(not row.Subtext:IsShown(), "row Subtext must be hidden")
    expect(row.Name:GetMaxLines() == 2,
           "row Name max lines must be 2, got " ..
           tostring(row.Name:GetMaxLines()))
    expect(not row.HighlightTexture:IsShown(),
           "row HighlightTexture must be hidden after Init")
  end

  local texturedRow = CreateFrame("Button", "TestPopupListRowInitTexturedButton",
                                 popup, "AutoCompletePopupListResultTemplate")
  expect(texturedRow ~= nil,
         "AutoCompletePopupListResultTemplate must instantiate textured row")

  if texturedRow ~= nil then
    local texturePath = "Interface\\Icons\\foo"
    texturedRow:Init({
      resultInfo = { text = "Textured" },
      index = 3,
      owner = popup,
      displayText = "Textured",
      subtext = "hint",
      displayTexture = texturePath,
    })

    expect(texturedRow.Icon:IsShown(), "textured row Icon must be shown")
    expect(texturedRow.IconFrame:IsShown(),
           "textured row IconFrame must be shown")
    expect(texturedRow.Icon:GetTexture() == texturePath,
           "textured row Icon texture must be " .. texturePath .. ", got " ..
           tostring(texturedRow.Icon:GetTexture()))
    expect(texturedRow.Subtext:IsShown(),
           "textured row Subtext must be shown")
    expect(texturedRow.Subtext:GetText() == "hint",
           "textured row Subtext text must be hint, got " ..
           tostring(texturedRow.Subtext:GetText()))
    expect(texturedRow.Name:GetMaxLines() == 1,
           "textured row Name max lines must be 1, got " ..
           tostring(texturedRow.Name:GetMaxLines()))
  end
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_row_init_without_icon_or_subtext() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList row Init can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ROW_INIT_PROBE_LUA)
                    .expect("AutoCompletePopupList row Init probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` row Init mismatches:\n{failures}"
                );
            });
        });
    });
}
