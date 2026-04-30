//! Behavior pin for `AccountStoreMixin:SetStoreFrontID(id)`.
//!
//! Spec/source mismatch finding (PLAN.md task for
//! `AccountStoreMixin:SetStoreFrontID(id)`): the plan describes the
//! method as doing TWO things — calling
//! `C_AccountStore.RequestStoreFrontInfoUpdate(id)` AND looking up the
//! title from a `ACCOUNT_STORE_TITLE_<storeFrontID>` dynamic global
//! string when present. BOTH claims diverge from the actual three-line
//! body at `Blizzard_AccountStore.lua:42-48`:
//!
//! ```lua
//! function AccountStoreMixin:SetStoreFrontID(storeFrontID)
//!     self.storeFrontID = storeFrontID;
//!     self:SetTitle(STORE_FRONT_TO_TITLE[storeFrontID] or "");
//!     EventRegistry:TriggerEvent("AccountStore.StoreFrontSet", storeFrontID);
//! end
//! ```
//!
//! 1. **Direct C_API call mismatch.** The body does NOT call
//!    `C_AccountStore.RequestStoreFrontInfoUpdate` itself. The actual
//!    invocation lives on a different mixin:
//!    `AccountStoreItemDisplayMixin:OnStoreFrontSet` at
//!    `Blizzard_AccountStoreItemDisplay.lua:104-109` calls it via
//!    `C_AccountStore.RequestStoreFrontInfoUpdate(self.storeFrontID)`
//!    after `InitializeStore` updates the cached id. That mixin
//!    receives the call by registering an EventRegistry callback in
//!    its `OnLoad` (line 58:
//!    `self:AddStaticEventMethod(EventRegistry, "AccountStore.StoreFrontSet", self.OnStoreFrontSet)`).
//!    So the actual cause-effect chain is `SetStoreFrontID` → fires
//!    `AccountStore.StoreFrontSet` → `OnStoreFrontSet` runs →
//!    `RequestStoreFrontInfoUpdate(id)` runs. The end-to-end side
//!    effect (the C_API gets called with the same id) is observable
//!    in principle but the call is INDIRECT through the EventRegistry
//!    chain — not a direct call from the SetStoreFrontID body. This
//!    test file does not pin the indirect chain end-to-end because
//!    the chain depends on `AccountStoreItemDisplayMixin:OnLoad` and
//!    `:InitializeStore` succeeding under the smoke harness, both of
//!    which traverse subwidgets the harness doesn't fully populate.
//!    The structural separation is documented here; the live trace is
//!    out of scope. The companion surface test
//!    `surface_events.rs::account_store_item_display_event_registry_callbacks_match_actual_event_names`
//!    pins that the EventRegistry callback IS registered for
//!    `AccountStore.StoreFrontSet`, so the wiring is in place even
//!    when the body of `OnStoreFrontSet` doesn't reach the C_API on
//!    every harness invocation.
//!
//! 2. **Title-lookup pattern mismatch.** The body uses a static local
//!    table `STORE_FRONT_TO_TITLE` declared at lines 2-5:
//!
//!    ```lua
//!    local STORE_FRONT_TO_TITLE = {
//!        [Constants.AccountStoreConsts.WowhackStoreFrontID] = WOWHACK_ACCOUNT_STORE_TITLE,
//!        [Constants.AccountStoreConsts.PlunderstormStoreFrontID] = PLUNDERSTORM_PLUNDER_STORE_TITLE,
//!    };
//!    ```
//!
//!    The map indirects from the constant id into ONE OF TWO
//!    specifically-named globals — `WOWHACK_ACCOUNT_STORE_TITLE` for
//!    the Wowhack store and `PLUNDERSTORM_PLUNDER_STORE_TITLE` for
//!    Plunderstorm. These are static, hard-coded names — there is no
//!    `ACCOUNT_STORE_TITLE_<id>` concatenation at all. Unknown ids
//!    fall through `or ""` to the empty string. The PLAN-shaped
//!    dynamic-global pattern (`_G["ACCOUNT_STORE_TITLE_" .. id]`)
//!    appears nowhere in the source.
//!
//! Two tests pin the title-lookup mismatch as a layered tripwire:
//!
//! - `set_store_front_id_does_not_lookup_plan_named_dynamic_title_globals`
//!   asserts `_G["ACCOUNT_STORE_TITLE_" .. id]` is nil for both real
//!   store-front ids (Wowhack and Plunderstorm) right after smoke
//!   load. This pins the ABSENCE of the PLAN-named dynamic-global
//!   pattern as a structural check — Blizzard adding such globals
//!   would flip this test.
//! - `set_store_front_id_does_not_resolve_title_through_plan_shape_dynamic_global_lookup`
//!   pre-seeds `_G["ACCOUNT_STORE_TITLE_<id>"]` with sentinel marker
//!   strings BEFORE calling `SetStoreFrontID`, then asserts the title
//!   FontString does NOT contain the marker. This pins the BEHAVIOR:
//!   even if the dynamic globals were present, `SetStoreFrontID` does
//!   not consult them — the marker would only surface in the title if
//!   the body adopted the PLAN-shaped lookup.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AccountStore";

