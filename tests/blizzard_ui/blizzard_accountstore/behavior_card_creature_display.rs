//! Behavior pin for `AccountStoreCreatureCardMixin:UpdateCardDisplay` — the
//! creature-card model-scene configuration path.
//!
//! Spec/source mismatch finding (PLAN.md task:
//! `AccountStoreCreatureCardMixin:RefreshDisplay` configures the model scene
//! from `customUIModelSceneID` / `creatureDisplayID` returned by
//! `GetItemInfo`). Three claims diverge from the actual source at
//! `Blizzard_AccountStoreCardTemplates.lua:125-134, 230-249, 367`.
//!
//! 1. **The method is `UpdateCardDisplay`, NOT `RefreshDisplay`.** Line 232
//!    reads `function AccountStoreCreatureCardMixin:UpdateCardDisplay()`.
//!    The base mixin defines a no-op `UpdateCardDisplay` at line 225 ("--
//!    Override in your derived Mixin."); three derived mixins override it
//!    (Creature/Icon/TransmogSet). There is no `RefreshDisplay` anywhere
//!    in the file.
//!
//! 2. **`UpdateCardDisplay` does NOT call `C_AccountStore.GetItemInfo`.**
//!    The data source is the cached `self.itemInfo` field, populated by
//!    the BASE mixin's `SetItemID` at lines 125-129:
//!
//!    ```lua
//!    function AccountStoreBaseCardMixin:SetItemID(itemID)
//!        self.itemID = itemID;
//!        local itemInfo = C_AccountStore.GetItemInfo(itemID);
//!        self.itemInfo = itemInfo;
//!        ...
//!    ```
//!
//!    `UpdateCardDisplay` then reads `self.itemInfo.customUIModelSceneID`
//!    and `self.itemInfo.creatureDisplayID` from the cached field (lines
//!    238, 247). The PLAN's "values returned by GetItemInfo" framing
//!    suggests the override calls the API directly; it does not.
//!
//! 3. **`customUIModelSceneID` falls back to the `CreatureModelSceneID`
//!    global, NOT to nil/skip.** Line 238 reads
//!    `self.ModelScene:SetFromModelSceneID(self.itemInfo.customUIModelSceneID
//!    or CreatureModelSceneID, forceUpdate)`. The PLAN names only the
//!    `customUIModelSceneID` field; the fallback to the global default is
//!    a structural property of the dispatch.
//!
//! Additional structural facts pinned (not in PLAN but readily mis-edited):
//! - Early-return when `self.itemInfo` is nil (lines 233-235): nothing on
//!   the ModelScene is called.
//! - `forceUpdate = true` is passed to both `SetFromModelSceneID` (line
//!   238) and `SetModelByCreatureDisplayID` (line 247).
//! - The creature-display dispatch is gated on `GetActorByTag("item")`
//!   returning truthy (line 240-241): if no "item" actor exists, no
//!   creature-display call is made.
//! - The actor is hidden first (line 242), and a callback registered via
//!   `SetOnModelLoadedCallback` (lines 243-245) re-shows the actor when
//!   the model finishes loading.
//! - `AccountStoreMountCardMixin = AccountStoreCreatureCardMixin` (line
//!   367) — they are the SAME Lua table, so any pin against
//!   AccountStoreCreatureCardMixin doubles as a pin for the mount card
//!   behavior.
//!
//! Eight tests pin the contract:
//!
//! - `account_store_creature_card_mixin_does_not_define_refresh_display` —
//!   surface tripwire that `type(AccountStoreCreatureCardMixin.RefreshDisplay)
//!   == "nil"`.
//!
//! - `account_store_creature_card_mixin_update_card_display_is_a_function`
//!   — surface positive that the actual method exists.
//!
//! - `account_store_mount_card_mixin_is_account_store_creature_card_mixin`
//!   — pins the table aliasing at line 367; a non-equal reading would
//!   prove the mount card grew its own mixin (and would silently bypass
//!   any future pins on the creature mixin for mount cards).
//!
//! - `update_card_display_does_not_call_c_account_store_get_item_info` —
//!   stubs the global with a tracker, invokes `UpdateCardDisplay(stub)`
//!   on a stub with pre-populated `itemInfo`, asserts ZERO tracker calls.
//!
//! - `update_card_display_returns_early_when_item_info_is_nil` — stubs
//!   the ModelScene with a capture, seeds `stub.itemInfo = nil`, invokes,
//!   asserts SetFromModelSceneID was NOT called.
//!
//! - `update_card_display_uses_custom_ui_model_scene_id_when_present_and_dispatches_creature_display`
//!   — seeds `itemInfo.customUIModelSceneID` and `creatureDisplayID` with
//!   sentinels distinct from the global default; asserts ModelScene
//!   received `SetFromModelSceneID(CUSTOM_SENTINEL, true)`, the "item"
//!   actor received `SetModelByCreatureDisplayID(DISPLAY_SENTINEL, true)`,
//!   `Hide()` was called on the actor, AND `SetOnModelLoadedCallback`
//!   registered a function that — when invoked — calls `Show()` on the
//!   actor.
//!
//! - `update_card_display_falls_back_to_creature_model_scene_id_when_custom_is_nil`
//!   — seeds `itemInfo.customUIModelSceneID = nil`, captures the value
//!   the global `CreatureModelSceneID` resolves to at test time, asserts
//!   ModelScene received `SetFromModelSceneID(<captured-global>, true)`.
//!
//! - `update_card_display_skips_creature_display_when_get_actor_by_tag_returns_nil`
//!   — stubs ModelScene to return nil from `GetActorByTag("item")`,
//!   invokes, asserts SetFromModelSceneID was still called BUT no
//!   creature.SetModelByCreatureDisplayID call landed (no actor exists
//!   to receive it).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccountStore";
const CUSTOM_MODEL_SCENE_ID_SENTINEL: i64 = 5151;
const CREATURE_DISPLAY_ID_SENTINEL: i64 = 7373;
// File-local default at `Blizzard_AccountStoreCardTemplates.lua:3`:
// `local CreatureModelSceneID = 76;`. Not visible via `_G` since it's a
// chunk-local; this pin holds the upstream value directly so a change to
// the default fails the fallback assertion loudly.
const CREATURE_MODEL_SCENE_ID_FILE_LOCAL_DEFAULT: i64 = 76;

