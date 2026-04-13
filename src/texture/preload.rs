use super::TextureManager;
use std::collections::BTreeSet;

pub(crate) fn collect_map_preload_texture_paths(map_id: u32) -> Vec<String> {
    let mut paths = BTreeSet::new();

    if let Some(map_art) = crate::map_art::get_map_art(map_id) {
        for file_data_id in map_art.tiles.iter().flat_map(|tiles| tiles.iter().copied()) {
            if let Some(path) = wow_texture_path_from_file_data_id(file_data_id) {
                paths.insert(path);
            }
        }
    }

    if let Some(overlays) = crate::map_exploration::get_overlays_for_map(map_id) {
        for file_data_id in overlays
            .iter()
            .flat_map(|overlay| overlay.file_data_ids.iter().copied())
        {
            if let Some(path) = wow_texture_path_from_file_data_id(file_data_id) {
                paths.insert(path);
            }
        }
    }

    paths.into_iter().collect()
}

fn wow_texture_path_from_file_data_id(file_data_id: u32) -> Option<String> {
    let path = crate::manifest_interface_data::get_texture_path(file_data_id)?;
    Some(format!("Interface\\{}", path.replace('/', "\\")))
}

impl TextureManager {
    /// Pre-load talent icon textures for the given tree to avoid on-demand lag.
    pub fn preload_talent_textures(&mut self, tree_id: u32) {
        use crate::traits::{TRAIT_DEFINITION_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB, TRAIT_TREE_DB};
        use std::collections::HashSet;

        let Some(tree) = TRAIT_TREE_DB.get(&tree_id) else {
            return;
        };
        let mut file_data_ids = HashSet::new();

        for &node_id in tree.node_ids {
            let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
                continue;
            };
            for &entry_id in node.entry_ids {
                let Some(entry) = TRAIT_ENTRY_DB.get(&entry_id) else {
                    continue;
                };
                let Some(def) = TRAIT_DEFINITION_DB.get(&entry.definition_id) else {
                    continue;
                };
                let icon_id = if def.override_icon != 0 {
                    def.override_icon
                } else {
                    let Some(spell) = crate::spells::get_spell(def.spell_id) else {
                        continue;
                    };
                    spell.icon_file_data_id
                };
                if icon_id != 0 {
                    file_data_ids.insert(icon_id);
                }
            }
        }

