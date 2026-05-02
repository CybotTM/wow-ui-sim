//! Global popup surface for `Blizzard_AnimaDiversionUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const POPUP_SURFACE_PROBE: &str = r#"
local channel = StaticPopupDialogs["ANIMA_DIVERSION_CONFIRM_CHANNEL"]
local reinforce = StaticPopupDialogs["ANIMA_DIVERSION_CONFIRM_REINFORCE"]

local function popupSurface(dialog)
    return type(dialog) == "table",
           type(dialog and dialog.text),
           dialog and dialog.button1 == YES,
           dialog and dialog.button2 == CANCEL,
           type(dialog and dialog.OnAccept),
           type(dialog and dialog.OnShow),
           type(dialog and dialog.OnHide),
           dialog and dialog.hideOnEscape == 1
end

local channelIsTable,
      channelTextType,
      channelButton1IsYes,
      channelButton2IsCancel,
      channelOnAcceptType,
      channelOnShowType,
      channelOnHideType,
      channelHideOnEscape = popupSurface(channel)
local reinforceIsTable,
      reinforceTextType,
      reinforceButton1IsYes,
      reinforceButton2IsCancel,
      reinforceOnAcceptType,
      reinforceOnShowType,
      reinforceOnHideType,
      reinforceHideOnEscape = popupSurface(reinforce)

return channelIsTable,
       channelTextType,
       channelButton1IsYes,
       channelButton2IsCancel,
       channelOnAcceptType,
       channelOnShowType,
       channelOnHideType,
       channelHideOnEscape,
       type(channel and channel.GetExpirationText),
       reinforceIsTable,
       reinforceTextType,
       reinforceButton1IsYes,
       reinforceButton2IsCancel,
       reinforceOnAcceptType,
       reinforceOnShowType,
       reinforceOnHideType,
       reinforceHideOnEscape,
       type(reinforce and reinforce.GetExpirationText)
"#;

#[test]
fn anima_diversion_ui_registers_static_popup_globals() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let surface: PopupSurface = env
            .eval(POPUP_SURFACE_PROBE)
            .expect("AnimaDiversionUI static popup surface probe must run cleanly");

        assert_popup_surface(surface);
    });
}

type PopupSurface = (
    bool,
    String,
    bool,
    bool,
    String,
    String,
    String,
    bool,
    String,
    bool,
    String,
    bool,
    bool,
    String,
    String,
    String,
    bool,
    String,
);
type PopupFields = (bool, String, bool, bool, String, String, String, bool);

fn assert_popup_surface(surface: PopupSurface) {
    assert_common_popup_fields("ANIMA_DIVERSION_CONFIRM_CHANNEL", channel_fields(&surface));
    assert_eq!(
        surface.8, "function",
        "channel popup must expose `GetExpirationText`"
    );

    assert_common_popup_fields(
        "ANIMA_DIVERSION_CONFIRM_REINFORCE",
        reinforce_fields(&surface),
    );
    assert_eq!(
        surface.17, "nil",
        "reinforce popup must not expose `GetExpirationText`"
    );
}

fn channel_fields(surface: &PopupSurface) -> PopupFields {
    (
        surface.0,
        surface.1.clone(),
        surface.2,
        surface.3,
        surface.4.clone(),
        surface.5.clone(),
        surface.6.clone(),
        surface.7,
    )
}

fn reinforce_fields(surface: &PopupSurface) -> PopupFields {
    (
        surface.9,
        surface.10.clone(),
        surface.11,
        surface.12,
        surface.13.clone(),
        surface.14.clone(),
        surface.15.clone(),
        surface.16,
    )
}

fn assert_common_popup_fields(name: &str, fields: PopupFields) {
    let (
        is_table,
        text_type,
        button1_is_yes,
        button2_is_cancel,
        on_accept_type,
        on_show_type,
        on_hide_type,
        hide_on_escape,
    ) = fields;

    assert!(
        is_table,
        "`{name}` must be registered as a StaticPopup table"
    );
    assert_eq!(text_type, "string", "`{name}` must define popup text");
    assert!(
        button1_is_yes,
        "`{name}` must use YES as the primary button"
    );
    assert!(
        button2_is_cancel,
        "`{name}` must use CANCEL as the secondary button"
    );
    assert_eq!(on_accept_type, "function", "`{name}` must define OnAccept");
    assert_eq!(on_show_type, "function", "`{name}` must define OnShow");
    assert_eq!(on_hide_type, "function", "`{name}` must define OnHide");
    assert!(
        hide_on_escape,
        "`{name}` must opt into hide-on-escape behavior"
    );
}
