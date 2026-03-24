use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_pool_release_all_behavior(env: &WowLuaEnv) {
    env.exec(
        r#"
        local resets = 0
        local pool = CreateFramePool("Frame", UIParent, nil, function(_, frame)
            resets = resets + 1
            frame.wasReset = true
        end)

        local a = pool:Acquire()
        local b = pool:Acquire()
        assert(a ~= nil and b ~= nil)
        assert(pool:IsActive(a))
        assert(pool:IsActive(b))

        pool:ReleaseAll()

        TEST_POOL_RESETS = resets
        TEST_POOL_A_ACTIVE = pool:IsActive(a)
        TEST_POOL_B_ACTIVE = pool:IsActive(b)
        TEST_POOL_A_RESET = a.wasReset
        TEST_POOL_B_RESET = b.wasReset
        TEST_POOL_ENUM_EMPTY = pool:EnumerateActive()() == nil
        "#,
    )
    .unwrap();
}

fn assert_pool_collection_behavior(env: &WowLuaEnv) {
    env.exec(
        r#"
        local collection = CreateFramePoolCollection()
        local resets = 0
        local pool = collection:CreatePool("Frame", UIParent, nil, function(_, frame)
            resets = resets + 1
            frame.collectionReset = true
        end)

        local frame = collection:Acquire(nil)
        assert(frame ~= nil)
        assert(pool:IsActive(frame))
        assert(collection:IsActive(frame))

        collection:Release(frame)

        TEST_COLLECTION_RESETS = resets
        TEST_COLLECTION_ACTIVE = collection:IsActive(frame)
        TEST_COLLECTION_RESET = frame.collectionReset
        TEST_COLLECTION_ENUM_EMPTY = collection:EnumerateActive()() == nil
        "#,
    )
    .unwrap();
}

#[test]
fn create_frame_pool_supports_release_all_and_reset_callback() {
    let env = WowLuaEnv::new().unwrap();
    assert_pool_release_all_behavior(&env);

    let resets: i32 = env.eval("return TEST_POOL_RESETS").unwrap();
    let a_active: bool = env.eval("return TEST_POOL_A_ACTIVE").unwrap();
    let b_active: bool = env.eval("return TEST_POOL_B_ACTIVE").unwrap();
    let a_reset: bool = env.eval("return TEST_POOL_A_RESET").unwrap();
    let b_reset: bool = env.eval("return TEST_POOL_B_RESET").unwrap();
    let enum_empty: bool = env.eval("return TEST_POOL_ENUM_EMPTY").unwrap();

    assert_eq!(resets, 2);
    assert!(!a_active);
    assert!(!b_active);
    assert!(a_reset);
    assert!(b_reset);
    assert!(enum_empty);
}

#[test]
fn create_frame_pool_collection_can_create_and_release_by_template() {
    let env = WowLuaEnv::new().unwrap();
    assert_pool_collection_behavior(&env);

    let resets: i32 = env.eval("return TEST_COLLECTION_RESETS").unwrap();
    let active: bool = env.eval("return TEST_COLLECTION_ACTIVE").unwrap();
    let reset: bool = env.eval("return TEST_COLLECTION_RESET").unwrap();
    let enum_empty: bool = env.eval("return TEST_COLLECTION_ENUM_EMPTY").unwrap();

    assert_eq!(resets, 1);
    assert!(!active);
    assert!(reset);
    assert!(enum_empty);
}
