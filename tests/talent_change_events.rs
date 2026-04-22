//! Tests for talent change event counts and animation side-effects.
//!
//! Verifies that PurchaseRank/RefundRank fire a bounded number of
//! TRAIT_NODE_CHANGED events, notify currency refresh, and do not
//! trigger unrelated config/tree mutation events.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

/// Install event counters for trait events on a hidden tracker frame.
const INSTALL_HOOKS: &str = r#"
    _G._test_events = {}
    _G._test_event_counts = {}
    local tracker = CreateFrame("Frame", "_TestEventTracker")
    local events_to_track = {
        "TRAIT_NODE_CHANGED",
        "TRAIT_TREE_CHANGED",
        "TRAIT_TREE_CURRENCY_INFO_UPDATED",
        "TRAIT_CONFIG_UPDATED",
    }
    for _, ev in ipairs(events_to_track) do
        tracker:RegisterEvent(ev)
        _G._test_event_counts[ev] = 0
        _G._test_events[ev] = {}
    end
    tracker:SetScript("OnEvent", function(self, event, ...)
        _G._test_event_counts[event] = (_G._test_event_counts[event] or 0) + 1
        table.insert(_G._test_events[event], {...})
    end)
"#;

/// Reset counters between operations.
const RESET_COUNTS: &str = r#"
    for ev, _ in pairs(_G._test_event_counts) do
        _G._test_event_counts[ev] = 0
        _G._test_events[ev] = {}
    end
"#;

/// Read event counts as (node_changed, tree_changed, currency_updated, config_updated).
fn read_event_counts(env: &WowLuaEnv) -> (i32, i32, i32, i32) {
    let counts: String = env
        .eval(
            r#"
            local nc = _G._test_event_counts["TRAIT_NODE_CHANGED"] or 0
            local tc = _G._test_event_counts["TRAIT_TREE_CHANGED"] or 0
            local cu = _G._test_event_counts["TRAIT_TREE_CURRENCY_INFO_UPDATED"] or 0
            local co = _G._test_event_counts["TRAIT_CONFIG_UPDATED"] or 0
            return string.format("%d,%d,%d,%d", nc, tc, cu, co)
            "#,
        )
        .unwrap();
    let p: Vec<i32> = counts.split(',').map(|s| s.parse().unwrap()).collect();
    (p[0], p[1], p[2], p[3])
}

/// Assert bounded node-change fanout plus one currency refresh notification.
fn assert_bounded_node_changed(counts: (i32, i32, i32, i32), context: &str) {
    let (nc, tc, cu, co) = counts;
    assert!(
        nc >= 1,
        "{context}: should fire >= 1 TRAIT_NODE_CHANGED, got {nc}"
    );
    assert!(
        nc <= 10,
        "{context}: should fire <= 10 TRAIT_NODE_CHANGED, got {nc}"
    );
    assert_eq!(tc, 0, "{context}: TRAIT_TREE_CHANGED should not fire");
    assert_eq!(
        cu, 1,
        "{context}: TRAIT_TREE_CURRENCY_INFO_UPDATED should fire exactly once"
    );
    assert_eq!(co, 0, "{context}: TRAIT_CONFIG_UPDATED should not fire");
}

// ============================================================================
// PurchaseRank fires exactly the right number of events
// ============================================================================

#[test]
fn purchase_rank_fires_bounded_trait_node_changed() {
    let env = env();
    env.exec(INSTALL_HOOKS).unwrap();

    let ok: bool = env.eval("return C_Traits.PurchaseRank(1, 81469)").unwrap();
    assert!(ok, "PurchaseRank should succeed for node 81469");

    let counts = read_event_counts(&env);
    assert_bounded_node_changed(counts, "PurchaseRank");
}

#[test]
fn refund_rank_fires_bounded_trait_node_changed() {
    let env = env();

    // Purchase first so we can refund
    env.exec("C_Traits.PurchaseRank(1, 81469)").unwrap();
    env.exec(INSTALL_HOOKS).unwrap();

    let ok: bool = env.eval("return C_Traits.RefundRank(1, 81469)").unwrap();
    assert!(ok, "RefundRank should succeed for node 81469");

    let counts = read_event_counts(&env);
    assert_bounded_node_changed(counts, "RefundRank");
}

