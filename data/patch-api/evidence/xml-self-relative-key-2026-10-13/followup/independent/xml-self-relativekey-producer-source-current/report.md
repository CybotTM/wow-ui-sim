# Bounded producer source gate — PASS (source only)

Revision: 480b9f934e017ff87e0c79c2ba28b47b33cded19. Scope: src/loader/xml_texture.rs and docs/specs/xml-self-relative-key.md. Current HEAD equals this revision; git diff against it for both files is empty. No tracked edits. Existing untracked .code-index.db and verification/ were untouched.

## Coverage and proof limits

| Contract | Result | Proof |
|---|---|---|
| Observed ProductChoice Icon direct self-key targets parent | PASS source gate | Cached ProductChoice.xml:66–70 has parentKey Icon, CENTER/$parent.Icon/CENTER/(0,15). xml_texture.rs:351–367 matches exactly and clears relative_key, sets relative_to to $parent. helpers.rs:319–324 resolves this to local parent; :354–372 emits a direct parent argument, not the named-key string path. Runtime GREEN unknown. |
| IconBorder retains sibling Icon | PASS source gate | Cached ProductChoice.xml:74–77 has own key IconBorder, reference $parent.Icon and (0,-5). Exact comparison cannot match $parent.IconBorder; borrowed anchors retain named-key emission. Runtime GREEN unknown. |
| ParentKey publication precedes anchors | PASS source gate | xml_texture.rs:169–188 publishes parent field; :193–198 emits this after color/gradient/tiling; :318–319 immediately emits anchors next. Observed Icon has no parentArray. Inheritance-resolved TextureXml enters build_texture_lua at xml_layer_batch.rs:94–105, batch executes in document order. No deferred attachment assumption is needed. |
| Direct Lua two-point rejection/control | historical PASS only | Actual a90 RED stdout says direct Lua case ok; unchanged test asserts rejection leaves both points intact and distinct-target replacement preserves other point. No current runtime assertion. |
| Complete ProductChoice subtree/scripts/errors | historical RED; current unknown | Actual a90 fails during CreateFrame at layer-child SetPoint self rejection, before subsequent output assertions; new source has not been executed by this gate. |

## Historical actual RED classification

Read all six original files in full: stdout, stderr, admission.json, argv.json, result.json, launcher.txt (embedded verbatim in proof.json). Admission allowed=true. Actual launcher status101; 2 tests, 1 passed, 1 failed, 8246 filtered. result.json attributes source a90f84c5de9d09aedcec93aeda24c6bf5463e2b2, profile mists. argv launches saved integration binary with xml_self_relative_key_, --nocapture, --test-threads=1, timeout86 and read-only source binding. No binary/source archive hashing or independent provenance audit was performed.

Failure verbatim: "instantiate unchanged ProductChoice template through production XML layers: Lua(Runtime(RuntimeError { message: "Lua error: layer children on ProductChoiceAnchorCard: Action[SetPoint] failed because[Cannot anchor to itself]: attempted from: Frame:SetPoint.", level: 0, traceback: [] }))" at tests/xml_self_relative_key.rs:110:6. Successful prerequisite assertions precede that call (:62–101): actual cached ProductChoice and ModelScene scripts load, real function/font prerequisites exist, and setup error accumulator is empty. This is an executed disputed-anchor failure, not a prerequisite/setup failure. Unit name includes 'green' but outcome/source/argv classify it as historical RED. Earlier admission denials are not this launch and were not re-audited.

## Minimal implementation and readability

No accepted in-scope source/readability findings. New helper has 23 body lines, shallow nesting (loop plus matching if), descriptive predicate, two arguments, no warning suppression or new TODO/FIXME/HACK/XXX. Approximate manual cyclomatic complexity 6; no high branching density. No metrics tool/broad audit was launched.

Cow lifetime 'a depends solely on input anchors; texture and parent_key do not escape. The predicate captures a shared borrow of local self_key, is reusable (shared-reference capture is Copy), and has no side effects; any() only gates cloning. Cow is borrowed for absent own key, dotted own key, or no exact matching anchor. When any match exists, anchors.clone() clones the entire AnchorsXml/list once (NOT only matching individual anchors); only matching anchors' two target fields are mutated. Other point/offset data and original input remain unchanged. &Cow deref-coerces into &AnchorsXml while the local owned value remains alive for code generation. This is source reasoning, not compiler proof.

Existing eight-argument build_texture_lua and existing unrelated emitter complexity predate this revision; no redesign task accepted. No Frame/fontstring/global SetPoint change, missing-key fallback or vendor change occurs in this two-file commit. Other aliases, unusual parentArray/key interactions and native WoW parity are outside the literal observed-case contract, not failures of this gate.

## Formatting and boundaries

Exactly once: rustfmt --check --edition 2024 <REPO>/src/loader/xml_texture.rs, cwd <REPO>. exit0, stdout empty, stderr empty. Does not prove compilation or runtime.

No build/check/runtime/broad fmt, authentication, delegation, working-directory switch, source archive hashing or tracked edits. Main-owned GREEN compile was not touched. Spec explicitly retains unchecked runtime/startup/prefork proof and excludes generic aliases/native parity; no overall feature completion claim.
