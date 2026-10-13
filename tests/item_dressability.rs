//! Approved empty host dressability policy; not native item classification proof.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn item_dressability_empty_input_returns_exactly_one_boolean_false() {
    let env = WowLuaEnv::new().expect("fresh dressability environment");
    env.exec(
        r#"
        local query = C_Item.IsDressableItemByID
        assert(type(query) == 'function', 'missing public C_Item.IsDressableItemByID')
        local function miss(...)
            assert(select('#', ...) == 1, 'dressability miss must return exactly one value')
            local result = ...
            assert(type(result) == 'boolean' and result == false,
                'empty supplied classification must return boolean false')
        end
        local _, link = C_Item.GetItemInfo(6948)
        assert(type(link) == 'string' and link:find('|Hitem:6948:', 1, true),
            'existing generated item hyperlink required')
        miss(query(6948))
        miss(query(901001))
        miss(query('item:6948'))
        miss(query(link))
        "#,
    )
    .expect("numeric and supported item-link misses use the approved empty default");
}

#[test]
fn item_dressability_requires_item_info_and_authenticates_vm_secrets() {
    let env = WowLuaEnv::new().expect("fresh dressability argument environment");
    for expected in [false, true] {
        env.state()
            .borrow_mut()
            .item_dressability
            .insert(6948, expected);
        let script = format!(
            "local expectedDressability = {expected}\n{}",
            SECURITY_CONTRACT
        );
        env.exec(&script)
            .expect("required ItemInfo and AllowedWhenUntainted use real VM security");
    }
}

const SECURITY_CONTRACT: &str = r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil, 'untainted setup required')
        local query = C_Item.IsDressableItemByID
        assert(type(query) == 'function', 'missing public C_Item.IsDressableItemByID')
        local function argumentError(...)
            local ok, message = pcall(query, ...)
            assert(not ok and type(message) == 'string', 'required ItemInfo must reject input')
            assert(not message:find('untainted caller', 1, true),
                'ordinary or untainted malformed argument must reach argument validation')
        end
        argumentError()
        argumentError(nil)
        argumentError(false)
        argumentError({})
        argumentError(function() end)

        local _, link = C_Item.GetItemInfo(6948)
        assert(type(link) == 'string' and link:find('|Hitem:6948:', 1, true),
            'existing generated item hyperlink required')
        local numeric = secretwrap(6948)
        local hyperlink = secretwrap(link)
        local malformed = secretwrap(false)
        local missing = secretwrap(nil)
        local function suppliedValue(...)
            assert(select('#', ...) == 1, 'exact secret/ordinary supplied arity')
            assert(type((...)) == 'boolean' and (...) == expectedDressability,
                'supplied boolean must project through VM secret boundary')
        end
        for _, value in ipairs({numeric, hyperlink, malformed, missing}) do
            assert(issecretvalue(value), 'authentic VM secret required')
        end
        suppliedValue(query(numeric))
        suppliedValue(query(hyperlink))
        argumentError(malformed)
        argumentError(missing)

        local function denied(value)
            assert(issecretvalue(value), 'original VM wrapper required before denial')
            local ok, message = pcall(query, value)
            assert(not ok and type(message) == 'string', 'tainted secret must be denied')
            assert(message:find('untainted caller', 1, true),
                'AllowedWhenUntainted denial must precede parsing/lookup: ' .. message)
            assert(issecretvalue(value), 'denial must retain original wrapper')
            assert(not pcall(secretunwrap, value), 'tainted caller cannot unwrap secret')
            assert(debug.getstacktaint() == 'DressabilityTest', 'denial preserves caller taint')
        end
        local function addon()
            assert(debug.getstacktaint() == 'DressabilityTest', 'real tainted closure required')
            denied(numeric)
            denied(hyperlink)
            denied(malformed)
            denied(missing)
            suppliedValue(query(6948))
            suppliedValue(query(link))
            argumentError(false)
            assert(debug.getstacktaint() == 'DressabilityTest', 'ordinary queries preserve taint')
        end
        debug.setobjecttaint(addon, 'DressabilityTest')
        addon()
        assert(debug.getstacktaint() == nil, 'outer untainted caller restored')
        assert(issecretvalue(numeric) and issecretvalue(hyperlink), 'wrappers survive round trip')
        suppliedValue(query(numeric))
        suppliedValue(query(hyperlink))
        "#;

const REAL_ITEM: u32 = 6948;
const UNCATALOGUED_ITEM: u32 = 901001;