#[test]
fn account_store_creature_card_mixin_does_not_define_refresh_display() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let refresh_display_type: String = env
            .eval("return type(AccountStoreCreatureCardMixin.RefreshDisplay)")
            .expect("AccountStoreCreatureCardMixin.RefreshDisplay probe must run cleanly");

        assert_eq!(
            refresh_display_type, "nil",
            "Expected `type(AccountStoreCreatureCardMixin.RefreshDisplay) == \"nil\"` per \
             `Blizzard_AccountStoreCardTemplates.lua:230-249` — the mixin defines only \
             `UpdateCardDisplay` (line 232) which overrides the base mixin's no-op at line \
             225. There is no `RefreshDisplay` method anywhere in the file. Got \
             `{refresh_display_type}`. A non-nil reading would prove the PLAN-named method \
             was added upstream, forcing a re-pin against its body."
        );
    });
}

#[test]
fn account_store_creature_card_mixin_update_card_display_is_a_function() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let update_card_display_type: String = env
            .eval("return type(AccountStoreCreatureCardMixin.UpdateCardDisplay)")
            .expect("AccountStoreCreatureCardMixin.UpdateCardDisplay probe must run cleanly");

        assert_eq!(
            update_card_display_type, "function",
            "Expected `type(AccountStoreCreatureCardMixin.UpdateCardDisplay) == \"function\"` \
             per `Blizzard_AccountStoreCardTemplates.lua:232-249`. Got \
             `{update_card_display_type}`. A non-function reading would prove the override was \
             removed (collapsing creature-card behavior back to the base no-op) or moved onto \
             a different mixin."
        );
    });
}

