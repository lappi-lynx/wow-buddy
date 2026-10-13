# Wowhead source artifacts

For data curated from Wowhead's WoW Forever pages (spell/item icons and the
mappings behind them), this directory holds what was scraped, from where, and
when. Same purpose as `docs/kb-sources/`: a claim can be **checked**.

- **Filename:** `<topic>.md` (e.g. `spell-and-item-icons.md`).
- **Contents:** date, the exact page URLs, and the extracted mapping
  (entity → Wowhead id → icon name) that the shipped data uses.
- **Not embedded in the app** — repo-only provenance.

## Workflow

1. Wowhead blocks plain HTTP scrapers (AWS WAF); pages were read in a real
   browser session and the icon names extracted from the rendered DOM.
2. Record the page URLs and the mapping here.
3. Import icons via `tools/assets` (CDN download → normalize → manifest →
   generator) — only what the app actually references.
