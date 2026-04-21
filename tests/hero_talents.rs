//! Tests for hero talent spec resolution.

use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::shader::load_texture_or_crop;
use wow_ui_sim::texture::TextureManager;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn env_with_full_ui() -> WowLuaEnv {
    let env = env();
    env.set_screen_size(1024.0, 768.0);

    let ui = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI");
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        if let Err(e) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }

    env.apply_post_load_workarounds();
    wow_ui_sim::startup::fire_startup_events(&env);
    env.apply_post_event_workarounds();
    wow_ui_sim::startup::process_pending_timers(&env);
    wow_ui_sim::startup::fire_one_on_update_tick(&env);

    env
}

#[test]
fn test_protection_hero_subtrees() {
    let env = env();
    // Protection (specID=66) should get Templar (48) and Lightsmith (49)
    let result: String = env
        .eval(
            r#"
            local ids, level = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 66)
            assert(ids, "subtree IDs should not be nil")
            assert(level == 71, "unlock level should be 71")
            table.sort(ids)
            return table.concat(ids, ",")
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "48,49",
        "Protection should have Templar(48) + Lightsmith(49)"
    );
}

#[test]
fn test_retribution_hero_subtrees() {
    let env = env();
    // Retribution (specID=70) should get Templar (48) and Herald of the Sun (50)
    let result: String = env
        .eval(
            r#"
            local ids, level = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 70)
            assert(ids, "subtree IDs should not be nil")
            table.sort(ids)
            return table.concat(ids, ",")
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "48,50",
        "Retribution should have Templar(48) + Herald of the Sun(50)"
    );
}

#[test]
fn test_holy_hero_subtrees() {
    let env = env();
    // Holy (specID=65) should get Lightsmith (49) and Herald of the Sun (50)
    let result: String = env
        .eval(
            r#"
            local ids, level = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 65)
            assert(ids, "subtree IDs should not be nil")
            table.sort(ids)
            return table.concat(ids, ",")
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "49,50",
        "Holy should have Lightsmith(49) + Herald of the Sun(50)"
    );
}