#[test]
fn set_store_front_id_does_not_lookup_plan_named_dynamic_title_globals() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let plunderstorm_id: i64 = env
            .eval("return Constants.AccountStoreConsts.PlunderstormStoreFrontID")
            .expect("`PlunderstormStoreFrontID` probe must run cleanly");

        let wowhack_id: i64 = env
            .eval("return Constants.AccountStoreConsts.WowhackStoreFrontID")
            .expect("`WowhackStoreFrontID` probe must run cleanly");

        for (label, id) in &[("Plunderstorm", plunderstorm_id), ("Wowhack", wowhack_id)] {
            let plan_named_dynamic_global_type: String = env
                .eval(&format!(
                    "return type(_G[\"ACCOUNT_STORE_TITLE_\" .. {id}])"
                ))
                .unwrap_or_else(|error| {
                    panic!(
                        "failed to probe `_G[\"ACCOUNT_STORE_TITLE_\" .. {id}]` ({label}): \
                         {error}"
                    )
                });

            assert_eq!(
                plan_named_dynamic_global_type, "nil",
                "Expected `_G[\"ACCOUNT_STORE_TITLE_\" .. {id}]` to be nil for the {label} \
                 store-front id ({id}) after `{ROOT}` loads. PLAN.md describes the title \
                 lookup as `ACCOUNT_STORE_TITLE_<storeFrontID>` (a dynamic global formed by \
                 concatenating the prefix with the id), but no such global is defined \
                 anywhere in the simulator's global_strings data or in any UI source we \
                 ship. The actual title comes from a static local map \
                 `STORE_FRONT_TO_TITLE` (`Blizzard_AccountStore.lua:2-5`) that indirects \
                 each id to a specifically-named global \
                 (`WOWHACK_ACCOUNT_STORE_TITLE` / `PLUNDERSTORM_PLUNDER_STORE_TITLE`). A \
                 non-nil reading here means either (a) Blizzard added the PLAN-shaped \
                 dynamic-global pattern (forcing a re-pin against the new lookup shape — \
                 and likely retiring the static map), (b) some addon or the simulator's \
                 global-string seeding accidentally created the global (worth investigating \
                 because it would shadow a future Blizzard rename), or (c) the per-id \
                 dynamic pattern was added as part of a localization refactor (look at the \
                 latest `data/global_strings.rs` to confirm)."
            );
        }
    });
}

