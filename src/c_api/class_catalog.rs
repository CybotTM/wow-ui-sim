//! Selected-profile enumeration indexes versus canonical class identifiers.
//! Vanilla IDs skip Death Knight (6) and Monk (10); Druid retains ID 11.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::game_data::class_info_by_index;

pub(crate) fn selected_ids() -> &'static [i32] {
    match ACTIVE {
        ClientProfile::Era | ClientProfile::Anniversary => &[1, 2, 3, 4, 5, 7, 8, 9, 11],
        ClientProfile::Mists => &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        _ => &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
    }
}

pub(crate) fn by_enumeration(index: i32) -> Option<(&'static str, &'static str, i32)> {
    if !matches!(ACTIVE, ClientProfile::Era | ClientProfile::Anniversary) {
        return Some(class_info_by_index(index));
    }
    let offset = usize::try_from(index.checked_sub(1)?).ok()?;
    let id = *selected_ids().get(offset)?;
    Some(class_info_by_index(id))
}

pub(crate) fn by_id(id: i32) -> Option<(&'static str, &'static str, i32)> {
    if !matches!(ACTIVE, ClientProfile::Era | ClientProfile::Anniversary) {
        return Some(class_info_by_index(id));
    }
    selected_ids()
        .contains(&id)
        .then(|| class_info_by_index(id))
}
