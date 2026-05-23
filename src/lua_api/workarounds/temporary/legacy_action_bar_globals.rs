//! Temporary legacy action-bar global wrappers.
//!
//! `C_ActionBar` is registered by the Rust action-bar API. These globals remain
//! for older Blizzard/addon callers until deprecated global action-bar access is
//! no longer required.

const LEGACY_ACTION_BAR_GLOBALS_LUA: &str = r#"
if ActionButtonUtil == nil then
  ActionButtonUtil = {}
end

local function __wow_legacy_action_bar_noop()
end

ActionButtonUtil.ActionBarActionStatus = ActionButtonUtil.ActionBarActionStatus or {
  NotMissing = 1,
  MissingFromAllBars = 2,
  OnInactiveBonusBar = 3,
  OnDisabledActionBar = 4,
}

ActionButtonUtil.ActionBarButtonNames = ActionButtonUtil.ActionBarButtonNames or {}

if ActionButtonUtil.ShowAllActionButtonGrids == nil then
  ActionButtonUtil.ShowAllActionButtonGrids = __wow_legacy_action_bar_noop
end

if ActionButtonUtil.HideAllActionButtonGrids == nil then
  ActionButtonUtil.HideAllActionButtonGrids = __wow_legacy_action_bar_noop
end

if ActionButtonUtil.SetAllQuickKeybindButtonHighlights == nil then
  ActionButtonUtil.SetAllQuickKeybindButtonHighlights = __wow_legacy_action_bar_noop
end

if ActionButtonUtil.ShowAllQuickKeybindButtonHighlights == nil then
  ActionButtonUtil.ShowAllQuickKeybindButtonHighlights = __wow_legacy_action_bar_noop
end

if ActionButtonUtil.HideAllQuickKeybindButtonHighlights == nil then
  ActionButtonUtil.HideAllQuickKeybindButtonHighlights = __wow_legacy_action_bar_noop
end

if ActionButtonUtil.GetActionBarStatusForSpell == nil then
  function ActionButtonUtil.GetActionBarStatusForSpell(_spellID, _excludeNonPlayerBars, _excludeSpecialPlayerBars)
    return ActionButtonUtil.ActionBarActionStatus.NotMissing
  end
end

if ActionButtonUtil.GetActionBarStatusForPetAction == nil then
  function ActionButtonUtil.GetActionBarStatusForPetAction(_petActionID)
    return ActionButtonUtil.ActionBarActionStatus.NotMissing
  end
end

if ActionButtonUtil.GetActionBarStatusForFlyout == nil then
  function ActionButtonUtil.GetActionBarStatusForFlyout(_flyoutActionID)
    return ActionButtonUtil.ActionBarActionStatus.NotMissing
  end
end

local function __wow_legacy_action_bar_forward(globalName, methodName)
    if _G[globalName] == nil and C_ActionBar ~= nil and type(C_ActionBar[methodName]) == "function" then
        _G[globalName] = function(...)
            return C_ActionBar[methodName](...)
        end
    end
end

__wow_legacy_action_bar_forward("GetBonusBarIndex", "GetBonusBarIndex")
__wow_legacy_action_bar_forward("GetBonusBarOffset", "GetBonusBarOffset")
__wow_legacy_action_bar_forward("GetExtraBarIndex", "GetExtraBarIndex")
__wow_legacy_action_bar_forward("GetMultiCastBarIndex", "GetMultiCastBarIndex")
__wow_legacy_action_bar_forward("GetOverrideBarIndex", "GetOverrideBarIndex")
__wow_legacy_action_bar_forward("GetOverrideBarSkin", "GetOverrideBarSkin")
__wow_legacy_action_bar_forward("GetTempShapeshiftBarIndex", "GetTempShapeshiftBarIndex")
__wow_legacy_action_bar_forward("GetVehicleBarIndex", "GetVehicleBarIndex")
__wow_legacy_action_bar_forward("GetActionBarPage", "GetActionBarPage")
__wow_legacy_action_bar_forward("ChangeActionBarPage", "SetActionBarPage")
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(LEGACY_ACTION_BAR_GLOBALS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn keeps_legacy_bonus_bar_offset_global_backed_by_c_action_bar() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local original = C_ActionBar.GetBonusBarIndex
                C_ActionBar.GetBonusBarIndex = function() return 11 end
                local namespaceOffset = C_ActionBar.GetBonusBarOffset()
                local globalOffset = GetBonusBarOffset()
                C_ActionBar.GetBonusBarIndex = original
                if namespaceOffset ~= 5 or globalOffset ~= 5 then return "offset" end
                if GetBonusBarIndex() ~= C_ActionBar.GetBonusBarIndex() then return "bonus" end
                if GetExtraBarIndex() ~= C_ActionBar.GetExtraBarIndex() then return "extra" end
                if GetMultiCastBarIndex() ~= C_ActionBar.GetMultiCastBarIndex() then return "multicast" end
                if GetOverrideBarIndex() ~= C_ActionBar.GetOverrideBarIndex() then return "override" end
                if GetOverrideBarSkin() ~= C_ActionBar.GetOverrideBarSkin() then return "skin" end
                if GetTempShapeshiftBarIndex() ~= C_ActionBar.GetTempShapeshiftBarIndex() then return "temp" end
                if GetVehicleBarIndex() ~= C_ActionBar.GetVehicleBarIndex() then return "vehicle" end
                if GetActionBarPage() ~= C_ActionBar.GetActionBarPage() then return "page" end
                ChangeActionBarPage(4)
                if C_ActionBar.GetActionBarPage() ~= 4 then return "change_page" end
                return "ok"
                "#,
            )
            .expect("legacy action-bar global should run");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_legacy_bonus_bar_offset_global() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("GetBonusBarOffset = function() return 42 end")
            .expect("fixture should install existing legacy global");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("legacy action-bar globals should apply");
        }

        let offset: i32 = env
            .eval("return GetBonusBarOffset()")
            .expect("preserved legacy action-bar global should run");

        assert_eq!(offset, 42);
    }

    #[test]
    fn installs_action_button_util_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local enum = ActionButtonUtil and ActionButtonUtil.ActionBarActionStatus
                if not enum then return "missing_enum" end
                if enum.NotMissing ~= 1 or enum.MissingFromAllBars ~= 2 then return "bad_enum" end
                if type(ActionButtonUtil.ActionBarButtonNames) ~= "table" then return "missing_names" end
                if ActionButtonUtil.GetActionBarStatusForSpell(1) ~= enum.NotMissing then return "spell" end
                if ActionButtonUtil.GetActionBarStatusForPetAction(1) ~= enum.NotMissing then return "pet" end
                if ActionButtonUtil.GetActionBarStatusForFlyout(1) ~= enum.NotMissing then return "flyout" end
                ActionButtonUtil.ShowAllActionButtonGrids()
                ActionButtonUtil.HideAllActionButtonGrids()
                ActionButtonUtil.SetAllQuickKeybindButtonHighlights()
                ActionButtonUtil.ShowAllQuickKeybindButtonHighlights()
                ActionButtonUtil.HideAllQuickKeybindButtonHighlights()
                return "ok"
                "#,
            )
            .expect("ActionButtonUtil defaults should run");

        assert_eq!(result, "ok");
    }
}