#[test]
fn test_subtree_info_has_selection_node_ids() {
    let env = env();
    // Templar (subtree 48) should have subTreeSelectionNodeIDs
    let result: String = env
        .eval(
            r#"
            local info = C_Traits.GetSubTreeInfo(1, 48)
            assert(info, "subtree info should not be nil")
            assert(info.name == "Templar", "name should be Templar, got: " .. tostring(info.name))
            assert(info.subTreeSelectionNodeIDs, "should have subTreeSelectionNodeIDs")
            assert(#info.subTreeSelectionNodeIDs > 0, "should have at least one selection node")
            return tostring(#info.subTreeSelectionNodeIDs)
            "#,
        )
        .unwrap();
    assert!(
        result.parse::<i32>().unwrap() > 0,
        "Templar should have selection nodes"
    );
}

#[test]
fn test_activate_hero_spec_lightsmith() {
    let env = env();
    // Activate Lightsmith by selecting entry 123361 on node 99838 (Protection's selection node)
    // Then verify: nodeInfo has the selection, and GetActiveHeroTalentSpec returns subtree 49
    let result: String = env
        .eval(
            r#"
            -- Clear auto-selected hero spec so we start from a clean state
            C_Traits.SetSelection(1, 99838, nil)
            assert(C_ClassTalents.GetActiveHeroTalentSpec() == nil, "should be nil after clearing")

            -- Selection node 99838 should be visible (Protection spec)
            local nodeInfo = C_Traits.GetNodeInfo(1, 99838)
            assert(nodeInfo.isVisible, "selection node should be visible for Protection")
            assert(nodeInfo.activeEntry == nil, "no entry selected yet")

            -- Activate Lightsmith: select entry 123361 (subtree 49) on node 99838
            local ok = C_Traits.SetSelection(1, 99838, 123361)
            assert(ok, "SetSelection should succeed")

            -- Verify nodeInfo now reflects the selection
            local updated = C_Traits.GetNodeInfo(1, 99838)
            assert(updated.activeEntry.entryID == 123361, "entry should be 123361, got: " .. tostring(updated.activeEntry.entryID))
            assert(updated.ranksPurchased == 1, "should have rank 1 after selection")

            -- Verify GetActiveHeroTalentSpec returns Lightsmith's subtree
            local active = C_ClassTalents.GetActiveHeroTalentSpec()
            assert(active == 49, "active hero spec should be 49 (Lightsmith), got: " .. tostring(active))

            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_tcontains_nil_safe() {
    let env = env();
    // tContains should return false when passed nil table
    let result: bool = env.eval("return tContains(nil, 42)").unwrap();
    assert!(!result, "tContains(nil, x) should return false");
}

#[test]
fn test_set_atlas_numeric_element_id() {
    let env = env();
    // SetAtlas should accept numeric element IDs (e.g. iconElementID from subtree info)
    // Element 26680 = "talents-heroclass-paladin-lightsmith"
    let result: String = env
        .eval(
            r#"
            local t = UIParent:CreateTexture(nil, "ARTWORK")
            t:SetAtlas(26680)
            return t:GetAtlas() or "nil"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "talents-heroclass-paladin-lightsmith",
        "SetAtlas should resolve numeric element ID to atlas name"
    );
}

#[test]
fn test_class_talents_switch_methods_update_seeded_spec_and_loadout_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local protectionConfigs = C_ClassTalents.GetConfigIDsBySpecID(66)
            assert(#protectionConfigs == 2, "Protection should expose two seeded loadouts")
            assert(C_ClassTalents.GetActiveConfigID() == protectionConfigs[1], "Protection default loadout should start active")

            C_ClassTalents.SwitchToLoadoutByName("Protection Mythic+")
            local protectionMythic = C_ClassTalents.GetActiveConfigID()
            assert(protectionMythic == protectionConfigs[2], "SwitchToLoadoutByName should pick the named Protection loadout")
            assert(C_ClassTalents.GetLastSelectedSavedConfigID(66) == protectionMythic, "last selected Protection config should update")
            assert(C_Traits.GetConfigInfo(protectionMythic).name == "Protection Mythic+", "config info should expose the active loadout name")

            C_ClassTalents.SwitchToSpecializationByName("Holy")
            assert(GetSpecialization() == 1, "Holy should become the active specialization")
            local holyConfigs = C_ClassTalents.GetConfigIDsBySpecID(65)
            assert(#holyConfigs == 2, "Holy should expose two seeded loadouts")
            assert(C_ClassTalents.GetActiveConfigID() == holyConfigs[1], "SwitchToSpecializationByName should activate Holy's default loadout")
            assert(C_ClassTalents.GetLastSelectedSavedConfigID(65) == holyConfigs[1], "Holy's last selected config should track the active default")
            local holyHeroSpecs = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 65)
            assert(tContains(holyHeroSpecs, C_ClassTalents.GetActiveHeroTalentSpec()), "Holy reset should auto-select one of Holy's hero subtrees")

            C_ClassTalents.SwitchToLoadoutByIndex(2)
            assert(C_ClassTalents.GetActiveConfigID() == holyConfigs[2], "SwitchToLoadoutByIndex should use the current specialization's loadout order")
            assert(C_Traits.GetConfigInfo(holyConfigs[2]).name == "Holy Raid", "Holy loadout names should be queryable")

            C_ClassTalents.SwitchToSpecializationByIndex(3)
            assert(GetSpecialization() == 3, "Retribution should become the active specialization")
            local retributionConfigs = C_ClassTalents.GetConfigIDsBySpecID(70)
            assert(C_ClassTalents.GetActiveConfigID() == retributionConfigs[1], "Retribution default loadout should become active")
            assert(C_ClassTalents.GetLastSelectedSavedConfigID(70) == retributionConfigs[1], "Retribution's last selected config should update")
            local retributionHeroSpecs = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 70)
            assert(tContains(retributionHeroSpecs, C_ClassTalents.GetActiveHeroTalentSpec()), "Retribution reset should auto-select one of Retribution's hero subtrees")

            return table.concat({
                tostring(protectionMythic),
                tostring(holyConfigs[2]),
                tostring(retributionConfigs[1]),
            }, ",")
            "#,
        )
        .unwrap();

    assert_eq!(result, "202,102,301");
}

#[test]
fn test_active_hero_node_icon_texture_path_resolves_to_real_asset() {
    let env = env();
    let texture_path: String = env
        .eval(
            r#"
            local config = C_ClassTalents.GetActiveConfigID()
            local configInfo = C_Traits.GetConfigInfo(config)
            local treeID = configInfo and configInfo.treeIDs and configInfo.treeIDs[1]
            assert(treeID, "expected active trait tree")

            local nodes = C_Traits.GetTreeNodes(treeID) or {}
            for _, nodeID in ipairs(nodes) do
                local node = C_Traits.GetNodeInfo(config, nodeID)
                local entryID = node and node.activeEntry and node.activeEntry.entryID
                if entryID and node.subTreeID then
                    local entry = C_Traits.GetEntryInfo(config, entryID)
                    local definitionID = entry and entry.definitionID
                    if definitionID and definitionID > 0 then
                        local definition = C_Traits.GetDefinitionInfo(definitionID)
                        local spellID = definition and definition.spellID
                        if spellID and spellID > 0 then
                            local texturePath = C_Spell.GetSpellTexture(spellID)
                            if type(texturePath) == "string" and texturePath ~= "" then
                                return texturePath
                            end
                        end
                    end
                end
            end

            error("expected at least one visible hero node with spell-backed icon path")
            "#,
        )
        .unwrap();

    let mut mgr = TextureManager::new(PathBuf::from("./textures"));
    let texture = load_texture_or_crop(&mut mgr, &texture_path).unwrap_or_else(|| {
        panic!("hero node spell texture did not resolve: {texture_path}");
    });

    assert!(
        texture.rgba.chunks_exact(4).any(|px| px[3] > 0),
        "hero node texture is fully transparent: {}",
        texture_path
    );
}

#[test]
fn test_active_hero_subtree_exposes_multiple_visible_nodes_and_edges() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Force a deterministic active hero subtree for Protection.
            local ok = C_Traits.SetSelection(1, 99838, 123361) -- Lightsmith
            assert(ok, "expected SetSelection for hero subtree to succeed")
            assert(C_ClassTalents.GetActiveHeroTalentSpec() == 49, "expected active subtree 49")

            local treeID = C_ClassTalents.GetTraitTreeForSpec(66)
            local nodes = C_Traits.GetTreeNodes(treeID) or {}
            local total, visible, edgeReady = 0, 0, 0
            for _, nodeID in ipairs(nodes) do
                local node = C_Traits.GetNodeInfo(1, nodeID)
                if node and node.subTreeID == 49 then
                    total = total + 1
                    if node.isVisible then visible = visible + 1 end
                    if node.meetsEdgeRequirements then edgeReady = edgeReady + 1 end
                end
            end

            local root = C_Traits.GetNodeInfo(1, 95228)
            assert(root and root.isVisible, "expected representative hero node to be visible")
            assert(total >= 10, "expected many hero nodes in subtree; got " .. tostring(total))
            assert(visible >= 10, "expected most hero nodes visible; got " .. tostring(visible))
            assert(edgeReady >= 2, "expected at least two edge-ready hero nodes; got " .. tostring(edgeReady))

            return table.concat({tostring(total), tostring(visible), tostring(edgeReady)}, ",")
            "#,
        )
        .unwrap();

    let parts: Vec<u32> = result
        .split(',')
        .map(|s| s.parse::<u32>().expect("count should parse"))
        .collect();
    assert_eq!(parts.len(), 3, "expected total,visible,edgeReady");
    assert!(parts[0] >= 10, "unexpected total hero node count: {result}");
    assert!(
        parts[1] >= 10,
        "unexpected visible hero node count: {result}"
    );
    assert!(
        parts[2] >= 2,
        "unexpected edge-ready hero node count: {result}"
    );
}

