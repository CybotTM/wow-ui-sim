//! Temporary legacy LFG global defaults.
//!
//! Modern LFG list behavior is state-backed elsewhere. This module keeps small
//! legacy global probes explicit until those callers are modeled properly.

const LFG_LEGACY_DEFAULTS_LUA: &str = r#"
if GetLFGCategoryForID == nil then
    function GetLFGCategoryForID() return 0 end
end
if rawget(C_LFGInfo or {}, "CanPlayerUseGroupFinder") == nil then
    C_LFGInfo = C_LFGInfo or __wow_namespace()
    function C_LFGInfo.CanPlayerUseGroupFinder()
        return false, ""
    end
end
if rawget(C_LFGInfo or {}, "IsInLFGFollowerDungeon") == nil then
    C_LFGInfo = C_LFGInfo or __wow_namespace()
    function C_LFGInfo.IsInLFGFollowerDungeon()
        return false
    end
end
if GetLFGProposal == nil then
    function GetLFGProposal()
        return false, 0, 0, 0, "", "", "", false, 0, 0, 0, false, false, nil, false
    end
end
if GetLFGProposalEncounter == nil then
    function GetLFGProposalEncounter(_i)
        return "", "", false
    end
end
if GetLFGInfoServer == nil then
    function GetLFGInfoServer()
        return false, false, false, false, false, 0, 0, 0, ""
    end
end
if GetLFGRoleUpdate == nil then
    function GetLFGRoleUpdate()
        return false, 0, 0, 0, 0, false
    end
end
if GetLFGQueuedList == nil then
    function GetLFGQueuedList(_category, queuedList)
        queuedList = queuedList or {}
        for key in pairs(queuedList) do
            queuedList[key] = nil
        end
        return queuedList
    end
end
if GetLFGReadyCheckUpdate == nil then
    function GetLFGReadyCheckUpdate()
        return false, false
    end
end
if GetPartyLFGID == nil then
    function GetPartyLFGID() return 0 end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(LFG_LEGACY_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_lfg_category_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let category_id: i32 = env
            .eval("return GetLFGCategoryForID(123)")
            .expect("legacy LFG category probe should run");

        assert_eq!(category_id, 0);
    }

    #[test]
    fn installs_legacy_lfg_fallback_shapes_when_missing() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_LFGInfo.CanPlayerUseGroupFinder = nil
            C_LFGInfo.IsInLFGFollowerDungeon = nil
            GetLFGProposal = nil
            GetLFGProposalEncounter = nil
            GetLFGInfoServer = nil
            GetLFGRoleUpdate = nil
            GetLFGQueuedList = nil
            GetLFGReadyCheckUpdate = nil
            GetPartyLFGID = nil
            "#,
        )
        .expect("fixture should clear legacy LFG globals");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("legacy LFG defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                local canUse, reason = C_LFGInfo.CanPlayerUseGroupFinder()
                if canUse ~= false or reason ~= "" then return "group_finder" end
                if C_LFGInfo.IsInLFGFollowerDungeon() ~= false then return "follower" end

                local proposalExists, id, typeID, subtypeID, name, bg, role, responded, bosses, completed, members, leader, holiday, proposalCategory, silent = GetLFGProposal()
                if proposalExists ~= false or id ~= 0 or typeID ~= 0 or subtypeID ~= 0 then return "proposal_ids" end
                if name ~= "" or bg ~= "" or role ~= "" or responded ~= false then return "proposal_strings" end
                if bosses ~= 0 or completed ~= 0 or members ~= 0 or leader ~= false or holiday ~= false or proposalCategory ~= nil or silent ~= false then return "proposal_tail" end

                local bossName, texture, killed = GetLFGProposalEncounter(1)
                if bossName ~= "" or texture ~= "" or killed ~= false then return "encounter" end

                local server1, server2, server3, server4, server5, server6, server7, server8, server9 = GetLFGInfoServer()
                if server1 ~= false or server2 ~= false or server3 ~= false or server4 ~= false or server5 ~= false then return "server_flags" end
                if server6 ~= 0 or server7 ~= 0 or server8 ~= 0 or server9 ~= "" then return "server_tail" end

                local inProgress, slots, memberCount, category, lfgID, bgQueue = GetLFGRoleUpdate()
                if inProgress ~= false or slots ~= 0 or memberCount ~= 0 or category ~= 0 or lfgID ~= 0 or bgQueue ~= false then return "role_update" end

                local queued = { stale = true }
                local returned = GetLFGQueuedList(1, queued)
                if returned ~= queued or next(queued) ~= nil then return "queued" end

                local ready, accepted = GetLFGReadyCheckUpdate()
                if ready ~= false or accepted ~= false then return "ready" end
                if GetPartyLFGID() ~= 0 then return "party_id" end
                return "ok"
                "#,
            )
            .expect("legacy LFG fallback shape probe should run");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_lfg_category_function() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("function GetLFGCategoryForID() return 7 end")
            .expect("fixture should install existing LFG category function");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("legacy LFG defaults should apply");
        }

        let category_id: i32 = env
            .eval("return GetLFGCategoryForID(123)")
            .expect("legacy LFG category preservation probe should run");

        assert_eq!(category_id, 7);
    }
}
