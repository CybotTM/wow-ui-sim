//! Supplied availability through selected original transmog interactions.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::{WowLuaEnv, state::AppearanceSourceInfo};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

const UNSUPPLIED: &str =
    "C_TransmogOutfitInfo.IsTransmogEnabled: transmog availability has not been supplied";
const SOURCE: i64 = 190001;

fn supply(env: &WowLuaEnv, enabled: Option<bool>) {
    env.state().borrow_mut().transmog_availability.enabled = enabled;
}

fn load_original_transmog() -> WowLuaEnv {
    assert!(matches!(ACTIVE, ClientProfile::Retail | ClientProfile::Ptr));
    let ui = blizzard_ui_dir();
    let toc_path = find_toc_file(&ui.join("Blizzard_Transmog")).unwrap();
    assert_eq!(toc_path.file_name().unwrap(), "Blizzard_Transmog.toc");
    let toc = TocFile::from_file(&toc_path).unwrap();
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    let env = new_blizzard_addon_env(&ui);
    assert_eq!(env.state().borrow().transmog_availability.enabled, None);
    load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_Transmog"], &[]);
    eprintln!(
        "unfiltered load diagnostics: {:?}",
        env.state().borrow().lua_errors
    );
    seed_source(&env);
    initialize_original_inputs(&env);
    env
}

fn seed_source(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.player.money = 500;
    sim.transmog_outfits.slot_cost = 50;
    sim.transmog_sets.slot_sources.insert((71, 0), vec![SOURCE]);
    sim.transmog_appearance_sources.insert(
        SOURCE,
        AppearanceSourceInfo {
            category: 1,
            item_appearance_id: 90001,
            can_have_illusion: false,
            icon: 135771,
            is_collected: true,
            item_link: String::new(),
            transmoglink: String::new(),
            source_type: None,
            item_subclass: 0,
            ignore_model_attachment_checks_for_illusion: false,
        },
    );
}

