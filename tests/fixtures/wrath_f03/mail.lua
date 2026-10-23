-- INFERRED global row identities from native MailFrame.lua/xml.
-- Rust supplies data and dispatches MAIL_SHOW through the real event path.
-- This is not proof that A_Admin.OpenMailbox publishes the native event.
assert(MailFrame:IsShown() and InboxFrame:IsShown(), "native mail and inbox panels open")
assert(GetInboxNumItems() == 2, "legacy inbox count")
assert(MailItem1Button:IsShown() and MailItem1Button.index == 1)
assert(MailItem1Sender:GetText() == "Thrall")
assert(MailItem1Subject:GetText() == "Unread Orders")
assert(MailItem2Button:IsShown())
assert(MailItem2Sender:GetText() == "Jaina")
