//! Wrath 12340 positional contracts and original FrameXML consumers.
#![cfg(feature = "client-wrath")]
use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn casting_tuple_and_produced_events_use_legacy_slots() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        observed = {}
        local watcher = CreateFrame('Frame')
        for _, event in ipairs({'UNIT_SPELLCAST_START', 'UNIT_SPELLCAST_DELAYED',
            'UNIT_SPELLCAST_FAILED', 'UNIT_SPELLCAST_STOP'}) do
            watcher:RegisterEvent(event)
        end
        watcher:SetScript('OnEvent', function(_, event, ...)
            observed[#observed + 1] = {event, n = select('#', ...), ...}
        end)
        CastSpellByID(19750)
        local name, rank, text, icon, first, last, trade, count, locked = UnitCastingInfo('player')
        assert(select('#', UnitCastingInfo('player')) == 9)
        assert(name == 'Flash of Light' and type(rank) == 'string' and text == name)
        assert(type(icon) == 'string' and type(first) == 'number' and last > first)
        assert(trade == false and type(count) == 'number' and locked == false)
        castCount = count
        castRank = rank
        assert(A_Admin.DelayCasting(0.75))
        assert(A_Admin.FailCasting())
        assert(select('#', UnitCastingInfo('player')) == 0)
        assert(#observed == 4)
        for i, event in ipairs({'UNIT_SPELLCAST_START', 'UNIT_SPELLCAST_DELAYED',
            'UNIT_SPELLCAST_FAILED', 'UNIT_SPELLCAST_STOP'}) do
            local row = observed[i]
            assert(row[1] == event and row.n == 4)
            assert(row[2] == 'player' and row[3] == 'Flash of Light' and row[4] == castRank)
            assert(row[5] == castCount)
        end
    "#,
    )
    .unwrap();
}

#[test]
fn casting_expiry_and_completion_keep_native_empty_and_event_contracts() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        completed = {}
        local watcher = CreateFrame('Frame')
        watcher:RegisterEvent('UNIT_SPELLCAST_STOP')
        watcher:SetScript('OnEvent', function(_, _, ...)
            completed = {n = select('#', ...), ...}
        end)
        CastSpellByID(19750)
        expectedCount = select(8, UnitCastingInfo('player'))
    "#,
    )
    .unwrap();
    env.state().borrow_mut().casting.as_mut().unwrap().duration = 0.0;
    env.exec("assert(select('#', UnitCastingInfo('player')) == 0)")
        .unwrap();
    wow_ui_sim::lua_api::cast_completion::tick_casting(&env);
    env.exec(r#"
        assert(completed.n == 4 and completed[1] == 'player')
        assert(completed[2] == 'Flash of Light' and type(completed[3]) == 'string')
        assert(completed[4] == expectedCount)
        assert(select('#', UnitCastingInfo('unavailable')) == 0)
        for _, entry in ipairs({{UnitCastingInfo, 'UnitCastingInfo'}, {UnitChannelInfo, 'UnitChannelInfo'}}) do
            local ok, message = pcall(entry[1], false)
            assert(not ok and string.find(message, 'Usage: '..entry[2], 1, true))
        end
    "#).unwrap();
}

#[test]
fn channel_tuple_drives_original_native_bar() {
    let env = common::action_bar_fixture::wrath_env();
    env.exec("CastSpellByID(19750)").unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.channeling = state.casting.take();
    }
    // Supplied incoming notification: Wrath has no modeled channel transport.
    env.fire_event_with_args(
        "UNIT_SPELLCAST_CHANNEL_START",
        &[
            env.lua_string("player"),
            env.lua_string("Flash of Light"),
            env.lua_string(""),
            rilua::Val::Num(0.0),
        ],
    )
    .unwrap();
    env.exec(
        r#"
        local name, rank, text, icon, first, last, trade, locked = UnitChannelInfo('player')
        assert(select('#', UnitChannelInfo('player')) == 8)
        assert(name == 'Flash of Light' and type(rank) == 'string' and text == name)
        assert(type(icon) == 'string' and last > first and trade == false and locked == false)
        assert(CastingBarFrame.channeling and CastingBarFrame:IsShown())
        assert(CastingBarFrameText:GetText() == text)
        assert(math.abs(CastingBarFrame.maxValue - (last - first) / 1000) < 0.000001)
    "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .channeling
        .as_mut()
        .unwrap()
        .duration = 0.0;
    env.exec("assert(select('#', UnitChannelInfo('player')) == 0)")
        .unwrap();
}