fn initialize_original_inputs(env: &WowLuaEnv) {
    env.exec(
        r#"
        local suffix = 'Blizzard_Transmog/Blizzard_TransmogTemplates.lua'
        local function original(fn, file)
            local info = debug.getinfo(fn, 'S')
            local path = info.source:gsub('\\', '/')
            assert(info.what == 'Lua' and path:sub(-#file) == file, path)
            print('original transmog consumer', path, info.linedefined)
        end
        original(TransmogSetModelMixin.OnMouseDown, suffix)
        original(TransmogCustomSetModelMixin.OnMouseDown, suffix)
        original(TransmogSlotMixin.GetSlotInfo, suffix)
        local dialog = StaticPopupDialogs.CONFIRM_TRANSMOG_USABLE_DISCOUNT
        original(dialog.OnButton1, 'Blizzard_Transmog/Blizzard_Transmog.lua')
        original(dialog.OnButton2, 'Blizzard_Transmog/Blizzard_Transmog.lua')
        SetProbe = CreateFrame('DressUpModel', nil, nil, 'TransmogSetModelTemplate')
        SetProbe.elementData = {set = {setID = 71}}
        CustomProbe = CreateFrame('DressUpModel', nil, nil, 'TransmogCustomSetModelTemplate')
        local list = {{appearanceID=190001, secondaryAppearanceID=0, illusionID=0}}
        CustomProbe.elementData = {customSetID = C_TransmogCollection.NewCustomSet('Supplied head',135771,list)}
        assert(SetProbe.OnMouseDown == TransmogSetModelMixin.OnMouseDown)
        assert(CustomProbe.OnMouseDown == TransmogCustomSetModelMixin.OnMouseDown)
        assert(type(SetProbe:GetScript('OnMouseDown')) == 'function')
        assert(type(CustomProbe:GetScript('OnMouseDown')) == 'function')
        SlotProbe = CreateFromMixins(TransmogSlotMixin)
        SlotProbe.slotData = {
            transmogLocation = TransmogUtil.GetTransmogLocation('HEADSLOT', Enum.TransmogType.Appearance, false),
            currentWeaponOptionInfo = {weaponOption = 0},
            currentOptionInfo = {type = 0},
        }
        C_TransmogOutfitInfo.AddNewOutfit('Supplied outfit',135771)
        ProbeOutfit = C_TransmogOutfitInfo.GetOutfitInfoByName('Supplied outfit').outfitID
        C_TransmogOutfitInfo.ChangeViewedOutfit(ProbeOutfit)
        function ClickOriginal(custom)
            local frame = custom and CustomProbe or SetProbe
            frame:GetScript('OnMouseDown')(frame, 'LeftButton')
        end
        function ObserveOriginalSlot()
            local row = SlotProbe:GetSlotInfo()
            assert(row.transmogID == 190001 and row.hasPending and row.isPendingCollected)
            return row.canTransmogrify
        end
        "#,
    )
    .expect("selected templates bind original consumer methods");
}

fn stage_and_observe(env: &WowLuaEnv, custom: bool, enabled: bool) {
    supply(env, Some(enabled));
    env.exec(&format!("ClickOriginal({custom})"))
        .expect("original selected mouse callback stages supplied availability");
    let observed: bool = env.eval("return ObserveOriginalSlot()").unwrap();
    assert_eq!(
        observed, enabled,
        "original slot output must reflect supply"
    );
}

fn exercise_original_click(custom: bool) {
    let first = load_original_transmog();
    stage_and_observe(&first, custom, false);
    let second = load_original_transmog();
    stage_and_observe(&second, custom, true);
    stage_and_observe(&first, custom, true);
    stage_and_observe(&second, custom, false);
    assert!(first.eval::<bool>("return ObserveOriginalSlot()").unwrap());
    supply(&first, None);
    let error = first
        .exec(&format!("ClickOriginal({custom})"))
        .expect_err("unset observation must reject original interaction");
    assert!(error.to_string().contains(UNSUPPLIED), "{error}");
    assert!(first.eval::<bool>("return ObserveOriginalSlot()").unwrap());
    stage_and_observe(&first, custom, false);
    assert!(!second.eval::<bool>("return ObserveOriginalSlot()").unwrap());
    stage_and_observe(&first, custom, true);
}

#[test]
fn original_set_click_supplied_transitions_isolation_and_unset() {
    exercise_original_click(false);
}

#[test]
fn original_custom_set_click_supplied_transitions_isolation_and_unset() {
    exercise_original_click(true);
}

fn assert_pending_unchanged(env: &WowLuaEnv, money: i64) {
    env.exec(&format!(
        r#"
        assert(GetMoney() == {money})
        assert(C_TransmogOutfitInfo.HasPendingOutfitTransmogs())
        assert(C_TransmogOutfitInfo.GetActiveOutfitID() == 0)
        assert(ObserveOriginalSlot())
        "#,
    ))
    .unwrap();
    let outfit: i64 = env.eval("return ProbeOutfit").unwrap();
    assert!(
        env.state().borrow().transmog_outfits.saved[&outfit]
            .slots
            .is_empty()
    );
}

#[test]
fn original_apply_popup_checks_current_supply_and_unset() {
    let env = load_original_transmog();
    stage_and_observe(&env, false, true);
    supply(&env, Some(false));
    let error = env
        .exec("StaticPopupDialogs.CONFIRM_TRANSMOG_USABLE_DISCOUNT.OnButton1()")
        .expect_err("current false must reject original apply even with staged true records");
    assert!(
        error
            .to_string()
            .contains("pending outfit contains unavailable transmogs"),
        "{error}"
    );
    assert_pending_unchanged(&env, 500);
    supply(&env, None);
    let error = env
        .exec("StaticPopupDialogs.CONFIRM_TRANSMOG_USABLE_DISCOUNT.OnButton2()")
        .expect_err("unset current availability must reject original apply");
    assert!(error.to_string().contains(UNSUPPLIED), "{error}");
    assert_pending_unchanged(&env, 500);
    supply(&env, Some(true));
    env.exec(
        r#"
        StaticPopupDialogs.CONFIRM_TRANSMOG_USABLE_DISCOUNT.OnButton2()
        assert(GetMoney() == 450)
        assert(not C_TransmogOutfitInfo.HasPendingOutfitTransmogs())
        assert(C_TransmogOutfitInfo.GetActiveOutfitID() == ProbeOutfit)
        local row = SlotProbe:GetSlotInfo()
        assert(row.transmogID == 190001 and row.isTransmogrified and not row.hasPending)
        "#,
    )
    .unwrap();
    stage_and_observe(&env, true, true);
    env.exec("StaticPopupDialogs.CONFIRM_TRANSMOG_USABLE_DISCOUNT.OnButton1(); assert(GetMoney() == 400)")
        .unwrap();
}

#[test]
fn direct_query_supplied_arity_isolation_and_unset() {
    let first = WowLuaEnv::new().unwrap();
    let second = WowLuaEnv::new().unwrap();
    for enabled in [false, true, false] {
        supply(&first, Some(enabled));
        supply(&second, Some(!enabled));
        let probe = "local function pack(...) return select('#', ...), ... end; return pack(C_TransmogOutfitInfo.IsTransmogEnabled())";
        assert_eq!(first.eval::<(i32, bool)>(probe).unwrap(), (1, enabled));
        assert_eq!(second.eval::<(i32, bool)>(probe).unwrap(), (1, !enabled));
    }
    supply(&first, None);
    let error = first
        .eval::<bool>("return C_TransmogOutfitInfo.IsTransmogEnabled()")
        .expect_err("unset query must not infer true");
    assert!(error.to_string().contains(UNSUPPLIED), "{error}");
    supply(&first, Some(true));
    assert!(
        first
            .eval::<bool>("return C_TransmogOutfitInfo.IsTransmogEnabled()")
            .unwrap()
    );
}
