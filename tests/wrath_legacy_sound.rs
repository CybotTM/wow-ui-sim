//! Complete unchanged cached consumers; source-staged, runtime unverified.
#![cfg(feature = "client-wrath")]

use sha2::{Digest, Sha256};
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

const ORIGINALS: &[(&str, &str)] = &[
    (
        "OptionsPanelTemplates.xml",
        "6b8213261c8da907ed0f0ca7dd1c1db01d336b8b870268389bc8478be6b8db93",
    ),
    (
        "CharacterFrameTemplates.xml",
        "6c4fa77614d930fe3300a6a70e5c3f0b640fe5189065ca809f2088a678fa937c",
    ),
    (
        "FriendsFrame.xml",
        "97a2b0f11858e552636abe30874b62480d1a207ee15cb79f1e0138b4cec17b90",
    ),
    (
        "GlobalStrings.lua",
        "1d6662e6e77945a1d82d91dbc2fec70a5dffeb31e123c392e24adf7548d17dec",
    ),
    (
        "Constants.lua",
        "367f5024098edda7d8dbcc9f57cf2616c85da4a3a9eb9851d2d4fc0bea8b72fd",
    ),
    (
        "Fonts.xml",
        "cf08457a8fd1e1d1788532bb10c2de7e05280291c11f6a00294709f104d26757",
    ),
    (
        "FontStyles.xml",
        "618ec8ba7234c5725576c31b6d97f30588e8653474292d3783edf602d0d31d8a",
    ),
    (
        "Localization.xml",
        "57b8680f2e2b833143db01bbdb486499addffadd72ffda16eae94e1b95b6e9a6",
    ),
    (
        "Localization.lua",
        "d1e7748fb0774c11baa381c4b264e814b5e29e518882a008bde3783a153fce66",
    ),
    (
        "BasicControls.xml",
        "649b1dec3c05e6a0c4988ae703fc936ae9eca640f78c0eea48001100c0f2fc30",
    ),
    (
        "UIParent.lua",
        "a48687367d05ba9965bdd7a43bdecb92666c8ba74873bd7d9da08b205b34e177",
    ),
    (
        "MoneyFrame.lua",
        "f7392b7c139c8694fc3fe7581231c21342b9b6ed23b19f0b4bb77f3c156a7d90",
    ),
    (
        "MoneyFrame.xml",
        "2d72bc532d440ef7c01db88986be0a0151b9ff644e0b8a5a22def9998c109590",
    ),
    (
        "MoneyInputFrame.lua",
        "47b5cb82c6632450348b51b4f3301209eb37a5677190e4e8cd6cefa6410abee2",
    ),
    (
        "MoneyInputFrame.xml",
        "8c178e530b037adce610eed18cc17822656c9eec548962f5bb912fd8d5b71c5f",
    ),
    (
        "GameTooltip.xml",
        "d76a3add5eda00ed67e61369900241f0ded4b30d19ee828d92849e513857ed1e",
    ),
    (
        "HealthBar.lua",
        "093bbde4271332f1adb610deebec99718d1467986cfc583920049d363d833b4a",
    ),
    (
        "GameTooltip.lua",
        "29d0f65410030173d9041c415e914e86a57b4eb724c6b0e532d075a191a76587",
    ),
    (
        "GameTooltipTemplate.xml",
        "1ee083c247430dd66739acccb7119ad048019cd7b380b68849adad7f23531cbc",
    ),
    (
        "UIMenu.xml",
        "9ff104f1bbf97dc9feccf6692c6447eeccb48ec65f80cd991bab6a9180f0d5a2",
    ),
    (
        "UIMenu.lua",
        "ef9a84f42734253d1e42ab64a74b0baa67b0669bfabe66e1fd2e85e25d75f2a1",
    ),
    (
        "UIDropDownMenu.xml",
        "efe001430531c977c9d50c88d6e05419787f0a11c204f7b13981b9e44d7eaa61",
    ),
    (
        "UIDropDownMenu.lua",
        "d83c1d450884a8319045f671b3b81da442250ca9402dc94e85a542e7f1068a6a",
    ),
    (
        "UIDropDownMenuTemplates.xml",
        "5be5e90b8636c26df3af7976daf3094062c02164394b896588be048daf040619",
    ),
    (
        "UIPanelTemplates.lua",
        "d975f94a6d7deb8895a8ab47a2b6975cca4fd9e45f67cc15c54c8e100b37427e",
    ),
    (
        "UIPanelTemplates.xml",
        "e3cb10077e9296a5067ec32e33655b01c8930bee4019db37196ac292f602b91c",
    ),
    (
        "SecureTemplates.xml",
        "7c366e7784e01843801c8b6f50909645806250539a2c023318d8b4fc050ea426",
    ),
    (
        "SecureTemplates.lua",
        "8582f8a16032d25c50873fc74c50fe58a08c265dc7bc162bfae171e3454ea8e4",
    ),
    (
        "SecureStateDriver.lua",
        "c65dda91a5937c4fafce72d268b82d3bd273e5f139265d654d294ee5911652d3",
    ),
    (
        "SecureHoverDriver.lua",
        "11e5e856f226a45b0447c26ad26fd21100da2d287729cc927878fffa4c5b8dc8",
    ),
    (
        "SecureHandlerTemplates.xml",
        "b101d3c06fc76d456a58ed66ecfefbaabc4bb37ce4fd8efe8040e35a4c30dcda",
    ),
    (
        "RestrictedEnvironment.lua",
        "7bb78e7401ac1738c068c437cb0ee25836768a53635ed36dc084244e27d91a46",
    ),
    (
        "RestrictedFrames.lua",
        "4db2203bfab5d63fc5fb127b571476bf210153e6888b33b4794c80cf32b40059",
    ),
    (
        "RestrictedExecution.lua",
        "f63c5265d4cb23ff85f5b0751a2ee4dd749226871af9f0b2d325ee83b8f21593",
    ),
    (
        "SecureHandlers.lua",
        "dc14c17e7b788570f1d959c8452245aa9a3c82286a6b433151a2bb5fc6445326",
    ),
    (
        "ItemButtonTemplate.xml",
        "a6a8a4971f813a57089a0bd640d98c748367bea3564b86aa1565334b3acdbefd",
    ),
    (
        "ItemButtonTemplate.lua",
        "4e1f95ba20cb7285c8e044b0ddc609fd9cf2001ae2cf8083ee343c28e77cb4a0",
    ),
    (
        "SparkleFrame.xml",
        "818346fa1a5c58a1c38ecad2f00ab6680bb03789d01efdc6135dccecbbe2df6d",
    ),
    (
        "SparkleFrame.lua",
        "10ff8165a5aa51f2732dfecf884803723347109a8c8328487d55b2509b79dfb0",
    ),
    (
        "TextStatusBar.lua",
        "ceeed97ce260c1c505486ebc4a6757007585d450c3964c8b4428916a81f29c4d",
    ),
    (
        "TextStatusBar.xml",
        "a0f3aca312af2c229203ef9ca0bad7a662353e18ed63ceb505e4e95f00006006",
    ),
    (
        "AutoComplete.xml",
        "18d08285a969cea1878ad7f4742105083140bb0a6b68448ac916f8e71eedbb6f",
    ),
    (
        "AutoComplete.lua",
        "30102d7915abcc2125b63d220f560703da954d4512bf59aefcb6899f5ba5741a",
    ),
    (
        "Cooldown.xml",
        "e0a6f7edca59762e1c6a76dd45e53a25dc027dbf52dba192e434f3312f07483c",
    ),
    (
        "Cooldown.lua",
        "010ed45b7713b29d831d7a0e9afbb9a253dca4a28a2f95de910d4ec8caa32f97",
    ),
    (
        "ActionButtonTemplate.xml",
        "f1d376a849fca3867c2e1b3cb6dfeb4cd21db7838914482a16e48d5ce95c3ffa",
    ),
    (
        "BuffFrame.xml",
        "c4a1a1e6449c7ed866712f28c60e57080b69a8ade2b00e766bc58d3c0aa068ff",
    ),
    (
        "BuffFrame.lua",
        "c95749ebe1dbac513d3523e98c56231390486e48ae80e0e51072aa24368393bf",
    ),
    (
        "CombatFeedback.xml",
        "e3231c7fd2a01fee69d8bdd07b6eb0390f7f74c38ef007c9f92fb93736b3d168",
    ),
    (
        "CombatFeedback.lua",
        "8b40a5feda3b856d763228c9aaa40a4a062cee0e45fb90c036a72b0dd59732fa",
    ),
    (
        "CastingBarFrame.xml",
        "79252d78a74ccc765a37daf86726d841dfc0d0c6f93ee186cd8072f97dc94201",
    ),
    (
        "CastingBarFrame.lua",
        "14901aa0ad3a5fc07ba0ccec1ec7109b944a2a2b988d730cc2d0a8056201ef70",
    ),
    (
        "UnitPopup.xml",
        "fe0ae17443c88fbe98a154b4d99822aeaf7fda1deccc1cda42a59a7fd366ba16",
    ),
    (
        "UnitPopup.lua",
        "ed3f60d1152d7054fb846afce1febf3249494c89ebff42fe88e5c09b6be40209",
    ),
    (
        "UnitFrame.xml",
        "4ea79736e0fee81e4bae333eecbed526fb3f0325a54b1874c020d8378fb60d53",
    ),
    (
        "UnitFrame.lua",
        "4b44e519d2a1f5edbbb0c130aa93b408dea2139c8a2be7a64946dda13ce9ddde",
    ),
    (
        "QuestInfo.lua",
        "3bfe691acdb2a095a6633c3ce7bef4a13f53f464d20781b2e850e442f9b715cd",
    ),
    (
        "QuestFrame.lua",
        "4b2221da7f962535796ed500fbd1fc2adf95020e4564217ef457c1be7f9935f9",
    ),
    (
        "TargetFrame.lua",
        "dd2e492a097ac0da7731379a746012cd4031b3c4ac68ff204c7414872e99d3d9",
    ),
    (
        "SpellBookFrame.xml",
        "2512241621c7f2dcaff860def125088bb6e13035ecf43c222b777a0f06c9399a",
    ),
    (
        "SpellBookFrame.lua",
        "af2d753f865f5c04d1332eb4c667b22b7f3dd828696a7d5c4587d3c530c89c32",
    ),
    (
        "MailFrame.xml",
        "c31065aaaf2190613ac7c2cbdab743af3deedcc0680921a48c696f7d2d8cb2f4",
    ),
    (
        "MailFrame.lua",
        "d97121320cc2cc7ee64ecc52708b968a199126382a0f20b6263d93f50c03e167",
    ),
    (
        "PartyFrame.xml",
        "0d4fc268b2e4bd17f882f499345e893e0f6599d0447653771fccfce192977421",
    ),
    (
        "PartyMemberFrame.lua",
        "6b33ac81c75f15466d981d24b672465f89639b389e51ccea1a68bb58a3da98f1",
    ),
    (
        "PartyFrameTemplates.xml",
        "9d49b0808eba7a46ee6c43d5ee249446d667b05430a5aaac6d692a86c6db80f1",
    ),
    (
        "RaidFrame.lua",
        "1c2231c864f0d6e0c1bf02bd780ed1d12793ba3457323d86ebcf1d33079b6a1e",
    ),
];