#[test]
fn set_store_front_id_does_not_resolve_title_through_plan_shape_dynamic_global_lookup() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let plunderstorm_id: i64 = env
            .eval("return Constants.AccountStoreConsts.PlunderstormStoreFrontID")
            .expect("`PlunderstormStoreFrontID` probe must run cleanly");

        let wowhack_id: i64 = env
            .eval("return Constants.AccountStoreConsts.WowhackStoreFrontID")
            .expect("`WowhackStoreFrontID` probe must run cleanly");

        const PLUNDERSTORM_MARKER: &str = "PLAN_SHAPE_TRIPWIRE_PLUNDERSTORM_98765";
        const WOWHACK_MARKER: &str = "PLAN_SHAPE_TRIPWIRE_WOWHACK_43210";

        env.eval::<()>(&format!(
            "_G[\"ACCOUNT_STORE_TITLE_\" .. {plunderstorm_id}] = \"{PLUNDERSTORM_MARKER}\"; \
             _G[\"ACCOUNT_STORE_TITLE_\" .. {wowhack_id}] = \"{WOWHACK_MARKER}\"; \
             return"
        ))
        .expect("seeding PLAN-named dynamic globals must run cleanly");

        env.eval::<()>(&format!(
            "AccountStoreFrame:SetStoreFrontID({plunderstorm_id}); return"
        ))
        .expect("`SetStoreFrontID(<plunderstorm_id>)` after seeding must run cleanly");

        let title_after_plunderstorm_set: String = env
            .eval("return AccountStoreFrame:GetTitleText():GetText() or \"<nil>\"")
            .expect("post-Plunderstorm-set title probe must run cleanly");

        assert_ne!(
            title_after_plunderstorm_set, PLUNDERSTORM_MARKER,
            "Expected `AccountStoreFrame:GetTitleText():GetText()` NOT to equal \
             \"{PLUNDERSTORM_MARKER}\" after `AccountStoreFrame:SetStoreFrontID({plunderstorm_id})`. \
             The test pre-seeded `_G[\"ACCOUNT_STORE_TITLE_\" .. {plunderstorm_id}]` with the \
             marker BEFORE the `SetStoreFrontID` call. If the body resolved the title via the \
             PLAN-shaped dynamic-global lookup (`_G[\"ACCOUNT_STORE_TITLE_\" .. id]`), the \
             marker would have surfaced as the title text. The actual body uses a static \
             local map `STORE_FRONT_TO_TITLE` (`Blizzard_AccountStore.lua:2-5`) whose values \
             were captured at module-load time — the seeded global never enters that lookup \
             path, so the title MUST be something other than the marker (typically empty \
             string for unknown ids, or the load-time-captured value of \
             `PLUNDERSTORM_PLUNDER_STORE_TITLE` for Plunderstorm). A marker reading here \
             means the body adopted the PLAN-shaped lookup pattern (forcing a re-pin against \
             the new shape — and likely retiring the static map at lines 2-5)."
        );

        env.eval::<()>(&format!(
            "AccountStoreFrame:SetStoreFrontID({wowhack_id}); return"
        ))
        .expect("`SetStoreFrontID(<wowhack_id>)` after seeding must run cleanly");

        let title_after_wowhack_set: String = env
            .eval("return AccountStoreFrame:GetTitleText():GetText() or \"<nil>\"")
            .expect("post-Wowhack-set title probe must run cleanly");

        assert_ne!(
            title_after_wowhack_set, WOWHACK_MARKER,
            "Expected `AccountStoreFrame:GetTitleText():GetText()` NOT to equal \
             \"{WOWHACK_MARKER}\" after `AccountStoreFrame:SetStoreFrontID({wowhack_id})`. \
             Same tripwire as the Plunderstorm case but for the Wowhack id: pre-seeded \
             `_G[\"ACCOUNT_STORE_TITLE_\" .. {wowhack_id}]` with the marker, then called \
             `SetStoreFrontID({wowhack_id})` and asserted the title FontString does not \
             surface the marker. Both ids are tested because the static-map path \
             (`Blizzard_AccountStore.lua:2-5`) routes each id through its own named global \
             (`WOWHACK_ACCOUNT_STORE_TITLE` for Wowhack, `PLUNDERSTORM_PLUNDER_STORE_TITLE` \
             for Plunderstorm); a regression that adopted the PLAN-shape might do so for \
             only one id and not the other, so pinning both is more robust than pinning a \
             single id."
        );
    });
}