#[test]
fn item_dressability_projects_explicit_numeric_and_link_booleans() {
    let env = WowLuaEnv::new().expect("supplied dressability environment");
    assert!(env.state().borrow().item_dressability.is_empty());
    assert!(wow_ui_sim::items::get_item(UNCATALOGUED_ITEM).is_none());
    for expected in [false, true, false] {
        {
            let mut state = env.state().borrow_mut();
            state.item_dressability.insert(REAL_ITEM, expected);
            state.item_dressability.insert(UNCATALOGUED_ITEM, expected);
        }
        assert_supplied_dressability(&env, expected);
    }
}

#[test]
fn item_dressability_replacement_removal_and_environment_isolation() {
    let supplied = WowLuaEnv::new().expect("supplied dressability environment");
    let isolated = WowLuaEnv::new().expect("isolated dressability environment");
    supplied
        .state()
        .borrow_mut()
        .item_dressability
        .insert(REAL_ITEM, true);
    assert_real_item_dressability(&supplied, true);
    assert_real_item_dressability(&isolated, false);
    supplied
        .state()
        .borrow_mut()
        .item_dressability
        .insert(REAL_ITEM, false);
    assert_real_item_dressability(&supplied, false);
    supplied
        .state()
        .borrow_mut()
        .item_dressability
        .insert(REAL_ITEM, true);
    assert_real_item_dressability(&supplied, true);
    supplied
        .state()
        .borrow_mut()
        .item_dressability
        .remove(&REAL_ITEM);
    assert_real_item_dressability(&supplied, false);
    assert_real_item_dressability(&isolated, false);
    assert!(supplied.state().borrow().item_dressability.is_empty());
    assert!(isolated.state().borrow().item_dressability.is_empty());
}

#[test]
fn item_dressability_is_independent_of_mount_equipment_and_consumable_inputs() {
    let env = WowLuaEnv::new().expect("independent item inputs environment");
    {
        let mut state = env.state().borrow_mut();
        state.equippable_items.insert(REAL_ITEM);
        state.consumable_items.insert(REAL_ITEM);
        state.mount_item_relations.insert(REAL_ITEM as i32, 401);
    }
    assert_real_item_dressability(&env, false);
    for expected in [true, false] {
        env.state()
            .borrow_mut()
            .item_dressability
            .insert(REAL_ITEM, expected);
        assert_real_item_dressability(&env, expected);
        env.exec(
            r#"
            assert(C_Item.IsEquippableItem(6948) == true)
            assert(C_Item.IsConsumableItem(6948) == true)
            assert(C_MountJournal.GetMountFromItem(6948) == 401)
            "#,
        )
        .expect("dressability insertion does not mutate independent item inputs");
    }
    {
        let mut state = env.state().borrow_mut();
        state.equippable_items.remove(&REAL_ITEM);
        state.consumable_items.remove(&REAL_ITEM);
        state.mount_item_relations.remove(&(REAL_ITEM as i32));
        state.item_dressability.insert(REAL_ITEM, true);
    }
    assert_real_item_dressability(&env, true);
    let state = env.state().borrow();
    assert!(!state.equippable_items.contains(&REAL_ITEM));
    assert!(!state.consumable_items.contains(&REAL_ITEM));
    assert!(!state.mount_item_relations.contains_key(&(REAL_ITEM as i32)));
}

fn assert_real_item_dressability(env: &WowLuaEnv, expected: bool) {
    let results = env
        .eval::<(bool, bool, bool)>(
            r#"
            local query = C_Item.IsDressableItemByID
            local _, link = C_Item.GetItemInfo(6948)
            assert(type(link) == 'string' and link:find('|Hitem:6948:', 1, true))
            local function booleanResult(...)
                assert(select('#', ...) == 1, 'exact supplied result arity')
                assert(type((...)) == 'boolean', 'supplied result must be boolean')
                return ...
            end
            return booleanResult(query(6948)), booleanResult(query('item:6948')),
                booleanResult(query(link))
            "#,
        )
        .expect("real numeric, bare-link and generated-link dressability queries");
    assert_eq!(results, (expected, expected, expected));
}

fn assert_supplied_dressability(env: &WowLuaEnv, expected: bool) {
    assert_real_item_dressability(env, expected);
    let results = env
        .eval::<(bool, bool)>(
            r#"
            local function booleanResult(...)
                assert(select('#', ...) == 1, 'exact uncatalogued result arity')
                assert(type((...)) == 'boolean', 'uncatalogued result must be boolean')
                return ...
            end
            return booleanResult(C_Item.IsDressableItemByID(901001)),
                booleanResult(C_Item.IsDressableItemByID('item:901001'))
            "#,
        )
        .expect("explicit uncatalogued numeric and link dressability queries");
    assert_eq!(results, (expected, expected));
}
