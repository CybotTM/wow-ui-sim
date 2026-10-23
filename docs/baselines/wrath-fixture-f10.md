# Wrath fixture batch F10: separate spellbook and talent consumers

Status: **PARTIAL runtime evidence** at merged source `93a115a93`: Wrath original50 **2 PASS / 48 FAIL**; separate native10 **5 PASS / 5 FAIL**. Retail integration43 passes; seven existing prefork-target cases, after-runs and final gates remain pending. No producer edits.

The 50 retained modern tests remain byte-for-byte unchanged. A native analogue
never substitutes for the original row or establishes that it passed. Potential
profile-gate decisions are report-only; no gates or ignores were added.

## Native fixture boundary

`tests/common/wrath_spell_panels_f10.rs` is attached beneath the existing
Wrath-only `wrath_spellbook_tabinfo` module, without changing its existing gate.
It reuses the ordered original FrameXML startup and loads the unchanged native
`Blizzard_TalentUI` TOC separately. Known spell IDs are explicit fixture data;
no Lua consumer/API implementations are replaced. Talent source digests and the
existing spellbook digest bind consumer bytes.

Laptop 12340 spellbook facts establish six `GetSpellTabInfo` returns and the
full/compact distinction; these tests choose the full-ranks view. Cached-file
selectors, geometry, UI lifecycle, and talent behavior remain **INFERRED**.
Laptop 12340 panel declarations are **UNCONFIRMED**; cached-consumer runtime
execution does not authenticate native-client parity. No unknown native edge behavior is
encoded as a producer requirement. Missing native talent state, tooltip backing,
or lifecycle semantics must first produce an observed original-consumer RED.

## Bounded shared-capability mapping

Only the shared capability below is covered, not modern routing/art/metadata.

| New native case (under `wrath_spellbook_tabinfo::f10`) | F10 original intent | Proof |
|---|---|---|
| `talent_root_remains_hidden_after_native_addon_load` | `blizzard_player_spells_loads::blizzard_player_spells_panel_root_is_hidden_after_load` | Before PASS; after pending |
| `talent_addon_reports_loaded_after_explicit_native_load` | `blizzard_player_spells_loads::blizzard_player_spells_is_addon_loaded_after_explicit_load` | Before PASS; after pending |
| `spellbook_shows_supplied_spell_on_first_native_open` | `spellbook::spellbook_spells_visible_on_first_open` | Before FAIL; after pending |
| `first_spell_icon_is_ancestor_visible` | `spellbook::spellbook_icon_textures_in_ancestor_visible` | Before FAIL; after pending |
| `first_spell_button_has_nonzero_resolved_geometry` | `spellbook_layout::spellbook_spell_items_have_nonzero_rect` | Before PASS; after pending |
| `first_spell_icon_matches_native_spellbook_texture` | `test_keybindings_spellbook_talents::spellbook_first_visible_item_icon_matches_spellbook_texture` | Before FAIL; after pending |
| `first_open_formats_native_spellbook_page_label` | `test_keybindings_spellbook_talents::spellbook_paging_label_is_formatted_on_first_open` | Before PASS; after pending |
| `first_spell_hover_populates_native_tooltip` | `spellbook_layout::spellbook_hover_shows_spell_tooltip` | Before FAIL; after pending |
| `native_spellbook_toggle_closes_on_second_call` | `test_keybindings_spellbook_talents::keybind_s_toggles_spellbook_closed_on_second_press` | Before PASS; after pending |
| `native_talent_panel_has_visible_talent_button` | `test_keybindings_spellbook_talents::talent_panel_has_at_least_one_visible_talent_node_frame` | Before FAIL; after pending |

## Unchanged report-only gate candidates

Historical RED receipts are evidence from revision `773fab374`, not new runs.
Every pre-edit function matches that revision's bytes. Current Wrath execution
retains 48 FAIL; the two direct thin-dispatch routing cases already PASS. They are
not a new native panel capability. No gate or forced failure was introduced.

