//! Complete-bootstrap contract for the stage's strict surfaces only.
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
fn retail_ptr_social_ui_keeps_tip_member_factory_behavior() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(C_SocialUI.IsSystemEnabled() == false)
        local synthesized = C_SocialUI.NotARealMethod
        assert(type(synthesized) == 'function')
        assert(C_SocialUI.NotARealMethod == synthesized)
        assert(select('#', synthesized()) == 1 and synthesized() == nil)
        "#,
    )
    .unwrap();
}

#[test]
fn non_strict_namespace_keeps_tip_member_factory_behavior() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(C_CVar.GetCVar('loadDeprecationFallbacks') == GetCVar('loadDeprecationFallbacks'))
        local synthesized = C_CVar.NotARealMethod
        assert(type(synthesized) == 'function')
        assert(C_CVar.NotARealMethod == synthesized)
        assert(select('#', synthesized()) == 1 and synthesized() == nil)
        local supplied = { ReadFixture = function() return 'registered' end }
        local namespace = __wow_namespace(supplied)
        assert(namespace == supplied and namespace.ReadFixture() == 'registered')
        local later = namespace.NotARealMethod
        assert(type(later) == 'function' and namespace.NotARealMethod == later)
        assert(later() == nil)
        "#,
    )
    .unwrap();
}
