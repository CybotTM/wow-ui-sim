/// Proves deprecated chat and spell-script wrappers remain published and forward.
#[test]
fn vendor_deprecated_chat_spell_globals_are_published_and_forward() {
    let env = load_full_game_ui_with_all_lod();
    let result: String = env
        .eval(
            r#"
            if not GetCVarBool("loadDeprecationFallbacks") then
                return "fallbacks-disabled"
            end
            local required = {
                "CancelEmote",
                "DoEmote",
                "SpellIsPriorityAura",
                "SpellIsSelfBuff",
            }
            for _, name in ipairs(required) do
                if type(rawget(_G, name)) ~= "function" then
                    return "missing=" .. name
                end
            end

            local cancelCalls = 0
            local oldCancelEmote = C_ChatInfo.CancelEmote
            C_ChatInfo.CancelEmote = function()
                cancelCalls = cancelCalls + 1
                return "cancel-sentinel"
            end
            local cancelOK = pcall(CancelEmote)
            C_ChatInfo.CancelEmote = oldCancelEmote
            if not cancelOK or cancelCalls ~= 1 then
                return "cancel-wrapper"
            end

            local namedArgs
            local oldPerformEmote = C_ChatInfo.PerformEmote
            C_ChatInfo.PerformEmote = function(emoteName, targetName, suppressMoveError)
                namedArgs = {emoteName, targetName, suppressMoveError}
                return "emote-sentinel"
            end
            local namedOK, namedResult = pcall(DoEmote, "wave", "target", true)
            C_ChatInfo.PerformEmote = oldPerformEmote
            if not namedOK or namedResult ~= "emote-sentinel"
                    or namedArgs[1] ~= "wave"
                    or namedArgs[2] ~= "target"
                    or namedArgs[3] ~= true then
                return "named-emote-wrapper"
            end

            local nilCancelCalls = 0
            oldCancelEmote = C_ChatInfo.CancelEmote
            C_ChatInfo.CancelEmote = function()
                nilCancelCalls = nilCancelCalls + 1
            end
            local nilOK, nilResult = pcall(DoEmote, nil)
            C_ChatInfo.CancelEmote = oldCancelEmote
            if not nilOK or nilResult ~= false or nilCancelCalls ~= 1 then
                return "nil-emote-wrapper"
            end

            local priorityID
            local oldPriorityAura = C_Spell.IsPriorityAura
            C_Spell.IsPriorityAura = function(spellID)
                priorityID = spellID
                return "priority-sentinel"
            end
            local priorityOK, priorityResult = pcall(SpellIsPriorityAura, 123)
            C_Spell.IsPriorityAura = oldPriorityAura
            if not priorityOK or priorityID ~= 123 or priorityResult ~= "priority-sentinel" then
                return "priority-wrapper"
            end

            local selfBuffID
            local oldSelfBuff = C_Spell.IsSelfBuff
            C_Spell.IsSelfBuff = function(spellID)
                selfBuffID = spellID
                return "self-buff-sentinel"
            end
            local selfBuffOK, selfBuffResult = pcall(SpellIsSelfBuff, 456)
            C_Spell.IsSelfBuff = oldSelfBuff
            if not selfBuffOK or selfBuffID ~= 456 or selfBuffResult ~= "self-buff-sentinel" then
                return "self-buff-wrapper"
            end

            return "ok"
            "#,
        )
        .expect("deprecated chat/spell runtime probe succeeds");

    assert_eq!(result, "ok");
}
