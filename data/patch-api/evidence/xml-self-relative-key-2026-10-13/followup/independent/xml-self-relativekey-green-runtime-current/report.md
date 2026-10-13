# Focused XML self-relativeKey verification

PASS — saved Mists focused execution at source metadata 480b9f934e017ff87e0c79c2ba28b47b33cded19: 2/2 passed, 0 failed, 8246 filtered; actual service exit 0 and launcher exit 0. Complete stdout/stderr/argv/admission/result/launcher read; retained in proof.json. Admission allowed; bounded 86s timeout, 4G MemoryMax, 3G MemoryHigh, CPUQuota 200%, isolated read-only source mount, network unshared, CASC disabled. 1.239s service runtime, 1.3G memory peak.

## Behavioral coverage
- ProductChoice test reads cached ProductChoice XML/Lua and ModelScene XML/Lua, registers real dependencies and instantiates ProductChoiceItemDisplayTemplate through CreateFrame production inheritance. Setup and final whole Lua error accumulators asserted empty.
- Icon identity/parent/key/63x63/one point asserted; CENTER → parent card CENTER (0,15), not self. IconBorder remains distinct child of card; CENTER → sibling Icon CENTER (0,-5).
- Later decorations, ModelScene parenting/OnLoad state, PreviewButton subtree/inherited texture, Covers subtree, and real mouse-down/up bindings asserted. This is completion of the selected item-display template and asserted subtrees/scripts, not every parent/store UI path.
- Direct Lua self SetPoint throws and preserves both pre-existing point identities/offsets/count; distinct-target replacement succeeds and preserves untouched point.

## Saved RED comparison
RED a90f84c5de9d09aedcec93aeda24c6bf5463e2b2, budget9 epoch: identical saved test text (ordinary content comparison, no hash/authentication). Same focused filter, 2 cases, Mists. 1 passed/1 failed, actual exit 101. Direct Lua preservation control passed. ProductChoice failed at tests/xml_self_relative_key.rs:110 during production creation: Cannot anchor to itself. Therefore RED reaches the disputed creation/anchor boundary, not an unmet setup prerequisite; GREEN reaches subsequent sibling/subtree/script assertions. RED was neither rehashed nor rerun.

## Separate normal-copy authentication
receipt-wow-sim.json created only after confirming absence. Exactly one streaming SHA-256 read; path stat before/after and open-file stats agree with seal. PASS, sha256_matches_seal=true, hash_reads=1. Exact stat_after recorded in receipt/proof. Receipt mode 0600; file and parent directory fsynced; immediate parsed readback matched. This normal wow-sim copy is not the integration executable used by the focused test. Cargo metadata associates the normal copy with Mists/source480; that is not source cryptographic provenance.

## Proof limits
Association only, not native/render/full-parent proof. No other byte/source authentication. External cached vendor scripts, libraries/settings and source archive unauthenticated. No builds, launches, RED reruns, source edits, commits or delegation.