#[test]
fn test_non_selectable_hero_nodes_do_not_show_selectable_glow() {
    let env = env_with_full_ui();
    let result: String = env
        .eval(
            r#"
            C_ClassTalents.SwitchToSpecializationByName("Protection")
            local ok = C_Traits.SetSelection(1, 99838, 123361) -- Lightsmith
            assert(ok, "expected deterministic hero subtree selection")
            assert(C_ClassTalents.GetActiveHeroTalentSpec() == 49, "expected active subtree 49")

            assert(PlayerSpellsUtil and PlayerSpellsUtil.OpenToClassTalentsTab, "expected class talents UI helper")
            PlayerSpellsUtil.OpenToClassTalentsTab()

            local frame = PlayerSpellsFrame and PlayerSpellsFrame.TalentsFrame
            assert(frame and frame.EnumerateAllTalentButtons, "expected class talents frame")

            local selectable = 0
            local nonSelectable = 0
            local nonSelectableGlowShown = 0

            for button in frame:EnumerateAllTalentButtons() do
                local node = button:GetNodeInfo()
                if node and node.subTreeID == 49 and button:IsShown() then
                    if button:IsSelectable() then
                        selectable = selectable + 1
                    else
                        nonSelectable = nonSelectable + 1
                        local glow = button.SelectableGlow
                        if glow and glow:IsShown() then
                            nonSelectableGlowShown = nonSelectableGlowShown + 1
                        end
                    end
                end
            end

            assert(selectable >= 1, "expected at least one selectable hero node")
            assert(nonSelectable >= 1, "expected at least one non-selectable hero node")
            assert(nonSelectableGlowShown == 0, "non-selectable hero nodes should not show selectable glow")

            return table.concat({
                tostring(selectable),
                tostring(nonSelectable),
                tostring(nonSelectableGlowShown),
            }, ",")
            "#,
        )
        .unwrap();

    let parts: Vec<u32> = result
        .split(',')
        .map(|s| s.parse::<u32>().expect("count should parse"))
        .collect();
    assert_eq!(
        parts.len(),
        3,
        "expected selectable,nonSelectable,glowShown"
    );
    assert!(
        parts[0] >= 1,
        "unexpected selectable hero node count: {result}"
    );
    assert!(
        parts[1] >= 1,
        "unexpected non-selectable hero node count: {result}"
    );
    assert_eq!(
        parts[2], 0,
        "non-selectable hero nodes unexpectedly showed selectable glow: {result}"
    );
}