        let mut loaded = 0u32;
        for id in &file_data_ids {
            if let Some(path) = crate::manifest_interface_data::get_texture_path(*id) {
                let wow_path = format!("Interface\\{}", path.replace('/', "\\"));
                if self.load(&wow_path).is_some() {
                    loaded += 1;
                }
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} talent icon textures (tree {})",
            loaded,
            file_data_ids.len(),
            tree_id
        ));
    }

    /// Pre-load talent panel UI textures for the active class.
    ///
    /// Shared talent panel assets are always included. Class background atlases
    /// are filtered to the active class so startup does not decode every class'
    /// legacy background textures.
    pub fn preload_talent_panel_textures(&mut self, class_name: &str) {
        use crate::atlas::ATLAS_DB;
        use std::collections::HashSet;

        let class_key = normalize_talent_class_key(class_name);
        let mut files = HashSet::new();
        for (key, info) in ATLAS_DB.entries() {
            if should_preload_talent_atlas_key(key, class_key.as_deref()) {
                files.insert(info.file);
            }
        }

        let mut loaded = 0u32;
        for file in &files {
            if self.load(file).is_some() {
                loaded += 1;
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} talent panel textures ({})",
            loaded,
            files.len(),
            class_key.as_deref().unwrap_or("shared")
        ));
    }

    /// Pre-load common game HUD atlases that otherwise cause large first-use
    /// stalls when PlayerSpells and other game UI panels open.
    pub fn preload_game_hud_textures(&mut self) {
        const FILES: &[&str] = &[
            r"Interface\hud\uiminimap",
            r"Interface\hud\uiminimapbackground",
            r"Interface\hud\uiminimapvertical",
            r"Interface\hud\uiactionbar",
            r"Interface\hud\uiactionbarvertical",
            r"Interface\hud\uimicromenu2x",
            r"Interface\hud\uiunitframe",
            r"Interface\hud\uipartyframe",
            r"Interface\hud\uigroupmanager",
            r"Interface\hud\uicalendar",
            r"Interface\hud\uipartyframeportraitonmanamask",
            r"Interface\hud\uipartyframeportraitonhealthmask",
            r"Interface\hud\uiunitframeplayerportraitmask",
            r"Interface\hud\uiunitframeplayermanamask",
            r"Interface\hud\uiunitframeplayerhealthmask",
            r"Interface\questframe\questtracker",
            r"Interface\questframe\questimportantmapicons",
            r"Interface\questframe\questinprogressicons",
            r"Interface\chatframe\chatframe",
            r"Interface\ChatFrame\ChatFrameBackground",
            r"Interface\ChatFrame\UI-ChatFrame-BorderTop",
            r"Interface\ChatFrame\UI-ChatFrame-BorderLeft",
            r"Interface\ChatFrame\UI-ChatFrame-BorderCorner",
            r"Interface\ChatFrame\ChatFrameTab-BGMid",
            r"Interface\ChatFrame\ChatFrameTab-BGRight",
            r"Interface\ChatFrame\ChatFrameTab-BGLeft",
            r"Interface\containerframe\bagslots2x",
            r"Interface\buttons\minimalscrollbarproportional",
            r"Interface\masks\circlemask",
            r"Interface\Minimap\placeholder-map",
        ];

        let mut loaded = 0usize;
        for file in FILES {
            if self.load(file).is_some() {
                loaded += 1;
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} game HUD textures",
            loaded,
            FILES.len()
        ));
    }

    /// Pre-load non-glue UI atlases that are heavily used when opening the
    /// PlayerSpells / talents panels in the live renderer.
    pub fn preload_playerspells_runtime_textures(&mut self) {
        use crate::atlas::ATLAS_DB;
        use std::collections::HashSet;

        const FILES: &[&str] = &[
            r"Interface\Buttons\UI-Panel-Button-Up",
            r"Interface\FrameGeneral\UI-Background-Rock",
            r"Interface\TutorialFrame\UI-TutorialFrame-CalloutGlow",
        ];
        const PREFIXES: &[&str] = &[
            r"Interface\talentframe\",
            r"Interface\framegeneral\uiframe",
            r"Interface\common\commondropdown",
            r"Interface\common\commonmask",
            r"Interface\helpframe\newplayerexperienceparts",
            r"Interface\tutorialframe\",
        ];

        let mut files = HashSet::new();
        for path in FILES {
            files.insert(*path);
        }
        for (_, info) in ATLAS_DB.entries() {
            if PREFIXES.iter().any(|prefix| {
                info.file
                    .to_ascii_lowercase()
                    .starts_with(&prefix.to_ascii_lowercase())
            }) {
                files.insert(info.file);
            }
        }

        let mut loaded = 0usize;
        for file in &files {
            if self.load(file).is_some() {
                loaded += 1;
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} PlayerSpells runtime textures",
            loaded,
            files.len()
        ));
    }

    /// Pre-load spellbook / PlayerSpells icon textures from the static spell DB.
    pub fn preload_spellbook_icons(&mut self) {
        use crate::lua_api::globals::spellbook_data;
        use std::collections::HashSet;

        let mut file_data_ids = HashSet::new();
        for skill_line_index in 1..=spellbook_data::num_skill_lines() {
            if let Some(skill_line) = spellbook_data::get_skill_line(skill_line_index) {
                file_data_ids.insert(skill_line.icon_id);
                for entry in skill_line.spells {
                    if let Some(spell) = crate::spells::get_spell(entry.spell_id) {
                        if spell.icon_file_data_id != 0 {
                            file_data_ids.insert(spell.icon_file_data_id);
                        }
                    }
                }
            }
        }

        let mut loaded = 0u32;
        for id in &file_data_ids {
            if let Some(path) = crate::manifest_interface_data::get_texture_path(*id) {
                let wow_path = format!("Interface\\{}", path.replace('/', "\\"));
                if self.load(&wow_path).is_some() {
                    loaded += 1;
                }
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} spellbook icons",
            loaded,
            file_data_ids.len()
        ));
    }
}

pub(super) fn normalize_talent_class_key(class_name: &str) -> Option<String> {
    let normalized = class_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect::<String>();
    (!normalized.is_empty()).then_some(normalized)
}

pub(super) fn should_preload_talent_atlas_key(key: &str, class_key: Option<&str>) -> bool {
    if !key.starts_with("talents-") {
        return false;
    }
    match key.strip_prefix("talents-background-") {
        Some(rest) => {
            !rest.contains('-')
                || class_key
                    .map(|class_key| rest.starts_with(&format!("{class_key}-")))
                    .unwrap_or(false)
        }
        None => true,
    }
}
