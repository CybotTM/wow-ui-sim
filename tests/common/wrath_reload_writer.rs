//! Authenticated external grammar; numeric values use documented simulator units.
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::SavedVariablesManager;

pub fn assert_writer_grammar_and_rounding() {
    let mut env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1000.0, 800.0);
    env.exec(
        r#"
        UIParent:SetScale(1)
        local relative = CreateFrame('Frame', nil, UIParent)
        relative:SetSize(50, 50)
        relative:SetPoint('BOTTOMLEFT', UIParent, 'BOTTOMLEFT', 41, -31)
        local frame = CreateFrame('Frame', 'ReloadWriter', UIParent)
        frame:SetScale(0.75)
        frame:SetSize(200, 100)
        frame:SetPoint('BOTTOMLEFT', relative, 'BOTTOMLEFT', 17, 23)
        frame:SetFrameLevel(9)
        frame:SetMovable(true)
        frame:SetResizable(true)
        frame:SetUserPlaced(true)
        ReloadUI()
    "#,
    )
    .unwrap();
    let storage = tempfile::tempdir().unwrap();
    let mut manager = Some(SavedVariablesManager::with_storage_dir(
        storage.path().to_path_buf(),
    ));
    env.service_reload(&mut manager).unwrap();
    let path = storage.path().join("SimRealm/SimPlayer/layout-local.txt");
    assert!(
        path.is_file(),
        "12340 per-character placement snapshot must be written"
    );
    let written = std::fs::read_to_string(path).unwrap();
    assert_eq!(
        written,
        "Version: 1\nFrame: ReloadWriter\nFrameLevel: 9\nAnchor: BOTTOMLEFT\nX: 72\nY: -18\nW: 200\nH: 100\n"
    );
    assert!(
        env.eval::<bool>("return ReloadWriter == nil").unwrap(),
        "missing new frame is ignored, not recreated from storage"
    );
}

pub fn assert_inherited_scale_roundtrip() {
    let mut env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1000.0, 800.0);
    env.exec("local p = CreateFrame('Frame', 'ReloadScaledParent', UIParent); p:SetScale(2); p:SetSize(100,100); p:SetPoint('BOTTOMLEFT', UIParent, 'BOTTOMLEFT',40,20); local f = CreateFrame('Frame', 'ReloadInheritedScale', p); f:SetScale(0.75); f:SetSize(200,100); f:SetPoint('BOTTOMLEFT',p,'BOTTOMLEFT',17,23); f:SetMovable(true); f:SetResizable(true); f:SetUserPlaced(true); ReloadUI()").unwrap();
    env.set_reload_loader(|env, _| env.exec("local p = CreateFrame('Frame', 'ReloadScaledParent', UIParent); p:SetScale(0.5); p:SetSize(100,100); p:SetPoint('CENTER',UIParent,'CENTER'); local f = CreateFrame('Frame', 'ReloadInheritedScale',p); f:SetSize(9,8); f:SetMovable(true); f:SetResizable(true)"));
    let storage = tempfile::tempdir().unwrap();
    let mut manager = Some(SavedVariablesManager::with_storage_dir(
        storage.path().to_path_buf(),
    ));
    env.service_reload(&mut manager).unwrap();
    let written =
        std::fs::read_to_string(storage.path().join("SimRealm/SimPlayer/layout-local.txt"))
            .unwrap();
    assert!(
        written.contains("Anchor: BOTTOMLEFT\nX: 70\nY: 50\nW: 200\nH: 100\n"),
        "inherited scale divides both axes and size: {written}"
    );
    env.exec("assert(ReloadInheritedScale:GetEffectiveScale() == 0.5, 'scale is not restored'); assert(ReloadInheritedScale:GetWidth() == 200 and ReloadInheritedScale:GetHeight() == 100); local p,r,rp,x,y = ReloadInheritedScale:GetPoint(); assert(p == 'BOTTOMLEFT' and rp == p and r == UIParent and x == 70 and y == 50)").unwrap();
}

pub fn assert_empty_writer() {
    let mut env = WowLuaEnv::new().unwrap();
    let storage = tempfile::tempdir().unwrap();
    let mut manager = Some(SavedVariablesManager::with_storage_dir(
        storage.path().to_path_buf(),
    ));
    env.exec("ReloadUI()").unwrap();
    env.service_reload(&mut manager).unwrap();
    let path = storage.path().join("SimRealm/SimPlayer/layout-local.txt");
    assert!(
        path.is_file(),
        "empty snapshot still rewrites the version header"
    );
    assert_eq!(std::fs::read_to_string(path).unwrap(), "Version: 1\n");
}

pub fn assert_disabled_persistence_writes_no_layout() {
    let data = std::path::PathBuf::from(std::env::var_os("XDG_DATA_HOME").unwrap());
    let storage = data.join("wow-sim/SavedVariables");
    let existed = storage.try_exists().unwrap();
    let mut env = WowLuaEnv::new().unwrap();
    env.exec("local f = CreateFrame('Frame', 'ReloadDisabledWriter', UIParent); f:SetMovable(true); f:SetUserPlaced(true); f:SetSize(200, 100); f:SetPoint('CENTER', UIParent, 'CENTER'); ReloadUI()").unwrap();
    env.service_reload(&mut None).unwrap();
    assert_eq!(
        storage.try_exists().unwrap(),
        existed,
        "manager None must not create a persistence root"
    );
    assert!(
        !storage
            .join("SimRealm/SimPlayer/layout-local.txt")
            .try_exists()
            .unwrap(),
        "manager None must not write layout-local.txt"
    );
}
