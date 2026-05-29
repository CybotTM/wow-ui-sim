//! Mists CharacterFrame parent bootstrap.
//!
//! The current cache has Mists `Blizzard_UIPanels_Game` depending on a
//! `Blizzard_CharacterFrame` addon while the actual CharacterFrame/PaperDoll
//! source split is absent. TokenUI and some UIPanels XML attach frames to
//! `CharacterFrame`, so create the parent and currency tab anchor before those
//! addons run.

use std::path::Path;

use crate::loader::{LoadError, LoadResult};
use crate::lua_api::LoaderEnv;
use crate::toc::TocFile;

const CHARACTER_FRAME_BOOTSTRAP_LUA: &str = r#"
if type(CharacterFrame) ~= "table" then
  CharacterFrame = CreateFrame("Frame", "CharacterFrame", UIParent, "ButtonFrameTemplate")
  CharacterFrame:Hide()
end

for index = 1, 4 do
  local name = "CharacterFrameTab" .. index
  if type(_G[name]) ~= "table" then
    local tab = CreateFrame("Button", name, CharacterFrame)
    tab:SetID(index)
    tab:Hide()
  end
end
"#;

pub(crate) fn ensure_before_addon(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    _toc_path: &Path,
) -> Result<Option<LoadResult>, LoadError> {
    if !needs_character_frame_preload(&toc.name) {
        return Ok(None);
    }

    let _ = env.exec(CHARACTER_FRAME_BOOTSTRAP_LUA);
    Ok(None)
}

fn needs_character_frame_preload(addon_name: &str) -> bool {
    matches!(addon_name, "Blizzard_UIPanels_Game" | "Blizzard_TokenUI")
}
