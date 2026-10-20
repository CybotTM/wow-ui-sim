//! XML handlers must observe globals when invoked, not when installed.
use std::path::Path;
use wow_ui_sim::loader::{LoadTiming, create_frame_from_xml};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, parse_xml};

fn install(env: &WowLuaEnv, body: &str) {
    let xml = format!(
        r#"<Ui><CheckButton name="LateBoundFrame" parent="UIParent" id="7"><Scripts><OnClick>{body}</OnClick></Scripts></CheckButton></Ui>"#
    );
    let ui = parse_xml(&xml).unwrap();
    let XmlElement::CheckButton(frame) = &ui.elements[0] else {
        panic!("expected CheckButton");
    };
    create_frame_from_xml(
        &env.loader_env(),
        frame,
        "CheckButton",
        None,
        None,
        None,
        &mut LoadTiming::default(),
    )
    .unwrap();
}

#[test]
fn global_defined_after_xml_load_is_called() {
    let env = WowLuaEnv::new().unwrap();
    install(&env, "LateBoundCallback()");
    env.exec("function LateBoundCallback() lateBoundResult = 17 end")
        .unwrap();
    env.exec("LateBoundFrame:GetScript('OnClick')(LateBoundFrame)")
        .unwrap();
    assert_eq!(env.eval::<i32>("return lateBoundResult").unwrap(), 17);
}

#[test]
fn global_redefinition_and_secure_hook_after_install_are_observed() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("function LateBoundCallback() lateBoundResult = 1 end")
        .unwrap();
    install(&env, "LateBoundCallback()");
    env.exec(
        r#"
        function LateBoundCallback() lateBoundResult = 2 end
        hooksecurefunc('LateBoundCallback', function() lateBoundHook = true end)
        LateBoundFrame:GetScript('OnClick')(LateBoundFrame)
        assert(lateBoundResult == 2, 'redefinition bypassed')
        assert(lateBoundHook == true, 'secure hook bypassed')
    "#,
    )
    .unwrap();
}

#[test]
fn function_family_shapes_read_current_handler_environment() {
    let bodies = [
        "LateBoundCallback(self)",
        "LateBoundCallback()",
        "LateBoundCallback(self:GetText())",
        "LateBoundCallback(self:GetID())",
        "LateBoundCallback(self, event, ...)",
        "LateBoundCallback(self, button)",
        "LateBoundCallback(self, elapsed)",
        "LateBoundCallback('literal')",
        "LateBoundCallback(self, 'literal')",
        "LateBoundCallback(3)",
        "LateBoundCallback(self, 3)",
        "LateBoundCallback('literal', 3)",
        "LateBoundCallback('a', self, 'b', 3, 4)",
        "LateBoundCallback(LateBoundValue)",
        "LateBoundCallback(LateBoundValue, LateBoundValue)",
        "LateBoundCallback(LateBoundValue, LateBoundValue, 3)",
        "LateBoundCallback(LateBoundValue, LateBoundValue, LateBoundValue)",
        "LateBoundCallback('a', LateBoundValue, true)",
        "LateBoundCallback('a', nil, nil, LateBoundValue)",
        "LateBoundCallback(LateBoundValue, self)",
        "LateBoundCallback(LateBoundValue, self:GetID())",
        "LateBoundCallback(LateBoundValue, self:GetID(), self:GetID(), true)",
        "LateBoundCallback(self:GetParent())",
        "LateBoundCallback(self:GetParent():GetParent())",
        "LateBoundCallback(self:GetParent():GetID())",
        "LateBoundCallback(self:GetParent().value)",
        "LateBoundCallback(self, self:GetParent().value)",
        "LateBoundCallback(self:GetParent().value, self:GetParent().child.nested:GetValue())",
        "LateBoundCallback(LateBoundArg())",
        "LateBoundCallback(self:GetChecked())",
        "LateBoundCallback(LateBoundObject:GetValue())",
        "LateBoundNamespace.Callback()",
    ];
    for body in bodies {
        let env = WowLuaEnv::new().unwrap();
        env.exec(
            r#"
            function LateBoundCallback() error('old callback') end
            LateBoundValue = 'old'
            function LateBoundArg() error('old argument') end
            LateBoundObject = {GetValue = function() error('old object') end}
            LateBoundNamespace = {Callback = LateBoundCallback}
        "#,
        )
        .unwrap();
        install(&env, body);
        let result = env.exec(
            r#"
            local handler = LateBoundFrame:GetScript('OnClick')
            local parent = CreateFrame('Frame', nil, UIParent)
            parent.value = 42
            parent.child = {nested = {GetValue = function() return 42 end}}
            LateBoundFrame:SetParent(parent)
            local calls = 0
            local function callback(...)
                for i = 1, select('#', ...) do
                    assert(select(i, ...) ~= 'old', 'captured stale global argument')
                end
                calls = calls + 1
            end
            local environment = setmetatable({
                LateBoundCallback = callback,
                LateBoundNamespace = {Callback = callback},
                LateBoundValue = 42,
                LateBoundArg = function() return 42 end,
                LateBoundObject = {GetValue = function() return 42 end},
            }, {__index = _G})
            setfenv(handler, environment)
            handler(LateBoundFrame, 'LeftButton', 0.25)
            assert(calls == 1, 'handler did not use its environment')
        "#,
        );
        assert!(result.is_ok(), "{body}: {result:?}");
    }
}

