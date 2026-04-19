//! Addon loading internals.

use crate::lua_api::LoaderEnv;
use crate::saved_variables::SavedVariablesManager;
use crate::toc::TocFile;
use rilua::Val;
use std::path::{Path, PathBuf};
use std::time::Instant;

use super::error::LoadError;
use super::lua_file::load_lua_file;
use super::xml_file::load_xml_file;
use super::{LoadResult, LoadTiming};

/// Context for loading addon files (name, private table, and addon root for path resolution).
pub struct AddonContext<'a> {
    pub name: &'a str,
    pub table: Val,
    /// Addon root directory for fallback path resolution
    pub addon_root: &'a Path,
    /// Whether this addon uses the secure Lua environment (UseSecureEnvironment: 1)
    pub use_secure_env: bool,
    /// Whether to taint code with the addon name (false for Blizzard base UI).
    pub taint: bool,
}

impl<'a> AddonContext<'a> {
    #[cfg(test)]
    pub fn new<L>(
        _lua: L,
        name: &'a str,
        table: Val,
        addon_root: &'a Path,
        use_secure_env: bool,
        taint: bool,
    ) -> crate::Result<Self> {
        Ok(Self {
            name,
            table,
            addon_root,
            use_secure_env,
            taint,
        })
    }
}

/// Initialize saved variables for an addon (WTF first, then JSON fallback).
fn init_saved_variables(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    mgr: &mut SavedVariablesManager,
) -> Vec<String> {
    let mut warnings = Vec::new();
    match env.with_state(|state| mgr.load_wtf_for_addon(state, folder_name)) {
        Ok(count) if count > 0 => {
            tracing::debug!(
                "Loaded {} WTF SavedVariables file(s) for {}",
                count,
                toc.name
            );
        }
        Ok(_) => {
            let saved_vars = toc.saved_variables();
            let saved_vars_per_char = toc.saved_variables_per_character();
            if (!saved_vars.is_empty() || !saved_vars_per_char.is_empty())
                && let Err(e) = env.with_state(|state| {
                    mgr.init_for_addon(state, folder_name, &saved_vars, &saved_vars_per_char)
                })
            {
                warnings.push(format!(
                    "Failed to initialize saved variables for {}: {}",
                    folder_name, e
                ));
            }
        }
        Err(e) => {
            warnings.push(format!(
                "Failed to load WTF SavedVariables for {}: {}",
                folder_name, e
            ));
        }
    }
    warnings
}

/// Internal addon loading with optional saved variables.
pub fn load_addon_internal(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    saved_vars_mgr: Option<&mut SavedVariablesManager>,
) -> Result<LoadResult, LoadError> {
    let folder_name = toc
        .addon_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&toc.name);

    let mut result = LoadResult {
        name: toc.name.clone(),
        lua_files: 0,
        xml_files: 0,
        timing: LoadTiming::default(),
        warnings: Vec::new(),
    };

    maybe_init_saved_variables(env, toc, folder_name, saved_vars_mgr, &mut result);
    let ctx = build_addon_context(env, toc, folder_name)?;
    let nil_symbol_access_start = env.state().borrow().nil_symbol_accesses.len();
    let addon_name = result.name.clone();

    load_addon_files(env, toc, folder_name, &ctx, &mut result);
    apply_blizzard_post_load_patches(env, folder_name, &mut result);
    append_nil_symbol_access_warnings(env, &addon_name, nil_symbol_access_start, &mut result);
    mark_addon_loaded(env, folder_name);
    Ok(result)
}

/// Run hand-written workarounds that must fire after specific Blizzard addons
/// finish loading (e.g. restoring globals the addon's cleanup wiped, or
/// monkey-patching mixins that don't quite line up with our stub state).
/// Each patch is keyed by addon folder name and only fires for that addon.
fn apply_blizzard_post_load_patches(
    env: &LoaderEnv<'_>,
    folder_name: &str,
    result: &mut LoadResult,
) {
    match folder_name {
        "Blizzard_EnvironmentCleanup" => patch_environment_cleanup(env, result),
        "Blizzard_Menu" => patch_menu_descriptor_fallback(env, result),
        "Blizzard_AccountStore" => patch_account_store_set_storefront(env, result),
        "Blizzard_SharedXML" => patch_shared_xml_anim_mixins(env, result),
        "Blizzard_UIParent" => patch_uiparent_managed_frame_mixin(env, result),
        "Blizzard_GlueParent" => patch_glueparent_uiparent_attributes(env, result),
        "Blizzard_MapCanvas" => patch_map_canvas_scroll_container(env, result),
        "Blizzard_SharedMapDataProviders" => patch_unit_position_frame_mixin(env, result),
        "Blizzard_UIPanels_Game" => patch_quest_log_mixin(env, result),
        "Blizzard_PlayerSpells" => patch_playerspells_onload_backfill(env, result),
        _ => {}
    }
}

