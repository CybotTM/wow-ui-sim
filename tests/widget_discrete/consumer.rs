//! Actual cache consumer over typed host feeds; explicit record deletion uses a real event.
use super::fixtures::{ID, SET_ID, assert_initial_snapshot, assert_lua, supplied_record};
use crate::common;
use common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use wow_ui_sim::c_api::ui_widget_manager::{
    DiscreteProgressStepsRecord, WidgetId, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

fn discrete_entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: 19,
        unit_token: None,
    }
}

fn supply_membership(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: vec![discrete_entry()],
        }),
    )
    .expect("host membership supplied before cached registration");
}

fn inject_discrete(env: &WowLuaEnv, record: DiscreteProgressStepsRecord) {
    env.update_ui_widget(
        SET_ID,
        discrete_entry(),
        WidgetVisualization::DiscreteProgressSteps(record),
    )
    .expect("host discrete feed commits record before notification");
}

fn dispatch_incremental_update(env: &WowLuaEnv) {
    let payload = env
        .eval::<rilua::Val>("return {widgetSetID=421, widgetID=760301, widgetType=19}")
        .expect("root incremental event payload");
    env.fire_event_with_args("UPDATE_UI_WIDGET", &[payload])
        .expect("actual registered frame-event dispatch");
}

fn flush_and_assert_counts(env: &WowLuaEnv, count: usize) {
    env.fire_on_update(0.0)
        .expect("flush actual cached container layout");
    assert_lua(
        env,
        "pool/layout after actual OnUpdate",
        &format!(
            r#"
        local c = DiscreteContainer
        assert(not c.dirtyLayout)
        assert(c:GetNumWidgetsShowing() == {count})
        assert(c.widgetPools:GetNumActive() == {count})
        assert(c.numTimers == 0 and c.ticker == nil)
    "#
        ),
    );
}

