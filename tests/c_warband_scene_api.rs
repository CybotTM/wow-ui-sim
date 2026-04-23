//! Tests for `C_WarbandScene` campsites collection APIs.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn warband_scene_search_returns_seeded_entries_and_filters() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local all = C_WarbandScene.SearchWarbandSceneEntries({})
            if type(all) ~= "table" then return "all_not_table" end
            if #all ~= 4 then return "all_" .. tostring(#all) end

            local owned = C_WarbandScene.SearchWarbandSceneEntries({ ownedOnly = true })
            if #owned ~= 2 then return "owned_" .. tostring(#owned) end

            local favorites = C_WarbandScene.SearchWarbandSceneEntries({ favoritesOnly = true })
            if #favorites ~= 1 then return "favorites_" .. tostring(#favorites) end
            if favorites[1] ~= 1 then return "favorite_id_" .. tostring(favorites[1]) end

            return "ok"
            "#,
        )
        .expect("search probe should execute");

    assert_eq!(result, "ok", "C_WarbandScene search filters: {result}");
}

#[test]
fn warband_scene_entry_shape_matches_expected_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local entry = C_WarbandScene.GetWarbandSceneEntry(1)
            if not entry then return "missing_entry" end
            if entry.warbandSceneID ~= 1 then return "id_" .. tostring(entry.warbandSceneID) end
            if type(entry.name) ~= "string" or entry.name == "" then return "name" end
            if type(entry.description) ~= "string" then return "description" end
            if type(entry.source) ~= "string" then return "source" end
            if type(entry.quality) ~= "number" then return "quality" end
            if type(entry.textureKit) ~= "string" or entry.textureKit == "" then return "texture_kit" end
            if type(entry.isFavorite) ~= "boolean" then return "is_favorite" end
            if type(entry.hasFanfare) ~= "boolean" then return "has_fanfare" end
            if type(entry.sourceType) ~= "number" then return "source_type" end
            return "ok"
            "#,
        )
        .expect("entry probe should execute");

    assert_eq!(result, "ok", "C_WarbandScene entry shape: {result}");
}

#[test]
fn warband_scene_owned_state_matches_seeded_data() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_WarbandScene.HasWarbandScene(1) ~= true then return "owned_1" end
            if C_WarbandScene.HasWarbandScene(3) ~= false then return "owned_3" end
            if C_WarbandScene.HasWarbandScene(999) ~= false then return "owned_unknown" end
            return "ok"
            "#,
        )
        .expect("owned-state probe should execute");

    assert_eq!(result, "ok", "C_WarbandScene ownership: {result}");
}

#[test]
fn warband_scene_set_favorite_updates_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if not C_WarbandScene.IsFavorite(1) then return "seed_favorite_missing" end
            C_WarbandScene.SetFavorite(1, false)
            if C_WarbandScene.IsFavorite(1) then return "favorite_not_cleared" end
            C_WarbandScene.SetFavorite(1, true)
            if not C_WarbandScene.IsFavorite(1) then return "favorite_not_set" end
            return "ok"
            "#,
        )
        .expect("favorite toggle probe should execute");

    assert_eq!(result, "ok", "C_WarbandScene favorite toggle: {result}");
}

#[test]
fn warband_scene_random_entry_is_supported() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local randomID = C_WarbandScene.GetRandomEntryID()
            if type(randomID) ~= "number" then return "random_id_type" end
            local randomEntry = C_WarbandScene.GetWarbandSceneEntry(randomID)
            if not randomEntry then return "random_entry_nil" end
            if randomEntry.warbandSceneID ~= randomID then return "random_id_mismatch" end
            if not C_WarbandScene.HasWarbandScene(randomID) then return "random_not_owned" end
            return "ok"
            "#,
        )
        .expect("random-entry probe should execute");

    assert_eq!(result, "ok", "C_WarbandScene random entry: {result}");
}

#[test]
fn admin_can_collect_and_uncollect_campsites() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_WarbandScene.HasWarbandScene(3) then return "seed_should_be_unowned" end
            A_Admin.SetCampsiteCollected(3, true)
            if not C_WarbandScene.HasWarbandScene(3) then return "not_collected" end

            C_WarbandScene.SetFavorite(3, true)
            if not C_WarbandScene.IsFavorite(3) then return "favorite_not_set" end

            A_Admin.SetCampsiteCollected(3, false)
            if C_WarbandScene.HasWarbandScene(3) then return "still_collected" end
            if C_WarbandScene.IsFavorite(3) then return "favorite_not_cleared" end
            return "ok"
            "#,
        )
        .expect("admin campsite probe should execute");

    assert_eq!(result, "ok", "A_Admin campsite control: {result}");
}