fn push_patch_warning(
    result: &mut LoadResult,
    folder_name: &str,
    what: &str,
    e: &dyn std::fmt::Display,
) {
    result
        .warnings
        .push(format!("Failed to {what} for {folder_name}: {e}"));
}

fn patch_environment_cleanup(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.restore_post_cleanup_globals() {
        push_patch_warning(
            result,
            "Blizzard_EnvironmentCleanup",
            "restore post-cleanup globals",
            &e,
        );
    }
}

fn patch_menu_descriptor_fallback(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.ensure_menu_descriptor_fallback() {
        push_patch_warning(
            result,
            "Blizzard_Menu",
            "install Menu descriptor fallback",
            &e,
        );
    }
}

fn patch_account_store_set_storefront(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = crate::lua_api::workarounds::patch_account_store_set_storefront(env) {
        push_patch_warning(
            result,
            "Blizzard_AccountStore",
            "patch AccountStoreFrame.SetStoreFrontID",
            &e,
        );
    }
}

const SHARED_XML_ANIM_MIXIN_PATCH: &str = r#"
    local mixins = {
        VisibleWhilePlayingAnimGroupMixin,
        TargetsVisibleWhilePlayingAnimGroupMixin,
        SyncedAnimGroupMixin,
    }

    for _, mixin in ipairs(mixins) do
        if type(mixin) == "table" and type(mixin.SetPlaying) ~= "function" then
            function mixin:SetPlaying(playing)
                if playing then
                    if type(self.PlaySynced) == "function" then
                        self:PlaySynced()
                    else
                        self:Play()
                    end
                else
                    self:Stop()
                end
            end
        end
    end
"#;

fn patch_shared_xml_anim_mixins(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.exec(SHARED_XML_ANIM_MIXIN_PATCH) {
        push_patch_warning(
            result,
            "Blizzard_SharedXML",
            "patch Blizzard_SharedXML animation mixins",
            &e,
        );
    }
}

fn patch_uiparent_managed_frame_mixin(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.patch_managed_frame_mixin() {
        push_patch_warning(
            result,
            "Blizzard_UIParent",
            "patch UIParentManagedFrameMixin",
            &e,
        );
    }
}

/// `Blizzard_GlueParent/Mainline/GlueParent.lua:74` does `UIParent = self` in
/// `GlueParentMixin:OnLoad`, aliasing the global to the glue-screen frame.
/// In real WoW that's harmless because the glue addon only loads on the
/// character-select screen, but the simulator loads everything — so
/// `UpdateUIPanelPositions` reads `UIParent:GetAttribute("TOP_OFFSET")` off
/// the GlueParent frame and gets nil, blowing up panel-frame anchoring.
/// Re-apply the static `<Attribute>` block from
/// `Blizzard_UIParent/Mainline/UIParent.xml` so the in-game panel manager
/// sees the values it expects.
fn patch_glueparent_uiparent_attributes(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.exec(
        r#"
        if UIParent and type(UIParent.SetAttribute) == "function" then
            UIParent:SetAttribute("DEFAULT_FRAME_WIDTH", 384)
            UIParent:SetAttribute("TOP_OFFSET", -116)
            UIParent:SetAttribute("LEFT_OFFSET", 16)
            UIParent:SetAttribute("CENTER_OFFSET", 384)
            UIParent:SetAttribute("RIGHT_OFFSET", 768)
            UIParent:SetAttribute("RIGHT_OFFSET_BUFFER", 80)
            UIParent:SetAttribute("PANEl_SPACING_X", 32)
        end
        "#,
    ) {
        push_patch_warning(
            result,
            "Blizzard_GlueParent",
            "restore UIParent attributes after GlueParent alias",
            &e,
        );
    }
}

fn patch_unit_position_frame_mixin(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.patch_unit_position_frame_mixin() {
        push_patch_warning(
            result,
            "Blizzard_SharedMapDataProviders",
            "patch UnitPositionFrameMixin",
            &e,
        );
    }
}

