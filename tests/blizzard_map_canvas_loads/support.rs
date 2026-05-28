use std::path::PathBuf;

use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

pub(super) fn map_canvas_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_MapCanvas")
}

pub(super) fn map_canvas_toc() -> PathBuf {
    map_canvas_dir().join("Blizzard_MapCanvas.toc")
}

pub(super) const MAP_CANVAS_TOC_FILES: &[&str] = &[
    "MapCanvas_DataProviderBase.lua",
    "MapCanvas_PinFrameLevelsManager.lua",
    "Blizzard_MapCanvas.xml",
];

pub(super) const MAP_CANVAS_DEPENDENCIES: &[&str] =
    &["Blizzard_SharedXMLBase", "Blizzard_MapCanvasSecureUtil"];

pub(super) const MAP_CANVAS_PIN_FRAME_LEVELS_MANAGER_METHODS: &[&str] = &[
    "Initialize",
    "ValidateContiguous",
    "AddDefinition",
    "AddFrameLevel",
    "InsertFrameLevelAbove",
    "InsertFrameLevelBelow",
    "SetOverride",
    "ClearOverride",
    "GetFrameLevelStart",
    "GetFrameLevelRange",
    "GetValidFrameLevel",
];

pub(super) const MAP_CANVAS_DETAIL_LAYER_METHODS: &[&str] = &[
    "OnLoad",
    "SetMapAndLayer",
    "GetLayerIndex",
    "IsFullyLoaded",
    "SetLayerAlpha",
    "GetLayerAlpha",
    "SetGlobalAlpha",
    "GetGlobalAlpha",
    "RefreshDetailTiles",
    "OnUpdate",
    "RefreshAlpha",
];

pub(super) const CVAR_DATA_PROVIDER_METHODS: &[&str] =
    &["Init", "IsCVarSet", "OnShow", "OnHide", "OnEvent"];

pub(super) fn assert_mixin_methods_present(
    env: &WowLuaEnv,
    mixin: &str,
    methods: &[&str],
    context: &str,
) {
    for method in methods {
        let kind: String = env
            .eval(&format!("return type({mixin}.{method})"))
            .unwrap_or_else(|err| panic!("{mixin}.{method} probe failed: {err}"));
        assert_eq!(
            kind, "function",
            "{mixin}.{method} must be a function — {context}"
        );
    }
}

pub(super) fn count_mixin_methods(env: &WowLuaEnv, mixin: &str) -> i64 {
    env.eval(&format!(
        "local n = 0 for k, v in pairs({mixin}) do if type(v) == 'function' then n = n + 1 end end return n"
    ))
    .unwrap_or_else(|err| panic!("{mixin} method count probe failed: {err}"))
}