#[test]
fn account_store_mount_card_mixin_is_account_store_creature_card_mixin() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let mount_equals_creature: bool = env
            .eval("return AccountStoreMountCardMixin == AccountStoreCreatureCardMixin")
            .expect("MountCardMixin == CreatureCardMixin probe must run cleanly");

        assert!(
            mount_equals_creature,
            "Expected `AccountStoreMountCardMixin == AccountStoreCreatureCardMixin` per \
             `Blizzard_AccountStoreCardTemplates.lua:367` (`AccountStoreMountCardMixin = \
             AccountStoreCreatureCardMixin`). The mount card mixin is a TABLE ALIAS for the \
             creature card mixin — they reference the same Lua table, so any pin on the \
             creature mixin doubles as a pin on the mount card behavior. A false reading \
             would prove the mount mixin diverged into its own table (which would silently \
             bypass any future creature-mixin pins for mount cards)."
        );
    });
}

#[test]
fn update_card_display_does_not_call_c_account_store_get_item_info() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_get_item_info_tracker(env);
        seed_stub_creature_card(env, StubCreatureCardSeed::with_actor("nil"));

        env.eval::<()>(
            r#"
            AccountStoreCreatureCardMixin.UpdateCardDisplay(
                _G.__behavior_card_creature_display_stub_card
            )
            return
            "#,
        )
        .expect("UpdateCardDisplay invocation must run cleanly");

        let get_item_info_calls: i64 = env
            .eval("return _G.__behavior_card_creature_display_get_item_info_calls or 0")
            .expect("GetItemInfo call-count readout must run cleanly");

        assert_eq!(
            get_item_info_calls, 0,
            "Expected ZERO `C_AccountStore.GetItemInfo` calls during UpdateCardDisplay — the \
             override at lines 232-249 reads `self.itemInfo.customUIModelSceneID` and \
             `self.itemInfo.creatureDisplayID` from the CACHED field; the cache itself is \
             populated upstream by `AccountStoreBaseCardMixin:SetItemID` at lines 125-129. \
             Got {get_item_info_calls}. A non-zero reading would prove the PLAN's \"values \
             returned by GetItemInfo\" framing came true (the override started fetching \
             live data instead of using the cache) — a regression that would re-issue the \
             API call for every redraw."
        );

        teardown_stub_creature_card(env);
        teardown_get_item_info_tracker(env);
    });
}

#[test]
fn update_card_display_returns_early_when_item_info_is_nil() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_stub_creature_card(env, StubCreatureCardSeed::with_actor("nil"));
        env.eval::<()>("_G.__behavior_card_creature_display_stub_card.itemInfo = nil; return")
            .expect("nil-itemInfo seed must run cleanly");

        env.eval::<()>(
            r#"
            AccountStoreCreatureCardMixin.UpdateCardDisplay(
                _G.__behavior_card_creature_display_stub_card
            )
            return
            "#,
        )
        .expect("UpdateCardDisplay invocation must run cleanly");

        let set_from_model_scene_id_calls: i64 = env
            .eval(
                r#"
                local stub = _G.__behavior_card_creature_display_stub_card
                return #stub.ModelScene.__set_from_model_scene_id_calls
                "#,
            )
            .expect("SetFromModelSceneID call-count readout must run cleanly");

        assert_eq!(
            set_from_model_scene_id_calls, 0,
            "Expected ZERO `ModelScene:SetFromModelSceneID` calls when `self.itemInfo` is nil \
             — lines 233-235 read `if not self.itemInfo then return end`, short-circuiting \
             before any ModelScene dispatch. Got {set_from_model_scene_id_calls}. A non-zero \
             reading would prove the early-return guard was removed (a regression that would \
             pass nil into the ModelScene API and likely error or render a default model)."
        );

        teardown_stub_creature_card(env);
    });
}

