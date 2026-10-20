use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;
#[path = "common/wrath_original_consumer.rs"]
mod originals;
fn errors_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    originals::load_original_files(&env, &["Fonts.xml", "FontStyles.xml", "UIErrorsFrame.xml"]);
    env.exec("UIErrorsFrame:Show(); UIErrorsFrame:SetPoint('TOPLEFT',UIParent,'TOPLEFT',20,-20)")
        .unwrap();
    env
}
fn original_error(env: &WowLuaEnv, message: &str) {
    env.exec(&format!("assert(UIErrorsFrame:GetScript('OnEvent')); UIErrorsFrame:GetScript('OnEvent')(UIErrorsFrame,'UI_ERROR_MESSAGE','{message}')")).unwrap();
    env.fire_on_update(0.0).unwrap();
}
fn advance_clock(env: &WowLuaEnv, seconds: f64) {
    let mut sim = env.state().borrow_mut();
    sim.start_time = sim
        .start_time
        .checked_sub(std::time::Duration::from_secs_f64(seconds))
        .unwrap();
    drop(sim);
    // Exercise production update boundary; never rewrite individual message timestamps.
    env.fire_on_update(seconds).unwrap();
}
#[cfg(feature = "gui")]
fn glyph_alphas(env: &WowLuaEnv) -> Vec<f32> {
    use std::collections::HashMap;
    use wow_ui_sim::iced_app::{
        DirtyStrataRebuildParams, rebuild_dirty_strata_batches_for_registry,
    };
    use wow_ui_sim::render::shader::GLYPH_ATLAS_TEX_INDEX;
    use wow_ui_sim::render::{GlyphAtlas, WowFontSystem};
    use wow_ui_sim::widget::FrameStrata;
    let mut fonts = WowFontSystem::new_without_casc();
    assert!(
        fonts.font_system.db().faces().next().is_some(),
        "SETUP: fonts required"
    );
    let mut glyphs = GlyphAtlas::new();
    let mut sim = env.state().borrow_mut();
    sim.ensure_layout_rects();
    let id = sim.widgets.get_id_by_name("UIErrorsFrame").unwrap();
    let index = sim.widgets.get(id).unwrap().frame_strata as usize;
    let mut buckets = vec![Vec::new(); FrameStrata::COUNT];
    buckets[index].push(id);
    let mut batches = std::array::from_fn(|_| None);
    let mut snapshots = std::array::from_fn(|_| None);
    let empty_tips = HashMap::new();
    let empty_quests = HashMap::new();
    rebuild_dirty_strata_batches_for_registry(
        &mut batches,
        &mut snapshots,
        &mut Some((&mut fonts, &mut glyphs)),
        DirtyStrataRebuildParams {
            dirty: u16::MAX,
            dirty_ids: None,
            size: iced::Size::new(1600.0, 1200.0),
            strata_buckets: &buckets,
            widgets: &sim.widgets,
            pressed_frame: None,
            hovered_frame: None,
            message_frames: &sim.message_frames,
            tooltip_data: &empty_tips,
            quest_blobs: &empty_quests,
            elapsed_secs: sim.start_time.elapsed().as_secs_f64(),
        },
    );
    batches
        .iter()
        .flatten()
        .flat_map(|batch| batch.vertices.iter())
        .filter(|vertex| vertex.tex_index == GLYPH_ATLAS_TEX_INDEX)
        .map(|vertex| vertex.color[3])
        .collect()
}
#[cfg(feature = "gui")]
fn assert_alpha(env: &WowLuaEnv, expected: f32) {
    let alpha = glyph_alphas(env);
    if expected == 0.0 {
        assert!(alpha.is_empty(), "expired row still emitted: {alpha:?}");
        return;
    }
    assert!(!alpha.is_empty(), "SETUP/output: no glyphs for active row");
    assert!(
        alpha.iter().all(|v| (*v - expected).abs() < 0.04),
        "expected alpha {expected}, got {alpha:?}"
    );
}
#[test]
fn b6_c6_defaults_and_original_xml_duration() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    env.exec("local f=CreateFrame('MessageFrame'); assert(f:GetFading()); assert(f:GetTimeVisible()==10 and f:GetFadeDuration()==3)").unwrap();
    errors_env()
        .exec("assert(UIErrorsFrame:GetTimeVisible()==5 and UIErrorsFrame:GetFadeDuration()==3)")
        .unwrap();
}
#[test]
fn b6_c6_original_consumer_queues_one_row_per_update() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = errors_env();
    env.exec(
        r#"
        local handler = UIErrorsFrame:GetScript('OnEvent')
        handler(UIErrorsFrame, 'UI_ERROR_MESSAGE', 'first')
        handler(UIErrorsFrame, 'UI_ERROR_MESSAGE', 'second')
        assert(UIErrorsFrame:GetNumMessages() == 0, 'AddMessage must queue')
    "#,
    )
    .unwrap();
    env.fire_on_update(0.0).unwrap();
    env.exec("assert(UIErrorsFrame:GetNumMessages() == 1, 'one insertion per update')")
        .unwrap();
    env.fire_on_update(0.0).unwrap();
    env.exec("assert(UIErrorsFrame:GetNumMessages() == 2)")
        .unwrap();
}