fn patch_quest_log_mixin(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.patch_quest_log_mixin() {
        push_patch_warning(result, "Blizzard_UIPanels_Game", "patch QuestLogMixin", &e);
    }
}

fn patch_map_canvas_scroll_container(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.exec(
        r#"
        local function __wow_find_first_scroll_frame_child(parent)
            if type(parent) ~= "table" or type(parent.GetNumChildren) ~= "function" or type(parent.GetChildren) ~= "function" then
                return nil
            end

            local count = parent:GetNumChildren()
            for index = 1, count do
                local child = select(index, parent:GetChildren())
                if type(child) == "table" then
                    local isScrollFrame =
                        (type(child.IsObjectType) == "function" and child:IsObjectType("ScrollFrame")) or
                        (type(child.GetObjectType) == "function" and child:GetObjectType() == "ScrollFrame")
                    if isScrollFrame then
                        return child
                    end
                end
            end

            return nil
        end

        local function __wow_ensure_map_canvas_scroll_container(frame)
            if type(frame) ~= "table" then
                return nil
            end

            local existing = rawget(frame, "ScrollContainer")
            if existing ~= nil then
                return existing
            end

            local scroll = __wow_find_first_scroll_frame_child(frame)
            if scroll ~= nil then
                rawset(frame, "ScrollContainer", scroll)
            end
            return scroll
        end

        local function __wow_try_init_map_canvas(frame)
            if type(frame) ~= "table" then
                return
            end

            __wow_ensure_map_canvas_scroll_container(frame)
            if rawget(frame, "__wow_map_canvas_onload_ran") then
                return
            end

            local scroll = rawget(frame, "ScrollContainer")
            if scroll == nil then
                return
            end

            rawset(frame, "__wow_map_canvas_onload_ran", true)
            local originalOnLoad = rawget(_G, "__wow_map_canvas_original_onload")
            if type(originalOnLoad) == "function" then
                originalOnLoad(frame)
            end
        end

        if type(MapCanvasMixin) == "table" and not rawget(_G, "__wow_map_canvas_scroll_container_patched") then
            if rawget(_G, "__wow_map_canvas_original_onload") == nil and type(MapCanvasMixin.OnLoad) == "function" then
                _G.__wow_map_canvas_original_onload = MapCanvasMixin.OnLoad
                MapCanvasMixin.OnLoad = function(self, ...)
                    if rawget(self, "__wow_map_canvas_onload_ran") then
                        return
                    end
                    __wow_try_init_map_canvas(self)
                end
            end

            if type(MapCanvasMixin.SetMapID) == "function" then
                local originalSetMapID = MapCanvasMixin.SetMapID
                MapCanvasMixin.SetMapID = function(self, ...)
                    __wow_try_init_map_canvas(self)
                    return originalSetMapID(self, ...)
                end
            end

            if type(MapCanvasMixin.GetCanvas) == "function" then
                local originalGetCanvas = MapCanvasMixin.GetCanvas
                MapCanvasMixin.GetCanvas = function(self, ...)
                    __wow_try_init_map_canvas(self)
                    return originalGetCanvas(self, ...)
                end
            end

            if type(MapCanvasMixin.GetCanvasContainer) == "function" then
                local originalGetCanvasContainer = MapCanvasMixin.GetCanvasContainer
                MapCanvasMixin.GetCanvasContainer = function(self, ...)
                    __wow_try_init_map_canvas(self)
                    return originalGetCanvasContainer(self, ...)
                end
            end

            if type(MapCanvasMixin.OnFrameSizeChanged) == "function" then
                local originalOnFrameSizeChanged = MapCanvasMixin.OnFrameSizeChanged
                MapCanvasMixin.OnFrameSizeChanged = function(self, ...)
                    __wow_try_init_map_canvas(self)
                    return originalOnFrameSizeChanged(self, ...)
                end
            end

            __wow_map_canvas_scroll_container_patched = true
        end
        "#,
    ) {
        push_patch_warning(
            result,
            "Blizzard_MapCanvas",
            "patch map canvas scroll container",
            &e,
        );
    }
}