#[test]
fn aura_tuple_uses_native_types_and_zero_results() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.ClearBuffs(); A_Admin.AddBuff(465, 'F05 aura', 136001, 300, 3)")
        .unwrap();
    env.state().borrow_mut().player.buffs[0].is_stealable = true;
    env.exec(r#"
        local name, rank, icon, count, dispel, duration, expiry, caster, steal, consolidated, spell = UnitAura('player', 1)
        assert(select('#', UnitAura('player', 1)) == 11)
        assert(name == 'F05 aura' and type(rank) == 'string')
        assert(icon == nil or type(icon) == 'string')
        assert(count == 3 and dispel == nil and duration == 300 and expiry > 0)
        assert(caster == 'player' and steal == 1 and consolidated == nil and spell == 465)
        assert(UnitBuff('player', '1') == name)
        assert(select('#', UnitBuff('player', 0)) == 0)
        assert(select('#', UnitAura('unavailable', 1)) == 0)
        assert(select('#', UnitDebuff('player', 1)) == 0)
        assert(select('#', UnitAura('player', 99)) == 0)
    "#).unwrap();
    {
        let mut state = env.state().borrow_mut();
        let aura = &mut state.player.buffs[0];
        aura.duration = -1.0;
        aura.expiration_time = 99.0;
        aura.is_stealable = false;
        aura.source_unit.clear();
    }
    env.exec(
        r#"
        local _, _, _, _, _, duration, expiry, caster, steal = UnitAura('player', 1)
        assert(duration == 0 and expiry == 0 and caster == nil and steal == nil)
    "#,
    )
    .unwrap();
}

#[test]
fn aura_filters_accept_native_separators_and_force_polarity() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.ClearBuffs(); A_Admin.AddBuff(465, 'F05 aura', 136001, 300, 1)")
        .unwrap();
    {
        let mut state = env.state().borrow_mut();
        let helpful = &mut state.player.buffs[0];
        helpful.is_raid = true;
        helpful.is_from_player_or_player_pet = true;
        let mut harmful = helpful.clone();
        harmful.name = "F05 debuff".into();
        harmful.is_helpful = false;
        harmful.aura_instance_id += 1;
        state.player.buffs.push(harmful);
    }
    env.exec(r#"
        assert(UnitAura('player', '1', 'helpful player|raid') == 'F05 aura')
        assert(UnitAura('player', 1, 'harmful|player raid') == 'F05 debuff')
        assert(UnitBuff('player', 1, 'HARMFUL') == 'F05 aura')
        assert(UnitDebuff('player', 1, 'HELPFUL') == 'F05 debuff')
        for name, query in pairs({UnitAura = UnitAura, UnitBuff = UnitBuff, UnitDebuff = UnitDebuff}) do
            local ok, message = pcall(query, false, 1)
            assert(not ok and string.find(message, 'Usage: '..name, 1, true))
            assert(not pcall(query, 'player', {}))
            assert(not pcall(query, 'player', 1, {}))
        end
    "#).unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.player.buffs[0].is_from_player_or_player_pet = false;
        state.player.buffs[0].source_unit.clear();
    }
    env.exec("assert(select('#', UnitAura('player', 1, 'HELPFUL PLAYER')) == 0)")
        .unwrap();
}

#[test]
fn party_aura_fallback_projects_legacy_sparse_data() {
    let env = common::action_bar_fixture::wrath_env();
    env.exec("A_Admin.ClearBuffs(); A_Admin.AddBuff(465, 'F05 party aura', 136001, 300, 4)")
        .unwrap();
    {
        let mut state = env.state().borrow_mut();
        let mut aura = state.player.buffs[0].clone();
        aura.is_stealable = true;
        state.party_group_active = true;
        state.party_members[0].buffs = vec![aura];
    }
    env.exec(r#"
        TargetFrame.unit = 'party1'
        TargetFrame.showAuraCount = true
        TargetFrame_UpdateAuras(TargetFrame)
        assert(TargetFrameBuff1 and not TargetFrameBuff1Count:IsShown(), 'native party aura count must be sparse')
        assert(not TargetFrameBuff1Cooldown:IsShown(), 'native party aura has no duration')
        assert(not TargetFrameBuff1Stealable:IsShown(), 'native party aura is not stealable')
        local name, rank, icon, count, dispel, duration, expiry, caster, steal = UnitBuff('party1', 1)
        assert(name == 'F05 party aura' and type(rank) == 'string')
        assert(count == 1 and duration == 0 and expiry == 0 and caster == nil and steal == nil)
    "#).unwrap();
}

#[test]
fn missing_spell_and_empty_icon_preserve_native_query_shape() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("CastSpellByID(19750)").unwrap();
    env.state()
        .borrow_mut()
        .casting
        .as_mut()
        .unwrap()
        .icon_path
        .clear();
    env.exec("assert(select(4, UnitCastingInfo('player')) == '')")
        .unwrap();
    env.state().borrow_mut().casting.as_mut().unwrap().spell_id = 0;
    env.exec("assert(select('#', UnitCastingInfo('player')) == 0)")
        .unwrap();
}
