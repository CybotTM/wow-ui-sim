use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";
const VALID_ITEM_ID: i32 = 158_041;
const INSTALL_HELP_TIP_DROP_SPIES_LUA: &str = r#"
local mixinEnv = debug.getfenv(AzeriteRespecMixin.OnShow)
local originalSetCVarBitfield = mixinEnv.SetCVarBitfield or SetCVarBitfield

mixinEnv.PlaySound = function() end
mixinEnv.AzeriteEmpoweredItemDataSource = {
    CreateFromItemLocation = function()
        return { hasSelectedPower = true }
    end,
}
mixinEnv.AzeriteUtil = {
    HasSelectedAnyAzeritePower = function(azeriteItem)
        return azeriteItem.hasSelectedPower
    end,
}
mixinEnv.Item = {
    CreateFromItemLocation = function(_, location)
        return {
            LockItem = function()
                _G.__azerite_respec_help_tip_locked_location = location
            end,
            UnlockItem = function() end,
        }
    end,
}
mixinEnv.HelpTip = {
    ButtonStyle = { Close = "Close" },
    Point = { RightEdgeCenter = "RightEdgeCenter" },
    Show = function(_, parent, info, relativeRegion)
        _G.__azerite_respec_help_tip_show_count =
            (_G.__azerite_respec_help_tip_show_count or 0) + 1
        _G.__azerite_respec_help_tip_show_parent = parent
        _G.__azerite_respec_help_tip_show_info = info
        _G.__azerite_respec_help_tip_show_relative_region = relativeRegion
    end,
    Hide = function(_, parent, text)
        _G.__azerite_respec_help_tip_hide_count =
            (_G.__azerite_respec_help_tip_hide_count or 0) + 1
        _G.__azerite_respec_help_tip_hide_parent = parent
        _G.__azerite_respec_help_tip_hide_text = text
    end,
}
mixinEnv.SetCVarBitfield = function(name, bit, enabled)
    _G.__azerite_respec_help_tip_cvar_call = {
        name = name,
        bit = bit,
        enabled = enabled,
    }
    return originalSetCVarBitfield(name, bit, enabled)
end
AzeriteRespecFrame.ItemSlot.RefreshIcon = function() end
AzeriteRespecFrame.ItemSlot.RefreshTooltip = function() end
"#;

type FirstShowProbe = (i64, bool, bool, String, String, i64);
type DropDismissalProbe = (i64, bool, String, String, i64, bool, bool);

#[test]
fn blizzard_azerite_respec_ui_help_tip_dismisses_when_item_is_dropped() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_valid_respec_item(env);
                clear_recorded_lua_errors(env);
                load_azerite_respec_ui(env);
                install_help_tip_drop_spies(env);

                show_panel_with_tutorial_bit_clear(env);
                assert_first_show_displays_tutorial(env);

                set_valid_respec_item(env);
                assert_drop_hides_tutorial_and_sets_cvar(env);

                show_panel_after_drop(env);
                assert_tutorial_stays_hidden(env);
                assert_no_lua_errors(env);
            });
        });
    });
}

fn seed_valid_respec_item(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state
        .azerite_empowered
        .empowered_items
        .insert(VALID_ITEM_ID);
    state.azerite_empowered.respec_cost = 50_000;
    state.player.money = 100_000;
}

fn load_azerite_respec_ui(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, reason): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
        .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
    assert!(loaded, "`{ROOT}` should load: {reason:?}");
}

fn install_help_tip_drop_spies(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(INSTALL_HELP_TIP_DROP_SPIES_LUA)
        .expect("AzeriteRespec HelpTip drop spies should install");
    env.exec(&format!(
        "_G.__azerite_respec_valid_location = {{ itemID = {VALID_ITEM_ID}, bagID = 0, slotIndex = 10 }}"
    ))
    .expect("AzeriteRespec valid item location should install");
}