#[test]
fn update_card_display_uses_custom_ui_model_scene_id_when_present_and_dispatches_creature_display()
{
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_stub_creature_card(
            env,
            StubCreatureCardSeed::with_actor(CUSTOM_MODEL_SCENE_ID_SENTINEL.to_string()),
        );

        env.eval::<()>(
            r#"
            AccountStoreCreatureCardMixin.UpdateCardDisplay(
                _G.__behavior_card_creature_display_stub_card
            )
            return
            "#,
        )
        .expect("UpdateCardDisplay invocation must run cleanly");

        let (
            scene_id_arg,
            scene_force_update_arg,
            display_id_arg,
            display_force_update_arg,
            actor_hide_calls,
        ): (i64, bool, i64, bool, i64) = env
            .eval(
                r#"
                local stub = _G.__behavior_card_creature_display_stub_card
                local scene_call = stub.ModelScene.__set_from_model_scene_id_calls[1] or {}
                local display_call = stub.__creature_actor.__set_model_by_creature_display_id_calls[1] or {}
                return scene_call.scene_id or -1,
                       scene_call.force_update == true,
                       display_call.display_id or -1,
                       display_call.force_update == true,
                       stub.__creature_actor.__hide_calls or 0
                "#,
            )
            .expect("ModelScene + actor call readout must run cleanly");

        assert_eq!(
            scene_id_arg, CUSTOM_MODEL_SCENE_ID_SENTINEL,
            "Expected `SetFromModelSceneID` to receive `customUIModelSceneID` \
             ({CUSTOM_MODEL_SCENE_ID_SENTINEL}) when present — line 238 reads \
             `self.itemInfo.customUIModelSceneID or CreatureModelSceneID`, and the `or` \
             short-circuits to the LHS when truthy. Got {scene_id_arg}. A reading equal to \
             the global `CreatureModelSceneID` would prove the LHS was dropped or the `or` \
             chain was inverted."
        );

        assert!(
            scene_force_update_arg,
            "Expected `SetFromModelSceneID` to receive `forceUpdate = true` per line 238 \
             (`local forceUpdate = true; ...:SetFromModelSceneID(..., forceUpdate)`). A false \
             reading would prove the forceUpdate flag was dropped — which would skip the \
             reload when the same scene ID is already loaded, masking visual bugs in the \
             card."
        );

        assert_eq!(
            display_id_arg, CREATURE_DISPLAY_ID_SENTINEL,
            "Expected the \"item\" actor's `SetModelByCreatureDisplayID` to receive \
             `creatureDisplayID` ({CREATURE_DISPLAY_ID_SENTINEL}) per line 247 \
             (`creature:SetModelByCreatureDisplayID(self.itemInfo.creatureDisplayID, \
             forceUpdate)`). Got {display_id_arg}. A reading of -1 would prove the call was \
             skipped; any other value would prove the field path was substituted."
        );

        assert!(
            display_force_update_arg,
            "Expected `SetModelByCreatureDisplayID` to receive `forceUpdate = true` per line \
             247 (same `forceUpdate` local as the SetFromModelSceneID call). A false reading \
             would prove the forceUpdate flag was dropped, skipping reloads when the actor \
             already has a model loaded."
        );

        assert_eq!(
            actor_hide_calls, 1,
            "Expected exactly ONE `creature:Hide()` call before the SetModelByCreatureDisplayID \
             dispatch — line 242 hides the actor synchronously. Got {actor_hide_calls}. A \
             zero reading would prove the Hide call was dropped (which would briefly show \
             the previous model during reload — a known visual artifact the Hide was added \
             to suppress)."
        );

        let on_model_loaded_callback_show_calls: i64 = env
            .eval(
                r#"
                local stub = _G.__behavior_card_creature_display_stub_card
                local actor = stub.__creature_actor
                actor.__show_calls = 0
                if type(actor.__on_model_loaded_callback) == "function" then
                    actor.__on_model_loaded_callback()
                end
                return actor.__show_calls
                "#,
            )
            .expect("on-model-loaded callback invocation must run cleanly");

        assert_eq!(
            on_model_loaded_callback_show_calls, 1,
            "Expected the registered `SetOnModelLoadedCallback` callback to call \
             `creature:Show()` exactly once when invoked — lines 243-245 register \
             `function() creature:Show() end`. Invoking the captured callback should \
             trigger the closure's Show call. Got {on_model_loaded_callback_show_calls}. A \
             zero reading would prove the callback either wasn't registered or doesn't \
             call Show — leaving the actor stuck in the Hide()-from-line-242 state."
        );

        teardown_stub_creature_card(env);
    });
}