#[test]
fn checked_assignment_callbacks_read_current_handler_environment() {
    let bodies = [
        "local checked = self:GetChecked() if (checked) then LateBoundSettings.enabled = true; else LateBoundSettings.enabled = false; end LateBoundChanged(); LateBoundSound(checked)",
        "local checked = self:GetChecked() if (checked) then LateBoundSettings.a = true; LateBoundSettings.b = true; LateBoundSettings.c = true; else LateBoundSettings.a = false; LateBoundSettings.b = false; LateBoundSettings.c = false; end LateBoundChanged(); LateBoundSound(checked)",
        "local checked = self:GetChecked() if (checked) then LateBoundSettings.enabled = true; else LateBoundSettings.enabled = false; end LateBoundChanged(); LateBoundOther(); LateBoundSound(checked)",
        "local checked = self:GetChecked(); LateBoundSettings.enabled = 3; LateBoundChanged(); LateBoundSound(checked)",
    ];
    for body in bodies {
        let env = WowLuaEnv::new().unwrap();
        env.exec(
            r#"
            LateBoundSettings = {}
            function LateBoundChanged() error('old changed callback') end
            function LateBoundOther() error('old other callback') end
            function LateBoundSound() error('old sound callback') end
        "#,
        )
        .unwrap();
        install(&env, body);
        let result = env.exec(
            r#"
            local handler = LateBoundFrame:GetScript('OnClick')
            local changed, sound = 0, 0
            local settings = {}
            setfenv(handler, setmetatable({
                LateBoundSettings = settings,
                LateBoundChanged = function() changed = changed + 1 end,
                LateBoundOther = function() end,
                LateBoundSound = function(checked) assert(checked); sound = sound + 1 end,
            }, {__index = _G}))
            LateBoundFrame:SetChecked(true)
            handler(LateBoundFrame)
            assert(changed == 1 and sound == 1, 'callbacks bypassed environment')
            assert(next(settings), 'assignment bypassed environment')
        "#,
        );
        assert!(result.is_ok(), "{body}: {result:?}");
    }
}

#[test]
fn nil_global_error_matches_unoptimized_lua_body() {
    for body in ["LateBoundCallback()", "LateBoundNamespace.Callback()"] {
        let env = WowLuaEnv::new().unwrap();
        env.exec("LateBoundNamespace = {}").unwrap();
        install(&env, body);
        let fast: String = env
            .eval(
                r#"
            local ok, message = pcall(LateBoundFrame:GetScript('OnClick'), LateBoundFrame)
            assert(not ok)
            return message:match('(attempt to .*)')
        "#,
            )
            .unwrap();
        install(&env, &format!("local unused = true; {body}"));
        let slow: String = env
            .eval(
                r#"
            local ok, message = pcall(LateBoundFrame:GetScript('OnClick'), LateBoundFrame)
            assert(not ok)
            return message:match('(attempt to .*)')
        "#,
            )
            .unwrap();
        assert_eq!(fast, slow, "{body}");
    }
}

#[cfg(feature = "client-wrath")]
#[test]
fn original_wrath_game_menu_resolves_later_framexml_globals() {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    env.set_screen_mode(wow_ui_sim::screen::ScreenKind::Game);
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    env.state().borrow_mut().addon_base_paths = vec![ui];
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
    let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    eprintln!("LOAD_WARNINGS: {:?}", loaded.warnings);
    for name in [
        "UpdateMicroButtons",
        "Disable_BagButtons",
        "Enable_BagButtons",
        "VoiceChat_Toggle",
    ] {
        assert_eq!(
            env.eval::<String>(&format!("return type({name})")).unwrap(),
            "function",
            "{name}"
        );
    }
    env.exec("assert(GameMenuFrame); GameMenuFrame:Show()")
        .unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("GameMenuFrame:Hide()").unwrap();
    env.fire_on_update(0.016).unwrap();
    let errors = env.state().borrow().lua_errors.clone();
    eprintln!("ALL_LUA_ERRORS: {errors:#?}");
    let missing: Vec<_> = errors
        .iter()
        .filter(|message| {
            [
                "UpdateMicroButtons",
                "Disable_BagButtons",
                "Enable_BagButtons",
                "VoiceChat_Toggle",
            ]
            .iter()
            .any(|name| message.contains(&format!("attempt to call global '{name}'")))
        })
        .collect();
    assert!(
        missing.is_empty(),
        "late-bound GameMenu globals: {missing:#?}"
    );
}

