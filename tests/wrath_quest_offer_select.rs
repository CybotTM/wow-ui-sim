//! Original Wrath available-list/click seam, not whole-panel or lifecycle coverage.
#![cfg(feature = "client-wrath")]

use rilua::{LuaApiMut, LuaResult, Val, vm::state::LuaState};
use sha2::{Digest, Sha256};
use std::{cell::RefCell, rc::Rc};
use wow_ui_sim::lua_api::state::{GossipQuestRow, GossipState};
use wow_ui_sim::lua_api::{SimState, WowLuaEnv};
use wow_ui_sim::lua_bridge::FromStack;

const SOURCES: &[(&str, &str)] = &[
    (
        "GossipFrame.lua",
        "ce4f5d9f2cd745a7ccc0ebefd4490dc8bffce20fff0098587dcc1fa65d421128",
    ),
    (
        "QuestFrame.lua",
        "4b2221da7f962535796ed500fbd1fc2adf95020e4564217ef457c1be7f9935f9",
    ),
];

fn assert_error_boundaries(env: &WowLuaEnv) {
    let errors: String = env.eval("return table.concat(OfferErrors, '\\n')").unwrap();
    let state = env.state().borrow();
    eprintln!(
        "error boundaries: handler={errors:?}; lua_errors={:?}; lua_error_counts={:?}",
        state.lua_errors, state.lua_error_counts
    );
    assert!(errors.is_empty(), "installed error handler: {errors}");
    assert!(state.lua_errors.is_empty(), "{:?}", state.lua_errors);
    assert!(
        state.lua_error_counts.is_empty(),
        "{:?}",
        state.lua_error_counts
    );
}

fn exec(env: &WowLuaEnv, source: &str) {
    let result = env.exec(source);
    eprintln!("direct eval result: {result:?}");
    assert_error_boundaries(env);
    result.expect("original consumer and behavioral assertions");
}

fn load_originals(env: &WowLuaEnv) {
    let root = dirs::cache_dir()
        .unwrap()
        .join("wow-ui-sim/blizzard-ui/wrath/FrameXML");
    for (file, expected) in SOURCES {
        let path = root.join(file);
        let bytes = std::fs::read(&path).expect("pinned Wrath cache must exist");
        let hash = format!("{:x}", Sha256::digest(&bytes));
        assert_eq!(&hash, expected, "cached source drift: {}", path.display());
        eprintln!("unchanged source: {file} sha256={hash}");
        let result = env.exec_named(
            std::str::from_utf8(&bytes).unwrap(),
            &format!("@{}", path.display()),
        );
        assert_error_boundaries(env);
        result.expect("load whole unchanged cached consumer");
    }
}

// Read-only test instrumentation: the observer samples actual state during dispatch.
fn observe_offer(state: &mut LuaState) -> LuaResult<u32> {
    let (selected, pending) = {
        let Val::Userdata(handle) = Val::from_stack(state, 1)? else {
            return Err(rilua::runtime_error("observer state handle required"));
        };
        let handle = state.gc.userdata.get(handle).unwrap();
        let sim = handle
            .downcast_ref::<Rc<RefCell<SimState>>>()
            .unwrap()
            .borrow();
        (sim.selected_quest_log_id, sim.pending_quest_offer.clone())
    };
    state.push(selected.map_or(Val::Nil, |id| Val::Num(id as f64)));
    let Some(offer) = pending else {
        return Ok(1);
    };
    state.push(Val::Num(offer.quest_id as f64));
    let title = state.gc.intern_string(offer.title.as_bytes());
    state.push(Val::Str(title));
    state.push(Val::Num(offer.level as f64));
    Ok(4)
}

