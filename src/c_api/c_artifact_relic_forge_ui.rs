//! `C_ArtifactRelicForgeUI` namespace — currently exposes only
//! `IsAtForge`, which the LoD `Blizzard_ArtifactUI` addon consults at
//! lua:115-117 to decide whether `ARTIFACT_UPDATE` should auto-show the
//! relic-forge panel. Backed by `state.relic_forge_at_forge`.
//!
//! This is a separate namespace from `C_ArtifactUI.IsAtForge` (which
//! reads the artifact-panel forge state out of `viewed_artifact`).
//! Retail keeps the two probes distinct because the relic-forge is its
//! own NPC interaction.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_artifact_relic_forge_ui_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ArtifactRelicForgeUI")?;
    table_set_rust_fn_static(state, ns, "IsAtForge", is_at_forge)?;
    Ok(())
}

fn is_at_forge(state: &mut LuaState) -> LuaResult<u32> {
    let at_forge = borrow_state(state)?.relic_forge_at_forge;
    state.push(Val::Bool(at_forge));
    Ok(1)
}