fn load_cached_consumer(env: &WowLuaEnv) {
    let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("active Retail/PTR cache required; no vendor override");
    eprintln!(
        "[discrete-cache] profile={} path={}",
        wow_ui_sim::client_profile::ACTIVE.cache_subdir(),
        cache.display()
    );
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(wow_ui_sim::screen::ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![cache.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    let loaded = load_blizzard_addon_closure_into_env(
        env,
        &cache,
        &[
            "Blizzard_SharedXMLGame",
            "Blizzard_Fonts_Shared",
            "Blizzard_FrameEffects",
            "Blizzard_UIWidgets",
        ],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_UIWidgets"));
}

fn register_and_assert_geometry(env: &WowLuaEnv) {
    assert_lua(
        env,
        "unchanged cached manager/template and real catalog prerequisites",
        r#"
        assert(Enum.UIWidgetVisualizationType.DiscreteProgressSteps == 19)
        assert(UIWidgetManager:GetWidgetTypeInfo(19))
        -- Source-grounded catalog names; runtime presence/application NOT previously proven.
        -- Prerequisites occur only AFTER genuine supplied-query boundary.
        assert(C_Texture.GetAtlasInfo('eyeofthejailer-bar'), 'catalog bar atlas')
        assert(C_Texture.GetAtlasInfo('eyeofthejailer-runebg'), 'catalog rune atlas')
        for i = 1, 5 do
            assert(C_Texture.GetAtlasInfo('eyeofthejailer-step' .. i .. '-enable'))
            assert(C_Texture.GetAtlasInfo('eyeofthejailer-step' .. i .. '-disable'))
        end
        assert(HelpTip and type(HelpTip.Show) == 'function', 'unchanged tutorial prerequisite')
        function AssertDiscreteGeometry(frame, markers, completed, firstAngle, remaining)
            assert(frame:IsShown())
            assert(frame:GetWidth() == 140 and frame:GetHeight() == 140)
            assert(frame.Steps:GetWidth() == 128 and frame.Steps:GetHeight() == 128)
            assert(frame.Background:IsShown() and frame.Background:GetAtlas() == 'eyeofthejailer-bar')
            assert(frame.Background:GetWidth() == 128 and frame.Background:GetHeight() == 128)
            local _, relative, _, x, y = frame.Steps.BGArray[1]:GetPoint(1)
            local radians = math.rad(firstAngle)
            assert(relative == frame.Steps)
            assert(math.abs(x + 53 * math.sin(radians)) < .0001, 'actual rotated x')
            assert(math.abs(y - 53 * math.cos(radians)) < .0001, 'actual rotated y')
            for i = 1, 5 do
                local background = frame.Steps.BGArray[i]
                assert(background:GetAtlas() == 'eyeofthejailer-runebg', 'applied rune atlas')
                assert(background:GetWidth() == 40 and background:GetHeight() == 40)
                assert(frame.Steps.EnabledArray[i]:GetAtlas() == 'eyeofthejailer-step' .. i .. '-enable')
                assert(frame.Steps.DisabledArray[i]:GetAtlas() == 'eyeofthejailer-step' .. i .. '-disable')
                assert(background:IsShown() == (i <= markers), 'marker count')
                assert(frame.Steps.EnabledArray[i]:IsShown() == (i <= completed), 'completion')
                if i > markers then
                    assert(not frame.Steps.DisabledArray[i]:IsShown())
                    assert(not frame.Steps.ActivationArray[i]:IsShown())
                end
            end
            -- Cached circular Setup calls SetCooldown(GetTime()-100*remaining, 100).
            assert(frame.Bar.Progress:IsPaused(), 'actual cooldown primitive paused')
            local start, duration = frame.Bar.Progress:GetCooldownTimes()
            assert(math.abs(duration - 100000) < .0001)
            assert(math.abs((GetTime()*1000 - start)/duration - remaining) < .0001,
                'gap-adjusted circular progress')
        end
        DiscreteContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
        DiscreteContainer:SetPoint('CENTER', UIParent, 'CENTER')
        DiscreteContainer:RegisterForWidgetSet(421)
        assert(DiscreteContainer:IsRegisteredForWidgetSet(421))
        assert(UIWidgetManager.registeredWidgetContainers[DiscreteContainer])
        assert(DiscreteContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(DiscreteContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        DiscreteFrame = assert(DiscreteContainer.widgetFrames[760301], 'cached template acquired')
        assert(DiscreteFrame.tooltip == 'Supplied discrete tooltip')
        -- Range60/steps4/current40: 2 complete, 3 markers, first angle108, .5 remaining.
        AssertDiscreteGeometry(DiscreteFrame, 3, 2, 108, .5)
    "#,
    );
    flush_and_assert_counts(env, 1);
}

fn update_hide_restore(env: &WowLuaEnv) {
    let mut record = supplied_record();
    record.progress_val = 52.0;
    record.num_steps = 6.0;
    record.tooltip = "Updated discrete tooltip".into();
    inject_discrete(env, record.clone());
    assert_lua(
        env,
        "incremental progress and step geometry",
        r#"
        assert(DiscreteContainer.widgetFrames[760301] == DiscreteFrame)
        assert(DiscreteFrame.tooltip == 'Updated discrete tooltip')
        -- range60 / steps6 = 10; 4 complete. Gap-adjusted value53.8.
        -- percent .73 -> dead-space percent .684 -> remaining .316.
        AssertDiscreteGeometry(DiscreteFrame, 5, 4, 84, .316)
    "#,
    );
    flush_and_assert_counts(env, 1);
    record.shown_state = 0;
    inject_discrete(env, record.clone());
    assert_lua(
        env,
        "hidden adapter releases actual frame",
        r#"
        assert(DiscreteContainer.widgetFrames[760301] == nil)
        assert(not DiscreteFrame:IsShown())
        for i = 1, 5 do
            assert(not DiscreteFrame.Steps.ActivationAnimArray[i]:IsPlaying())
            assert(not DiscreteFrame.Steps.DisableAnimArray[i]:IsPlaying())
        end
    "#,
    );
    flush_and_assert_counts(env, 0);
    record.shown_state = 1;
    record.progress_val = 16.0;
    record.num_steps = 2.0;
    inject_discrete(env, record);
    assert_lua(
        env,
        "shown restores pooled template with changed geometry",
        r#"
        DiscreteRestored = assert(DiscreteContainer.widgetFrames[760301])
        -- Range60 / steps2 = 30; 0 complete. Adjusted value15.4.
        -- normalized .09 -> dead-space .172 -> remaining .828.
        AssertDiscreteGeometry(DiscreteRestored, 1, 0, 180, .828)
        assert(DiscreteRestored.tooltip == 'Updated discrete tooltip')
    "#,
    );
    flush_and_assert_counts(env, 1);
}

fn remove_and_unregister(env: &WowLuaEnv) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .discrete_progress_steps
        .remove(&WidgetId::from_number(ID));
    dispatch_incremental_update(env);
    assert_lua(
        env,
        "record removal",
        r#"
        assert(DiscreteContainer.widgetFrames[760301] == nil)
        assert(not DiscreteRestored:IsShown())
    "#,
    );
    flush_and_assert_counts(env, 0);
    inject_discrete(env, supplied_record());
    flush_and_assert_counts(env, 1);
    assert!(
        env.remove_ui_widget(SET_ID, ID)
            .expect("host membership removal triggers real reconciliation")
    );
    assert_lua(
        env,
        "membership removal",
        r#"
        assert(next(DiscreteContainer.widgetFrames) == nil)
    "#,
    );
    flush_and_assert_counts(env, 0);
    inject_discrete(env, supplied_record());
    flush_and_assert_counts(env, 1);
    assert_lua(
        env,
        "unregister releases active template and subscriptions",
        r#"
        DiscreteBeforeUnregister = assert(DiscreteContainer.widgetFrames[760301])
        DiscreteContainer:UnregisterForWidgetSet()
        assert(not DiscreteContainer:IsRegisteredForWidgetSet(421))
        assert(not UIWidgetManager.registeredWidgetContainers[DiscreteContainer])
        assert(not DiscreteContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not DiscreteContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(DiscreteContainer.widgetFrames) == nil)
        assert(not DiscreteBeforeUnregister:IsShown())
    "#,
    );
    inject_discrete(env, supplied_record());
    env.fire_event("UPDATE_ALL_UI_WIDGETS")
        .expect("unregistered full event ignored");
    flush_and_assert_counts(env, 0);
    assert_lua(
        env,
        "no post-unregister reacquisition",
        r#"
        assert(next(DiscreteContainer.widgetFrames) == nil)
        assert(DiscreteContainer.horizontalRowContainerPool:GetNumActive() == 0)
    "#,
    );
}

#[test]
fn discrete_actual_cached_manager_updates_hides_restores_removes_unregisters() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().expect("isolated consumer environment");
        supply_membership(&env);
        inject_discrete(&env, supplied_record());
        assert_initial_snapshot(&env);
        // Query still precedes cached-template/atlas prerequisites.
        load_cached_consumer(&env);
        register_and_assert_geometry(&env);
        update_hide_restore(&env);
        remove_and_unregister(&env);
    });
}
