//! PTR 12.1.7.70493 Edit Mode declarations, not Lua initialization replacements.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{create_table, table_set};
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register(state: &mut LuaState) {
    if ACTIVE != ClientProfile::Ptr {
        return;
    }
    let enums = super::helpers::ensure_global_table(state, "Enum");
    for &(name, fields) in DECLARATIONS {
        let values = create_table(state);
        for &(field, value) in fields {
            table_set(state, values, field, Val::Num(f64::from(value)));
        }
        table_set(state, enums, name, values);
    }
}

type EnumFields = (&'static str, &'static [(&'static str, i32)]);
const THREE_VALUES_META: &[(&str, i32)] = &[("NumValues", 3), ("MinValue", 0), ("MaxValue", 2)];

// Pinned Blizzard_APIDocumentationGenerated/EditModeManagerConstantsDocumentation.lua:
// BossFrameGrowDirection at 111–122; EditModePresetLayouts at 639–650;
// EditModeSwingTimerSetting at 702–717; EditModeSwingTimerVisibility at 732–743.
const DECLARATIONS: &[EnumFields] = &[
    (
        "BossFrameGrowDirection",
        &[("Left", 0), ("Center", 1), ("Right", 2)],
    ),
    ("BossFrameGrowDirectionMeta", THREE_VALUES_META),
    (
        "EditModePresetLayouts",
        &[("Modern", 0), ("Classic", 1), ("Gamepad", 2)],
    ),
    ("EditModePresetLayoutsMeta", THREE_VALUES_META),
    (
        "EditModeSwingTimerSetting",
        &[
            ("Scale", 0),
            ("Opacity", 1),
            ("Visibility", 2),
            ("Width", 3),
            ("Height", 4),
            ("ShowBarTitle", 5),
            ("ShowTime", 6),
        ],
    ),
    (
        "EditModeSwingTimerSettingMeta",
        &[("NumValues", 7), ("MinValue", 0), ("MaxValue", 6)],
    ),
    (
        "EditModeSwingTimerVisibility",
        &[("Always", 0), ("InCombat", 1), ("Hidden", 2)],
    ),
    ("EditModeSwingTimerVisibilityMeta", THREE_VALUES_META),
];
