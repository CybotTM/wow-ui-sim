use super::fixtures::{self, ID};
use super::helpers::assert_lua;
use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::WidgetId;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn item_display_complete_deep_snapshots_are_fresh_and_caller_mutations_are_isolated() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::seed(&env, ID, fixtures::supplied());
        fixtures::install_assertions(&env);
        assert_lua(
            &env,
            "complete15+16 and deep freshness",
            r#"
            local first = AssertItemAt(934227, ItemExpected)
            Retained = AssertItemAt(934227, ItemExpected)
            assert(first ~= Retained and first.itemInfo ~= Retained.itemInfo)
            for key in pairs(first.itemInfo) do first.itemInfo[key] = nil end
            first.itemInfo.infoText = 'caller nested edit'
            first.itemInfo.extra = true
            for key in pairs(first) do first[key] = nil end
            first.itemInfo = {itemID=99}
            first.extra = true
            AssertSuppliedSnapshot(Retained)
            local again = AssertItemAt(934227, ItemExpected)
            assert(again ~= first and again ~= Retained and again.itemInfo ~= Retained.itemInfo)
        "#,
        );
    });
}

#[test]
fn item_display_all_fields_replace_and_independent_animation_values_preserve_old_snapshots() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::install_assertions(&env);
        fixtures::seed(&env, ID, fixtures::supplied());
        assert_lua(
            &env,
            "retain complete original",
            r#"
            Retained = AssertItemAt(934227, ItemExpected)
            RetainedItem = Retained.itemInfo
        "#,
        );
        fixtures::seed(&env, ID, fixtures::replacement());
        assert_lua(
            &env,
            "replace every outer/nested field",
            r#"
            local current = AssertItemAt(934227, ItemReplacement)
            assert(current ~= Retained and current.itemInfo ~= RetainedItem)
            assert(Retained.itemInfo == RetainedItem)
            AssertSuppliedSnapshot(Retained)
        "#,
        );
        let mut asymmetric = fixtures::supplied();
        asymmetric.in_anim_type = 1;
        fixtures::seed(&env, ID, asymmetric);
        assert_lua(
            &env,
            "in/out are independent supplied values",
            r#"
            local expected = CopyItemExpected(ItemExpected)
            expected.inAnimType = 1
            AssertItemAt(934227, expected)
            AssertSuppliedSnapshot(Retained)
        "#,
        );
    });
}

#[test]
fn item_display_four_nullable_fields_distinguish_some_none_empty_strings_and_zero() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::install_assertions(&env);
        fixtures::seed(&env, ID, fixtures::supplied());
        assert_lua(
            &env,
            "all four Some",
            "SomeSnapshot = AssertItemAt(934227, ItemExpected)",
        );
        for missing in [
            "stackCount",
            "overrideItemName",
            "infoText",
            "overrideTooltip",
        ] {
            let mut partial = fixtures::supplied();
            match missing {
                "stackCount" => partial.item_info.stack_count = None,
                "overrideItemName" => partial.item_info.override_item_name = None,
                "infoText" => partial.item_info.info_text = None,
                "overrideTooltip" => partial.item_info.override_tooltip = None,
                _ => unreachable!(),
            }
            fixtures::seed(&env, ID, partial);
            assert_lua(
                &env,
                "each optional independently nullable",
                &format!(
                    r#"
                local expected = CopyItemExpected(ItemExpected)
                expected.itemInfo.{missing} = nil
                AssertItemAt(934227, expected)
                AssertSuppliedSnapshot(SomeSnapshot)
            "#
                ),
            );
        }
        fixtures::seed(&env, ID, fixtures::no_optionals());
        assert_lua(
            &env,
            "four None omitted, other twelve nested fields retained",
            r#"
            NoneSnapshot = AssertItemAt(934227, ItemNoOptionals)
            for _, key in ipairs({'stackCount','overrideItemName','infoText','overrideTooltip'}) do
                assert(NoneSnapshot.itemInfo[key] == nil)
            end
            local second = AssertItemAt(934227, ItemNoOptionals)
            assert(second ~= NoneSnapshot and second.itemInfo ~= NoneSnapshot.itemInfo)
            AssertSuppliedSnapshot(SomeSnapshot)
        "#,
        );
        let mut empty = fixtures::supplied();
        empty.item_info.stack_count = Some(0.0);
        empty.item_info.override_item_name = Some(String::new());
        empty.item_info.info_text = Some(String::new());
        empty.item_info.override_tooltip = Some(String::new());
        fixtures::seed(&env, ID, empty);
        assert_lua(
            &env,
            "Some empty strings and zero are not nil",
            r#"
            EmptySnapshot = AssertItemAt(934227, ItemEmptyStrings)
            local previous = FixtureExpected
            FixtureExpected = ItemNoOptionals
            AssertSuppliedSnapshot(NoneSnapshot)
            FixtureExpected = previous
            AssertSuppliedSnapshot(SomeSnapshot)
        "#,
        );
        fixtures::seed(&env, ID, fixtures::supplied());
        assert_lua(
            &env,
            "restore without mutating old nullable snapshots",
            r#"
            AssertItemAt(934227, ItemExpected)
            FixtureExpected = ItemEmptyStrings
            AssertSuppliedSnapshot(EmptySnapshot)
            FixtureExpected = ItemNoOptionals
            AssertSuppliedSnapshot(NoneSnapshot)
            FixtureExpected = ItemExpected
            AssertSuppliedSnapshot(SomeSnapshot)
        "#,
        );
    });
}

