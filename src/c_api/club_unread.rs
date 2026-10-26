//! Supplied club streams and unread view markers; no parallel notification store.
use super::club_members::{read_club, string_arg, sync_guild};
use super::club_model::Stream;
use super::helpers::set_table_array;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set,
};
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const NOTIFY_ALL: u8 = 2;

pub(super) fn register(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    for (name, function) in [
        (
            "GetStreams",
            get_streams as fn(&mut LuaState) -> LuaResult<u32>,
        ),
        ("GetStreamInfo", get_stream_info),
        ("IsSubscribedToStream", is_subscribed_to_stream),
        ("DoesAnyCommunityHaveUnreadMessages", any_unread),
        ("GetClubStreamNotificationSettings", notification_settings),
        ("GetStreamViewMarker", view_marker),
        ("AdvanceStreamViewMarker", advance_view_marker),
    ] {
        table_set_rust_fn_static(state, table, name, function)?;
    }
    Ok(())
}

fn get_streams(state: &mut LuaState) -> LuaResult<u32> {
    let club = read_club(state)?;
    let array = create_table(state);
    if let Some(club) = club {
        for (index, stream) in club.streams.iter().enumerate() {
            let row = build_stream(state, stream);
            set_table_array(state, array, index as i64 + 1, row);
        }
    }
    state.push(array);
    Ok(1)
}

fn read_stream(state: &mut LuaState) -> LuaResult<Option<Stream>> {
    let stream_id = f64::from_stack(state, 2)?;
    Ok(read_club(state)?.and_then(|club| {
        club.streams
            .into_iter()
            .find(|stream| stream.id as f64 == stream_id)
    }))
}

fn get_stream_info(state: &mut LuaState) -> LuaResult<u32> {
    let stream = read_stream(state)?;
    let row = stream
        .as_ref()
        .map(|stream| build_stream(state, stream))
        .unwrap_or(Val::Nil);
    state.push(row);
    Ok(1)
}

fn is_subscribed_to_stream(state: &mut LuaState) -> LuaResult<u32> {
    let subscribed = read_stream(state)?.is_some();
    state.push(Val::Bool(subscribed));
    Ok(1)
}

fn any_unread(state: &mut LuaState) -> LuaResult<u32> {
    sync_guild(state)?;
    // INFERRED from CommunitiesUtil's per-community predicate, not native proof
    // of the aggregate's guild/mention/exclusion policies (those are UNKNOWN).
    let unread = borrow_state(state)?.clubs.clubs.values().any(|club| {
        club.streams.iter().any(|stream| {
            stream.notification_filter == Some(NOTIFY_ALL) && stream.last_read_time.is_some()
        })
    });
    state.push(Val::Bool(unread));
    Ok(1)
}

fn notification_settings(state: &mut LuaState) -> LuaResult<u32> {
    let club = read_club(state)?;
    let array = create_table(state);
    if let Some(club) = club {
        let settings = club
            .streams
            .iter()
            .filter_map(|stream| stream.notification_filter.map(|filter| (stream.id, filter)));
        for (index, (stream_id, filter)) in settings.enumerate() {
            let row = create_table(state);
            table_set(state, row, "streamId", Val::Num(stream_id as f64));
            table_set(state, row, "filter", Val::Num(filter as f64));
            set_table_array(state, array, index as i64 + 1, row);
        }
    }
    state.push(array);
    Ok(1)
}

fn view_marker(state: &mut LuaState) -> LuaResult<u32> {
    let marker = read_stream(state)?.and_then(|stream| stream.last_read_time);
    state.push(marker.map(|time| Val::Num(time as f64)).unwrap_or(Val::Nil));
    Ok(1)
}

fn advance_view_marker(state: &mut LuaState) -> LuaResult<u32> {
    let club_id = string_arg(state, 1)?;
    let stream_id = f64::from_stack(state, 2)?;
    sync_guild(state)?;
    let changed = clear_marker(state, &club_id, stream_id)?;
    if changed {
        let club = create_string(state, &club_id);
        // State is current before synchronous consumers observe the nil marker.
        // Native event deduplication/idempotence policy remains UNKNOWN.
        fire_named_event_state(
            state,
            "STREAM_VIEW_MARKER_UPDATED",
            &[club, Val::Num(stream_id), Val::Nil],
        );
    }
    Ok(0)
}

fn clear_marker(state: &LuaState, club_id: &str, stream_id: f64) -> LuaResult<bool> {
    let mut sim = borrow_state_mut(state)?;
    let stream = sim.clubs.clubs.get_mut(club_id).and_then(|club| {
        club.streams
            .iter_mut()
            .find(|stream| stream.id as f64 == stream_id)
    });
    Ok(stream.is_some_and(|stream| stream.last_read_time.take().is_some()))
}

fn build_stream(state: &mut LuaState, stream: &Stream) -> Val {
    let row = create_table(state);
    let name = create_string(state, &stream.name);
    let subject = create_string(state, &stream.subject);
    table_set(state, row, "streamId", Val::Num(stream.id as f64));
    table_set(state, row, "name", name);
    table_set(state, row, "subject", subject);
    table_set(
        state,
        row,
        "streamType",
        Val::Num(stream.stream_type as f64),
    );
    table_set(
        state,
        row,
        "creationTime",
        Val::Num(stream.creation_time as f64),
    );
    table_set(
        state,
        row,
        "leadersAndModeratorsOnly",
        Val::Bool(stream.leaders_and_moderators_only),
    );
    row
}
