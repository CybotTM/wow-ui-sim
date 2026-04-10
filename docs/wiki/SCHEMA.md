# Wiki Schema

This wiki is an LLM-maintained knowledge base for the wow-ui-sim project. The LLM owns all files in `docs/wiki/` — it creates, updates, and cross-references them. Humans read; the LLM writes.

## Structure

```
docs/wiki/
├── SCHEMA.md          # This file — conventions and workflows
├── index.md           # Content catalog (categories → pages with summaries)
├── log.md             # Chronological record of ingests/queries/lints
├── systems/           # Simulator systems (layout, rendering, Lua, events, etc.)
├── design/            # Architecture decisions, design specs, phase plans
├── investigations/    # Debug logs, root cause analyses, bug workarounds
└── reference/         # API coverage, external resources, tooling
```

## Page Format

Every wiki page uses this template:

```markdown
# Page Title

One-paragraph summary.

## Content

Main content organized with headers.

## Sources

- [source-name](../relative-path.md) — what was used from this source

## See Also

- [[other-wiki-page]] — why it's related
```

Use `[[page-name]]` for internal wiki links (Obsidian-compatible). Use relative markdown links for source docs.

## Workflows

### Ingest

When processing a new source document:

1. Read the source fully
2. Identify which wiki pages it touches
3. Create new pages or update existing ones
4. Update cross-references (`See Also` sections)
5. Update `index.md` with any new pages
6. Append an entry to `log.md`

### Query

1. Read `index.md` to find relevant pages
2. Read those pages and synthesize an answer
3. If the answer produces a valuable new page, file it into the wiki

### Lint

Look for: contradictions, stale claims, orphan pages, missing cross-references, pages referencing removed source files.

## Conventions

- **File names**: lowercase, hyphens, no dates unless inherently temporal
- **Categories are directories**, not tags
- **One concept per page** — split rather than merge
- **Sources section is mandatory**
- **Keep pages current** — update when newer sources contradict
