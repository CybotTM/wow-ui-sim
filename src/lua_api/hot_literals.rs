//! Pre-intern whitelist for hot Lua string literals used during Blizzard
//! UI startup.
//!
//! # What lives here
//!
//! A static, hand-curated list of byte-slices that the VM sees many times
//! during `--no-addons --no-saved-vars` startup. These are candidates for
//! `Gc::intern_string_static(&'static [u8])` so the pointer-keyed intern
//! cache short-circuits them on every lookup.
//!
//! The list is organized by semantic category (globals, namespaces, method
//! / property keys, metatable keys, loader sentinels) so later sub-items
//! (registry module, hot-path conversions) can opt into categories
//! selectively.
//!
//! # What does NOT live here
//!
//! - The registry itself (sub-item 2 in Track 1).
//! - Any `intern_string_static` call — sub-items 3 convert the hot paths
//!   to consume the registry. This file is pure data.
//! - Runtime-discovered literals. The spirit of the PLAN task is "static
//!   and versioned, not runtime-discovered", so additions to this list
//!   should be deliberate and reviewed.
//!
//! # Versioning
//!
//! Bump [`WHITELIST_VERSION`] whenever entries change in a way that could
//! invalidate downstream invariants (e.g. the Track 3 slotted-global
//! fast-path is keyed by index into this list). Adding new entries at the
//! end is a soft bump; reordering or removing entries requires a hard bump
//! and invalidates any on-disk bytecode keyed on the old version.
//!
//! # Source of truth
//!
//! Entries originate from grepping the existing rilua wow-ui-sim codebase
//! for repeated string literals in startup paths (namespace names,
//! frame-method identifiers, metatable keys, chunk tags) plus first-party
//! knowledge of the Blizzard addon API surface. The `wow_ui_sim` runtime
//! will compare its own `intern_string` traffic against this list in a
//! measurement step (Track 1 sub-item 4).

/// Increment on any change that could invalidate downstream ABI (e.g.
/// Track 3's slotted global vector is indexed off [`HOT_GLOBALS`]).
pub const WHITELIST_VERSION: u32 = 1;

// ── Global symbols ─────────────────────────────────────────────────────────
//
// Bare globals the Blizzard UI reads many times during startup. Most enter
// via `SETGLOBAL` at addon-load time and are then read every frame by the
// layout / event / script dispatch paths.

pub const HOT_GLOBALS: &[&[u8]] = &[
    b"_G",
    b"UIParent",
    b"WorldFrame",
    b"GameTooltip",
    b"UIErrorsFrame",
    b"ChatFrame1",
    b"MainMenuBar",
    b"MainActionBar",
    b"PlayerFrame",
    b"TargetFrame",
    b"MinimapCluster",
    b"Minimap",
    b"SettingsPanel",
    b"PlayerSpellsFrame",
    b"QuestFrame",
    b"GossipFrame",
    b"MerchantFrame",
    b"CharacterFrame",
    b"FriendsFrame",
    b"LFGListFrame",
    b"Constants",
    // Blizzard "util" globals that load early and are called from many
    // addons' OnLoad handlers.
    b"Mixin",
    b"CreateFromMixins",
    b"CreateFrame",
    b"CopyTable",
    b"Clamp",
    b"SetParentFrameLevel",
    b"GetTime",
    b"GetCVar",
    b"SetCVar",
    b"SecureHandlerExecute",
    b"securecall",
    b"issecure",
    b"issecurevariable",
    b"hooksecurefunc",
];

// ── Namespace tables ───────────────────────────────────────────────────────
//
// All C_* namespaces plus Enum/Constants. These appear both as global
// lookups (`C_Foo.Bar(...)`) and as table-key interns during the
// registration phase. Full set is 83 entries (grepped from
// `grep -rEo '"(C_[A-Za-z]+)"' src/lua_api/`); listed exhaustively because
// Track 3 wants a stable slot per namespace.