const PLAYERSPELLS_ONLOAD_BACKFILL_PATCH: &str = r#"
    HasAttachedGlyph = HasAttachedGlyph or function()
        return false
    end

    IsSpellValidForPendingGlyph = IsSpellValidForPendingGlyph or function()
        return false
    end

    local function backfill_onload(frame, needs_init)
        if not frame or not needs_init then
            return
        end
        if type(frame.OnLoad) == "function" then
            frame:OnLoad()
            return
        end
        local handler = frame.GetScript and frame:GetScript("OnLoad")
        if type(handler) == "function" then
            handler(frame)
        end
    end

    local function backfill_playerspells_tab(frame_tab)
        if not PlayerSpellsFrame or not PlayerSpellsUtil or not PlayerSpellsUtil.FrameTabs then
            return
        end

        if frame_tab == PlayerSpellsUtil.FrameTabs.ClassSpecializations then
            backfill_onload(
                PlayerSpellsFrame.SpecFrame,
                PlayerSpellsFrame.SpecFrame and PlayerSpellsFrame.SpecFrame.SpecContentFramePool == nil
            )
        elseif frame_tab == PlayerSpellsUtil.FrameTabs.ClassTalents then
            backfill_onload(
                PlayerSpellsFrame.TalentsFrame,
                PlayerSpellsFrame.TalentsFrame and PlayerSpellsFrame.TalentsFrame.initialBasePanOffsetX == nil
            )
        elseif frame_tab == PlayerSpellsUtil.FrameTabs.SpellBook then
            backfill_onload(
                PlayerSpellsFrame.SpellBookFrame,
                PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame.internalTabTracker == nil
            )
        end
    end

    if PlayerSpellsFrame then
        backfill_onload(PlayerSpellsFrame, PlayerSpellsFrame.internalTabTracker == nil)
        if not __wow_uisim_playerspells_backfill_wrapped then
            __wow_uisim_playerspells_backfill_wrapped = true

            if type(PlayerSpellsFrame.TrySetTab) == "function" then
                local original_try_set_tab = PlayerSpellsFrame.TrySetTab
                PlayerSpellsFrame.TrySetTab = function(self, frame_tab)
                    backfill_playerspells_tab(frame_tab)
                    return original_try_set_tab(self, frame_tab)
                end
            end

            if type(PlayerSpellsFrame.SetInspecting) == "function" then
                local original_set_inspecting = PlayerSpellsFrame.SetInspecting
                PlayerSpellsFrame.SetInspecting = function(self, inspect_unit, inspect_string, inspect_string_level)
                    if inspect_unit or inspect_string then
                        backfill_playerspells_tab(PlayerSpellsUtil.FrameTabs.ClassTalents)
                    end
                    return original_set_inspecting(self, inspect_unit, inspect_string, inspect_string_level)
                end
            end
        end
    end
"#;

fn patch_playerspells_onload_backfill(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.exec(PLAYERSPELLS_ONLOAD_BACKFILL_PATCH) {
        push_patch_warning(
            result,
            "Blizzard_PlayerSpells",
            "backfill PlayerSpells OnLoad state",
            &e,
        );
    }
}

/// Flip the addon's `loaded` flag in `SimState.addons` and clear the
/// "currently loading" index so that subsequent `IsAddOnLoaded` calls and
/// nested-load detection see the correct state.
fn mark_addon_loaded(env: &LoaderEnv<'_>, folder_name: &str) {
    let mut state = env.state().borrow_mut();
    if let Some(addon) = state
        .addons
        .iter_mut()
        .find(|addon| addon.folder_name == folder_name)
    {
        addon.loaded = true;
    }
    state.loading_addon_index = None;
}

fn maybe_init_saved_variables(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    saved_vars_mgr: Option<&mut SavedVariablesManager>,
    result: &mut LoadResult,
) {
    let sv_start = Instant::now();
    match saved_vars_mgr {
        Some(mgr) => result
            .warnings
            .extend(init_saved_variables(env, toc, folder_name, mgr)),
        None => seed_console_saved_variables_without_persistence(env, toc, folder_name, result),
    }
    result.timing.saved_vars_time = sv_start.elapsed();
}

fn seed_console_saved_variables_without_persistence(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    result: &mut LoadResult,
) {
    if folder_name != "Blizzard_Console" {
        return;
    }

    let saved_vars = toc.saved_variables();
    if saved_vars.is_empty() {
        return;
    }

    if let Err(error) = env.with_state(|state| {
        SavedVariablesManager::seed_declared_globals(state, &saved_vars, &[]);
        Ok::<(), crate::Error>(())
    }) {
        result.warnings.push(format!(
            "Failed to seed console saved variables for {} without persistence: {}",
            folder_name, error
        ));
    }
}