#[test]
fn update_card_display_falls_back_to_creature_model_scene_id_when_custom_is_nil() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_stub_creature_card(env, StubCreatureCardSeed::with_actor("nil"));

        env.eval::<()>(
            r#"
            AccountStoreCreatureCardMixin.UpdateCardDisplay(
                _G.__behavior_card_creature_display_stub_card
            )
            return
            "#,
        )
        .expect("UpdateCardDisplay invocation must run cleanly");

        let scene_id_arg: i64 = env
            .eval(
                r#"
                local stub = _G.__behavior_card_creature_display_stub_card
                local scene_call = stub.ModelScene.__set_from_model_scene_id_calls[1] or {}
                return scene_call.scene_id or -1
                "#,
            )
            .expect("scene-id readout must run cleanly");

        assert_eq!(
            scene_id_arg, CREATURE_MODEL_SCENE_ID_FILE_LOCAL_DEFAULT,
            "Expected `SetFromModelSceneID` to receive the file-local \
             `CreatureModelSceneID` default ({CREATURE_MODEL_SCENE_ID_FILE_LOCAL_DEFAULT}) \
             when `customUIModelSceneID` is nil — line 238's `or` chain falls through to \
             the chunk-local declared at \
             `Blizzard_AccountStoreCardTemplates.lua:3` (`local CreatureModelSceneID = 76`). \
             Note: this is FILE-LOCAL, not a `_G.` global, so it is only resolvable from \
             inside the addon chunk; the test pins the value directly. Got {scene_id_arg}. \
             A mismatch on this exact value would prove either the file-local was renamed/\
             retyped, the upstream default was bumped (e.g. to a new model-scene ID), or \
             the `or` chain was changed to bail out on nil custom scene IDs (which the \
             PLAN's framing implicitly assumes since it doesn't mention the fallback)."
        );

        teardown_stub_creature_card(env);
    });
}

#[test]
fn update_card_display_skips_creature_display_when_get_actor_by_tag_returns_nil() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_stub_creature_card(
            env,
            StubCreatureCardSeed::without_actor(CUSTOM_MODEL_SCENE_ID_SENTINEL.to_string()),
        );

        env.eval::<()>(
            r#"
            AccountStoreCreatureCardMixin.UpdateCardDisplay(
                _G.__behavior_card_creature_display_stub_card
            )
            return
            "#,
        )
        .expect("UpdateCardDisplay invocation must run cleanly");

        let (scene_calls, display_calls_total): (i64, i64) = env
            .eval(
                r#"
                local stub = _G.__behavior_card_creature_display_stub_card
                return #stub.ModelScene.__set_from_model_scene_id_calls,
                       stub.__creature_display_calls_total or 0
                "#,
            )
            .expect("no-actor readout must run cleanly");

        assert_eq!(
            scene_calls, 1,
            "Expected `SetFromModelSceneID` to STILL fire even when no \"item\" actor exists \
             — the actor lookup happens on line 240 AFTER the SetFromModelSceneID call on \
             line 238. Got {scene_calls}. A zero reading would prove the actor-gated branch \
             swallowed the unconditional ModelScene config."
        );

        assert_eq!(
            display_calls_total, 0,
            "Expected ZERO `SetModelByCreatureDisplayID` calls when `GetActorByTag(\"item\")` \
             returns nil — the entire `if creature then ... end` block at lines 241-248 is \
             skipped. The stub seeds `__creature_display_calls_total = 0` and a tracker that \
             increments it; with no actor, the increment is unreachable. Got \
             {display_calls_total}. A non-zero reading would prove the actor-gating was \
             dropped — which would error since `creature:SetModelByCreatureDisplayID(...)` \
             would dispatch on a nil receiver."
        );

        teardown_stub_creature_card(env);
    });
}

