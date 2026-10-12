//! Real current-profile QuestUtils currency rewards and their independent inputs.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{CurrencyInfo, QuestRewardCurrency};

const QUEST_ID: i32 = 901001;
const CURRENCY_ID: i32 = 90001;
const REWARD_ICON: &str = "Interface\\Icons\\INV_Misc_QuestionMark";

fn probe(env: &WowLuaEnv, body: &str) {
    env.exec(body)
        .expect("public War Mode reward input behavior");
}

fn currency_reward_env() -> WowLuaEnv {
    let env = load_currency_reward_consumer();
    supply_currency_reward(&env);
    env
}

fn load_currency_reward_consumer() -> WowLuaEnv {
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path()
        .expect("current profile Blizzard cache");
    let (env, loaded) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_FrameXMLUtil"],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_FrameXMLUtil"));
    let source: String = env
        .eval("return debug.getinfo(QuestUtils_AddQuestCurrencyRewardsToTooltip, 'S').source")
        .expect("loaded public QuestUtils closure provenance");
    eprintln!("Loaded public currency reward consumer: {source}");
    env
}

fn supply_currency_reward(env: &WowLuaEnv) {
    let mut quest = env.state().borrow().quest_log_entries.entries[0].clone();
    quest.quest_id = QUEST_ID;
    quest.title = "War Mode currency fixture".to_string();
    quest.is_world_quest = true;
    quest.currency_rewards = vec![build_currency_reward(CURRENCY_ID, 7, "Fixture Marks")];
    let mut sim = env.state().borrow_mut();
    sim.quest_log_entries.entries.push(quest);
    sim.currency_info.insert(
        CURRENCY_ID,
        CurrencyInfo {
            name: "Fixture Marks".to_string(),
            quality: 2,
            ..CurrencyInfo::default()
        },
    );
}

fn build_currency_reward(currency_id: i32, amount: i32, name: &str) -> QuestRewardCurrency {
    QuestRewardCurrency {
        currency_id,
        name: name.to_string(),
        texture: REWARD_ICON.to_string(),
        total_reward_amount: amount,
        base_reward_amount: None,
    }
}

fn reward_lines(env: &WowLuaEnv) -> Vec<String> {
    probe(
        env,
        r#"
        GameTooltip:ClearLines()
        local added, container, info = QuestUtils_AddQuestCurrencyRewardsToTooltip(
            901001, GameTooltip, nil, nil)
        assert(added == 1 and container == false and info == nil,
            'actual ordinary currency reward result')
        "#,
    );
    let sim = env.state().borrow();
    let id = sim.widgets.get_id_by_name("GameTooltip").unwrap();
    sim.tooltips
        .get(&id)
        .unwrap()
        .lines
        .iter()
        .map(|line| line.left_text.clone())
        .collect()
}

#[test]
fn quest_currency_war_mode_desired_has_one_boolean_result() {
    let env = WowLuaEnv::new().unwrap();
    probe(
        &env,
        r#"
        local function desired(...)
            assert(select('#', ...) == 1, 'desired query returns one value')
            local value = ...
            assert(type(value) == 'boolean', 'War Mode desired is a real boolean')
            assert(value == false, 'normal modeled mode initially disabled')
        end
        desired(C_PvP.IsWarModeDesired())
        assert(C_PvP.GetWarModeRewardBonus() == 10, 'preserve existing simulator percentage')
        assert(C_PvP.GetWarModeRewardBonusDefault() == 10, 'separate default query unchanged')
    "#,
    );
}

#[test]
fn quest_currency_war_mode_eligibility_requires_numeric_identity() {
    let env = WowLuaEnv::new().unwrap();
    probe(
        &env,
        r#"
        local get = C_QuestLog.QuestCanHaveWarModeBonus
        for _, value in ipairs({'901001', false, {}, function() end}) do
            local ok, message = pcall(get, value)
            assert(not ok and type(message) == 'string' and string.find(message, 'number', 1, true),
                'quest eligibility must reject nonnumeric identity')
        end
        assert(not pcall(get), 'quest identity required')
        assert(not pcall(get, nil), 'nil quest identity rejected')
        assert(get(901001) == false, 'absent eligibility is modeled false, not native catalog truth')
    "#,
    );
}

#[test]
fn quest_currency_war_mode_faction_query_requires_numeric_identity() {
    let env = WowLuaEnv::new().unwrap();
    probe(
        &env,
        r#"
        local get = C_CurrencyInfo.GetFactionGrantedByCurrency
        for _, value in ipairs({'90001', false, {}, function() end}) do
            local ok, message = pcall(get, value)
            assert(not ok and type(message) == 'string' and string.find(message, 'number', 1, true),
                'faction query must reject nonnumeric identity')
        end
        assert(not pcall(get), 'currency identity required')
        assert(not pcall(get, nil), 'nil currency identity rejected')
        local function absent(...)
            assert(select('#', ...) == 1, 'faction query returns one value')
            assert((...) == nil, 'no supplied faction relation')
        end
        absent(get(90001))
    "#,
    );
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn quest_currency_war_mode_relation_queries_authenticate_tainted_secrets_first() {
    let env = WowLuaEnv::new().unwrap();
    probe(
        &env,
        r#"
        debug.settaintmode(true)
        local valid, invalid = secretwrap(901001), secretwrap('invalid')
        local getters = {C_QuestLog.QuestCanHaveWarModeBonus, C_CurrencyInfo.GetFactionGrantedByCurrency}
        local function addon()
            for _, get in ipairs(getters) do
                for _, value in ipairs({valid, invalid}) do
                    local ok, message = pcall(get, value)
                    assert(not ok and type(message) == 'string', 'tainted relation secret denied')
                    assert(string.find(message, 'untainted caller', 1, true),
                        'authentication precedes type and lookup')
                    assert(issecretvalue(value), 'wrapper preserved after denial')
                    assert(not pcall(secretunwrap, value), 'no declassification')
                    assert(debug.getstacktaint() == 'QuestCurrencyWarModeProbe', 'caller taint preserved')
                end
            end
        end
        debug.setobjecttaint(addon, 'QuestCurrencyWarModeProbe')
        addon()
        assert(debug.getstacktaint() == nil, 'caller restored')
    "#,
    );
}

#[test]
fn quest_currency_war_mode_actual_vendor_ordinary_reward_baseline() {
    let env = currency_reward_env();
    assert_eq!(
        reward_lines(&env),
        vec![format!("|T{REWARD_ICON}:16:16|t 7 Fixture Marks")]
    );
    assert!(env.state().borrow().lua_errors.is_empty());
}
