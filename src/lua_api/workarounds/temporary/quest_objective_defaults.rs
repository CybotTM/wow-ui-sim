//! Temporary quest/objective tracker query defaults.
//!
//! The simulator does not model bonus objective task lists, spell-targetable
//! quest state, or auto quest popup toasts yet. Keep these startup-safe empty
//! defaults in the workaround layer instead of the central runtime bootstrap.

const QUEST_OBJECTIVE_DEFAULTS_LUA: &str = r#"
-- Bonus / world-quest objective trackers iterate the task list at startup.
-- Return an empty table so the `for ... in ipairs(tasksTable)` loops no-op.
if GetTasksTable == nil then
  function GetTasksTable()
    return {}
  end
end

if SpellCanTargetQuest == nil then
  function SpellCanTargetQuest()
    return false
  end
end

-- Auto quest popups (tutorial toasts). Not simulated; `for i = 1, N do`
-- loops in AutoQuestPopUpTracker iterate zero times.
if GetNumAutoQuestPopUps == nil then
  function GetNumAutoQuestPopUps() return 0 end
end
if GetAutoQuestPopUp == nil then
  function GetAutoQuestPopUp(_index) return nil, nil end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(QUEST_OBJECTIVE_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_empty_quest_objective_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if type(GetTasksTable()) ~= "table" then return "tasks_type" end
                if #GetTasksTable() ~= 0 then return "tasks_count" end
                if SpellCanTargetQuest() ~= false then return "target_quest" end
                if GetNumAutoQuestPopUps() ~= 0 then return "popup_count" end
                local popupQuestID, popupType = GetAutoQuestPopUp(1)
                if popupQuestID ~= nil or popupType ~= nil then return "popup_entry" end
                return "ok"
                "#,
            )
            .expect("quest objective defaults should be callable");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_quest_objective_members() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function GetTasksTable() return { 7 } end
            function SpellCanTargetQuest() return true end
            function GetNumAutoQuestPopUps() return 2 end
            function GetAutoQuestPopUp(index) return 1000 + index, "COMPLETE" end
            "#,
        )
        .expect("fixture should install existing quest members");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("quest objective defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if GetTasksTable()[1] ~= 7 then return "overwrote_tasks" end
                if SpellCanTargetQuest() ~= true then return "overwrote_target_quest" end
                if GetNumAutoQuestPopUps() ~= 2 then return "overwrote_popup_count" end
                local popupQuestID, popupType = GetAutoQuestPopUp(4)
                if popupQuestID ~= 1004 or popupType ~= "COMPLETE" then return "overwrote_popup_entry" end
                return "ok"
                "#,
            )
            .expect("quest objective preservation probe should run");

        assert_eq!(result, "ok");
    }
}