fn assert_no_errors(env: &WowLuaEnv) {
    env.exec("assert(#BoundaryErrors == 0, table.concat(BoundaryErrors, '\\n'))")
        .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}

fn exec(env: &WowLuaEnv, code: &str) {
    let result = env.exec(code);
    assert_no_errors(env);
    result.expect("original consumer and assertions");
}

fn load(env: &WowLuaEnv, files: &[&str]) {
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("SETUP FAILURE: Wrath cache required");
    let cache = toc.parent().unwrap();
    // Check all transitive inputs, including XML scripts/includes, before loading.
    for (name, digest) in ORIGINALS {
        let path = cache.join(name);
        let bytes = std::fs::read(&path)
            .unwrap_or_else(|e| panic!("SETUP FAILURE: {}: {e}", path.display()));
        assert_eq!(
            format!("{:x}", Sha256::digest(bytes)),
            *digest,
            "SETUP FAILURE: {}",
            path.display()
        );
    }
    let dir = tempfile::tempdir().unwrap();
    let mut source = String::from("## Interface: 30300\n## Title: WrathBoundary\n");
    for file in files {
        let original = cache.join(file);
        let common = dir
            .path()
            .ancestors()
            .find(|base| original.starts_with(base))
            .unwrap();
        let mut relative = std::path::PathBuf::new();
        for _ in dir.path().strip_prefix(common).unwrap().components() {
            relative.push("..");
        }
        relative.push(original.strip_prefix(common).unwrap());
        source.push_str(relative.to_str().unwrap());
        source.push('\n');
    }
    let path = dir.path().join("WrathBoundary.toc");
    std::fs::write(&path, source).unwrap();
    let result = load_addon(&env.loader_env(), &path);
    assert_no_errors(env);
    let loaded = result.expect("complete unchanged original files");
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
}

