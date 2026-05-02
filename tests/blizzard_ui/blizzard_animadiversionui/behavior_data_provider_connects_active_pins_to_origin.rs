//! `AnimaDiversionDataProviderMixin:RefreshAllData` connection probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const ACTIVE_PIN_CONNECTION_PROBE: &str = r#"
local originalGetProgress = C_AnimaDiversion.GetReinforceProgress
local originalGetTextureKit = C_AnimaDiversion.GetTextureKit
local originalGetOriginPosition = C_AnimaDiversion.GetOriginPosition
local originalGetNodes = C_AnimaDiversion.GetAnimaDiversionNodes
local originalGetTalentUnlockWorldQuest = C_Garrison.GetTalentUnlockWorldQuest
local state = Enum.AnimaDiversionNodeState
local acquireCount = 0
local connectionSetupCount = 0
local connectedPinMatched = false
local connectedPin = nil

C_AnimaDiversion.GetReinforceProgress = function()
    return 0
end
C_AnimaDiversion.GetTextureKit = function()
    return "Kyrian"
end
C_AnimaDiversion.GetOriginPosition = function()
    return { x = 0.4, y = 0.6 }
end
C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return {
        {
            talentID = 701,
            name = "Connected",
            state = state.SelectedTemporary,
            normalizedPosition = { x = 0.2, y = 0.3 },
        },
        {
            talentID = 702,
            name = "Cooling",
            state = state.Cooldown,
            normalizedPosition = { x = 0.7, y = 0.8 },
        },
    }
end
C_Garrison.GetTalentUnlockWorldQuest = function()
    return nil
end

local function buildTexture()
    return {
        shown = true,
        GetObjectType = function()
            return "Texture"
        end,
        SetAtlas = function()
            return true
        end,
        SetDesaturated = function() end,
        Hide = function(self)
            self.shown = false
        end,
        Show = function(self)
            self.shown = true
        end,
        IsShown = function(self)
            return self.shown
        end,
    }
end

local function buildPin()
    local pin = {
        Icon = buildTexture(),
        IconBorder = buildTexture(),
        IconDisabledOverlay = buildTexture(),
        SetPosition = function(self, x, y)
            self.x = x
            self.y = y
        end,
        SetSize = function(self, width, height)
            self.width = width
            self.height = height
        end,
        Show = function(self)
            self.shown = true
        end,
    }
    setmetatable(pin, { __index = AnimaDiversionPinMixin })
    return pin
end

local acquiredPins = {}
local map = {
    RemoveAllPinsByTemplate = function() end,
    GetCanvas = function()
        return UIParent
    end,
    AcquirePin = function(self, template)
        local pin = buildPin()
        pin.template = template
        table.insert(acquiredPins, pin)
        return pin
    end,
}
local provider = {
    GetMap = function()
        return map
    end,
    AddModelScene = function() end,
    ResetModelScene = function(self)
        self.pinEffects = {}
    end,
    connectionPool = {
        ReleaseAll = function() end,
        Acquire = function()
            acquireCount = acquireCount + 1
            return {
                Setup = function(self, textureKit, origin, pin)
                    connectionSetupCount = connectionSetupCount + 1
                    connectedPinMatched = pin == connectedPin
                    self.textureKit = textureKit
                    self.origin = origin
                    self.pin = pin
                end,
                Show = function(self)
                    self.shown = true
                end,
            }
        end,
    },
}
setmetatable(provider, { __index = AnimaDiversionDataProviderMixin })

provider.SetupConnectionOnPin = function(self, pin)
    connectedPin = pin
    return AnimaDiversionDataProviderMixin.SetupConnectionOnPin(self, pin)
end

provider:RefreshAllData()

local originBorderShown = provider.origin.IconBorder:IsShown()
local firstNodeConnected = acquiredPins[2]:IsConnected()
local secondNodeConnected = acquiredPins[3]:IsConnected()

C_AnimaDiversion.GetReinforceProgress = originalGetProgress
C_AnimaDiversion.GetTextureKit = originalGetTextureKit
C_AnimaDiversion.GetOriginPosition = originalGetOriginPosition
C_AnimaDiversion.GetAnimaDiversionNodes = originalGetNodes
C_Garrison.GetTalentUnlockWorldQuest = originalGetTalentUnlockWorldQuest

return #acquiredPins,
       acquireCount,
       connectionSetupCount,
       connectedPinMatched,
       firstNodeConnected,
       secondNodeConnected,
       originBorderShown,
       provider.origin.textureKit == "Kyrian",
       acquiredPins[2].nodeData.name == "Connected"
"#;

#[test]
fn refresh_all_data_connects_active_pins_to_origin() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: ActivePinConnectionState = env
            .eval(ACTIVE_PIN_CONNECTION_PROBE)
            .expect("active pin connection probe must run cleanly");

        assert_active_pin_connection_state(state);
    });
}

type ActivePinConnectionState = (i64, i64, i64, bool, bool, bool, bool, bool, bool);

fn assert_active_pin_connection_state(state: ActivePinConnectionState) {
    assert_pin_iteration((state.0, state.4, state.5, state.7, state.8));
    assert_connection((state.1, state.2, state.3));
    assert!(
        state.6,
        "`SetupConnectionOnPin` must show the origin icon border"
    );
}

fn assert_pin_iteration(state: (i64, bool, bool, bool, bool)) {
    let (pin_count, first_node_connected, second_node_connected, origin_kit_matches, node_matches) =
        state;

    assert_eq!(
        pin_count, 3,
        "Refresh must acquire origin plus two node pins"
    );
    assert!(
        first_node_connected,
        "SelectedTemporary node must be connected"
    );
    assert!(
        !second_node_connected,
        "Cooldown node must not be connected"
    );
    assert!(
        origin_kit_matches,
        "Origin pin must receive the texture kit"
    );
    assert!(
        node_matches,
        "First node pin must receive the connected node data"
    );
}

fn assert_connection(state: (i64, i64, bool)) {
    let (acquire_count, setup_count, connected_pin_matched) = state;

    assert_eq!(
        acquire_count, 1,
        "Only the connected node should acquire a connection"
    );
    assert_eq!(
        setup_count, 1,
        "Only the connected node should set up a connection"
    );
    assert!(
        connected_pin_matched,
        "Connection setup must receive the connected pin"
    );
}