pub const HOT_NAMESPACES: &[&[u8]] = &[
    b"Enum",
    b"Constants",
    // C_* namespaces, alphabetized. Additions here are an ABI bump candidate.
    b"C_AchievementInfo",
    b"C_ActionBar",
    b"C_AddOns",
    b"C_AdventureMap",
    b"C_AreaPoiInfo",
    b"C_AuctionHouse",
    b"C_Bank",
    b"C_BattleNet",
    b"C_BehavioralMessaging",
    b"C_CVar",
    b"C_CampaignInfo",
    b"C_CharacterCreation",
    b"C_CharacterServices",
    b"C_ChatBubbles",
    b"C_ChatInfo",
    b"C_ChromieTime",
    b"C_ClassColor",
    b"C_ClassTrial",
    b"C_Club",
    b"C_ClubFinder",
    b"C_Commentator",
    b"C_Console",
    b"C_Container",
    b"C_ContributionCollector",
    b"C_Covenants",
    b"C_CreatureInfo",
    b"C_CurrencyInfo",
    b"C_DateAndTime",
    b"C_DeathRecap",
    b"C_DelvesUI",
    b"C_DurationUtil",
    b"C_EncounterJournal",
    b"C_EquipmentSet",
    b"C_EventUtils",
    b"C_FogOfWar",
    b"C_FriendList",
    b"C_GMTicketInfo",
    b"C_GameRules",
    b"C_Garrison",
    b"C_GossipInfo",
    b"C_GuildInfo",
    b"C_Heirloom",
    b"C_Housing",
    b"C_InvasionInfo",
    b"C_IslandsQueue",
    b"C_Item",
    b"C_ItemSocketInfo",
    b"C_ItemUpgrade",
    b"C_LFGInfo",
    b"C_LFGList",
    b"C_Loot",
    b"C_LootHistory",
    b"C_LootJournal",
    b"C_Mail",
    b"C_Map",
    b"C_MapExplorationInfo",
    b"C_MerchantFrame",
    b"C_ModelInfo",
    b"C_MountJournal",
    b"C_Navigation",
    b"C_NewItems",
    b"C_PaperDollInfo",
    b"C_PartyInfo",
    b"C_PartyPose",
    b"C_PetBattles",
    b"C_PetInfo",
    b"C_PetJournal",
    b"C_PhotoSharing",
    b"C_PlayerInfo",
    b"C_ProfSpecs",
    b"C_PvP",
    b"C_QuestLine",
    b"C_QuestLog",
    b"C_RaidLocks",
    b"C_RecruitAFriend",
    b"C_Reputation",
    b"C_ScenarioInfo",
    b"C_ScriptedAnimations",
    b"C_Seasons",
    b"C_SharedCharacterServices",
    b"C_Social",
    b"C_Soulbinds",
    b"C_Spell",
    b"C_SpellBook",
    b"C_StorePublic",
    b"C_SummonInfo",
    b"C_System",
    b"C_TaskQuest",
    b"C_Texture",
    b"C_Timer",
    b"C_TooltipInfo",
    b"C_ToyBox",
    b"C_TradeSkillUI",
    b"C_Transmog",
    b"C_Tutorial",
    b"C_UI",
    b"C_UIWidgetManager",
    b"C_UnitAuras",
    b"C_VideoOptions",
    b"C_VoiceChat",
    b"C_WowEntitlementInfo",
    b"C_WowTokenSecure",
    b"C_ZoneAbility",
];

// ── Frame method / property keys ──────────────────────────────────────────
//
// The most-called method names on FrameRef userdata during startup.
// Sourced from grep over `define_methods!` call sites plus the set of
// method names the XML loader synthesizes for setters (SetXxx) and
// getters (GetXxx) on every frame it instantiates.

pub const HOT_FRAME_METHODS: &[&[u8]] = &[
    // Identity / lifecycle.
    b"GetName",
    b"GetObjectType",
    b"IsObjectType",
    b"GetID",
    b"SetID",
    b"Hide",
    b"Show",
    b"IsShown",
    b"IsVisible",
    b"SetShown",
    // Layout / geometry.
    b"SetPoint",
    b"SetAllPoints",
    b"ClearAllPoints",
    b"GetPoint",
    b"GetNumPoints",
    b"SetWidth",
    b"SetHeight",
    b"GetWidth",
    b"GetHeight",
    b"SetSize",
    b"GetSize",
    b"GetRect",
    b"GetScaledRect",
    b"GetLeft",
    b"GetRight",
    b"GetTop",
    b"GetBottom",
    b"GetCenter",
    b"SetScale",
    b"GetScale",
    b"SetFrameStrata",
    b"GetFrameStrata",
    b"SetFrameLevel",
    b"GetFrameLevel",
    // Script dispatch.
    b"SetScript",
    b"GetScript",
    b"HookScript",
    b"RegisterEvent",
    b"UnregisterEvent",
    b"UnregisterAllEvents",
    b"IsEventRegistered",
    b"RegisterForDrag",
    b"RegisterForClicks",
    // Parent / children.
    b"GetParent",
    b"SetParent",
    b"GetChildren",
    b"GetNumChildren",
    b"GetRegions",
    b"GetNumRegions",
    // Attributes.
    b"SetAttribute",
    b"GetAttribute",
    // Visuals.
    b"SetAlpha",
    b"GetAlpha",
    b"SetText",
    b"GetText",
    b"SetFont",
    b"SetFontObject",
    b"GetFontObject",
    b"SetTexture",
    b"GetTexture",
    b"SetAtlas",
    b"GetAtlas",
    b"SetTexCoord",
    b"SetVertexColor",
    b"SetColorTexture",
    b"SetDesaturated",
    b"SetDrawLayer",
    b"GetDrawLayer",
    b"CreateTexture",
    b"CreateFontString",
    b"CreateLine",
    b"CreateAnimationGroup",
];