fn seed_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec("OfferErrors = {}; seterrorhandler(function(e) OfferErrors[#OfferErrors+1] = tostring(e) end)").unwrap();
    {
        let mut sim = env.state().borrow_mut();
        sim.gossip = GossipState {
            active: true,
            num_available_quests: 2,
            available_quests: [
                (990101, "Wrath supplied first", 17),
                (990202, "Wrath supplied second", 23),
            ]
            .into_iter()
            .map(|(quest_id, title, quest_level)| GossipQuestRow {
                quest_id,
                title: title.into(),
                quest_level,
                repeatable: Some(false),
                is_complete: Some(false),
                ..Default::default()
            })
            .collect(),
            ..Default::default()
        };
        for id in [990101, 990202] {
            assert!(!sim.quest_log.contains(&id));
            assert!(
                !sim.quest_log_entries
                    .entries
                    .iter()
                    .any(|row| row.quest_id == id as i32)
            );
            assert!(
                !sim.quest_log_entries
                    .completed_quest_ids
                    .contains(&(id as i32))
            );
        }
        assert!(sim.pending_quest_offer.is_none());
        assert!(sim.selected_quest_log_id.is_none());
    }
    let loader = env.loader_env();
    {
        let mut lua = loader.rilua_mut();
        let handle = lua.create_userdata(Rc::clone(env.state()));
        lua.set_global_val("OfferObservedState", Val::Userdata(handle.gc_ref()))
            .unwrap();
        lua.register_function("ObserveOfferState", observe_offer)
            .unwrap();
    }
    exec(
        &env,
        r#"
        OfferDetails = {}; OfferAccepted = 0
        local observer = CreateFrame("Frame")
        observer:RegisterEvent("QUEST_DETAIL")
        observer:RegisterEvent("QUEST_ACCEPTED")
        observer:SetScript("OnEvent", function(_, event)
            if event == "QUEST_DETAIL" then
                OfferDetails[#OfferDetails+1] = {ObserveOfferState(OfferObservedState)}
            else
                OfferAccepted = OfferAccepted + 1
            end
        end)
    "#,
    );
    env
}

fn preserved_state(env: &WowLuaEnv) -> String {
    let sim = env.state().borrow();
    format!(
        "{:?}|{:?}|{:?}|{}|{}",
        sim.quest_log, sim.quest_log_entries, sim.gossip, sim.player.money, sim.player.xp
    )
}

fn install_widgets(env: &WowLuaEnv) {
    exec(
        env,
        r#"
        GossipFrame = CreateFrame("Frame", "GossipFrame")
        GossipFrame.buttonIndex = 1
        for i = 1, 32 do
            local name = "GossipTitleButton" .. i
            local button = CreateFrame("Button", name, GossipFrame)
            button:SetFontString(button:CreateFontString(nil, "ARTWORK"))
            button:Hide()
            button:CreateTexture(name .. "GossipIcon", "ARTWORK")
        end
        GreetingButton = CreateFrame("Button")
        GreetingButton.isActive = 0
    "#,
    );
}