fn find_original_frame<'a>(
    frame: &'a wow_ui_sim::xml::FrameXml,
    name: &str,
) -> Option<&'a wow_ui_sim::xml::FrameXml> {
    if frame.name.as_deref() == Some(name) {
        return Some(frame);
    }
    frame
        .all_frame_elements()
        .into_iter()
        .find_map(|(child, _)| find_original_frame(child, name))
}

fn original_on_click(path: &Path, name: &str) -> String {
    let source = std::fs::read_to_string(path).unwrap();
    let ui = parse_xml(&source).unwrap();
    let frame = ui
        .elements
        .iter()
        .filter_map(|element| element.as_frame_data())
        .find_map(|(frame, _)| find_original_frame(frame, name))
        .unwrap_or_else(|| panic!("missing original frame {name} in {}", path.display()));
    frame
        .scripts()
        .flat_map(|scripts| &scripts.on_click)
        .last()
        .and_then(|script| script.body.clone())
        .expect("original inline OnClick")
}

#[cfg(feature = "client-wrath")]
fn keyring_body() -> String {
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
    original_on_click(
        &toc.parent().unwrap().join("MainMenuBarBagButtons.xml"),
        "KeyRingButton",
    )
}

#[cfg(feature = "client-wrath")]
#[test]
fn followup_original_keyring_observes_predicate_redefinition_and_hook() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function CursorHasItem() return false end
        function PutKeyInKeyRing() lateBoundKeyring = 'put' end
        function ToggleKeyRing() lateBoundKeyring = 'toggle' end
    "#,
    )
    .unwrap();
    install(&env, &keyring_body());
    env.exec(
        r#"
        function CursorHasItem() return true end
        hooksecurefunc('CursorHasItem', function() lateBoundPredicateHook = true end)
        LateBoundFrame:GetScript('OnClick')(LateBoundFrame)
        assert(lateBoundKeyring == 'put', 'stale KeyRing predicate')
        assert(lateBoundPredicateHook, 'predicate secure hook bypassed')
    "#,
    )
    .unwrap();
}

#[cfg(feature = "client-wrath")]
#[test]
fn followup_original_keyring_resolves_predicate_defined_after_install() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CursorHasItem = nil
        function PutKeyInKeyRing() lateBoundKeyring = 'put' end
        function ToggleKeyRing() lateBoundKeyring = 'toggle' end
    "#,
    )
    .unwrap();
    install(&env, &keyring_body());
    env.exec(
        r#"
        function CursorHasItem() return true end
        LateBoundFrame:GetScript('OnClick')(LateBoundFrame)
        assert(lateBoundKeyring == 'put', 'late KeyRing predicate not called')
    "#,
    )
    .unwrap();
}

#[cfg(feature = "client-retail")]
fn chat_config_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        ChatConfigFrame = CreateFrame('Frame')
        LateBoundChatFrame = CreateFrame('Frame')
        function HideUIPanel(frame) assert(frame == ChatConfigFrame) end
        function PlaySound() end
        function FCF_GetCurrentChatFrame() return LateBoundChatFrame end
        function IsCombatLog() return false end
        function Blizzard_CombatLog_RefreshGlobalLinks() lateBoundRefresh = true end
        Blizzard_CombatLog_CurrentSettings = {}
        C_CombatLog = {
            ApplyFilterSettings = function(settings)
                assert(settings == Blizzard_CombatLog_CurrentSettings)
            end,
            RefilterEntries = function() lateBoundRefilter = true end,
        }
    "#,
    )
    .unwrap();
    env
}

#[cfg(feature = "client-retail")]
fn chat_config_body() -> String {
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    original_on_click(
        &ui.join("Blizzard_ChatFrame/Mainline/ChatConfigFrame.xml"),
        "$parentOkayButton",
    )
}

#[cfg(feature = "client-retail")]
#[test]
fn followup_original_chat_config_observes_predicate_redefinition_and_hook() {
    let env = chat_config_env();
    install(&env, &chat_config_body());
    env.exec(
        r#"
        function IsCombatLog(frame)
            assert(frame == LateBoundChatFrame)
            return true
        end
        hooksecurefunc('IsCombatLog', function() lateBoundPredicateHook = true end)
        LateBoundFrame:GetScript('OnClick')(LateBoundFrame)
        assert(lateBoundRefresh and lateBoundRefilter, 'stale ChatConfig predicate')
        assert(lateBoundPredicateHook, 'predicate secure hook bypassed')
    "#,
    )
    .unwrap();
}