// ── Lua metatable keys ────────────────────────────────────────────────────
//
// The 5.1 metamethod names plus Blizzard-convention `__*` registry keys
// the rilua runtime uses for its own bookkeeping.

pub const HOT_METATABLE_KEYS: &[&[u8]] = &[
    b"__index",
    b"__newindex",
    b"__tostring",
    b"__gc",
    b"__eq",
    b"__lt",
    b"__le",
    b"__add",
    b"__sub",
    b"__mul",
    b"__div",
    b"__mod",
    b"__pow",
    b"__unm",
    b"__concat",
    b"__len",
    b"__call",
    b"__metatable",
    // rilua / wow-ui-sim registry keys.
    b"__rilua_frame_mt",
    b"__rilua_frame_refs",
    b"__sim_print",
    b"__secureenv",
    b"__cvars",
    b"__original_string_format",
];

// ── Loader / compiler sentinels ───────────────────────────────────────────
//
// Chunk-name prefixes and registry tags the XML loader / template chain
// and secure-env bootstrap pass to `load_template` and friends. These are
// intern-cache hot because every template install synthesises one and
// compares against prior registrations.

pub const HOT_LOADER_SENTINELS: &[&[u8]] = &[
    b"getfenv",
    b"setfenv",
    b"MARK_SECURE_PROBE",
    b"from-secureenv",
    // Common `template-inline-*` chunk tags emitted by the template chain
    // builders. The full set is large and regenerated; the ten entries
    // below cover the highest-count handlers seen on `--no-addons` startup.
    b"template-inline-function-noargs",
    b"template-inline-function-self-id",
    b"template-inline-function-event-varargs",
    b"template-inline-function-button",
    b"template-inline-function-elapsed",
    b"template-inline-function-self-string",
    b"template-inline-function-string-arg",
    b"template-inline-function-global-arg",
    b"template-inline-function-two-global-args",
    b"template-global-method-handler",
];

/// Total count of whitelisted literals, sum of all category slices.
pub const HOT_LITERAL_COUNT: usize = HOT_GLOBALS.len()
    + HOT_NAMESPACES.len()
    + HOT_FRAME_METHODS.len()
    + HOT_METATABLE_KEYS.len()
    + HOT_LOADER_SENTINELS.len();

// ── Registry (Track 1 sub-item 2) ────────────────────────────────────────

use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::string::LuaString;

/// Interned handles for every entry in every category.
///
/// Built once during VM bootstrap via [`HotLiteralRegistry::install`] and
/// stashed on [`crate::lua_api::env::WowLuaAppData`]. Each `GcRef<LuaString>`
/// is a pointer into rilua's string arena; rilua's `intern_string_static`
/// cache holds a parallel entry that keeps the string alive as a GC root,
/// so the handles here remain valid for the life of the VM.
///
/// Indexed accessors mirror position in the underlying [`HOT_GLOBALS`],
/// [`HOT_NAMESPACES`], etc. slices. Callers that want compile-time-checked
/// symbolic access can build enums over the category in a follow-up patch
/// (Track 1 sub-item 3).
#[derive(Clone)]
pub struct HotLiteralHandles {
    globals: Box<[GcRef<LuaString>]>,
    namespaces: Box<[GcRef<LuaString>]>,
    frame_methods: Box<[GcRef<LuaString>]>,
    metatable_keys: Box<[GcRef<LuaString>]>,
    loader_sentinels: Box<[GcRef<LuaString>]>,
}