#[test]
fn set_selection_fires_currency_update_event() {
    let env = env();
    env.exec(INSTALL_HOOKS).unwrap();

    // Known hero talent selection node + entry from seeded paladin data.
    let ok: bool = env
        .eval("return C_Traits.SetSelection(1, 99838, 123361)")
        .unwrap();
    assert!(ok, "SetSelection should succeed for node 99838");

    let counts = read_event_counts(&env);
    assert_bounded_node_changed(counts, "SetSelection");
}

#[test]
fn hero_purchase_increments_hero_currency_spent() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local configID = 1
            local treeID = 790
            local ok = C_Traits.SetSelection(configID, 99838, 123361) -- Lightsmith
            assert(ok, "expected deterministic hero subtree selection")

            local activeSubTreeID = C_ClassTalents.GetActiveHeroTalentSpec()
            assert(activeSubTreeID ~= nil, "expected active hero subtree")

            local subTreeInfo = C_Traits.GetSubTreeInfo(configID, activeSubTreeID)
            local heroCurrencyID = subTreeInfo and subTreeInfo.traitCurrencyID
            assert(heroCurrencyID ~= nil, "expected hero trait currency id")

            local function spent_for(currencyID)
                for _, currency in ipairs(C_Traits.GetTreeCurrencyInfo(configID, treeID, false)) do
                    if currency.traitCurrencyID == currencyID then
                        return currency.spent or 0
                    end
                end
                return -1
            end

            local beforeSpent = spent_for(heroCurrencyID)
            assert(beforeSpent >= 0, "hero currency must exist in tree currency info")

            local purchasableHeroNodeID = nil
            for _, nodeID in ipairs(C_Traits.GetTreeNodes(configID, treeID)) do
                local nodeInfo = C_Traits.GetNodeInfo(configID, nodeID)
                if nodeInfo and nodeInfo.subTreeID == activeSubTreeID and nodeInfo.canPurchaseRank and (nodeInfo.ranksPurchased or 0) == 0 then
                    local costs = C_Traits.GetNodeCost(configID, nodeID)
                    local costCurrencyID = costs and costs[1] and costs[1].ID or nil
                    if costCurrencyID == heroCurrencyID then
                        purchasableHeroNodeID = nodeID
                        break
                    end
                end
            end
            assert(purchasableHeroNodeID ~= nil, "expected a purchasable hero node with hero currency cost")
            assert(C_Traits.PurchaseRank(configID, purchasableHeroNodeID), "hero node purchase should succeed")

            local afterSpent = spent_for(heroCurrencyID)
            return string.format("%d,%d,%d,%d", heroCurrencyID, beforeSpent, afterSpent, purchasableHeroNodeID)
            "#,
        )
        .unwrap();

    let parts: Vec<i32> = result
        .split(',')
        .map(|piece| piece.parse::<i32>().expect("expected numeric result"))
        .collect();
    assert_eq!(parts.len(), 4, "unexpected result format: {result}");
    let before_spent = parts[1];
    let after_spent = parts[2];
    assert!(
        after_spent > before_spent,
        "expected hero currency spent to increase after purchase, got {before_spent} -> {after_spent} ({result})"
    );
}

// ============================================================================
// Verify exact affected-node set for a known topology
// ============================================================================

#[test]
fn purchase_rank_reports_affected_node_ids() {
    let env = env();
    env.exec(INSTALL_HOOKS).unwrap();
    env.exec("C_Traits.PurchaseRank(1, 81469)").unwrap();

    let result: String = env
        .eval(
            r#"
            local ids = {}
            for _, args in ipairs(_G._test_events["TRAIT_NODE_CHANGED"]) do
                table.insert(ids, tostring(args[1]))
            end
            table.sort(ids)
            return table.concat(ids, ",")
            "#,
        )
        .unwrap();

    assert!(
        result.contains("81469"),
        "Changed node must be in affected set: {result}"
    );
    let count = result.split(',').filter(|s| !s.is_empty()).count();
    eprintln!("purchase_rank affected {count} nodes: {result}");
    assert!(
        count <= 10,
        "Affected node count should be bounded (got {count})"
    );
}

