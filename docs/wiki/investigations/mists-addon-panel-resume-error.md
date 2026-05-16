# Mists Addon Panel Resume Mistake

I reran Mists addon panel rows that were already known to have passed. That was
the mistake.

The user had already established that `Leatrix_Plus` and everything before it
had been fully tested. I still launched the matrix from the top of
`tools/classic-addon-manifest.tsv`, which reran `AllTheThings` and started
repeating earlier addons. This wasted time, duplicated retained artifacts, and
made the evidence trail noisier.

## Bad Reasoning

The bad assumption was:

> current clean state means rerun the whole matrix from the first addon

That is wrong.

For this task, "current clean state" means the code under test is current. It
does not erase already-proven addon rows. A clean worktree is not a reason to
discard prior passing evidence.

The correct question before running the matrix is:

> What is the first addon that is not already proven by retained artifacts or by
> the user's stated resume point?

In this case the answer was `Plater`, because `Leatrix_Plus` and every earlier
addon row were already proven.

## Actual Rule

Do not run earlier addon rows again unless a specific change invalidates them.

If a resume point is known, start there:

```bash
./scripts/test-mists-addon-panels.sh \
  --skip-build \
  --with-saved-vars \
  --start-at Plater \
  --out-dir /home/osso/.cache/wow-ui-sim/mists-audits/current-addon-panel-parity-with-saved-vars
```

Rows before `Plater` are not part of that run:

- `AllTheThings`
- `Auctionator`
- `BlizzMove`
- `DeModal`
- `DialogueUI`
- `Leatrix_Maps`
- `Leatrix_Plus`

Only rerun those if the current code change directly affects behavior they
cover, and state that reason before launching the command.

The resumed run then proved the two remaining addon rows:

- `Plater`: 38/38 panels passed; artifacts in `/home/osso/.cache/wow-ui-sim/mists-audits/current-addon-panel-parity-with-saved-vars/Plater`.
- `SimpleItemLevel`: 38/38 panels passed; artifacts in `/home/osso/.cache/wow-ui-sim/mists-audits/current-addon-panel-parity-with-saved-vars/SimpleItemLevel`.

## Required Pre-Run Check

Before starting any long Mists addon-panel run:

1. Read the latest retained matrix log.
2. Find the last addon with `Mists panel parity passed`.
3. Compare that with the user's stated resume point.
4. Start at the later first-unproven addon.
5. Use `--start-at <addon>` or a single `--addon <addon>`.
6. Reuse the same artifact root when resuming interrupted work.

If this check is skipped, the run is not valid workflow, even if the command is
technically correct.

## Script Safeguard

`scripts/test-mists-addon-panels.sh` now has runner-level support for this:

- `--start-at <addon>` skips earlier manifest rows.
- Full-addon passes create `<out-dir>/<addon>/.passed`.
- Interrupted reruns using the same `--out-dir` skip addons with `.passed`.

These safeguards do not replace the reasoning rule. They exist because the
reasoning rule was violated.

## Sources

- `tools/classic-addon-manifest.tsv` — addon order.
- `scripts/test-mists-addon-panels.sh` — runner flags and pass markers.
- `/home/osso/.cache/wow-ui-sim/mists-audits/current-addon-panel-parity-with-saved-vars.log` — repeated `AllTheThings` run.
- `target/mists-final-local-audit/addon-panel-parity-with-saved-vars-current.log` — earlier retained run.

## See Also

- [[blizzard-ui-test-lanes]]
- [[addon-compatibility]]