impl HotLiteralHandles {
    /// Total number of handles stored, across all categories. Always equal
    /// to [`HOT_LITERAL_COUNT`] after a successful [`install`].
    pub fn len(&self) -> usize {
        self.globals.len()
            + self.namespaces.len()
            + self.frame_methods.len()
            + self.metatable_keys.len()
            + self.loader_sentinels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Indexed handle for [`HOT_GLOBALS`]. Panics on out-of-range index.
    pub fn global(&self, index: usize) -> GcRef<LuaString> {
        self.globals[index]
    }

    /// Indexed handle for [`HOT_NAMESPACES`]. Panics on out-of-range index.
    pub fn namespace(&self, index: usize) -> GcRef<LuaString> {
        self.namespaces[index]
    }

    /// Indexed handle for [`HOT_FRAME_METHODS`]. Panics on out-of-range index.
    pub fn frame_method(&self, index: usize) -> GcRef<LuaString> {
        self.frame_methods[index]
    }

    /// Indexed handle for [`HOT_METATABLE_KEYS`]. Panics on out-of-range index.
    pub fn metatable_key(&self, index: usize) -> GcRef<LuaString> {
        self.metatable_keys[index]
    }

    /// Indexed handle for [`HOT_LOADER_SENTINELS`]. Panics on out-of-range index.
    pub fn loader_sentinel(&self, index: usize) -> GcRef<LuaString> {
        self.loader_sentinels[index]
    }
}

// ── Named accessors (Track 1 sub-item 3 foothold) ─────────────────────────
//
// Index constants for the entries that already have hot-path consumers in
// the current codebase. New conversions add entries here so the call sites
// stay symbolic (`handles.metatable_key(idx::RILUA_FRAME_MT)`) instead of
// relying on raw `intern_string_static(b"...")` calls.

/// Index constants into [`HOT_METATABLE_KEYS`]. Kept in lockstep with the
/// slice order — adding or reordering entries there requires updating
/// these and bumping [`WHITELIST_VERSION`].
pub mod metatable_idx {
    /// Position of `b"__rilua_frame_mt"` in [`super::HOT_METATABLE_KEYS`].
    pub const RILUA_FRAME_MT: usize = 18;
}

/// Owns the pre-intern step during VM bootstrap. Call [`install`] once
/// before any addon load so the subsequent hot paths (sub-item 3) find
/// every whitelisted literal already in rilua's static intern cache.
pub struct HotLiteralRegistry;

impl HotLiteralRegistry {
    /// Pre-intern every entry in the whitelist via
    /// `state.gc.intern_string_static(&'static [u8])`. Returns a
    /// [`HotLiteralHandles`] populated with the resulting handles in the
    /// same order as each category slice.
    pub fn install(state: &mut LuaState) -> HotLiteralHandles {
        HotLiteralHandles {
            globals: Self::intern_all(state, HOT_GLOBALS),
            namespaces: Self::intern_all(state, HOT_NAMESPACES),
            frame_methods: Self::intern_all(state, HOT_FRAME_METHODS),
            metatable_keys: Self::intern_all(state, HOT_METATABLE_KEYS),
            loader_sentinels: Self::intern_all(state, HOT_LOADER_SENTINELS),
        }
    }