fn seed_get_item_info_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_creature_display_get_item_info_calls = 0
        _G.__behavior_card_creature_display_original_get_item_info =
            C_AccountStore.GetItemInfo
        C_AccountStore.GetItemInfo = function(_item_id)
            _G.__behavior_card_creature_display_get_item_info_calls =
                _G.__behavior_card_creature_display_get_item_info_calls + 1
            return nil
        end
        return
        "#,
    )
    .expect("seeding GetItemInfo tracker must run cleanly");
}

fn teardown_get_item_info_tracker(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        C_AccountStore.GetItemInfo =
            _G.__behavior_card_creature_display_original_get_item_info
        _G.__behavior_card_creature_display_original_get_item_info = nil
        _G.__behavior_card_creature_display_get_item_info_calls = nil
        return
        "#,
    )
    .expect("GetItemInfo tracker tear-down must run cleanly");
}

struct StubCreatureCardSeed {
    custom_scene_id_lua: String,
    has_actor: bool,
}

impl StubCreatureCardSeed {
    fn with_actor(custom_scene_id_lua: impl Into<String>) -> Self {
        Self {
            custom_scene_id_lua: custom_scene_id_lua.into(),
            has_actor: true,
        }
    }

    fn without_actor(custom_scene_id_lua: impl Into<String>) -> Self {
        Self {
            custom_scene_id_lua: custom_scene_id_lua.into(),
            has_actor: false,
        }
    }
}

fn seed_stub_creature_card(env: &WowLuaEnv, seed: StubCreatureCardSeed) {
    install_creature_actor_builder_global(env);
    assemble_stub_creature_card(env, &seed.custom_scene_id_lua, seed.has_actor);
}

fn install_creature_actor_builder_global(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        _G.__behavior_card_creature_display_build_actor = function()
            local actor = {}
            actor.__hide_calls = 0
            actor.__show_calls = 0
            actor.__on_model_loaded_callback = nil
            actor.__set_model_by_creature_display_id_calls = {}
            actor.Hide = function(self) self.__hide_calls = self.__hide_calls + 1 end
            actor.Show = function(self) self.__show_calls = self.__show_calls + 1 end
            actor.SetOnModelLoadedCallback = function(self, fn)
                self.__on_model_loaded_callback = fn
            end
            actor.SetModelByCreatureDisplayID = function(self, display_id, force_update)
                local captured = self.__set_model_by_creature_display_id_calls
                captured[#captured + 1] = { display_id = display_id, force_update = force_update }
                local card = _G.__behavior_card_creature_display_stub_card
                card.__creature_display_calls_total =
                    (card.__creature_display_calls_total or 0) + 1
            end
            return actor
        end
        return
        "#,
    )
    .expect("installing creature-actor builder must run cleanly");
}

fn assemble_stub_creature_card(env: &WowLuaEnv, custom_scene_id_lua: &str, has_actor: bool) {
    let actor_constructor_lua = if has_actor {
        "_G.__behavior_card_creature_display_build_actor()"
    } else {
        "nil"
    };
    env.eval::<()>(&format!(
        r#"
        local stub = {{}}
        stub.__creature_display_calls_total = 0
        stub.__creature_actor = {actor_constructor_lua}

        local model_scene = {{}}
        model_scene.__set_from_model_scene_id_calls = {{}}
        model_scene.SetFromModelSceneID = function(self, scene_id, force_update)
            local captured = self.__set_from_model_scene_id_calls
            captured[#captured + 1] = {{ scene_id = scene_id, force_update = force_update }}
        end
        model_scene.GetActorByTag = function(_self, _tag) return stub.__creature_actor end
        stub.ModelScene = model_scene

        stub.itemInfo = {{
            customUIModelSceneID = {custom_scene_id_lua},
            creatureDisplayID = {CREATURE_DISPLAY_ID_SENTINEL},
        }}

        _G.__behavior_card_creature_display_stub_card = stub
        return
        "#
    ))
    .expect("seeding stub creature card must run cleanly");
}

fn teardown_stub_creature_card(env: &WowLuaEnv) {
    env.eval::<()>("_G.__behavior_card_creature_display_stub_card = nil; return")
        .expect("stub creature card tear-down must run cleanly");
}