#[test]
fn item_display_fractional_ids_environments_removal_and_restoration_are_isolated() {
    common::with_timeout(88, || {
        let first = WowLuaEnv::new().unwrap();
        let second = WowLuaEnv::new().unwrap();
        fixtures::install_assertions(&first);
        fixtures::install_assertions(&second);
        fixtures::seed(&first, ID, fixtures::supplied());
        assert_lua(
            &second,
            "fresh environment has no inferred record",
            r#"
            local function oneNil(...) assert(select('#', ...) == 1 and (...) == nil) end
            oneNil(FixtureGetter(934227))
        "#,
        );
        fixtures::seed(&first, ID + 0.5, fixtures::replacement());
        fixtures::seed(&second, ID, fixtures::replacement());
        assert_lua(
            &first,
            "distinct fractional identity",
            r#"
            Retained = AssertItemAt(934227, ItemExpected)
            Other = AssertItemAt(934227.5, ItemReplacement)
            assert(Retained.itemInfo ~= Other.itemInfo)
        "#,
        );
        assert_lua(
            &second,
            "second independent complete record",
            "AssertItemAt(934227, ItemReplacement)",
        );
        fixtures::seed(&second, ID, fixtures::supplied());
        assert_lua(
            &first,
            "second replacement cannot change first environment",
            r#"
            AssertItemAt(934227, ItemExpected)
            AssertItemAt(934227.5, ItemReplacement)
        "#,
        );
        fixtures::seed(&first, ID, fixtures::replacement());
        assert_lua(
            &first,
            "first replacement is local",
            "AssertItemAt(934227, ItemReplacement)",
        );
        assert_lua(
            &second,
            "first replacement cannot change second environment",
            "AssertItemAt(934227, ItemExpected)",
        );
        first
            .state()
            .borrow_mut()
            .ui_widget_inputs
            .item_display
            .remove(&WidgetId::from_number(ID));
        assert_lua(
            &first,
            "remove only exact record and preserve old snapshots",
            r#"
            local function oneNil(...) assert(select('#', ...) == 1 and (...) == nil) end
            oneNil(FixtureGetter(934227))
            AssertItemAt(934227.5, ItemReplacement)
            AssertSuppliedSnapshot(Retained)
            FixtureExpected = ItemReplacement
            AssertSuppliedSnapshot(Other)
            FixtureExpected = ItemExpected
        "#,
        );
        assert_lua(
            &second,
            "other environment survives replacement and removal",
            "AssertItemAt(934227, ItemExpected)",
        );
        fixtures::seed(&first, ID, fixtures::supplied());
        assert_lua(
            &first,
            "restoration allocates fresh nested snapshot",
            r#"
            local restored = AssertItemAt(934227, ItemExpected)
            assert(restored ~= Retained and restored.itemInfo ~= Retained.itemInfo)
            AssertItemAt(934227.5, ItemReplacement)
        "#,
        );
    });
}
