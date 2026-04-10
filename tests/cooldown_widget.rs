use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::widget::Color;

#[test]
fn cooldown_widget_methods_persist_runtime_state() {
    let env = WowLuaEnv::new().unwrap();

    let result: String = env
        .eval(
            r#"
            local cd = CreateFrame("Cooldown", "TestCooldown", UIParent)

            if cd:GetCooldownDisplayDuration() ~= 0 then
                return "display_duration_should_default_zero"
            end
            if cd:GetDrawBling() ~= true then
                return "draw_bling_should_default_true"
            end
            if cd:GetDrawEdge() ~= false then
                return "draw_edge_should_default_false"
            end
            if cd:GetDrawSwipe() ~= true then
                return "draw_swipe_should_default_true"
            end
            if cd:GetEdgeScale() ~= 1 then
                return "edge_scale_should_default_one"
            end
            if cd:GetMinimumCountdownDuration() ~= 0 then
                return "minimum_countdown_should_default_zero"
            end

            cd:SetDrawBling(false)
            cd:SetDrawEdge(true)
            cd:SetDrawSwipe(false)
            cd:SetEdgeScale(1.5)
            cd:SetEdgeColor(0.1, 0.2, 0.3, 0.4)
            cd:SetMinimumCountdownDuration(2500)
            cd:SetCountdownFont("GameFontHighlightSmall")
            cd:SetCooldownFromExpirationTime(20, 8, 1.25)

            local startTime, duration = cd:GetCooldownTimes()
            if startTime ~= 12 or duration ~= 8 then
                return "expiration_time_should_convert_to_start_and_duration"
            end
            if cd:GetCooldownDisplayDuration() ~= 8000 then
                return "display_duration_should_be_milliseconds"
            end
            if cd:GetDrawBling() ~= false then
                return "draw_bling_should_round_trip"
            end
            if cd:GetDrawEdge() ~= true then
                return "draw_edge_should_round_trip"
            end
            if cd:GetDrawSwipe() ~= false then
                return "draw_swipe_should_round_trip"
            end
            if cd:GetEdgeScale() ~= 1.5 then
                return "edge_scale_should_round_trip"
            end
            if cd:GetMinimumCountdownDuration() ~= 2500 then
                return "minimum_countdown_should_round_trip"
            end

            local countdown = cd:GetCountdownFontString()
            if countdown == nil then
                return "countdown_font_string_should_exist"
            end
            if countdown:GetObjectType() ~= "FontString" then
                return "countdown_font_string_should_be_fontstring"
            end

            local durationObject = {
                GetStartTime = function() return 30 end,
                GetTotalDuration = function() return 4 end,
                GetModRate = function() return 2 end,
                IsZero = function() return false end,
            }
            cd:SetCooldownFromDurationObject(durationObject, true)
            startTime, duration = cd:GetCooldownTimes()
            if startTime ~= 30 or duration ~= 4 then
                return "duration_object_should_update_cooldown"
            end
            if cd:GetCooldownDisplayDuration() ~= 4000 then
                return "duration_object_display_duration_should_use_milliseconds"
            end

            local zeroDurationObject = {
                GetStartTime = function() return 99 end,
                GetTotalDuration = function() return 0 end,
                GetModRate = function() return 1 end,
                IsZero = function() return true end,
            }
            cd:SetCooldownFromDurationObject(zeroDurationObject, true)
            startTime, duration = cd:GetCooldownTimes()
            if startTime ~= 0 or duration ~= 0 then
                return "zero_duration_object_should_clear_cooldown"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");

    let state = env.state().borrow();
    let cooldown_id = state.widgets.get_id_by_name("TestCooldown").unwrap();
    let cooldown = state.widgets.get(cooldown_id).unwrap();

    assert_eq!(cooldown.cooldown_edge_scale, 1.5);
    assert_eq!(cooldown.cooldown_min_countdown_duration_ms, 2500.0);
    assert_eq!(
        cooldown.cooldown_edge_color,
        Color::new(0.1, 0.2, 0.3, 0.4),
        "SetEdgeColor should persist the cooldown edge tint"
    );
    assert!(
        cooldown.cooldown_countdown_font_string_id.is_some(),
        "SetCountdownFont/GetCountdownFontString should create and retain a countdown fontstring"
    );
}