fn build_addon_context<'a>(
    env: &LoaderEnv<'a>,
    toc: &'a TocFile,
    folder_name: &'a str,
) -> Result<AddonContext<'a>, LoadError> {
    let addon_table = env
        .create_addon_table()
        .map_err(|e| LoadError::Lua(e.to_string()))?;
    register_loading_addon(env, folder_name, toc.is_secure_env());

    Ok(AddonContext {
        name: folder_name,
        table: addon_table,
        addon_root: &toc.addon_dir,
        use_secure_env: toc.is_secure_env(),
        taint: !is_blizzard_addon(toc),
    })
}

fn register_loading_addon(env: &LoaderEnv<'_>, folder_name: &str, use_secure_env: bool) {
    // Set loading_addon_index so frames created during this addon's load
    // are attributed to it. Panic if addon not registered — caller bug.
    let addon_idx = resolve_addon_index(env, folder_name);
    let mut state = env.state().borrow_mut();
    state.loading_addon_index = Some(addon_idx);
    if let Some(addon) = state.addons.get_mut(addon_idx as usize) {
        addon.use_secure_env = use_secure_env;
    }
}

fn is_blizzard_addon(toc: &TocFile) -> bool {
    // Blizzard base UI code runs securely (no taint). Third-party addons
    // get tainted with their folder name so issecurevariable tracks the source.
    toc.addon_dir.to_string_lossy().contains("BlizzardUI")
}

/// Find or auto-register addon in the addon list, returning its index.
fn resolve_addon_index(env: &LoaderEnv<'_>, folder_name: &str) -> u16 {
    let mut s = env.state().borrow_mut();
    let idx = s
        .addons
        .iter()
        .position(|a| a.folder_name == folder_name)
        .unwrap_or_else(|| {
            let idx = s.addons.len();
            s.addons.push(crate::lua_api::AddonInfo {
                folder_name: folder_name.to_string(),
                title: folder_name.to_string(),
                enabled: true,
                ..Default::default()
            });
            idx
        });
    idx as u16
}

/// Load all Lua/XML files listed in the TOC, applying local overlay paths.
fn load_addon_files(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    ctx: &AddonContext,
    result: &mut LoadResult,
) {
    let overlay_dir = Path::new("Interface/AddOns").join(folder_name);

    for (index, (file_rel, file)) in toc.files.iter().zip(toc.file_paths()).enumerate() {
        let resolved_file = resolve_addon_file_path(&overlay_dir, file_rel, file);
        let file_ctx = AddonContext {
            name: ctx.name,
            table: ctx.table,
            addon_root: ctx.addon_root,
            use_secure_env: toc.file_use_secure_env(index).unwrap_or(ctx.use_secure_env),
            taint: ctx.taint,
        };
        load_addon_file(env, &file_ctx, result, &resolved_file);
    }
}

fn resolve_addon_file_path(
    overlay_dir: &Path,
    file_rel: &Path,
    default_file: std::path::PathBuf,
) -> std::path::PathBuf {
    let overlay_file = overlay_dir.join(file_rel);
    if overlay_file.exists() {
        return overlay_file;
    }
    default_file
}

fn load_addon_file(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext<'_>,
    result: &mut LoadResult,
    file: &std::path::Path,
) {
    match file.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
        "lua" => load_addon_lua_file(env, ctx, result, file),
        "xml" => load_addon_xml_file(env, ctx, result, file),
        _ => result
            .warnings
            .push(format!("{}: unknown file type", file.display())),
    }
}

fn append_nil_symbol_access_warnings(
    env: &LoaderEnv<'_>,
    addon_name: &str,
    start_index: usize,
    result: &mut LoadResult,
) {
    let grouped_accesses = {
        let state = env.state().borrow();
        summarize_nil_symbol_accesses(&state.nil_symbol_accesses[start_index..])
    };
    result.warnings.extend(
        grouped_accesses
            .into_iter()
            .map(|report| format_missing_symbol_report(addon_name, &report)),
    );
}

