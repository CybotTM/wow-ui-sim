//! Temporary Collections journal namespace defaults.
//!
//! Collections startup paths expect a few mount/pet journal helpers before the
//! full journal state exists. Keep those placeholder defaults explicit.

use crate::lua_api::LoaderEnv;

const COLLECTIONS_JOURNAL_NAMESPACE_WORKAROUND_LUA: &str = r#"
if type(C_MountJournal) == "table" then
    if rawget(C_MountJournal, "IsUsingDefaultFilters") == nil then
        function C_MountJournal.IsUsingDefaultFilters()
            return true
        end
    end
    if rawget(C_MountJournal, "GetDisplayedMountID") == nil then
        function C_MountJournal.GetDisplayedMountID(_index)
            return nil
        end
    end
end

if type(C_PetJournal) == "table" and rawget(C_PetJournal, "IsUsingDefaultFilters") == nil then
    function C_PetJournal.IsUsingDefaultFilters()
        return true
    end
end

if type(MountJournalToggleDynamicFlightFlyoutButtonMixin) == "table"
    and type(MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation) == "function"
    and not MountJournalToggleDynamicFlightFlyoutButtonMixin.__wow_popup_guard then
    local original = MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation
    MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation = function(self, ...)
        if not self.popup then
            return
        end
        return original(self, ...)
    end
    MountJournalToggleDynamicFlightFlyoutButtonMixin.__wow_popup_guard = true
end
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) {
    let _ = env.exec(COLLECTIONS_JOURNAL_NAMESPACE_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn seeds_missing_mount_and_pet_journal_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("C_MountJournal = {}; C_PetJournal = {}")
            .expect("journal namespaces should install");
        let loader_env = LoaderEnv::new(&env);

        patch(&loader_env);

        let (mount_defaults, displayed_id_is_nil, pet_defaults): (bool, bool, bool) = env
            .eval(
                r#"
                return C_MountJournal.IsUsingDefaultFilters(),
                    C_MountJournal.GetDisplayedMountID(1) == nil,
                    C_PetJournal.IsUsingDefaultFilters()
                "#,
            )
            .expect("journal namespace defaults should be callable");

        assert!(mount_defaults);
        assert!(displayed_id_is_nil);
        assert!(pet_defaults);
    }

    #[test]
    fn preserves_existing_journal_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_MountJournal = {
                IsUsingDefaultFilters = function()
                    return false
                end,
                GetDisplayedMountID = function()
                    return 123
                end,
            }
            C_PetJournal = {
                IsUsingDefaultFilters = function()
                    return false
                end,
            }
            "#,
        )
        .expect("journal namespaces should install");
        let loader_env = LoaderEnv::new(&env);

        patch(&loader_env);

        let (mount_defaults, displayed_id, pet_defaults): (bool, i64, bool) = env
            .eval(
                r#"
                return C_MountJournal.IsUsingDefaultFilters(),
                    C_MountJournal.GetDisplayedMountID(1),
                    C_PetJournal.IsUsingDefaultFilters()
                "#,
            )
            .expect("existing journal namespace defaults should be callable");

        assert!(!mount_defaults);
        assert_eq!(displayed_id, 123);
        assert!(!pet_defaults);
    }

    #[test]
    fn guards_dynamic_flight_animation_when_popup_is_missing() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            animation_calls = 0
            MountJournalToggleDynamicFlightFlyoutButtonMixin = {
                UpdateUnspentGlyphsAnimation = function()
                    animation_calls = animation_calls + 1
                    return "animated"
                end,
            }
            "#,
        )
        .expect("dynamic flight mixin should install");
        let loader_env = LoaderEnv::new(&env);

        patch(&loader_env);

        let (missing_popup_result, with_popup_result, calls, guarded): (
            Option<String>,
            String,
            i64,
            bool,
        ) = env
            .eval(
                r#"
                local missingPopupResult =
                    MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation({})
                local withPopupResult =
                    MountJournalToggleDynamicFlightFlyoutButtonMixin.UpdateUnspentGlyphsAnimation({ popup = {} })
                return missingPopupResult,
                    withPopupResult,
                    animation_calls,
                    MountJournalToggleDynamicFlightFlyoutButtonMixin.__wow_popup_guard == true
                "#,
            )
            .expect("dynamic flight popup guard should run");

        assert_eq!(missing_popup_result, None);
        assert_eq!(with_popup_result, "animated");
        assert_eq!(calls, 1);
        assert!(guarded);
    }
}
