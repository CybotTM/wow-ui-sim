//! Retire when the native Wrath UI-error feedback model is supplied.
//! This preserves the original unselected-choice guard without inventing feedback.

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(
        r#"
        if rawget(_G, "QuestChooseRewardError") == nil then
            function QuestChooseRewardError() end
        end
        "#,
    )?;
    Ok(())
}
