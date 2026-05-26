//! Temporary `C_PrototypeDialog` active/removed dialog transition state.
//!
//! Prototype dialog server state is not modeled yet. These Lua-visible tables
//! preserve the small compatibility surface until a real dialog model exists.

const PROTOTYPE_DIALOG_STATE_LUA: &str = r#"
C_PrototypeDialog = C_PrototypeDialog or __wow_namespace()

local function ensureTableField(namespace, key)
    local tableValue = rawget(namespace, key)
    if type(tableValue) ~= "table" then
        tableValue = {}
        rawset(namespace, key, tableValue)
    end

    return tableValue
end

local function ensureState()
    return ensureTableField(C_PrototypeDialog, "_activeDialogs"),
           ensureTableField(C_PrototypeDialog, "_removedDialogs"),
           ensureTableField(C_PrototypeDialog, "_transitionHistory")
end

local function numberArgument(value)
    if type(value) == "number" then
        return value
    end

    return nil
end

local function nextSelectionCount(activeDialogs, dialogID)
    local priorState = activeDialogs[dialogID]
    if type(priorState) == "table" and type(priorState.selectionCount) == "number" then
        return priorState.selectionCount + 1
    end

    return 1
end

ensureState()

if rawget(C_PrototypeDialog, "SelectOption") == nil then
    function C_PrototypeDialog.SelectOption(dialogID, optionID)
        dialogID = numberArgument(dialogID)
        if dialogID == nil then
            return false
        end

        optionID = numberArgument(optionID)
        if optionID == nil then
            return false
        end

        local activeDialogs, removedDialogs, transitionHistory = ensureState()
        local selectionCount = nextSelectionCount(activeDialogs, dialogID)
        activeDialogs[dialogID] = {
            dialogID = dialogID,
            selectedOptionID = optionID,
            selectionCount = selectionCount,
        }
        removedDialogs[dialogID] = nil
        table.insert(transitionHistory, {
            transition = "selected",
            dialogID = dialogID,
            optionID = optionID,
            selectionCount = selectionCount,
        })

        return true
    end
end

if rawget(C_PrototypeDialog, "EnsureRemoved") == nil then
    function C_PrototypeDialog.EnsureRemoved(dialogID)
        dialogID = numberArgument(dialogID)
        if dialogID == nil then
            return false
        end

        local activeDialogs, removedDialogs, transitionHistory = ensureState()
        local hadActiveDialog = activeDialogs[dialogID] ~= nil
        activeDialogs[dialogID] = nil
        removedDialogs[dialogID] = true
        table.insert(transitionHistory, {
            transition = "removed",
            dialogID = dialogID,
        })

        return hadActiveDialog
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(PROTOTYPE_DIALOG_STATE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_prototype_dialog_state_tables_and_methods() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (bool, bool, bool, String) = env
            .eval(
                r#"
                return type(C_PrototypeDialog._activeDialogs) == "table",
                       type(C_PrototypeDialog._removedDialogs) == "table",
                       type(C_PrototypeDialog._transitionHistory) == "table",
                       type(C_PrototypeDialog.SelectOption)
                "#,
            )
            .expect("prototype dialog state should be installed");

        assert_eq!(result, (true, true, true, "function".to_string()));
    }

    #[test]
    fn tracks_selection_removal_and_transition_history() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (bool, i32, i32, bool, bool, String, String) = env
            .eval(
                r#"
                local selected = C_PrototypeDialog.SelectOption(10, 2)
                local first = C_PrototypeDialog._activeDialogs[10]
                local removed = C_PrototypeDialog.EnsureRemoved(10)
                local activeCleared = C_PrototypeDialog._activeDialogs[10] == nil
                local removedMarked = C_PrototypeDialog._removedDialogs[10] == true
                return selected,
                       first.selectedOptionID,
                       first.selectionCount,
                       removed,
                       activeCleared and removedMarked,
                       C_PrototypeDialog._transitionHistory[1].transition,
                       C_PrototypeDialog._transitionHistory[2].transition
                "#,
            )
            .expect("prototype dialog transitions should be tracked");

        assert_eq!(
            result,
            (
                true,
                2,
                1,
                true,
                true,
                "selected".to_string(),
                "removed".to_string(),
            )
        );
    }

    #[test]
    fn preserves_existing_prototype_dialog_provider_and_state() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_PrototypeDialog = C_PrototypeDialog or __wow_namespace()
            C_PrototypeDialog.ExistingMember = "kept"

            function C_PrototypeDialog.SelectOption(_dialogID, _optionID)
                return "existing"
            end
            "#,
        )
        .expect("fixture should install existing prototype dialog provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (String, String, bool) = env
            .eval(
                r#"
                return C_PrototypeDialog.SelectOption(1, 2),
                       C_PrototypeDialog.ExistingMember,
                       type(C_PrototypeDialog._activeDialogs) == "table"
                "#,
            )
            .expect("existing prototype dialog provider should remain callable");

        assert_eq!(result, ("existing".to_string(), "kept".to_string(), true));
    }
}