fn show_panel_with_tutorial_bit_clear(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_AZERITE_RESPEC, false)
        _G.__azerite_respec_help_tip_show_count = 0
        AzeriteRespecMixin.OnShow(AzeriteRespecFrame)
        "#,
    )
    .expect("AzeriteRespec OnShow should run with tutorial bit clear");
}

fn assert_first_show_displays_tutorial(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (count, parent_matches, slot_matches, text, cvar, bit) = first_show_probe(env);
    assert_eq!(count, 1);
    assert!(
        parent_matches,
        "`{ROOT}` HelpTip should parent to the panel"
    );
    assert!(
        slot_matches,
        "`{ROOT}` HelpTip should anchor to the item slot"
    );
    assert_eq!(
        text,
        "Drag a piece of Azerite Armor here to reforge its powers."
    );
    assert_eq!(cvar, "closedInfoFrames");
    assert_eq!(bit, 57);
}

fn first_show_probe(env: &wow_ui_sim::lua_api::WowLuaEnv) -> FirstShowProbe {
    env.eval(
        r#"
        local info = _G.__azerite_respec_help_tip_show_info or {}
        return _G.__azerite_respec_help_tip_show_count,
            _G.__azerite_respec_help_tip_show_parent == AzeriteRespecFrame,
            _G.__azerite_respec_help_tip_show_relative_region == AzeriteRespecFrame.ItemSlot,
            info.text or "",
            info.cvarBitfield or "",
            info.bitfieldFlag or -1
        "#,
    )
    .expect("first AzeriteRespec HelpTip show should be readable")
}

fn set_valid_respec_item(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec("AzeriteRespecFrame:SetRespecItem(_G.__azerite_respec_valid_location)")
        .expect("AzeriteRespec SetRespecItem(valid_loc) should run");
}

fn assert_drop_hides_tutorial_and_sets_cvar(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (hide_count, parent_matches, text, cvar_name, cvar_bit, cvar_enabled, bit_set) =
        drop_dismissal_probe(env);
    assert_eq!(hide_count, 1);
    assert!(
        parent_matches,
        "`{ROOT}` HelpTip:Hide should target the panel"
    );
    assert_eq!(
        text,
        "Drag a piece of Azerite Armor here to reforge its powers."
    );
    assert_eq!(cvar_name, "closedInfoFrames");
    assert_eq!(cvar_bit, 57);
    assert!(
        cvar_enabled,
        "`{ROOT}` should set the tutorial cvar bit to true"
    );
    assert!(
        bit_set,
        "`{ROOT}` tutorial cvar bit should be set after drop"
    );
}

fn drop_dismissal_probe(env: &wow_ui_sim::lua_api::WowLuaEnv) -> DropDismissalProbe {
    env.eval(
        r#"
        local cvarCall = _G.__azerite_respec_help_tip_cvar_call or {}
        return _G.__azerite_respec_help_tip_hide_count,
            _G.__azerite_respec_help_tip_hide_parent == AzeriteRespecFrame,
            _G.__azerite_respec_help_tip_hide_text or "",
            cvarCall.name or "",
            cvarCall.bit or -1,
            cvarCall.enabled == true,
            GetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_AZERITE_RESPEC)
        "#,
    )
    .expect("AzeriteRespec HelpTip drop state should be readable")
}

fn show_panel_after_drop(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        _G.__azerite_respec_help_tip_show_count = 0
        AzeriteRespecMixin.OnShow(AzeriteRespecFrame)
        "#,
    )
    .expect("AzeriteRespec OnShow should run after tutorial dismissal");
}

fn assert_tutorial_stays_hidden(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let show_count: i64 = env
        .eval("return _G.__azerite_respec_help_tip_show_count")
        .expect("post-drop AzeriteRespec HelpTip show count should be readable");
    assert_eq!(
        show_count, 0,
        "`{ROOT}` should not show the tutorial again once the drop sets the cvar bit"
    );
}

fn assert_no_lua_errors(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let errors = recorded_lua_errors(env);
    assert!(
        errors.is_empty(),
        "`{ROOT}` emitted Lua errors while checking HelpTip dismissal:\n{}",
        errors.join("\n")
    );
}
