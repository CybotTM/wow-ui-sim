use super::discrete_fixtures::assert_lua;
use super::fixtures::{ID, SET_ID, assert_initial_snapshot, entry, seed_record, supplied_record};
use wow_ui_sim::c_api::ui_widget_manager::{WidgetId, WidgetSetRecord, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) fn supply_empty_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0, // UIWidgetSetLayoutDirection.Vertical in both cached docs
            vertical_padding: 7.0,
            widgets: Vec::new(),
        }),
    )
    .expect("supplied empty SpellDisplay set");
}

fn observe_feed(env: &WowLuaEnv) {
    assert_lua(
        env,
        "SpellDisplay real frame event observer",
        r#"
        SpellDisplayFeedUpdates = {}
        SpellDisplayFullUpdates = 0
        SpellDisplayObserver = CreateFrame('Frame')
        SpellDisplayObserver:RegisterEvent('UPDATE_UI_WIDGET')
        SpellDisplayObserver:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
        SpellDisplayObserver:SetScript('OnEvent', function(_, event, info)
            if event == 'UPDATE_ALL_UI_WIDGETS' then
                SpellDisplayFullUpdates = SpellDisplayFullUpdates + 1
                assert(#C_UIWidgetManager.GetAllWidgetsBySetID(431) == 0)
                return
            end
            assert(info.widgetID == 761301 and info.widgetSetID == 431 and info.widgetType == 13)
            local members = C_UIWidgetManager.GetAllWidgetsBySetID(431)
            assert(#members == 1 and members[1].widgetID == 761301 and members[1].widgetType == 13)
            local row = C_UIWidgetManager.GetSpellDisplayVisualizationInfo(761301)
            AssertSpellSnapshot(row, expectedSpell)
            SpellDisplayFeedUpdates[#SpellDisplayFeedUpdates + 1] = row
        end)
    "#,
    );
}

#[test]
fn spell_display_feed_commits_before_events_retains_visualization_on_membership_removal() {
    let env = WowLuaEnv::new().expect("SpellDisplay feed environment");
    seed_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    supply_empty_set(&env);
    observe_feed(&env);
    let mut initial = supplied_record();
    initial.spell_info.text = "First event spell".into();
    assert_lua(
        &env,
        "expect first committed event snapshot",
        "expectedSpell.spellInfo.text = 'First event spell'",
    );
    env.update_ui_widget(SET_ID, entry(), WidgetVisualization::SpellDisplay(initial))
        .expect("initial typed SpellDisplay feed");
    // Query boundary already precedes event construction; later event proof is separate.
    assert_lua(
        &env,
        "SpellDisplay first event result boundary",
        "assert(#SpellDisplayFeedUpdates == 1, 'initial event must observe supplied record')",
    );
    let mut record = supplied_record();
    record.spell_info.text = "Updated spell".into();
    record.spell_info.tooltip = "Updated tooltip".into();
    assert_lua(
        &env,
        "expect replacement before notification",
        "expectedSpell.spellInfo.text = 'Updated spell'; expectedSpell.spellInfo.tooltip = 'Updated tooltip'",
    );
    env.update_ui_widget(
        SET_ID,
        entry(),
        WidgetVisualization::SpellDisplay(record.clone()),
    )
    .expect("replace visualization without duplicate membership");
    assert_eq!(
        env.state().borrow().ui_widget_inputs.spell_display[&WidgetId::from_number(ID)],
        record
    );
    assert_lua(
        &env,
        "SpellDisplay immutable event snapshots",
        r#"
        assert(#SpellDisplayFeedUpdates == 2 and SpellDisplayFullUpdates == 0)
        assert(SpellDisplayFeedUpdates[1].spellInfo.text == 'First event spell')
        assert(SpellDisplayFeedUpdates[2].spellInfo.text == 'Updated spell')
        assert(SpellDisplayFeedUpdates[1] ~= SpellDisplayFeedUpdates[2])
        assert(SpellDisplayFeedUpdates[1].spellInfo ~= SpellDisplayFeedUpdates[2].spellInfo)
    "#,
    );
    assert!(
        env.remove_ui_widget(SET_ID, ID)
            .expect("membership removal")
    );
    assert!(!env.remove_ui_widget(SET_ID, ID).expect("absent membership"));
    assert_lua(
        &env,
        "SpellDisplay independent visualization survives membership removal",
        r#"
        assert(SpellDisplayFullUpdates == 1 and #SpellDisplayFeedUpdates == 2)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(431) == 0)
        assert(C_UIWidgetManager.GetSpellDisplayVisualizationInfo(761301).spellInfo.text == 'Updated spell')
        assert(SpellDisplayFeedUpdates[1].spellInfo.text == 'First event spell')
    "#,
    );
}

#[test]
fn spell_display_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification() {
    let env = WowLuaEnv::new().expect("SpellDisplay rejected feed environment");
    seed_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    supply_empty_set(&env);
    observe_feed(&env);
    assert!(
        env.update_ui_widget(
            SET_ID + 1.0,
            entry(),
            WidgetVisualization::SpellDisplay(supplied_record()),
        )
        .is_err()
    );
    let mut wrong_type = entry();
    wrong_type.widget_type = 14; // Actual DoubleStateIconRow, not SpellDisplay.
    assert!(
        env.update_ui_widget(
            SET_ID,
            wrong_type,
            WidgetVisualization::SpellDisplay(supplied_record()),
        )
        .is_err()
    );
    assert!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .spell_display
            .get(&WidgetId::from_number(ID))
            == Some(&supplied_record())
    );
    assert_lua(
        &env,
        "SpellDisplay rejected feed has no side effects",
        r#"
        assert(#SpellDisplayFeedUpdates == 0 and SpellDisplayFullUpdates == 0)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(431) == 0)
        assert(C_UIWidgetManager.GetWidgetSetInfo(432) == nil)
    "#,
    );
}
