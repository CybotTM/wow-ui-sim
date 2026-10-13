//! Typed host feeds publish observable membership and records before synchronous events.
use super::fixtures::{ID, SET_ID, assert_lua, supplied_record};
use wow_ui_sim::c_api::ui_widget_manager::{WidgetSetEntry, WidgetSetRecord, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

fn entry(widget_id: f64, widget_type: u32) -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id,
        widget_type,
        unit_token: None,
    }
}

fn supply_empty_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: Vec::new(),
        }),
    )
    .expect("supplied empty set");
}

fn register_feed_observer(env: &WowLuaEnv) {
    assert_lua(
        env,
        "synchronous event observer",
        r#"
        DiscreteFeedUpdates = {}
        DiscreteFullUpdates = 0
        DiscreteFeedObserver = CreateFrame('Frame')
        DiscreteFeedObserver:RegisterEvent('UPDATE_UI_WIDGET')
        DiscreteFeedObserver:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
        DiscreteFeedObserver:SetScript('OnEvent', function(_, event, info)
            if event == 'UPDATE_ALL_UI_WIDGETS' then
                DiscreteFullUpdates = DiscreteFullUpdates + 1
                assert(#C_UIWidgetManager.GetAllWidgetsBySetID(421) == 0,
                    'removal is visible before full notification')
                return
            end
            assert(info.widgetID == 760301 and info.widgetSetID == 421 and info.widgetType == 19)
            local members = C_UIWidgetManager.GetAllWidgetsBySetID(421)
            assert(#members == 1 and members[1].widgetID == 760301 and members[1].widgetType == 19)
            local row = C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(760301)
            assert(type(row) == 'table', 'record committed before incremental notification')
            DiscreteFeedUpdates[#DiscreteFeedUpdates + 1] = row
        end)
    "#,
    );
}

fn create_feed_environment() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("discrete feed environment");
    supply_empty_set(&env);
    register_feed_observer(&env);
    env
}

#[test]
fn discrete_host_feed_commits_before_events_and_preserves_record_after_membership_removal() {
    let env = create_feed_environment();
    env.update_ui_widget(
        SET_ID,
        entry(ID, 19),
        WidgetVisualization::DiscreteProgressSteps(supplied_record()),
    )
    .expect("initial typed feed");
    let mut record = supplied_record();
    record.progress_val = 52.0;
    record.num_steps = 6.0;
    env.update_ui_widget(
        SET_ID,
        entry(ID, 19),
        WidgetVisualization::DiscreteProgressSteps(record),
    )
    .expect("replace typed visualization without duplicate membership");
    assert_lua(
        &env,
        "two synchronous snapshots",
        r#"
        assert(#DiscreteFeedUpdates == 2 and DiscreteFullUpdates == 0)
        assert(DiscreteFeedUpdates[1].progressVal == 40 and DiscreteFeedUpdates[1].numSteps == 4)
        assert(DiscreteFeedUpdates[2].progressVal == 52 and DiscreteFeedUpdates[2].numSteps == 6)
        assert(DiscreteFeedUpdates[1] ~= DiscreteFeedUpdates[2])
    "#,
    );
    assert!(
        env.remove_ui_widget(SET_ID, ID)
            .expect("membership removal")
    );
    assert!(!env.remove_ui_widget(SET_ID, ID).expect("absent membership"));
    assert_lua(
        &env,
        "removal retains independently queried record",
        r#"
        assert(DiscreteFullUpdates == 1 and #DiscreteFeedUpdates == 2)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(421) == 0)
        assert(C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(760301).progressVal == 52)
        assert(DiscreteFeedUpdates[1].progressVal == 40)
    "#,
    );
}

#[test]
fn discrete_host_feed_rejects_unknown_set_and_type_mismatch_without_mutation_or_events() {
    let env = create_feed_environment();
    assert!(
        env.update_ui_widget(
            SET_ID + 1.0,
            entry(ID, 19),
            WidgetVisualization::DiscreteProgressSteps(supplied_record()),
        )
        .is_err()
    );
    assert!(
        env.update_ui_widget(
            SET_ID,
            entry(ID, 2),
            WidgetVisualization::DiscreteProgressSteps(supplied_record()),
        )
        .is_err()
    );
    assert_lua(
        &env,
        "no rejected-feed side effects",
        r#"
        assert(#DiscreteFeedUpdates == 0 and DiscreteFullUpdates == 0)
        assert(C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(760301) == nil)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(421) == 0)
        assert(C_UIWidgetManager.GetWidgetSetInfo(422) == nil)
    "#,
    );
}