fn create_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec("BoundaryErrors = {}; seterrorhandler(function(e) BoundaryErrors[#BoundaryErrors+1] = tostring(e) end)").unwrap();
    load(
        &env,
        &[
            "GlobalStrings.lua",
            "Constants.lua",
            "Fonts.xml",
            "FontStyles.xml",
            "Localization.xml",
            "BasicControls.xml",
            "UIParent.lua",
            "MoneyFrame.lua",
            "MoneyFrame.xml",
            "MoneyInputFrame.lua",
            "MoneyInputFrame.xml",
            "GameTooltip.xml",
            "UIMenu.xml",
            "UIDropDownMenu.xml",
            "UIPanelTemplates.lua",
            "UIPanelTemplates.xml",
            "OptionsPanelTemplates.xml",
            "CharacterFrameTemplates.xml",
            "SecureTemplates.xml",
            "SecureHandlerTemplates.xml",
            "ItemButtonTemplate.xml",
            "SparkleFrame.xml",
            "TextStatusBar.lua",
            "TextStatusBar.xml",
            "AutoComplete.xml",
            "Cooldown.xml",
            "ActionButtonTemplate.xml",
            "BuffFrame.xml",
            "CombatFeedback.xml",
            "CastingBarFrame.xml",
            "UnitPopup.xml",
            "UnitFrame.xml",
        ],
    );
    register_original_friends_tab_template();
    env
}