    fn intern_all(state: &mut LuaState, slice: &[&'static [u8]]) -> Box<[GcRef<LuaString>]> {
        slice
            .iter()
            .map(|entry| state.gc.intern_string_static(entry))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Every category entry must be non-empty and unique within its
    /// category. Catches copy-paste duplicates when the list grows.
    #[test]
    fn every_category_has_unique_nonempty_entries() {
        for (name, slice) in [
            ("HOT_GLOBALS", HOT_GLOBALS),
            ("HOT_NAMESPACES", HOT_NAMESPACES),
            ("HOT_FRAME_METHODS", HOT_FRAME_METHODS),
            ("HOT_METATABLE_KEYS", HOT_METATABLE_KEYS),
            ("HOT_LOADER_SENTINELS", HOT_LOADER_SENTINELS),
        ] {
            let mut seen: HashSet<&[u8]> = HashSet::new();
            for entry in slice {
                assert!(!entry.is_empty(), "{name} contains an empty byte slice",);
                assert!(
                    seen.insert(entry),
                    "{name} contains duplicate entry {:?}",
                    std::str::from_utf8(entry).unwrap_or("<non-utf8>"),
                );
            }
        }
    }

    /// Confirms [`HOT_LITERAL_COUNT`] stays in sync with the sum of the
    /// per-category slices. Compile-time-ish; catches drift if a new
    /// category is added without updating the total.
    #[test]
    fn count_equals_sum_of_categories() {
        let sum = HOT_GLOBALS.len()
            + HOT_NAMESPACES.len()
            + HOT_FRAME_METHODS.len()
            + HOT_METATABLE_KEYS.len()
            + HOT_LOADER_SENTINELS.len();
        assert_eq!(HOT_LITERAL_COUNT, sum);
    }

    #[test]
    fn version_is_nonzero() {
        assert!(WHITELIST_VERSION >= 1);
    }

    /// End-to-end bootstrap: install the registry on a fresh VM and
    /// confirm each category's handles decode back to the source bytes.
    /// Pins the invariant that the static intern cache survives the
    /// prewarm step.
    #[test]
    fn registry_install_produces_handles_that_decode_to_source_bytes() {
        use rilua::{Lua, LuaApiMut};

        let mut lua = Lua::new().expect("fresh rilua VM");
        let handles = HotLiteralRegistry::install(lua.state_mut());

        assert_eq!(handles.len(), HOT_LITERAL_COUNT);

        // Spot-check one entry from each category — full roundtrip for
        // every entry in `every_handle_decodes_to_its_source_bytes`.
        let checks: &[(&'static [u8], GcRef<LuaString>)] = &[
            (HOT_GLOBALS[0], handles.global(0)),
            (HOT_NAMESPACES[0], handles.namespace(0)),
            (HOT_FRAME_METHODS[0], handles.frame_method(0)),
            (HOT_METATABLE_KEYS[0], handles.metatable_key(0)),
            (HOT_LOADER_SENTINELS[0], handles.loader_sentinel(0)),
        ];
        for (expected, handle) in checks {
            let s = lua
                .state_mut()
                .gc
                .string_arena
                .get(*handle)
                .expect("interned string alive");
            assert_eq!(s.data(), *expected);
        }
    }

    /// Full roundtrip: every position in every category slice must decode
    /// back to its source bytes via the arena lookup.
    #[test]
    fn every_handle_decodes_to_its_source_bytes() {
        use rilua::{Lua, LuaApiMut};

        let mut lua = Lua::new().expect("fresh rilua VM");
        let handles = HotLiteralRegistry::install(lua.state_mut());

        let state = lua.state_mut();
        let categories: &[(&'static str, &[&[u8]], &[GcRef<LuaString>])] = &[
            ("globals", HOT_GLOBALS, &handles.globals),
            ("namespaces", HOT_NAMESPACES, &handles.namespaces),
            ("frame_methods", HOT_FRAME_METHODS, &handles.frame_methods),
            ("metatable_keys", HOT_METATABLE_KEYS, &handles.metatable_keys),
            (
                "loader_sentinels",
                HOT_LOADER_SENTINELS,
                &handles.loader_sentinels,
            ),
        ];
        for (name, src, refs) in categories {
            assert_eq!(src.len(), refs.len(), "{name} length mismatch");
            for (i, (bytes, r)) in src.iter().zip(refs.iter()).enumerate() {
                let s = state
                    .gc
                    .string_arena
                    .get(*r)
                    .unwrap_or_else(|| panic!("{name}[{i}] interned string missing"));
                assert_eq!(s.data(), *bytes, "{name}[{i}] byte mismatch");
            }
        }
    }

    /// Pins each named index constant to its expected source byte slice.
    /// Catches drift when HOT_METATABLE_KEYS is reordered without bumping
    /// WHITELIST_VERSION and updating the `metatable_idx` constants.
    #[test]
    fn named_indexes_map_to_expected_slice_entries() {
        assert_eq!(
            HOT_METATABLE_KEYS[metatable_idx::RILUA_FRAME_MT],
            b"__rilua_frame_mt"
        );
    }

    /// Second call to `install` on the same VM must return equivalent
    /// handles (same arena pointers), exercising rilua's static intern
    /// cache hit path.
    #[test]
    fn second_install_returns_same_handles_via_cache_hit() {
        use rilua::{Lua, LuaApiMut};

        let mut lua = Lua::new().expect("fresh rilua VM");
        let first = HotLiteralRegistry::install(lua.state_mut());
        let second = HotLiteralRegistry::install(lua.state_mut());

        for i in 0..HOT_GLOBALS.len() {
            assert_eq!(
                first.global(i),
                second.global(i),
                "global[{i}] handle differs between installs",
            );
        }
        for i in 0..HOT_NAMESPACES.len() {
            assert_eq!(
                first.namespace(i),
                second.namespace(i),
                "namespace[{i}] handle differs between installs",
            );
        }
    }
}