#[test]
#[cfg(feature = "client-wrath")]
fn b6_c6_scrolling_history_is_immediate_and_does_not_acquire_row_timers() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("ChatHistory = CreateFrame('ScrollingMessageFrame', 'ChatHistory')")
        .unwrap();
    assert_scrolling_history_lifecycle(&env, "ChatHistory");
}

#[test]
#[cfg(feature = "client-wrath")]
fn b6_c6_xml_scrolling_history_retains_its_lifecycle() {
    let env = WowLuaEnv::new().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let toc = directory.path().join("ScrollingHistory.toc");
    std::fs::write(&toc, "## Title: ScrollingHistory\nhistory.xml\n").unwrap();
    std::fs::write(
        directory.path().join("history.xml"),
        r#"<Ui xmlns="http://www.blizzard.com/wow/ui/">
            <ScrollingMessageFrame name="XmlChatHistory"/>
            <Frame name="HistoryOwnerTemplate" virtual="true">
                <Frames><ScrollingMessageFrame name="$parentHistory"/></Frames>
            </Frame>
        </Ui>"#,
    )
    .unwrap();
    let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
    env.exec("CreateFrame('Frame', 'RuntimeHistoryOwner', nil, 'HistoryOwnerTemplate')")
        .unwrap();
    for name in ["XmlChatHistory", "RuntimeHistoryOwnerHistory"] {
        assert_scrolling_history_lifecycle(&env, name);
    }
}

#[cfg(feature = "client-wrath")]
fn assert_scrolling_history_lifecycle(env: &WowLuaEnv, name: &str) {
    env.exec(&format!(
        r#"
        local frame = {name}
        frame:AddMessage('selected', 0, 1, 0, 71, false, 11, 81)
        frame:AddMessage('other', 1, 0, 0, 72, false, 22, 82)
        assert(frame:GetNumMessages() == 2, 'scrolling history must be immediate')
        assert(frame:GetNumMessages(11) == 1)
        local text, access, line, extra = frame:GetMessageInfo(1, 11)
        assert(text == 'selected' and access == 11 and line == 71 and extra == 81)
        "#
    ))
    .unwrap();
    env.fire_on_update(0.0).unwrap();
    let sim = env.state().borrow();
    let id = sim.widgets.get_id_by_name(name).unwrap();
    let data = &sim.message_frames[&id];
    assert!(data.messages.iter().all(|row| row.wrath_timers.is_none()));
    drop(sim);
    env.exec(&format!(
        r#"
        local frame = {name}
        assert(frame:GetNumMessages() == 2, 'update must not duplicate history')
        frame:RemoveMessagesByAccessID(11)
        assert(frame:GetNumMessages() == 1 and frame:GetMessageInfo(1) == 'other')
        "#
    ))
    .unwrap();
}

#[cfg(feature = "gui")]
#[test]
fn b6_c6_set_time_visible_restarts_nonzero_active_timer() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = errors_env();
    env.exec("UIErrorsFrame:SetTimeVisible(5); UIErrorsFrame:SetFadeDuration(3)")
        .unwrap();
    original_error(&env, "lifetime");
    advance_clock(&env, 4.0);
    env.exec("UIErrorsFrame:SetTimeVisible(5)").unwrap();
    advance_clock(&env, 2.0);
    assert_alpha(&env, 1.0); // old timestamp model predicts about 2/3.
    advance_clock(&env, 4.0);
    assert_alpha(&env, 2.0 / 3.0); // six seconds after reset: one second into 3-second linear fade.
}
#[cfg(feature = "gui")]
#[test]
fn b6_c6_set_fade_duration_restarts_nonzero_fade_timer() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = errors_env();
    env.exec("UIErrorsFrame:SetTimeVisible(5); UIErrorsFrame:SetFadeDuration(3)")
        .unwrap();
    original_error(&env, "fade");
    advance_clock(&env, 6.0);
    assert_alpha(&env, 2.0 / 3.0);
    env.exec("UIErrorsFrame:SetFadeDuration(3)").unwrap();
    advance_clock(&env, 1.0);
    assert_alpha(&env, 2.0 / 3.0); // old model predicts 1/3.
}
#[cfg(feature = "gui")]
#[test]
fn b6_c6_expired_rows_not_revived_by_duration_setters() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = errors_env();
    env.exec("UIErrorsFrame:SetTimeVisible(5); UIErrorsFrame:SetFadeDuration(3)")
        .unwrap();
    original_error(&env, "expired");
    advance_clock(&env, 9.0);
    assert_alpha(&env, 0.0);
    env.exec("UIErrorsFrame:SetTimeVisible(20); UIErrorsFrame:SetFadeDuration(20)")
        .unwrap();
    assert_alpha(&env, 0.0);
}