fn register_original_friends_tab_template() {
    // Mail consumes this virtual prerequisite, not the unrelated FriendsFrame tree.
    // Parse the complete pinned original; retain the template without rewriting it.
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
    let source = std::fs::read_to_string(toc.parent().unwrap().join("FriendsFrame.xml")).unwrap();
    let ui = wow_ui_sim::xml::parse_xml(&source).unwrap();
    let (frame, kind) = ui
        .elements
        .iter()
        .filter_map(|element| element.as_frame_data())
        .find(|(frame, _)| frame.name.as_deref() == Some("FriendsFrameTabTemplate"))
        .unwrap();
    wow_ui_sim::xml::register_template("FriendsFrameTabTemplate", kind, frame.clone());
}

fn clear_request(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.last_sound_name_requested = None;
    sim.last_sound_kit_requested = None;
}

fn assert_request(env: &WowLuaEnv, name: Option<&str>, kit: Option<u32>) {
    let state = env.state();
    let sim = state.borrow();
    assert_eq!(
        sim.last_sound_name_requested.as_deref(),
        name.map(str::as_bytes)
    );
    assert_eq!(sim.last_sound_kit_requested, kit);
}

#[test]
fn original_wrath_target_hide_records_ambiguous_name_without_kit() {
    let env = create_env();
    load(&env, &["TargetFrame.lua"]);
    clear_request(&env);
    exec(
        &env,
        "TargetFrame_OnHide(CreateFrame('Frame', nil, UIParent))",
    );
    assert_request(&env, Some("INTERFACESOUND_LOSTTARGETUNIT"), None);
}