fn assert_selected(env: &WowLuaEnv, index: u32) {
    let (id, title, level) = if index == 1 {
        (990101, "Wrath supplied first", 17)
    } else {
        (990202, "Wrath supplied second", 23)
    };
    let sim = env.state().borrow();
    assert_eq!(sim.selected_quest_log_id, Some(id));
    let offer = sim
        .pending_quest_offer
        .as_ref()
        .expect("selected pending snapshot");
    assert_eq!(
        (offer.quest_id, offer.title.as_str(), offer.level),
        (id as i32, title, level)
    );
    assert!(
        !sim.events
            .pending()
            .iter()
            .any(|event| event.name == "QUEST_ACCEPTED")
    );
    drop(sim);
    exec(
        env,
        &format!(
            r#"
        assert(#OfferDetails == 1, "exactly one detail notification")
        local seen = OfferDetails[1]
        assert(seen[1] == {id} and seen[2] == {id}, "selection and pending committed before event")
        assert(seen[3] == "{title}" and seen[4] == {level}, "pending snapshot at dispatch")
        assert(OfferAccepted == 0, "no acceptance")
    "#
        ),
    );
}

fn exercise_click(index: u32, greeting: bool) {
    let env = seed_env();
    let independent = seed_env();
    let before = preserved_state(&env);
    let independent_before = preserved_state(&independent);
    load_originals(&env);
    install_widgets(&env);
    if greeting {
        exec(
            &env,
            &format!("GreetingButton:SetID({index}); QuestTitleButton_OnClick(GreetingButton)"),
        );
    } else {
        exec(
            &env,
            r#"
            GossipFrameAvailableQuestsUpdate(GetGossipAvailableQuests())
            assert(GossipTitleButton1:IsShown() and GossipTitleButton2:IsShown())
            assert(GossipTitleButton1:GetID() == 1 and GossipTitleButton2:GetID() == 2)
            assert(GossipTitleButton1.type == "Available" and GossipTitleButton2.type == "Available")
            assert(GossipTitleButton1:GetText() == string.format(NORMAL_QUEST_DISPLAY, "Wrath supplied first"))
            assert(GossipTitleButton2:GetText() == string.format(NORMAL_QUEST_DISPLAY, "Wrath supplied second"))
        "#,
        );
        exec(
            &env,
            &format!("GossipTitleButton_OnClick(GossipTitleButton{index}, 'LeftButton')"),
        );
    }
    assert_selected(&env, index);
    assert_eq!(
        preserved_state(&env),
        before,
        "membership/history/money/XP/supplied rows unchanged"
    );
    assert_eq!(preserved_state(&independent), independent_before);
    assert!(independent.state().borrow().selected_quest_log_id.is_none());
    assert!(independent.state().borrow().pending_quest_offer.is_none());
    exec(
        &independent,
        "assert(#OfferDetails == 0 and OfferAccepted == 0)",
    );
    assert_error_boundaries(&env);
}

#[test]
fn available_quest_true_flags_are_numeric_one() {
    let env = seed_env();
    {
        let mut sim = env.state().borrow_mut();
        sim.gossip.available_quests.truncate(1);
        sim.gossip.num_available_quests = 1;
        sim.gossip.available_quests[0].is_trivial = true;
        sim.gossip.available_quests[0].repeatable = Some(true);
    }
    exec(
        &env,
        r#"
        assert(select('#', GetGossipAvailableQuests()) == 5, "five raw returns per quest")
        local title, level, isLowLevel, isDaily, isRepeatable = GetGossipAvailableQuests()
        assert(title == "Wrath supplied first" and level == 17)
        assert(type(isLowLevel) == "number" and isLowLevel == 1, "trivial flag must be numeric 1")
        assert(type(isDaily) == "nil", "daily data remains absent")
        assert(type(isRepeatable) == "number" and isRepeatable == 1, "repeatable flag must be numeric 1")
    "#,
    );
}

#[test]
fn available_quest_false_flags_are_nil() {
    let env = seed_env();
    {
        let mut sim = env.state().borrow_mut();
        sim.gossip.available_quests.truncate(1);
        sim.gossip.num_available_quests = 1;
    }
    exec(
        &env,
        r#"
        assert(select('#', GetGossipAvailableQuests()) == 5, "five raw returns including trailing nils")
        local title, level, isLowLevel, isDaily, isRepeatable = GetGossipAvailableQuests()
        assert(title == "Wrath supplied first" and level == 17)
        assert(type(isLowLevel) == "nil", "non-trivial flag must be nil")
        assert(type(isDaily) == "nil", "daily data remains absent")
        assert(type(isRepeatable) == "nil", "non-repeatable flag must be nil")
    "#,
    );
}

#[test]
fn gossip_index_one() {
    exercise_click(1, false);
}

#[test]
fn gossip_index_two() {
    exercise_click(2, false);
}

#[test]
fn greeting_index_one_control() {
    exercise_click(1, true);
}

#[test]
fn greeting_index_two_control() {
    exercise_click(2, true);
}