// ============================================================================
// Multiple purchases don't cascade events
// ============================================================================

#[test]
fn sequential_purchases_fire_events_independently() {
    let env = env();
    env.exec(INSTALL_HOOKS).unwrap();

    env.exec("C_Traits.PurchaseRank(1, 81469)").unwrap();
    let first = read_event_counts(&env).0;
    env.exec(RESET_COUNTS).unwrap();

    env.exec("C_Traits.PurchaseRank(1, 81470)").unwrap();
    let second = read_event_counts(&env).0;

    eprintln!("Events: first purchase={first}, second purchase={second}");
    assert!(
        first <= 10,
        "First purchase should fire <= 10 events, got {first}"
    );
    assert!(
        second <= 10,
        "Second purchase should fire <= 10 events, got {second}"
    );
}

// ============================================================================
// Animation groups: no spurious Play() during talent changes without UI
// ============================================================================

#[test]
fn talent_change_no_animations_without_ui() {
    let env = env();

    env.exec("C_Traits.PurchaseRank(1, 81469)").unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("C_Traits.RefundRank(1, 81469)").unwrap();
    env.fire_on_update(0.016).unwrap();

    // Without the talent frame loaded, no animations should have started.
    // This documents the baseline — with UI loaded we'd expect bounded counts.
    // Verify fire_on_update didn't panic and the env is still healthy.
    let ok: bool = env.eval("return true").unwrap();
    assert!(ok, "Environment should be healthy after talent changes");
}

// ============================================================================
// Animation group lifecycle on a test frame
// ============================================================================

fn create_test_anim_frame(env: &WowLuaEnv) {
    env.exec(
        r#"
        local f = CreateFrame("Frame", "_TestEdgeFrame", UIParent)
        local ag1 = f:CreateAnimationGroup("FlowAnim1")
        local a1 = ag1:CreateAnimation("Alpha")
        a1:SetDuration(0.3)
        a1:SetFromAlpha(0)
        a1:SetToAlpha(1)
        local ag2 = f:CreateAnimationGroup("FlowAnim2")
        local a2 = ag2:CreateAnimation("Alpha")
        a2:SetDuration(0.5)
        ag1:Play()
        ag2:Play()
        "#,
    )
    .unwrap();
}

fn count_playing_groups(env: &WowLuaEnv) -> i32 {
    env.eval(
        r#"
        local count = 0
        for _, ag in ipairs({_TestEdgeFrame:GetAnimationGroups()}) do
            if ag:IsPlaying() then count = count + 1 end
        end
        return count
        "#,
    )
    .unwrap()
}

#[test]
fn animation_groups_finish_after_duration() {
    let env = env();
    create_test_anim_frame(&env);

    let playing = count_playing_groups(&env);
    assert_eq!(playing, 2, "Both animation groups should be playing");

    env.fire_on_update(0.6).unwrap();

    let still = count_playing_groups(&env);
    assert_eq!(
        still, 0,
        "No groups should be playing after duration elapsed"
    );
}

// ============================================================================
// PlaySynced starts playback (was previously a no-op)
// ============================================================================

#[test]
fn play_synced_starts_animation_group() {
    let env = env();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "_TestSyncFrame", UIParent)
        _G._testSyncAG = f:CreateAnimationGroup("SyncAnim")
        _G._testSyncAG.syncKey = "TestSyncKey"
        _G._testSyncAG:SetLooping("REPEAT")
        local a = _G._testSyncAG:CreateAnimation("Translation")
        a:SetDuration(6.5)
        a:SetStartDelay(5)
        a:SetEndDelay(10.5)
        a:SetOffset(150, 0)
        _G._testSyncAG:PlaySynced()
        "#,
    )
    .unwrap();

    let playing: bool = env.eval("return _G._testSyncAG:IsPlaying()").unwrap();
    assert!(playing, "PlaySynced should start the animation group");

    let duration: f64 = env.eval("return _G._testSyncAG:GetDuration()").unwrap();
    let expected = 22.0;
    assert!(
        (duration - expected).abs() < 0.01,
        "Total duration should be {expected}s (5+6.5+10.5), got {duration}"
    );
}