#[test]
fn original_wrath_spellbook_initial_tab_requests_page_turn_kit() {
    let env = create_env();
    // Keep this sound test independent of the separate default-loot patch.
    env.state().borrow_mut().loot_method.method = "freeforall".into();
    clear_request(&env);
    load(&env, &["SpellBookFrame.xml"]);
    assert_request(&env, Some("igAbiliityPageTurn"), Some(836));
    exec(&env, "assert(SpellBookFrame.selectedSkillLine == 1)");
}

#[test]
fn original_wrath_sendmail_xml_onload_requests_checkbox_kit() {
    let env = create_env();
    clear_request(&env);
    load(&env, &["MailFrame.xml"]);
    assert_request(&env, Some("igMainMenuOptionCheckBoxOn"), Some(856));
    exec(
        &env,
        "assert(SendMailFrame:GetScript('OnLoad')); assert(SendMailCODButton:GetChecked())",
    );
}

#[test]
fn original_wrath_quest_fading_update_requests_write_quest() {
    let env = create_env();
    load(&env, &["QuestFrame.lua", "QuestInfo.lua"]);
    // Ordinary widgets supply callback inputs; no original function is replaced.
    exec(
        &env,
        r#"
        QuestInfoFrame = CreateFrame("Frame", nil, UIParent)
        QuestInfoFrame.acceptButton = CreateFrame("Button", nil, QuestInfoFrame)
        QuestInfoDescriptionText = QuestInfoFrame:CreateFontString(nil, "ARTWORK", "GameFontNormal")
        QuestInfoDescriptionText:SetText("A quest description for the original fading callback.")
        QUEST_FADING_DISABLE = "1"
        SoundQuestFadingFrame = CreateFrame("Frame", nil, QuestInfoFrame)
        SoundQuestFadingFrame.fading = 1
        SoundQuestFadingFrame.fadingProgress = 0
        "#,
    );
    clear_request(&env);
    exec(
        &env,
        "QuestInfoFadingFrame_OnUpdate(SoundQuestFadingFrame, 0.25)",
    );
    assert_request(&env, Some("WriteQuest"), Some(3093));
    exec(&env, "assert(SoundQuestFadingFrame.fadingProgress == 17.5)");
}

#[test]
fn wrath_unknown_name_records_request_without_kit_or_error() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("PlaySound('WriteQuest')").unwrap();
    assert_request(&env, Some("WriteQuest"), Some(3093));
    env.exec("PlaySound('not_a_known_legacy_sound')").unwrap();
    assert_request(&env, Some("not_a_known_legacy_sound"), None);
    env.exec("PlaySound('')").unwrap();
    assert_request(&env, Some(""), None);
    env.exec("PlaySound(string.char(255, 0, 65))").unwrap();
    assert_eq!(
        env.state().borrow().last_sound_name_requested.as_deref(),
        Some(&[255, 0, 65][..])
    );
    assert_eq!(env.state().borrow().last_sound_kit_requested, None);
}

#[test]
fn wrath_named_global_and_numeric_namespace_keep_distinct_contracts() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("PlaySound(684)").unwrap();
    assert_request(&env, None, Some(684));
    env.exec("C_Sound.PlaySound(836)").unwrap();
    assert_request(&env, None, Some(836));
    assert!(env.exec("C_Sound.PlaySound('igAbiliityPageTurn')").is_err());
    assert_request(&env, None, Some(836));
    env.exec("PlaySound('igMainMenuOptionCheckBoxOn')").unwrap();
    assert_request(&env, Some("igMainMenuOptionCheckBoxOn"), Some(856));
    env.exec("PlaySound('INTERFACESOUND_LOSTTARGETUNIT')")
        .unwrap();
    assert_request(&env, Some("INTERFACESOUND_LOSTTARGETUNIT"), None);
    env.exec("PlaySound(900)").unwrap();
    assert_request(&env, Some("INTERFACESOUND_LOSTTARGETUNIT"), Some(900));
    env.exec("C_Sound.PlaySound(684)").unwrap();
    assert_request(&env, Some("INTERFACESOUND_LOSTTARGETUNIT"), Some(684));
}