- `blizzard_player_spells_loads::blizzard_player_spells_is_addon_loaded_after_explicit_load` — Literal modern Blizzard_PlayerSpells addon, panel, mixins, namespace, or template contract.
- `blizzard_player_spells_loads::blizzard_player_spells_loads_without_unrelated_lua_errors` — Literal modern Blizzard_PlayerSpells addon, panel, mixins, namespace, or template contract.
- `blizzard_player_spells_loads::blizzard_player_spells_panel_root_is_hidden_after_load` — Literal modern Blizzard_PlayerSpells addon, panel, mixins, namespace, or template contract.
- `blizzard_player_spells_loads::blizzard_player_spells_publishes_player_spells_util_namespace` — Literal modern Blizzard_PlayerSpells addon, panel, mixins, namespace, or template contract.
- `blizzard_player_spells_loads::blizzard_player_spells_publishes_root_panel_mixins` — Literal modern Blizzard_PlayerSpells addon, panel, mixins, namespace, or template contract.
- `blizzard_player_spells_loads::blizzard_player_spells_publishes_spell_book_mixins` — Literal modern Blizzard_PlayerSpells addon, panel, mixins, namespace, or template contract.
- `blizzard_player_spells_loads::class_talent_load_system_inherits_dropdown_template_mixin_methods` — Literal modern Blizzard_PlayerSpells addon, panel, mixins, namespace, or template contract.
- `blizzard_progression_lane::each_lane_addon_directory_exists_on_disk` — Literal modern progression addon inventory, dependency graph, or Mists TOC contract.
- `blizzard_progression_lane::lane_dep_edges_classify_each_addons_load_floor` — Literal modern progression addon inventory, dependency graph, or Mists TOC contract.
- `blizzard_progression_lane::shared_talent_ui_appears_in_eager_game_discovery_as_the_lane_startup_anchor` — Literal modern progression addon inventory, dependency graph, or Mists TOC contract.
- `blizzard_progression_lane::shared_talent_ui_is_the_only_eager_load_in_the_lane` — Literal modern progression addon inventory, dependency graph, or Mists TOC contract.
- `blizzard_progression_lane::talent_ui_ships_only_a_mists_toc_variant_unresolvable_by_find_toc_file` — Literal modern progression addon inventory, dependency graph, or Mists TOC contract.
- `blizzard_talent_ui_loads::find_toc_file_returns_none_on_mainline_target` — Literal Mainline resolution or Mists flavor-directory/TOC contract; not native 30300 layout.
- `blizzard_talent_ui_loads::flavor_subdirs_present_classic_subdir_absent` — Literal Mainline resolution or Mists flavor-directory/TOC contract; not native 30300 layout.
- `blizzard_talent_ui_loads::mists_toc_file_exists_at_expected_path` — Literal Mainline resolution or Mists flavor-directory/TOC contract; not native 30300 layout.
- `hero_talents::hero_talents_rendering::test_button_frame_level_change_relevels_connected_edges_on_update` — Modern class-talent edge-pool/connected-button contract.
- `hero_talents::hero_talents_rendering::test_class_talent_edges_render_below_visible_talent_buttons` — Modern class-talent edge-pool/connected-button contract.
- `spellbook::native_raised_spellbook_paper_renders_after_high_strata_unit_overlay` — Modern PlayerSpellsFrame descendant tree; raised cases additionally require a 12.1.0 capture, mask case requires modern icon-mask art.
- `spellbook::native_raised_spellbook_shares_positive_level_without_raising_unit_overlay` — Modern PlayerSpellsFrame descendant tree; raised cases additionally require a 12.1.0 capture, mask case requires modern icon-mask art.
- `spellbook::spellbook_first_visible_item_emits_icon_mask_quad_on_first_open` — Modern PlayerSpellsFrame descendant tree; raised cases additionally require a 12.1.0 capture, mask case requires modern icon-mask art.
- `spellbook::spellbook_icon_textures_in_ancestor_visible` — Modern PlayerSpellsFrame descendant tree; raised cases additionally require a 12.1.0 capture, mask case requires modern icon-mask art.
- `spellbook::spellbook_spell_items_in_ancestor_visible` — Modern PlayerSpellsFrame descendant tree; raised cases additionally require a 12.1.0 capture, mask case requires modern icon-mask art.
- `spellbook::spellbook_spells_visible_on_first_open` — Modern PlayerSpellsFrame descendant tree; raised cases additionally require a 12.1.0 capture, mask case requires modern icon-mask art.
- `spellbook_layout::spellbook_first_open_is_stable_with_real_tutorial_logic_restored` — Modern PlayerSpellsFrame geometry, tutorial, paging, hover, or texture traversal fixture.
- `spellbook_layout::spellbook_frame_positions` — Modern PlayerSpellsFrame geometry, tutorial, paging, hover, or texture traversal fixture.
- `spellbook_layout::spellbook_hover_shows_spell_tooltip` — Modern PlayerSpellsFrame geometry, tutorial, paging, hover, or texture traversal fixture.
- `spellbook_layout::spellbook_hover_tooltip_is_sized_and_on_screen` — Modern PlayerSpellsFrame geometry, tutorial, paging, hover, or texture traversal fixture.
- `spellbook_layout::spellbook_layout_stays_locked` — Modern PlayerSpellsFrame geometry, tutorial, paging, hover, or texture traversal fixture.
- `spellbook_layout::spellbook_passive_item_border_and_icon_match_master_geometry` — Modern PlayerSpellsFrame geometry, tutorial, paging, hover, or texture traversal fixture.
- `spellbook_layout::spellbook_spell_items_have_nonzero_rect` — Modern PlayerSpellsFrame geometry, tutorial, paging, hover, or texture traversal fixture.
- `spellbook_layout::spellbook_texture_requests_match_between_opens` — Modern PlayerSpellsFrame geometry, tutorial, paging, hover, or texture traversal fixture.
- `talent_panel_onupdate_perf::talent_panel_open_onupdate_tick_latency_stays_under_budget` — Modern PlayerSpellsUtil/PlayerSpellsFrame.TalentsFrame workload and budgets.
- `talent_panel_onupdate_perf::talent_panel_open_visible_onupdate_handlers_stay_within_ceiling` — Modern PlayerSpellsUtil/PlayerSpellsFrame.TalentsFrame workload and budgets.
- `test_keybindings_spellbook_talents::keybind_s_dispatches_directly_to_playerspellsutil_toggle_spellbook_frame` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::keybind_s_is_a_single_thin_dispatch_to_toggle_spellbook_frame` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::keybind_s_opens_spellbook_no_errors` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::keybind_s_opens_spellbook_tab_on_first_press` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::keybind_s_toggles_spellbook_closed_on_second_press` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::specialization_escape_after_direct_open_closes_panel` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::specialization_escape_after_micro_button_and_tab_click_closes_panel` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::spellbook_first_visible_item_icon_matches_spellbook_texture` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::spellbook_paging_label_is_formatted_on_first_open` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::spellbook_panel_spell_tooltip_has_lines_after_tab_switch_and_closes_without_errors` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::talent_panel_has_at_least_one_visible_talent_node_frame` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_keybindings_spellbook_talents::talent_panel_switches_spec_tabs_and_closes_without_errors` — Literal modern PlayerSpells routing, unified tabs/escape, or modern spell-item selectors.
- `test_showuipanel_lod_player_spells::keybind_n_loads_blizzard_player_spells_and_shows_talents` — Literal modern Blizzard_PlayerSpells LoD handoff/prewarm contract; native spellbook and talents are separate.
- `test_showuipanel_lod_player_spells::keybind_s_loads_blizzard_player_spells_and_shows_spellbook` — Literal modern Blizzard_PlayerSpells LoD handoff/prewarm contract; native spellbook and talents are separate.
- `test_showuipanel_lod_player_spells::raw_spellbook_toggle_hands_off_to_blizzard_playerspells_util_on_first_load` — Literal modern Blizzard_PlayerSpells LoD handoff/prewarm contract; native spellbook and talents are separate.
- `test_showuipanel_lod_player_spells::raw_toggle_spellbook_frame_loads_blizzard_player_spells_and_shows_spellbook` — Literal modern Blizzard_PlayerSpells LoD handoff/prewarm contract; native spellbook and talents are separate.
- `test_showuipanel_lod_player_spells::startup_prewarm_loads_blizzard_player_spells_and_keeps_it_hidden` — Literal modern Blizzard_PlayerSpells LoD handoff/prewarm contract; native spellbook and talents are separate.

## Observed runtime boundaries

The [F10 investigation](../wiki/investigations/wrath-spell-panels-f10.md) records
which assertions fail and which native consumers were reached. Name-oracle
failure is not independently native-consumer RED; icon-path diagnosis remains
source-derived rather than instrumented. Producer repair is outside this fixture
batch. Retail's original seven `blizzard_player_spells_loads` rows belong to the
existing `prefork_full_ui` target; zero-selected integration attempts are INVALID,
not PASS. No test moved between targets and no assertion changed.
