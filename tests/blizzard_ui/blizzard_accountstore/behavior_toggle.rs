//! Behavior pin for the toggle path on `Blizzard_AccountStore`.
//!
//! Spec/source mismatch finding (PLAN.md task for
//! `ToggleAccountStoreUI(storeFrontID)`): the plan describes a single
//! function that does TWO things in one call — flips
//! `AccountStoreFrame:IsShown()` AND assigns
//! `AccountStoreFrame.storeFrontID` to the passed id. The actual
//! source splits these responsibilities across TWO unrelated entry
//! points on TWO different objects:
//!
//! 1. **Visibility toggle** — `AccountStoreUtil.ToggleAccountStore()`
//!    at `Blizzard_AccountStoreUtil.lua:52`. Zero-arg. Body is one
//!    line: `AccountStoreUtil.SetAccountStoreShown(not
//!    AccountStoreFrame:IsShown())`. It does NOT touch `storeFrontID`,
//!    does NOT trigger any EventRegistry callback related to the store
//!    front, and silently ignores any positional arguments (Lua-style
//!    vararg-discard semantics — the function declaration has zero
//!    parameters so additional call arguments are dropped).
//!
//! 2. **storeFrontID setter** — `AccountStoreMixin:SetStoreFrontID(id)`
//!    at `Blizzard_AccountStore.lua:42`. Method on the AccountStoreFrame
//!    instance. Three-line body: assigns `self.storeFrontID = id`,
//!    calls `self:SetTitle(STORE_FRONT_TO_TITLE[id] or "")`, and
//!    triggers `EventRegistry:TriggerEvent("AccountStore.StoreFrontSet",
//!    id)`. It does NOT touch visibility (no `:Show()` / `:Hide()`).
//!
//! The PLAN-shaped single-call API does not exist anywhere in the
//! source. Callers that want both effects must invoke both paths
//! independently — typically the navbar redirect path
//! (`Blizzard_CharacterSelectNavBar.lua:44`'s `local function
//! ToggleAccountStoreUI()`) calls `AccountStoreUtil.ToggleAccountStore()`
//! while a separate flow (e.g., `AccountStoreItemDisplayMixin`'s
//! `OnStoreFrontSet` callback consumer at
//! `Blizzard_AccountStoreItemDisplay.lua:58`) reacts to the event
//! `SetStoreFrontID` triggers.
//!
//! Note: by the time the smoke harness finishes loading,
//! `AccountStoreFrame.storeFrontID` is ALREADY non-nil because
//! `UIParent_OnShow` (`Blizzard_UIParent/Mainline/UIParent.lua:351-352`)
//! calls `C_AddOns.LoadAddOn("Blizzard_AccountStore")` followed by
//! `AccountStoreFrame:SetStoreFrontID(Constants.AccountStoreConsts.PlunderstormStoreFrontID)`
//! during UIParent's load chain. The tests here therefore capture the
//! load-time id and assert change-or-no-change relative to that
//! starting value, instead of asserting nil-at-start.
//!
//! This file pins the behavioral consequences of the path split:
//!
//! - `toggle_with_extra_arg_flips_visibility_without_changing_store_front_id`
//!   asserts that calling `AccountStoreUtil.ToggleAccountStore(123)`
//!   (with a PLAN-shaped extra argument) flips `IsShown()` exactly the
//!   way the zero-arg call would AND leaves `storeFrontID` equal to
//!   the load-time value. Vararg-discard means the extra argument is
//!   dropped at the call boundary, so the body is identical to the
//!   zero-arg case.
//! - `set_store_front_id_overrides_load_time_default_and_persists_new_id`
//!   asserts that `AccountStoreFrame:SetStoreFrontID(<sentinel>)` (the
//!   actual storeFrontID-setting path) overwrites the load-time
//!   PlunderstormStoreFrontID with the sentinel — pinning the
//!   field-assignment line at `Blizzard_AccountStore.lua:43`.
//!
//! The shallower visibility-flip and PLAN-named-global-absent
//! assertions live in the surface lane
//! (`surface_globals.rs::account_store_toggle_is_callable_and_flips_frame_shown_state`).
//! This file deliberately does NOT re-pin those — its remit is the
//! deeper behavior split between the visibility and id-set paths.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn toggle_with_extra_arg_flips_visibility_without_changing_store_front_id() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let initial_store_front_id: i64 = env
            .eval("return AccountStoreFrame.storeFrontID")
            .expect("initial `AccountStoreFrame.storeFrontID` probe must run cleanly");

        let initially_hidden: bool = env
            .eval("return AccountStoreFrame:IsShown() == false")
            .expect("initial `AccountStoreFrame:IsShown()` probe must run cleanly");

        assert!(
            initially_hidden,
            "Expected `AccountStoreFrame:IsShown()` to be false before the toggle is invoked. \
             The XML at `Blizzard_AccountStore.xml:78` declares the frame with `hidden=\"true\"` \
             so it MUST start hidden after `{ROOT}` finishes loading. A regression that auto- \
             shows the frame on load would invert this test's pre-condition (the toggle would \
             flip it to hidden first and the storeFrontID-stays-pinned assertion would still \
             pass, but the hide-after-second-toggle round-trip below would land in the wrong \
             state)."
        );

        env.eval::<()>("AccountStoreUtil.ToggleAccountStore(123); return")
            .expect("first `ToggleAccountStore(123)` (with extra arg) call must run cleanly");

        let shown_after_toggle_with_arg: bool = env
            .eval("return AccountStoreFrame:IsShown() == true")
            .expect("post-toggle-with-arg `IsShown()` probe must run cleanly");

        assert!(
            shown_after_toggle_with_arg,
            "Expected `AccountStoreFrame:IsShown()` to flip to true after \
             `AccountStoreUtil.ToggleAccountStore(123)` — the extra argument MUST be silently \
             ignored. `AccountStoreUtil.ToggleAccountStore` at \
             `Blizzard_AccountStoreUtil.lua:52` is declared with zero parameters, so any \
             positional arguments at the call boundary are dropped (Lua vararg-discard \
             semantics). The body just calls `SetAccountStoreShown(not \
             AccountStoreFrame:IsShown())` which routes to the show path \
             (`ShowUIPanel(AccountStoreFrame)` falling back to `:Show()`). A false reading \
             here means either (a) the extra-arg call raised an error and the toggle never \
             reached `SetAccountStoreShown` (a regression that would force callers to drop \
             defensive extra-arg calls), or (b) `SetAccountStoreShown` started accepting an \
             id parameter and routed it differently (a Blizzard change toward the PLAN-shaped \
             API)."
        );

        let store_front_id_after_toggle_with_arg: i64 = env
            .eval("return AccountStoreFrame.storeFrontID")
            .expect("post-toggle-with-arg `storeFrontID` probe must run cleanly");

        assert_eq!(
            store_front_id_after_toggle_with_arg, initial_store_front_id,
            "Expected `AccountStoreFrame.storeFrontID` to be unchanged after \
             `AccountStoreUtil.ToggleAccountStore(123)` — the toggle path MUST NOT touch the \
             id field even when given a PLAN-shaped argument. This is the spec/source mismatch \
             tripwire: PLAN's `ToggleAccountStoreUI(storeFrontID)` describes a coupled \
             toggle+set-id call, but the actual toggle body \
             (`Blizzard_AccountStoreUtil.lua:53` — `SetAccountStoreShown(not \
             AccountStoreFrame:IsShown())`) only routes to `ShowUIPanel`/`HideUIPanel` and \
             never assigns `storeFrontID`. The load-time id was captured as the initial value \
             (set to `Constants.AccountStoreConsts.PlunderstormStoreFrontID` by \
             `UIParent_OnShow` at `Blizzard_UIParent/Mainline/UIParent.lua:351-352`); a \
             mismatch reading here means either (a) Blizzard merged the two paths into a \
             single coupled call (forcing a re-pin against the new shape — and likely \
             retiring `AccountStoreMixin:SetStoreFrontID` as the public id-setter), or \
             (b) some intermediate path (`SetAccountStoreShown`, the panel manager, the \
             frame's OnShow handler) started rewriting `storeFrontID` (a regression worth \
             investigating because it would mask the separate-path contract that downstream \
             `OnStoreFrontSet` callbacks depend on)."
        );

        env.eval::<()>("AccountStoreUtil.ToggleAccountStore(); return")
            .expect("second toggle (no arg) must run cleanly");

        let hidden_after_second_toggle: bool = env
            .eval("return AccountStoreFrame:IsShown() == false")
            .expect("post-second-toggle `IsShown()` probe must run cleanly");

        assert!(
            hidden_after_second_toggle,
            "Expected `AccountStoreFrame:IsShown()` to flip back to false after a second toggle \
             (zero-arg this time, exercising the canonical call shape after the extra-arg call \
             above). The double-toggle round-trip is what makes the extra-arg-discard claim \
             load-bearing: a single toggle could spuriously land in the right state, but two \
             toggles MUST return the frame to its starting state. A true reading here means \
             the hide path (`HideUIPanel(AccountStoreFrame)` falling back to `:Hide()`) \
             regressed for the post-extra-arg-call case — which would in turn mean the \
             extra-arg call corrupted some hidden state the toggle reads on the next \
             invocation."
        );

        let store_front_id_after_round_trip: i64 = env
            .eval("return AccountStoreFrame.storeFrontID")
            .expect("post-round-trip `storeFrontID` probe must run cleanly");

        assert_eq!(
            store_front_id_after_round_trip, initial_store_front_id,
            "Expected `AccountStoreFrame.storeFrontID` to STILL be unchanged after the \
             double-toggle round-trip. Two toggles produce zero net id-side effect — neither \
             the show transition (`ShowUIPanel` → `:Show()` → frame OnShow) nor the hide \
             transition (`HideUIPanel` → `:Hide()` → frame OnHide) writes to `storeFrontID`. \
             A mismatch here would mean some show/hide-triggered side path \
             (`AccountStoreMixin:OnShow` at `Blizzard_AccountStore.lua:26` triggers \
             `AccountStore.ShownState`; `OnHide` at line 31 triggers it again and runs \
             `AccountStoreUtil.CloseStaticPopups`) gained a new id-rewriting line — a \
             regression that would make the toggle path silently reseed the id and break the \
             separate-path contract."
        );
    });
}

