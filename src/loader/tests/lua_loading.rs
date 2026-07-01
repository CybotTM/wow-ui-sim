use super::*;

const MULTI_FILE_WIDGETS_LUA: &str = r#"
    local _, addon = ...
    local function updateKeyDirection(self) return "updated: " .. tostring(self) end
    local function onCVarUpdate(self, cvar)
        if cvar == "TestCVar" then
            if not updateKeyDirection then error("updateKeyDirection is nil!") end
            self.result = updateKeyDirection(self)
        end
    end
    function addon:CreateButton(name)
        local button = { name = name }
        onCVarUpdate(button, "TestCVar")
        return button
    end
"#;

const MULTI_FILE_BUTTON_LUA: &str = r#"
    local _, addon = ...
    function addon:CreateExtraButton(name) return addon:CreateButton(name .. "_extra") end
"#;

const MULTI_FILE_ADDON_LUA: &str = r#"
    local _, addon = ...
    local button = addon:CreateExtraButton("test")
    addon.testButton = button
"#;

/// Load multiple Lua files in sequence with a shared addon table.
fn load_test_lua_files(
    dir_suffix: &str,
    addon_name: &str,
    files: &[(&'static str, &str)],
) -> (TestCtx, Val) {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join(format!("wow-sim-{}", dir_suffix));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx = AddonContext {
        name: addon_name,
        table: addon_table.clone(),
        addon_root: &temp_dir,
        use_secure_env: false,
        taint: false,
    };

    for (filename, content) in files {
        let path = temp_dir.join(filename);
        std::fs::write(&path, content).unwrap();
        load_lua_file(&env.loader_env(), &path, &ctx, &mut LoadTiming::default())
            .unwrap_or_else(|e| panic!("{} should load: {}", filename, e));
    }

    (TestCtx { env, temp_dir }, addon_table)
}

#[test]
fn test_multi_file_closures() {
    let (t, addon_table) = load_test_lua_files(
        "test-multifile",
        "MultiFileTest",
        &[
            ("widgets.lua", MULTI_FILE_WIDGETS_LUA),
            ("button.lua", MULTI_FILE_BUTTON_LUA),
            ("addon.lua", MULTI_FILE_ADDON_LUA),
        ],
    );

    let test_button = table_get(&t.env, addon_table, "testButton");
    let result = val_to_rust_string(&t.env, table_get(&t.env, test_button, "result"));
    assert!(
        result.starts_with("updated:"),
        "updateKeyDirection should have been called, got: {}",
        result
    );
}

#[test]
fn test_text_to_speech_checkload_recovers_clobbered_dropdown_globals() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-tts-frame");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let lua_path = temp_dir.join("TextToSpeechFrame.lua");
    std::fs::write(
        &lua_path,
        r#"
        Enum = { TtsVoiceType = { Standard = "standard", Alternate = "alternate" } }
        CALLS = {}

        function SetupVoiceMenu(_, voiceType)
            table.insert(CALLS, voiceType)
        end

        function TextToSpeechFrame_SetupVoiceDropdown(self)
            SetupVoiceMenu(self.PanelContainer.TtsVoiceDropdown, Enum.TtsVoiceType.Standard);
        end

        function TextToSpeechFrame_SetupAlternateVoiceDropdown(self)
            SetupVoiceMenu(self.PanelContainer.TtsVoiceAlternateDropdown, Enum.TtsVoiceType.Alternate);
        end

        function IsReadyToLoad()
            return true
        end

        function TextToSpeechFrame_CheckLoad(self)
            if not self.loaded and IsReadyToLoad(self.loadedEvents) then
                self.loaded = true;

                TextToSpeechFrame_SetupVoiceDropdown(self);
                TextToSpeechFrame_SetupAlternateVoiceDropdown(self);
            end
        end
        "#,
    )
    .unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx = AddonContext {
        name: "TestAddon",
        table: addon_table,
        addon_root: &temp_dir,
        use_secure_env: false,
        taint: false,
    };
    load_lua_file(
        &env.loader_env(),
        &lua_path,
        &ctx,
        &mut LoadTiming::default(),
    )
    .unwrap();

    env.exec(
        r#"
        TextToSpeechFrame_SetupVoiceDropdown = true
        TextToSpeechFrame_SetupAlternateVoiceDropdown = false
        TTS_TEST_FRAME = {
            loaded = false,
            loadedEvents = {},
            PanelContainer = {
                TtsVoiceDropdown = {},
                TtsVoiceAlternateDropdown = {},
            },
        }
        TextToSpeechFrame_CheckLoad(TTS_TEST_FRAME)
        "#,
    )
    .unwrap();

    let (voice_ty, alt_ty): (String, String) = env
        .eval(
            "return type(TextToSpeechFrame_SetupVoiceDropdown), type(TextToSpeechFrame_SetupAlternateVoiceDropdown)",
        )
        .unwrap();
    assert_eq!(voice_ty, "function");
    assert_eq!(alt_ty, "function");

    let calls: (String, String) = env.eval("return CALLS[1], CALLS[2]").unwrap();
    assert_eq!(calls.0, "standard");
    assert_eq!(calls.1, "alternate");

    let loaded: bool = env.eval("return TTS_TEST_FRAME.loaded").unwrap();
    assert!(
        loaded,
        "TextToSpeechFrame_CheckLoad should still mark the frame loaded"
    );

    std::fs::remove_file(&lua_path).ok();
    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn third_party_bootstrap_files_are_not_loaded() {
    let env = WowLuaEnv::new().unwrap();
    let temp_root = std::env::temp_dir().join("wow-sim-third-party-bootstrap-skip-test");
    let addon_dir = temp_root.join("ThirdPartyBootstrapProbe");
    std::fs::create_dir_all(&addon_dir).unwrap();
    std::fs::write(
        addon_dir.join("Bootstrap.lua"),
        r#"ThirdPartyBootstrapProbeEvents = { "bootstrap" }"#,
    )
    .unwrap();
    std::fs::write(
        addon_dir.join("Normal.lua"),
        r#"
        ThirdPartyBootstrapProbeEvents = ThirdPartyBootstrapProbeEvents or {}
        table.insert(ThirdPartyBootstrapProbeEvents, "normal")
        "#,
    )
    .unwrap();

    let toc = crate::toc::TocFile::parse(
        &addon_dir,
        r#"
## Title: ThirdPartyBootstrapProbe
Bootstrap.lua [Bootstrap]
Normal.lua
"#,
    );

    load_addon_from_toc(&env.loader_env(), &toc).unwrap();
    let events: String = env
        .eval("return table.concat(ThirdPartyBootstrapProbeEvents, ',')")
        .unwrap();
    assert_eq!(events, "normal");

    std::fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn blizzard_bootstrap_pass_loads_lod_bootstrap_without_marking_addon_loaded() {
    let env = WowLuaEnv::new().unwrap();
    let temp_root = std::env::temp_dir().join("wow-sim-bootstrap-pass-test");
    let addon_dir = temp_root.join("Blizzard_BootstrapProbe");
    std::fs::create_dir_all(&addon_dir).unwrap();
    std::fs::write(
        addon_dir.join("Blizzard_BootstrapProbe.toc"),
        r#"
## Title: Blizzard_BootstrapProbe
## LoadOnDemand: 1
Bootstrap.lua [Bootstrap]
Normal.lua
"#,
    )
    .unwrap();
    std::fs::write(
        addon_dir.join("Bootstrap.lua"),
        r#"
        local _, private = ...
        private.bootstrapSeen = true
        BootstrapProbeEvents = BootstrapProbeEvents or {}
        table.insert(BootstrapProbeEvents, "bootstrap")
        function BootstrapProbe_LoadUI()
            return C_AddOns.LoadAddOn("Blizzard_BootstrapProbe")
        end
        "#,
    )
    .unwrap();
    std::fs::write(
        addon_dir.join("Normal.lua"),
        r#"
        local _, private = ...
        BootstrapProbeEvents = BootstrapProbeEvents or {}
        table.insert(BootstrapProbeEvents, private.bootstrapSeen and "normal sees bootstrap" or "normal missing bootstrap")
        "#,
    )
    .unwrap();

    crate::loader::load_blizzard_bootstrap_files_for_screen(
        &env.loader_env(),
        &temp_root,
        crate::screen::ScreenKind::Game,
    )
    .unwrap();

    let (events, loader_ty, loaded, lod): (String, String, bool, bool) = env
        .eval(
            r#"
            return table.concat(BootstrapProbeEvents, ","),
                   type(BootstrapProbe_LoadUI),
                   C_AddOns.IsAddOnLoaded("Blizzard_BootstrapProbe"),
                   C_AddOns.IsAddOnLoadOnDemand("Blizzard_BootstrapProbe")
            "#,
        )
        .unwrap();
    assert_eq!(events, "bootstrap");
    assert_eq!(loader_ty, "function");
    assert!(!loaded, "bootstrap pass must not mark LoD addon loaded");
    assert!(
        lod,
        "bootstrap registration should preserve LoadOnDemand metadata"
    );

    let toc = crate::toc::TocFile::from_file(&addon_dir.join("Blizzard_BootstrapProbe.toc"))
        .expect("probe toc should parse");
    load_addon_from_toc(&env.loader_env(), &toc).unwrap();
    let events: String = env
        .eval("return table.concat(BootstrapProbeEvents, ',')")
        .unwrap();
    assert_eq!(events, "bootstrap,normal missing bootstrap");

    std::fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn blizzard_lua_files_replay_into_secure_environment() {
    let env = WowLuaEnv::new().unwrap();
    let temp_root = std::env::temp_dir().join("wow-sim-secure-replay-test");
    let addon_dir = temp_root.join("Blizzard_SharedXMLBase");
    std::fs::create_dir_all(&addon_dir).unwrap();
    std::fs::write(
        addon_dir.join("Core.lua"),
        r#"ReplayLibraryValue = { marker = "shared" }"#,
    )
    .unwrap();
    std::fs::write(
        addon_dir.join("GlobalOnly.lua"),
        r#"ReplayGlobalOnlyValue = "global-only""#,
    )
    .unwrap();

    let toc = crate::toc::TocFile::parse(
        &addon_dir,
        r#"
## Title: Blizzard_SharedXMLBase
## AllowLoad: Game
Core.lua
GlobalOnly.lua [AllowLoadEnvironment Global]
"#,
    );

    let result = load_addon_from_toc(&env.loader_env(), &toc).unwrap();
    assert_eq!(result.lua_files, 3);

    let (global_marker, secure_marker, global_only_type): (String, String, String) = env
        .eval(
            r#"
            return _G.ReplayLibraryValue.marker,
                   __secureenv.ReplayLibraryValue.marker,
                   type(rawget(__secureenv, "ReplayGlobalOnlyValue"))
            "#,
        )
        .unwrap();
    assert_eq!(global_marker, "shared");
    assert_eq!(secure_marker, "shared");
    assert_eq!(global_only_type, "nil");

    std::fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn secure_xml_named_frames_bind_into_secure_environment() {
    let env = WowLuaEnv::new().unwrap();
    let temp_root = std::env::temp_dir().join("wow-sim-secure-xml-frame-test");
    let addon_dir = temp_root.join("SecureXmlAddon");
    std::fs::create_dir_all(&addon_dir).unwrap();
    std::fs::write(
        addon_dir.join("Frame.xml"),
        r#"<Ui xmlns="http://www.blizzard.com/wow/ui/">
            <GameTooltip name="SecureXmlTooltip"/>
        </Ui>"#,
    )
    .unwrap();

    let toc = crate::toc::TocFile::parse(
        &addon_dir,
        r#"
## Title: SecureXmlAddon
## UseSecureEnvironment: 1
Frame.xml
"#,
    );

    let result = load_addon_from_toc(&env.loader_env(), &toc).unwrap();
    assert_eq!(result.xml_files, 1);

    let (global_type, secure_same): (String, bool) = env
        .eval(
            r#"
            return type(_G.SecureXmlTooltip),
                   __secureenv.SecureXmlTooltip == _G.SecureXmlTooltip
            "#,
        )
        .unwrap();
    assert_eq!(global_type, "table");
    assert!(secure_same);

    std::fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn blizzard_shared_xml_lua_replays_into_secure_environment() {
    let env = WowLuaEnv::new().unwrap();
    let temp_root = std::env::temp_dir().join("wow-sim-sharedxml-secure-replay-test");
    let addon_dir = temp_root.join("Blizzard_SharedXML");
    std::fs::create_dir_all(&addon_dir).unwrap();
    std::fs::write(
        addon_dir.join("LoopingSoundEffect.lua"),
        r#"CreateLoopingSoundEffectEmitter = function() return "secure-visible" end"#,
    )
    .unwrap();

    let toc = crate::toc::TocFile::parse(
        &addon_dir,
        r#"
## Title: Blizzard_SharedXML
## AllowLoad: Game
LoopingSoundEffect.lua
"#,
    );

    let result = load_addon_from_toc(&env.loader_env(), &toc).unwrap();
    assert_eq!(result.lua_files, 2);

    let (global_type, secure_result): (String, String) = env
        .eval(
            r#"
            return type(_G.CreateLoopingSoundEffectEmitter),
                   __secureenv.CreateLoopingSoundEffectEmitter()
            "#,
        )
        .unwrap();
    assert_eq!(global_type, "function");
    assert_eq!(secure_result, "secure-visible");

    std::fs::remove_dir_all(&temp_root).ok();
}
