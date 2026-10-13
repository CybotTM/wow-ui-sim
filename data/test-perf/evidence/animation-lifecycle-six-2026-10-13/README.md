# Six-case animation lifecycle packet — 2026-10-13

Saved execution at sealed `a8a148e98d8dc32cf81e789fc8926830c9ed0161`: **6 PASS / 0 FAIL, actual wait exit0, 0.60s**, 10970 filtered out. Independent3775 saved-artifact inspection **PASS**: [report](independent/report.md), [proof](independent/proof.json). No execution rerun accompanies this packet.

| Exact selector | Model-covered behavior (PASS) |
| --- | --- |
| `animation_query_lifecycle::owner_queries_follow_pause_resume_restart_and_stop` | Owner/child state; pause freezes elapsed/progress, resume advances, restart/stop reset; independent idle owner |
| `animation_query_lifecycle::pending_finish_queries_are_settled_before_callbacks` | Pending Finish from pause; zero-delta tick settles queries before OnFinished; once-only callback and restart/natural completion |
| `animation_query_lifecycle::reverse_and_repeat_queries_follow_playback_direction` | Reverse elapsed/progress; forward/reverse REPEAT wrap and direction/loop flags |
| `animation_query_lifecycle::bounce_loop_callback_observes_direction_change` | Two reached OnLoop boundaries; active state, self identity and alternating direction |
| `animation_query_lifecycle::delayed_animation_queries_use_local_active_elapsed` | Start-delay flag, local active elapsed/progress, end-delay clamp and completion |
| `animation_query_lifecycle::delaying_uses_order_start_and_longest_parallel_duration` | Ordered/parallel maximum including end delay; forward/reverse exact boundaries; paused omitted direction and explicit-false resume |

[Retention manifest](retention-manifest.json): **9 selected complete TEXT projections / 27,208 public bytes**. **Bounded retention gate PASS**: [full report](retention-gate-pass/report.md), [proof](retention-gate-pass/proof.json). Independent runtime/source inspection remains separate from retention/privacy acceptance. Existing receipt/stat evidence is reused, not fresh binary/library authentication.

Full saved stderr inspected: **3,831 bytes / 66 VM startup diagnostic lines**, six initialization sequences; **not full Blizzard CLEAN**. Saved argv disables CASC and sound. Check and full Blizzard startup remain pending for this scope.

Six-case coexistence/query proof only. Historical c81 two controls remain separate; frozen CCD **15204/7/19** remains unchanged. No PTR, native, broad-suite, geometry, security, smoothing, performance-cause or parent-acceptance claim. [Animation contract](../../../../docs/specs/animation-query-lifecycle.md) owns requirements and prior security limits.
