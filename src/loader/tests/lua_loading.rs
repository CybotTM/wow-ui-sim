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
