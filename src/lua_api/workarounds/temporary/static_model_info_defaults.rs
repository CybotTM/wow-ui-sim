//! Temporary `StaticModelInfo` fallback.
//!
//! Static model scene rendering is intentionally not modeled in detail. This
//! keeps early model-scene helper probes safe until Blizzard's
//! `StaticModelInfo.lua` supplies the full helper table.

const STATIC_MODEL_INFO_DEFAULTS_LUA: &str = r#"
if type(StaticModelInfo) ~= "table" then
    StaticModelInfo = {}
end
if rawget(StaticModelInfo, "CreateModelSceneEntry") == nil then
    function StaticModelInfo.CreateModelSceneEntry(modelSceneID, effectFileID1, effectFileID2)
        return {
            modelSceneID = modelSceneID,
            sceneID = modelSceneID,
            effectFileID1 = effectFileID1,
            effectFileID2 = effectFileID2,
            displayID = effectFileID1,
        }
    end
end
if rawget(StaticModelInfo, "SetupModelScene") == nil then
    function StaticModelInfo.SetupModelScene(modelScene, modelSceneInfo, forceUpdate, stopAnim)
        if type(modelScene) == "table"
            and type(modelScene.SetFromModelSceneID) == "function"
            and type(modelSceneInfo) == "table" then
            modelScene:SetFromModelSceneID(modelSceneInfo.modelSceneID or modelSceneInfo.sceneID, forceUpdate)
        end
        return nil, nil
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(STATIC_MODEL_INFO_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_static_model_info_entry_factory() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("StaticModelInfo = nil")
            .expect("fixture should clear StaticModelInfo");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("StaticModelInfo defaults should apply");
        }

        let (model_scene_id, scene_id, effect_file_id, display_id): (i64, i64, i64, i64) = env
            .eval(
                r#"
                local entry = StaticModelInfo.CreateModelSceneEntry(55, 382335)
                return entry.modelSceneID, entry.sceneID, entry.effectFileID1, entry.displayID
                "#,
            )
            .expect("StaticModelInfo entry probe should run");

        assert_eq!(model_scene_id, 55);
        assert_eq!(scene_id, 55);
        assert_eq!(effect_file_id, 382335);
        assert_eq!(display_id, 382335);
    }

    #[test]
    fn installs_static_model_info_setup_model_scene_noop() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("StaticModelInfo = nil")
            .expect("fixture should clear StaticModelInfo");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("StaticModelInfo defaults should apply");
        }

        let (scene_id, force_update, effect1, effect2): (i64, bool, String, String) = env
            .eval(
                r#"
                local scene = {
                    SetFromModelSceneID = function(self, sceneID, forceUpdate)
                        self.sceneID = sceneID
                        self.forceUpdate = forceUpdate
                    end,
                }
                local entry = StaticModelInfo.CreateModelSceneEntry(55, 382335, 999)
                local effect1, effect2 = StaticModelInfo.SetupModelScene(scene, entry, true, false)
                return scene.sceneID, scene.forceUpdate, type(effect1), type(effect2)
                "#,
            )
            .expect("StaticModelInfo setup probe should run");

        assert_eq!(scene_id, 55);
        assert!(force_update);
        assert_eq!(effect1, "nil");
        assert_eq!(effect2, "nil");
    }

    #[test]
    fn preserves_existing_static_model_info_members() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            StaticModelInfo = {
                CreateModelSceneEntry = function(sceneID, displayID)
                    return {
                        sceneID = sceneID + 1,
                        displayID = displayID + 1,
                    }
                end,
                SetupModelScene = function()
                    return "existing"
                end,
            }
            "#,
        )
        .expect("fixture should install existing StaticModelInfo");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("StaticModelInfo defaults should apply");
        }

        let (scene_id, display_id, setup_result): (i64, i64, String) = env
            .eval(
                r#"
                local entry = StaticModelInfo.CreateModelSceneEntry(55, 382335)
                return entry.sceneID, entry.displayID, StaticModelInfo.SetupModelScene()
                "#,
            )
            .expect("StaticModelInfo preservation probe should run");

        assert_eq!(scene_id, 56);
        assert_eq!(display_id, 382336);
        assert_eq!(setup_result, "existing");
    }
}
