//! Private source-only augmentation. Neither case has executed or earned RED credit.
//! Required number / AllowedWhenUntainted: both profile-scoped cached API declarations.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::{WidgetSetEntry, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

// Adopt the earlier private row fixture separately; do not overwrite its source.
#[path = "widget_row_records/fixtures.rs"]
mod row_fixtures;
#[path = "widget_two_records/helpers.rs"]
mod shared_helpers;

use row_fixtures::{
    SET_ID, WIDGET_ID, WIDGET_TYPE, assert_lua, initialize_set, inject,
    install_snapshot_assertions, replacement, supplied,
};

fn install_deep_security_assertion(env: &WowLuaEnv) {
    install_snapshot_assertions(env);
    assert_lua(
        env,
        "adapt shared observation to complete row snapshot and recursively public fields",
        r#"
        FixtureGetter = RowGetter
        local function assertPublicTree(value)
            assert(not issecretvalue(value), 'public row value, including nested tables')
            if type(value) ~= 'table' then return end
            for key, child in pairs(value) do
                assert(not issecretvalue(key), 'public outer/array/entry key')
                assertPublicTree(child)
            end
        end
        function AssertSuppliedSnapshot(...)
            local row = AssertRowSnapshot(RowExpected, ...)
            assertPublicTree(row)
            return row
        end
        "#,
    );
}

#[test]
fn texture_and_text_row_required_number_selectors_preserve_exact_fractional_key() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().expect("row numeric selector environment");
        initialize_set(&env);
        inject(&env, supplied());
        install_deep_security_assertion(&env);
        let fractional_id = WIDGET_ID + 0.5;
        env.update_ui_widget(
            SET_ID,
            WidgetSetEntry {
                widget_id: fractional_id,
                widget_type: WIDGET_TYPE,
                unit_token: None,
            },
            WidgetVisualization::TextureAndTextRow(replacement()),
        )
        .expect("normal feed supplies a distinct exact fractional key");
        let (rows_before, sets_before) = {
            let state = env.state();
            let state = state.borrow();
            (
                state.ui_widget_inputs.texture_and_text_row.clone(),
                state.ui_widget_inputs.sets.clone(),
            )
        };
        shared_helpers::assert_selector_policy(&env, WIDGET_ID);
        assert_lua(
            &env,
            "fractional key remains distinct from integer and neighboring fractional keys",
            r#"
            AssertSuppliedSnapshot(FixtureGetter(761315))
            AssertRowSnapshot(RowReplacementExpected, FixtureGetter(761315.5))
            local function oneNil(...)
                assert(select('#', ...) == 1 and select(1, ...) == nil)
            end
            oneNil(FixtureGetter(761316))
            oneNil(FixtureGetter(761315.375))
            oneNil(FixtureGetter(761315.625))
            local members = C_UIWidgetManager.GetAllWidgetsBySetID(435)
            assert(#members == 2, 'integer and fractional members both supplied')
            local seen = {}
            for _, member in ipairs(members) do
                assert(member.widgetType == 15 and member.widgetSetID == 435)
                assert(member.unitToken == nil and not seen[member.widgetID])
                seen[member.widgetID] = true
            end
            assert(seen[761315] and seen[761315.5], 'no integer coercion in membership')
            "#,
        );
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.texture_and_text_row, rows_before);
        assert_eq!(state.ui_widget_inputs.sets, sets_before);
    });
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn texture_and_text_row_host_secrets_allow_untainted_deny_tainted_preserve_and_recover() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().expect("row authentic host-secret selector environment");
        initialize_set(&env);
        inject(&env, supplied());
        install_deep_security_assertion(&env);
        let (rows_before, sets_before) = {
            let state = env.state();
            let state = state.borrow();
            (
                state.ui_widget_inputs.texture_and_text_row.clone(),
                state.ui_widget_inputs.sets.clone(),
            )
        };
        assert_lua(
            &env,
            "retain complete public nested snapshot before permission queries",
            "RetainedSecurityRow = AssertSuppliedSnapshot(FixtureGetter(761315))",
        );
        // Shared helper creates true VM host-secret NUMBERs for present/absent IDs.
        // Its observer now delegates to the row-specific 18+2 deep assertion above.
        shared_helpers::assert_secret_selector_contract(&env, WIDGET_ID);
        assert_lua(
            &env,
            "recovery preserves retained entries and isolates caller nested mutations",
            r#"
            AssertSuppliedSnapshot(RetainedSecurityRow)
            local fresh = AssertSuppliedSnapshot(FixtureGetter(FixtureSecretPresent))
            assert(fresh ~= RetainedSecurityRow and fresh.entries ~= RetainedSecurityRow.entries)
            for i = 1, 2 do
                for j = 1, 2 do
                    assert(fresh.entries[i] ~= RetainedSecurityRow.entries[j])
                end
            end
            fresh.entries[1].text = 'security caller mutation'
            fresh.entries[2].tooltip = 'security caller tooltip mutation'
            AssertSuppliedSnapshot(RetainedSecurityRow)
            AssertSuppliedSnapshot(FixtureGetter(FixtureSecretPresent))
            assert(select('#', FixtureGetter(FixtureSecretAbsent)) == 1)
            assert(FixtureGetter(FixtureSecretAbsent) == nil)
            assert(issecretvalue(FixtureSecretPresent) and issecretvalue(FixtureSecretAbsent))
            assert(secretunwrap(FixtureSecretPresent) == 761315)
            assert(secretunwrap(FixtureSecretAbsent) == 761316)
            assert(debug.getstacktaint() == nil, 'outer caller remains untainted after recovery')
            "#,
        );
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.texture_and_text_row, rows_before);
        assert_eq!(state.ui_widget_inputs.sets, sets_before);
    });
}
