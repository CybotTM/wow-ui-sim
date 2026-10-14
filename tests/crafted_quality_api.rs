//! Private corrected TEST stage: unexecuted; main owns public-call RED and integration.
//! Denial fragment: pinned rilua table_security::unwrap_secret; not native exact text.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn crafted_quality_required_item_info_errors_have_context() {
    let env = WowLuaEnv::new().expect("fresh crafted-quality argument environment");
    env.exec(
        r#"
        local function requiredArgumentError(...)
            local ok, message = pcall(C_TradeSkillUI.GetItemCraftedQualityInfo, ...)
            assert(not ok, 'required ItemInfo must reject omitted/nil input')
            assert(type(message) == 'string', 'argument failure must carry context')
            assert(message:find('GetItemCraftedQualityInfo', 1, true),
                'argument error must name the public method: ' .. message)
            assert(message:find('itemInfo', 1, true),
                'argument error must name the required selector: ' .. message)
            assert(not message:find('requires an untainted caller', 1, true),
                'ordinary required-input errors must not be secret denials')
        end
        requiredArgumentError()
        requiredArgumentError(nil)
        "#,
    )
    .expect("public omitted/nil calls must reach contextual ItemInfo validation");
}

#[test]
fn crafted_quality_secret_nil_authenticates_before_required_validation() {
    let env = WowLuaEnv::new().expect("fresh crafted-quality secret environment");
    env.exec(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil, 'untainted setup required')
        local selector = secretwrap(nil)
        assert(issecretvalue(selector), 'authentic VM secret required')
        local function requiredArgumentError()
            local ok, message = pcall(C_TradeSkillUI.GetItemCraftedQualityInfo, selector)
            assert(not ok, 'untainted secret nil must reach required-input validation')
            assert(type(message) == 'string', 'required-input failure needs context')
            assert(message:find('GetItemCraftedQualityInfo', 1, true),
                'required-input failure must name method: ' .. message)
            assert(message:find('itemInfo', 1, true),
                'required-input failure must name selector: ' .. message)
            assert(not message:find('requires an untainted caller', 1, true),
                'AllowedWhenUntainted permits authentication before nil validation')
        end
        requiredArgumentError()
        assert(debug.getstacktaint() == nil, 'required error preserves untainted context')
        local function addon()
            assert(debug.getstacktaint() == 'CraftedQualityTest', 'tainted closure required')
            local ok, message = pcall(C_TradeSkillUI.GetItemCraftedQualityInfo, selector)
            assert(not ok, 'tainted secret nil must be denied before validation')
            assert(type(message) == 'string', 'secret denial needs context')
            assert(message:find('requires an untainted caller', 1, true),
                'secret authentication must precede required-input validation: ' .. message)
            assert(issecretvalue(selector), 'denial preserves original wrapper')
            assert(not pcall(secretunwrap, selector), 'tainted caller cannot unwrap selector')
            assert(debug.getstacktaint() == 'CraftedQualityTest', 'caller taint preserved')
        end
        debug.setobjecttaint(addon, 'CraftedQualityTest')
        addon()
        assert(debug.getstacktaint() == nil, 'outer untainted context restored')
        assert(issecretvalue(selector), 'wrapper survives both contexts')
        requiredArgumentError()
        assert(debug.getstacktaint() == nil, 'recovery preserves untainted context')
        assert(secretunwrap(selector) == nil, 'original nil payload accessible after recovery')
        "#,
    )
    .expect("AllowedWhenUntainted must authenticate real VM secret nil before parsing");
}
