//! New original frames restore selected layout, not old frame/Lua identities.
use wow_ui_sim::lua_api::state::MirrorTimer;

const CONTROL_FRAMES: &str = r#"
    for _, name in ipairs({'ReloadResizable', 'ReloadNotPlaced', 'ReloadImmovable', 'ReloadDontSave'}) do
        local f = CreateFrame('Frame', name, UIParent)
        f:SetSize(120, 80)
        f:SetPoint('BOTTOMLEFT', UIParent, 'BOTTOMLEFT', 10, 20)
        f:SetFrameLevel(7)
    end
"#;

pub fn assert_placed_frames() {
    let mut env = super::original::load_original_wrath_startup();
    env.exec(CONTROL_FRAMES).unwrap();
    env.set_reload_loader(|env, _| {
        super::original::load_original_wrath_ui(env);
        env.exec(CONTROL_FRAMES)
    });
    env.exec(
        r#"
        WorldMapFrame:SetMovable(true)
        WorldMapFrame:SetUserPlaced(true)
        WorldMapFrame:SetDontSavePosition(false)
        WorldMapFrame:ClearAllPoints()
        WorldMapFrame:SetPoint('BOTTOMLEFT', UIParent, 'BOTTOMLEFT', 81, 93)
        WorldMapFrame:SetSize(713, 491)
        WorldMapFrame:SetFrameLevel(42)
        ReloadResizable:SetResizable(true)
        ReloadResizable:SetUserPlaced(true)
        ReloadNotPlaced:SetMovable(true)
        ReloadDontSave:SetMovable(true)
        ReloadDontSave:SetUserPlaced(true)
        ReloadDontSave:SetDontSavePosition(true)
        ReloadImmovable:SetUserPlaced(true)
        for _, f in ipairs({ReloadResizable, ReloadNotPlaced, ReloadDontSave, ReloadImmovable}) do
            f:ClearAllPoints()
            f:SetPoint('BOTTOMLEFT', UIParent, 'BOTTOMLEFT', 177, 188)
            f:SetSize(321, 219)
            f:SetFrameLevel(31)
        end
        ReloadUnnamed = CreateFrame('Frame', nil, UIParent)
        ReloadUnnamed:SetMovable(true)
        ReloadUnnamed:SetUserPlaced(true)
        ReloadMissing = CreateFrame('Frame', 'ReloadMissing', UIParent)
        ReloadMissing:SetMovable(true)
        ReloadMissing:SetUserPlaced(true)
        OldReloadSentinel = {}
        C_Timer.After(0, function() OldReloadTimer = true end)
        ReloadUI()
    "#,
    )
    .unwrap();
    assert!(env.service_reload(&mut None).unwrap());
    env.exec(r#"
        assert(OldReloadSentinel == nil and ReloadUnnamed == nil and ReloadMissing == nil, 'no old globals/frames')
        local function placement(f, width, height, level, x, y)
            assert(f:GetWidth() == width and f:GetHeight() == height, 'placed size for '..tostring(f:GetName()))
            assert(f:GetFrameLevel() == level, 'placed frame level')
            local point, relative, relativePoint, actualX, actualY = f:GetPoint()
            assert(point == 'BOTTOMLEFT' and relative == UIParent and relativePoint == 'BOTTOMLEFT', 'new named anchor target')
            assert(actualX == x and actualY == y, 'placed offsets')
        end
        placement(WorldMapFrame, 713, 491, 42, 81, 93)
        placement(ReloadResizable, 321, 219, 31, 177, 188)
        for _, f in ipairs({ReloadNotPlaced, ReloadDontSave, ReloadImmovable}) do placement(f, 120, 80, 7, 10, 20) end
    "#).unwrap();
    wow_ui_sim::startup::run_lua_error_update_ticks(&env);
    assert!(env.eval::<bool>("return OldReloadTimer == nil").unwrap());
}

pub fn assert_selective_timers() {
    let mut env = super::original::load_original_wrath_startup();
    {
        let mut state = env.state().borrow_mut();
        state.in_world = true;
        state.player.class_index = 7;
        state.world.mirror_timers = vec![MirrorTimer {
            name: "BREATH".into(),
            start_value: 30000.0,
            max_value: 60000.0,
            scale: -1.0,
            label: "Breath".into(),
            ..Default::default()
        }];
        state.totem_slots = vec![
            Some(wow_ui_sim::lua_api::globals::real::totems::Totem {
                name: "Reload fire totem".into(),
                start_time: 0.0,
                duration: 600.0,
                icon: 135817,
            }),
            None,
            None,
            None,
        ];
    }
    let guid: String = env.eval("return UnitGUID('player')").unwrap();
    env.fire_event("PLAYER_ENTERING_WORLD").unwrap();
    env.exec("assert(GetMirrorTimerInfo(1) == 'BREATH'); assert(TotemFrame.activeTotems == 1, 'original totem consumer')").unwrap();
    env.set_reload_loader(|env, _| {
        super::original::load_original_wrath_ui(env);
        Ok(())
    });
    env.exec("ReloadUI()").unwrap();
    assert!(env.service_headless_reload(&mut None).unwrap());
    assert_eq!(
        env.eval::<String>("return GetMirrorTimerInfo(1)").unwrap(),
        "UNKNOWN",
        "12340 LeaveWorld cancels mirror timers"
    );
    assert_eq!(
        env.eval::<String>("return UnitGUID('player')").unwrap(),
        guid
    );
    env.exec(r#"
        assert(GetTotemInfo(1) == true and TotemFrame.activeTotems == 1, 'totem survives and original fresh consumer renders it')
        for i = 1, MIRRORTIMER_NUMTIMERS do assert(_G['MirrorTimer'..i].timer == nil, 'fresh original mirror bars do not restart') end
    "#).unwrap();
}