#[test]
fn set_store_front_id_overrides_load_time_default_and_persists_new_id() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let load_time_store_front_id: i64 = env
            .eval("return AccountStoreFrame.storeFrontID")
            .expect("load-time `AccountStoreFrame.storeFrontID` probe must run cleanly");

        let plunderstorm_store_front_id: i64 = env
            .eval("return Constants.AccountStoreConsts.PlunderstormStoreFrontID")
            .expect(
                "`Constants.AccountStoreConsts.PlunderstormStoreFrontID` probe must run cleanly",
            );

        assert_eq!(
            load_time_store_front_id, plunderstorm_store_front_id,
            "Expected `AccountStoreFrame.storeFrontID` to equal \
             `Constants.AccountStoreConsts.PlunderstormStoreFrontID` after `{ROOT}` loads. \
             `UIParent_OnShow` at `Blizzard_UIParent/Mainline/UIParent.lua:351-352` calls \
             `C_AddOns.LoadAddOn(\"Blizzard_AccountStore\")` and then \
             `AccountStoreFrame:SetStoreFrontID(Constants.AccountStoreConsts.PlunderstormStoreFrontID)` \
             unconditionally during the UIParent show chain — the smoke harness shows UIParent \
             so this path runs as a load-time side effect. A mismatch reading here means \
             either (a) UIParent_OnShow stopped seeding the id (a Blizzard change forcing a \
             re-pin against the new load-time default — and likely a downstream change in \
             how AccountStoreItemDisplayMixin's load-time category fetch works), or (b) the \
             harness short-circuited UIParent's show chain (a regression in the smoke \
             fixture — `UIParent_OnShow` is the canonical place where the simulator's shown \
             contract gets seeded)."
        );

        const SENTINEL_STORE_FRONT_ID: i64 = 42_424_242;
        env.eval::<()>(&format!(
            "AccountStoreFrame:SetStoreFrontID({SENTINEL_STORE_FRONT_ID}); return"
        ))
        .expect("`AccountStoreFrame:SetStoreFrontID(<sentinel>)` call must run cleanly");

        let store_front_id_after_set: i64 = env
            .eval("return AccountStoreFrame.storeFrontID")
            .expect("post-`SetStoreFrontID(<sentinel>)` `storeFrontID` probe must run cleanly");

        assert_eq!(
            store_front_id_after_set, SENTINEL_STORE_FRONT_ID,
            "Expected `AccountStoreFrame.storeFrontID` to be the sentinel value \
             ({SENTINEL_STORE_FRONT_ID}) after \
             `AccountStoreFrame:SetStoreFrontID({SENTINEL_STORE_FRONT_ID})`. \
             `AccountStoreMixin:SetStoreFrontID` at `Blizzard_AccountStore.lua:42-48` writes \
             `self.storeFrontID = storeFrontID` (line 43) UNCONDITIONALLY before doing \
             anything else — the assignment runs even when the id is unknown to \
             `STORE_FRONT_TO_TITLE` (the local title-lookup table at lines 2-5 only contains \
             `WowhackStoreFrontID` and `PlunderstormStoreFrontID`, so the sentinel id falls \
             into the `or \"\"` empty-string branch on line 45 — but that does NOT block the \
             id-assignment on line 43). The sentinel value 42424242 is intentionally chosen \
             to NOT match either real store-front id, proving that the id-assignment path \
             does not gate on title-lookup success. A mismatch reading here would mean \
             either (a) Blizzard added a guard that rejects unknown ids before the \
             field-assignment line (forcing a re-pin against the new validation contract — \
             and likely a corresponding downstream change in how `OnStoreFrontSet` callbacks \
             fire), or (b) the assignment got reordered after the title-lookup line (a \
             regression that would silently break callers passing yet-to-be-localized ids)."
        );

        assert_ne!(
            store_front_id_after_set, load_time_store_front_id,
            "Expected `AccountStoreFrame.storeFrontID` ({store_front_id_after_set}) to differ \
             from the load-time PlunderstormStoreFrontID ({load_time_store_front_id}) after \
             `SetStoreFrontID({SENTINEL_STORE_FRONT_ID})`. A match reading would mean either \
             (a) the SetStoreFrontID body silently dropped the new id and kept the old one (a \
             regression in `AccountStoreMixin:SetStoreFrontID`'s assignment), or (b) the \
             sentinel 42424242 collided with PlunderstormStoreFrontID (verify by printing \
             `Constants.AccountStoreConsts.PlunderstormStoreFrontID` and choosing a new \
             sentinel that's safely outside both real store-front ids). The differs-from-load \
             contract is what proves the id-setter path is observably DIFFERENT from the \
             toggle path: the toggle test pins the id stays pinned at the load-time value; \
             this test pins that the id-setter path overrides it."
        );
    });
}
