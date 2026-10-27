#![cfg(feature = "client-wrath")]
//! Wrath mailbox admin event projection, independent of the full UI fixture.
//!
//! Native consumer: FrameXML/MailFrame.lua:28-30 registers MAIL_SHOW/MAIL_CLOSED;
//! lines 43-59 and 79-82 open/close MailFrame. Repeated admin transitions and
//! inactive-close suppression are simulator policy, not native timing evidence.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn admin_mailbox_publishes_native_open_close_sequence() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let events: String = env
        .eval(
            r#"
            local events = {}
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("MAIL_SHOW")
            frame:RegisterEvent("MAIL_CLOSED")
            frame:SetScript("OnEvent", function(_, event)
                table.insert(events, event)
            end)
            A_Admin.OpenMailbox()
            A_Admin.CloseMailbox()
            A_Admin.CloseMailbox()
            A_Admin.OpenMailbox()
            A_Admin.CloseMailbox()
            return table.concat(events, ",")
            "#,
        )
        .expect("Wrath mailbox admin event sequence");
    assert_eq!(events, "MAIL_SHOW,MAIL_CLOSED,MAIL_SHOW,MAIL_CLOSED");
    assert!(
        env.state().borrow().active_player_interactions.is_empty(),
        "Wrath CloseMailbox should remove the active MailInfo interaction"
    );
}
