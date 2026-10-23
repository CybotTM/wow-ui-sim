//! Complete-bootstrap nil contract for strict, ordinary and constructed namespaces.
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn strict_namespace_missing_members_stay_nil_after_bootstrap() {
    let env = WowLuaEnv::new().unwrap();
    let mut names = Vec::new();
    if matches!(
        ACTIVE,
        ClientProfile::Mists
            | ClientProfile::Era
            | ClientProfile::Anniversary
            | ClientProfile::WowForever
    ) {
        names.extend(["C_CombatText", "C_SpellActivationOverlay"]);
    }
    if ACTIVE == ClientProfile::WowForever {
        names.extend([
            "C_PlayerChoice",
            "C_RecentAllies",
            "C_PlayerMentorship",
            "C_SocialUI",
            "C_Flyout",
        ]);
    }
    for name in names {
        env.exec(&format!(
            r#"
            local namespace = {name}
            assert(type(namespace) == 'table', '{name}')
            local function memberCount()
                local count = 0
                for _ in pairs(namespace) do count = count + 1 end
                return count
            end
            local before = memberCount()
            assert(namespace.NotARealMethod == nil, '{name}')
            assert(namespace.NotARealMethod == nil, '{name}')
            assert(memberCount() == before, '{name}')
            -- Reusing the exported constructor must preserve strictness.
            assert(__wow_namespace(namespace).NotARealMethod == nil, '{name}')
            "#,
        ))
        .unwrap();
    }
}

#[cfg(any(feature = "client-retail", feature = "client-ptr"))]
#[test]
fn retail_ptr_social_ui_keeps_explicit_member_and_unknown_nil() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(C_SocialUI.IsSystemEnabled() == false)
        assert(rawget(C_SocialUI, 'NotARealMethod') == nil)
        assert(C_SocialUI.NotARealMethod == nil)
        assert(C_SocialUI.NotARealMethod == nil)
        assert(type(C_SocialUI.NotARealMethod) == 'nil')
        "#,
    )
    .unwrap();
}

#[test]
fn non_strict_namespace_and_constructor_preserve_explicit_members_only() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(C_CVar.GetCVar('loadDeprecationFallbacks') == GetCVar('loadDeprecationFallbacks'))
        assert(rawget(C_CVar, 'NotARealMethod') == nil)
        assert(C_CVar.NotARealMethod == nil)
        assert(C_CVar.NotARealMethod == nil)
        assert(type(C_CVar.NotARealMethod) == 'nil')
        local supplied = { ReadFixture = function() return 'registered' end }
        local namespace = __wow_namespace(supplied)
        assert(namespace == supplied and namespace.ReadFixture() == 'registered')
        assert(namespace.NotARealMethod == nil)
        assert(namespace.NotARealMethod == nil)
        assert(rawget(namespace, 'NotARealMethod') == nil)
        for key in pairs(namespace) do
            assert(key ~= 'NotARealMethod')
        end
        "#,
    )
    .unwrap();
}