#[cfg(feature = "client-retail")]
#[test]
fn followup_original_chat_config_observes_getter_redefinition_and_hook() {
    let env = chat_config_env();
    env.exec(
        r#"
        local old = CreateFrame('Frame')
        function FCF_GetCurrentChatFrame() return old end
        function IsCombatLog(frame)
            assert(frame == LateBoundChatFrame, 'stale predicate input')
            return true
        end
    "#,
    )
    .unwrap();
    install(&env, &chat_config_body());
    env.exec(
        r#"
        function FCF_GetCurrentChatFrame() return LateBoundChatFrame end
        hooksecurefunc('FCF_GetCurrentChatFrame', function() lateBoundGetterHook = true end)
        LateBoundFrame:GetScript('OnClick')(LateBoundFrame)
        assert(lateBoundRefresh and lateBoundRefilter)
        assert(lateBoundGetterHook, 'predicate getter secure hook bypassed')
    "#,
    )
    .unwrap();
}

#[cfg(feature = "client-retail")]
fn lfd_body() -> String {
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    original_on_click(
        &ui.join("Blizzard_GroupFinder/Mainline/LFDFrame.xml"),
        "$parentFindGroupButton",
    )
}

#[cfg(feature = "client-retail")]
fn raidfinder_body() -> String {
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    original_on_click(
        &ui.join("Blizzard_GroupFinder/Shared/RaidFinder.xml"),
        "$parentFindRaidButton",
    )
}

#[cfg(feature = "client-retail")]
fn assert_lfg_branch(body: &str, mode: &str, expected: &str) {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        LE_LFG_CATEGORY_LFD = 1
        LE_LFG_CATEGORY_RF = 3
        RaidFinderQueueFrame = {raid = 17}
        function GetLFGMode(category, ...)
            if category == 1 then
                assert(select('#', ...) == 0, 'unexpected dungeon slot')
            else
                assert(category == 3 and select('#', ...) == 1 and (...) == 17,
                    'incorrect raid category/slot')
            end
            return lateBoundLfgMode
        end
        local function old() error('old LFG callback') end
        LeaveLFG, LeaveSingleLFG = old, old
        LFDQueueFrame_Join, RaidFinderQueueFrame_Join = old, old
    "#,
    )
    .unwrap();
    install(&env, body);
    env.exec(
        r#"
        function LeaveLFG(...)
            assert(select('#', ...) == 1 and (...) == 1)
            lateBoundLfgResult = 'leave-lfd'
        end
        function LeaveSingleLFG(...)
            local category, slot = ...
            assert(select('#', ...) == 2 and category == 3 and slot == 17)
            lateBoundLfgResult = 'leave-raid'
        end
        function LFDQueueFrame_Join(...)
            assert(select('#', ...) == 0)
            lateBoundLfgResult = 'join-lfd'
        end
        function RaidFinderQueueFrame_Join(...)
            assert(select('#', ...) == 0)
            lateBoundLfgResult = 'join-raid'
        end
        for _, name in ipairs({'LeaveLFG', 'LeaveSingleLFG', 'LFDQueueFrame_Join', 'RaidFinderQueueFrame_Join'}) do
            hooksecurefunc(name, function() lateBoundLfgHook = true end)
        end
    "#,
    )
    .unwrap();
    env.exec(&format!(
        "lateBoundLfgMode = {mode:?}; \
         LateBoundFrame:GetScript('OnClick')(LateBoundFrame); \
         assert(lateBoundLfgResult == {expected:?}, 'stale LFG callback'); \
         assert(lateBoundLfgHook, 'LFG secure hook bypassed')"
    ))
    .unwrap();
}

#[cfg(feature = "client-retail")]
#[test]
fn followup_original_lfd_leave_callback_observes_redefinition_and_hook() {
    assert_lfg_branch(&lfd_body(), "queued", "leave-lfd");
}

#[cfg(feature = "client-retail")]
#[test]
fn followup_original_lfd_join_callback_observes_redefinition_and_hook() {
    assert_lfg_branch(&lfd_body(), "none", "join-lfd");
}

#[cfg(feature = "client-retail")]
#[test]
fn followup_original_raid_leave_callback_observes_redefinition_and_hook() {
    assert_lfg_branch(&raidfinder_body(), "queued", "leave-raid");
}

#[cfg(feature = "client-retail")]
#[test]
fn followup_original_raid_join_callback_observes_redefinition_and_hook() {
    assert_lfg_branch(&raidfinder_body(), "none", "join-raid");
}
