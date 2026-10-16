-- External feeds only; no replacement of original Blizzard functions.
WorldMapButton = CreateFrame('Button', 'WorldMapButton', UIParent)
WorldMapButton:SetSize(800, 600)
WorldMapDetailFrame = CreateFrame('Frame', 'WorldMapDetailFrame', WorldMapButton)
WorldMapZoomOutButton = CreateFrame('Button')
OutlandButton = CreateFrame('Button'); AzerothButton = CreateFrame('Button')
NUM_WORLDMAP_DETAIL_TILES = 0
NUM_WORLDMAP_POIS = 1
WorldMapFramePOI1 = CreateFrame('Button', 'WorldMapFramePOI1', WorldMapButton)
WorldMapFramePOI1:CreateTexture('WorldMapFramePOI1Texture')
function GetNumMapOverlays() return 0 end
function HasDebugZoneMap() return false end
function GetNumMapDebugObjects() return 0 end