#[test]
fn test_class_talent_edges_render_below_visible_talent_buttons() {
    let env = env_with_full_ui();
    let result: String = env
        .eval(
            r#"
            C_ClassTalents.SwitchToSpecializationByName("Protection")
            local ok = C_Traits.SetSelection(1, 99838, 123361) -- Lightsmith
            assert(ok, "expected deterministic hero subtree selection")
            assert(C_ClassTalents.GetActiveHeroTalentSpec() == 49, "expected active subtree 49")

            assert(PlayerSpellsUtil and PlayerSpellsUtil.OpenToClassTalentsTab, "expected class talents UI helper")
            PlayerSpellsUtil.OpenToClassTalentsTab()

            local frame = PlayerSpellsFrame and PlayerSpellsFrame.TalentsFrame
            assert(frame and frame.edgePool, "expected class talents frame and edge pool")

            -- Force one explicit update pass so edge state/frame-level updates have run.
            if frame.OnUpdate then
                frame:OnUpdate()
            end

            local minHeroVisibleButtonLevel = nil
            local heroVisibleButtons = 0
            local minButtonParentLevel = nil
            local maxButtonParentLevel = nil
            for button in frame:EnumerateAllTalentButtons() do
                local nodeInfo = button.GetNodeInfo and button:GetNodeInfo()
                if button:IsShown() then
                    local parent = button:GetParent()
                    if parent and parent.GetFrameLevel then
                        local parentLevel = parent:GetFrameLevel()
                        if minButtonParentLevel == nil or parentLevel < minButtonParentLevel then
                            minButtonParentLevel = parentLevel
                        end
                        if maxButtonParentLevel == nil or parentLevel > maxButtonParentLevel then
                            maxButtonParentLevel = parentLevel
                        end
                    end
                end

                if button:IsShown() and nodeInfo and nodeInfo.subTreeID then
                    heroVisibleButtons = heroVisibleButtons + 1
                    local level = button:GetFrameLevel()
                    if minHeroVisibleButtonLevel == nil or level < minHeroVisibleButtonLevel then
                        minHeroVisibleButtonLevel = level
                    end
                end
            end
            assert(heroVisibleButtons > 0, "expected at least one visible hero talent button")
            assert(minHeroVisibleButtonLevel ~= nil, "expected hero talent frame levels")

            local checked = 0
            local violations = {}
            for edge in frame.edgePool:EnumerateActive() do
                local edgeLevel = edge:GetFrameLevel()
                local startLevel = edge:GetStartButton():GetFrameLevel()
                local endLevel = edge:GetEndButton():GetFrameLevel()
                checked = checked + 1
                if edgeLevel >= math.min(startLevel, endLevel) then
                    table.insert(
                        violations,
                        string.format(
                            "edge=%d expected<min(%d,%d)",
                            edgeLevel,
                            startLevel,
                            endLevel
                        )
                    )
                end
            end

            assert(checked > 0, "expected active talent edges after opening class talents")
            return table.concat({
                tostring(checked),
                tostring(heroVisibleButtons),
                tostring(minButtonParentLevel or -1),
                tostring(maxButtonParentLevel or -1),
                tostring(#violations),
                table.concat(violations, " | "),
            }, "::")
            "#,
        )
        .unwrap();

    let mut parts = result.splitn(6, "::");
    let checked = parts
        .next()
        .expect("checked count missing")
        .parse::<u32>()
        .expect("checked count should parse");
    let hero_visible = parts
        .next()
        .expect("hero visible count missing")
        .parse::<u32>()
        .expect("hero visible count should parse");
    let min_parent = parts
        .next()
        .expect("min parent level missing")
        .parse::<i32>()
        .expect("min parent level should parse");
    let max_parent = parts
        .next()
        .expect("max parent level missing")
        .parse::<i32>()
        .expect("max parent level should parse");
    let violations = parts
        .next()
        .expect("violation count missing")
        .parse::<u32>()
        .expect("violation count should parse");
    let details = parts.next().unwrap_or_default();

    assert!(checked > 0, "expected at least one checked edge");
    assert!(hero_visible > 0, "expected visible hero talent buttons");
    assert!(
        max_parent - min_parent <= 200,
        "talent button parent frame-level bands diverged unexpectedly: min={min_parent} max={max_parent}"
    );
    assert_eq!(
        violations, 0,
        "class talent edges should render below their endpoint buttons; violations={details}"
    );
}
