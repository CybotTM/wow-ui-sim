//! Integration test for the `SOUNDKIT.IG_INVENTORY_ROTATE_CHARACTER`
//! constant.
//!
//! Drives `Blizzard_AlliedRacesFrameUI.lua:173,186` and several other
//! call sites (CharacterSelect, CharacterCreate, PerksProgram,
//! ModelSceneControlFrame). Real WoW resolves the symbol to the
//! engine sound id `861` and `PlaySound` cues the audio. The
//! simulator stubs `PlaySound` for any numeric id, so the only
//! requirement is that the symbol exists with the canonical numeric
//! value (sourced from
//! `vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/Mainline/SoundKitConstants.lua:51`).

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn soundkit_ig_inventory_rotate_character_resolves_to_canonical_id() {
    let env = WowLuaEnv::new().expect("env");
    let id: f64 = env
        .eval("return SOUNDKIT.IG_INVENTORY_ROTATE_CHARACTER")
        .unwrap();
    assert_eq!(
        id as i64, 861,
        "must match the canonical engine sound id from SoundKitConstants.lua so addons that compare against the literal don't silently diverge"
    );
}

#[test]
fn play_sound_with_ig_inventory_rotate_character_does_not_error() {
    // The AlliedRaces banner click handlers call
    // `PlaySound(SOUNDKIT.IG_INVENTORY_ROTATE_CHARACTER)`. The
    // simulator's PlaySound is a no-op for audio, but the call must
    // accept the resolved numeric id without raising.
    let env = WowLuaEnv::new().expect("env");
    env.exec("PlaySound(SOUNDKIT.IG_INVENTORY_ROTATE_CHARACTER)")
        .expect("PlaySound must accept the resolved soundkit id");
}
