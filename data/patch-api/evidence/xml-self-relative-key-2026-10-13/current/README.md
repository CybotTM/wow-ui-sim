# XML self-relative-key: bounded selected TEXT retention

Owner: [XML self-relative-key spec](../../../../../docs/specs/xml-self-relative-key.md).

| Boundary | Retained observation / acceptance |
| --- | --- |
| Focused RED | 1 PASS / 1 FAIL, exit101; genuine pre-producer RED, not denial |
| Focused GREEN | 2 PASS / 0 FAIL, exit0; focused scope only |
| Normal startup | Saved exit0 and stdout []; normal acceptance PENDING |
| Independent | Independent XML acceptance PENDING; no invented PASS |
| methods_anchor | Actual 33 PASS versus prediction 15; not parent acceptance |
| template_child_anchor_override | Actual 3 PASS versus prediction 2; not parent acceptance |
| Store | 0 PASS / 2 FAIL, exit101; preservation FAIL, root attribution OPEN |
| Prefork | Terminal compilation: 741 records, build-finished success; runtime PENDING |

Producer: 480b9f934e017ff87e0c79c2ba28b47b33cded19. RED: a90f84c5de9d09aedcec93aeda24c6bf5463e2b2. Prefork compilation belongs to source 2d4b7e56a06ea0fbb400a37f2aabb819337e5872, not the producer revision.

Store: RightInset GetPoint(2) relative-frame identity assertion fails; card iteration encounters nil, so downstream card bounds remain unproved. Neither unrelated nor pre-existing attribution is established.

Both focused compile summaries report 741 records and six manifest deprecations. Their complete compile.stderr also retains the UnitRole Tank/Healer/Damage source dead-code warning at src/lua_api/globals/real/unit_roles.rs:17:5. Empty source_compiler_messages in summaries does not prove source-warning-free.

Retained receipts: [RED compiler stderr](xml-selfkey-mists-red-budget9-20261013T221509426256Z/compile.stderr); [GREEN compiler stderr](xml-selfkey-mists-green-20261013T222949790458Z/compile.stderr); [focused RED stdout](xml-selfkey-mists-red-budget9-20261013T221509426256Z/runtime-xml-selfkey-red/stdout); [focused GREEN stdout](xml-selfkey-mists-green-20261013T222949790458Z/runtime-xml-selfkey-green/stdout); [Store FAIL stdout](xml-selfkey-mists-green-20261013T222949790458Z/runtime-store-sibling-anchor-preservation/stdout); [Store stderr](xml-selfkey-mists-green-20261013T222949790458Z/runtime-store-sibling-anchor-preservation/stderr).

Original and retry1 denials remain whole historical receipts: allowed=false, zero launch; neither is test RED. Historical predictions and pending metadata remain unchanged. Default check has no selected terminal receipt. Native/render/CASC-enabled/default/other-profile/full-parent acceptance OPEN.

All 77 payloads are whole selected UTF-8 TEXT, projected only by frozen ordered literal replacements and CRLF/CR to LF. No clipping, JSON reserialization, trailing-newline stripping, executable/library/archive/source-tree hashing or authentication. Historical seal/auth digest strings are retained text, not new authentication. Retention does not establish runtime or independent acceptance.
