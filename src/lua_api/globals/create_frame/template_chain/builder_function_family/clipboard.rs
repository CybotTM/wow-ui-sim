//! Clipboard-copy specials for community club tickets (optionally with sound).

use super::super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_clipboard_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::CopyClubTicketToClipboardFromParent => {
            build_copy_club_ticket_to_clipboard_from_parent_handler(state).map(Some)
        }
        FastHandlerRef::PlaySoundThenCopyClubTicketToClipboardFromParent { sound_path } => {
            build_play_sound_then_copy_club_ticket_to_clipboard_from_parent_handler(
                state, sound_path,
            )
            .map(Some)
        }
        _ => Ok(None),
    }
}

fn build_copy_club_ticket_to_clipboard_from_parent_handler(state: &mut LuaState) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                local clubId = parent:GetClubId()
                local clubInfo = clubId and C_Club.GetClubInfo(clubId)
                if clubInfo and parent.LinkIDText and parent.LinkIDText.GetText then
                    return CopyToClipboard(ClubTicketUtil.FormatTicket(clubInfo, parent.LinkIDText:GetText()))
                end
            end
        "#,
        "template-inline-copy-club-ticket-to-clipboard-from-parent",
    )?;
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[])
}

fn build_play_sound_then_copy_club_ticket_to_clipboard_from_parent_handler(
    state: &mut LuaState,
    sound_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local sound = ...
            return function(self, ...)
                PlaySound(sound)
                local clubId = self:GetParent():GetClubId()
                local clubInfo = clubId and C_Club.GetClubInfo(clubId)
                if clubInfo then
                    return CopyToClipboard(
                        ClubTicketUtil.FormatTicket(
                            clubInfo,
                            self:GetParent().LinkIDText:GetText()
                        )
                    )
                end
            end
        "#,
        "template-play-sound-then-copy-club-ticket",
    )?;
    let sound = resolve_global_path(state, sound_path);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[sound])
}