fn summarize_nil_symbol_accesses(
    accesses: &[crate::lua_api::state::NilSymbolAccess],
) -> Vec<MissingSymbolReport> {
    let mut reports = std::collections::BTreeMap::new();
    for access in accesses {
        let need = classify_nil_symbol_access(access);
        let location = format_nil_symbol_location(access);
        reports.entry(need).or_insert(location);
    }

    reports
        .into_iter()
        .map(|(need, location)| MissingSymbolReport { need, location })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MissingSymbolNeed {
    Global(String),
    CNamespace(String),
    CMethod { namespace: String, method: String },
}

fn classify_nil_symbol_access(
    access: &crate::lua_api::state::NilSymbolAccess,
) -> MissingSymbolNeed {
    if access.container == "_G" {
        return classify_global_nil_access(&access.key);
    }

    if access.container.starts_with("C_") {
        return MissingSymbolNeed::CMethod {
            namespace: access.container.clone(),
            method: access.key.clone(),
        };
    }

    MissingSymbolNeed::Global(format!("{}.{}", access.container, access.key))
}

fn classify_global_nil_access(key: &str) -> MissingSymbolNeed {
    if key.starts_with("C_") {
        return MissingSymbolNeed::CNamespace(key.to_string());
    }

    MissingSymbolNeed::Global(key.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MissingSymbolReport {
    need: MissingSymbolNeed,
    location: Option<String>,
}

fn format_missing_symbol_report(addon_name: &str, report: &MissingSymbolReport) -> String {
    let need = match &report.need {
        MissingSymbolNeed::Global(name) => format!("global {name}"),
        MissingSymbolNeed::CNamespace(namespace) => namespace.clone(),
        MissingSymbolNeed::CMethod { namespace, method } => format!("{namespace}.{method}"),
    };

    match &report.location {
        Some(location) => format!("{addon_name} needs {need} (accessed at {location})"),
        None => format!("{addon_name} needs {need}"),
    }
}

fn format_nil_symbol_location(access: &crate::lua_api::state::NilSymbolAccess) -> Option<String> {
    let source = access.source.as_deref()?;
    let line = access.line?;
    Some(format!("{}:{line}", summarize_chunk_source(source)))
}

fn summarize_chunk_source(source: &str) -> String {
    let stripped = source.trim_start_matches(['@', '=']);
    let path = PathBuf::from(stripped);
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| stripped.to_string())
}

fn load_addon_lua_file(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext<'_>,
    result: &mut LoadResult,
    file: &std::path::Path,
) {
    match load_lua_file(env, file, ctx, &mut result.timing) {
        Ok(()) => result.lua_files += 1,
        Err(error) => result
            .warnings
            .push(format!("{}: {}", file.display(), error)),
    }
    apply_cpp_mixin_stubs(env);
}

fn load_addon_xml_file(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext<'_>,
    result: &mut LoadResult,
    file: &std::path::Path,
) {
    match load_xml_file(env, file, ctx, &mut result.timing) {
        Ok(count) => {
            result.xml_files += 1;
            result.lua_files += count;
        }
        Err(error) => result
            .warnings
            .push(format!("{}: {}", file.display(), error)),
    }
}

/// Patch Lua mixin tables with methods normally provided by the C++ engine.
///
/// WoW's C++ engine provides OnLoad for certain base control button mixins.
/// The Lua side creates empty tables (e.g. `ModelSceneControlButtonMixin = {}`),
/// and derived mixins call `BaseMixin.OnLoad(self)` expecting the C++ method.
/// Runs after each .lua file so stubs are available before the next .xml file
/// creates frames that depend on them.
fn apply_cpp_mixin_stubs(env: &LoaderEnv<'_>) {
    let _ = env.exec(
        r#"
        local ModelSceneControlButtonMixin = rawget(_G, "ModelSceneControlButtonMixin")
        if ModelSceneControlButtonMixin and not ModelSceneControlButtonMixin.OnLoad then
            ModelSceneControlButtonMixin.OnLoad = function() end
        end
        local PerksModelSceneControlButtonMixin = rawget(_G, "PerksModelSceneControlButtonMixin")
        if PerksModelSceneControlButtonMixin and not PerksModelSceneControlButtonMixin.OnLoad then
            PerksModelSceneControlButtonMixin.OnLoad = function() end
        end
        local PetActionBarMixin = rawget(_G, "PetActionBarMixin")
        if PetActionBarMixin and PetActionBarMixin.Update and not PetActionBarMixin._update_guarded then
            PetActionBarMixin._update_guarded = true
            local origUpdate = PetActionBarMixin.Update
            PetActionBarMixin.Update = function(self)
                if not self.actionButtons or #self.actionButtons == 0 then return end
                return origUpdate(self)
            end
        end
        "#,
    );
}
