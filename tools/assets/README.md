# Asset pipeline (tools/assets)

The app ships **100% offline**: asset bytes are embedded as `data:` URIs in
`src/assets/generated.rs`, and asset metadata lives in
`assets/manifest.json`. This directory documents the **curator step** that
keeps them in sync. It is run by a human/agent when content changes — there
is no runtime or CI network access.

## What to extract

Only the items actually referenced by `assets/data/gear.json` (their
`item_id` / `icon_id`). Do **not** bulk-import the item database.

## Steps

1. **Extract** — with `wow.export`, pull each required `item_id`'s icon
   (BLP). Export from the public CDN or a local client. Keep the original
   `item_id` → file mapping.
2. **Normalize** — convert to WebP at a small size (target ≤ 8 KB each),
   preserving aspect ratio. Name the file `<icon_id>.webp`.
3. **Declare** — add an entry to `assets/manifest.json`:
   `"<icon_id>": { "path": "assets/icons/items/<icon_id>.webp",
   "kind": "item_icon", "source": "<where it came from>", "license": "blizzard-fan-use" }`.
4. **Generate** — regenerate `src/assets/generated.rs` so each declared id
   maps to a `data:image/webp;base64,…` URI.
5. **Verify** — `cargo test --test asset_registry_test` (manifest integrity +
   fallback) and `cargo test` (full suite). Commit the manifest, the
   generated file, and the source images under `assets/icons/items/`.

## Rules

- Every manifest entry needs a non-empty `source` (attribution).
- Unknown / missing `icon_id` must fall back to a glyph — never a broken
  image or a panic.
- Keep the total embedded size within budget; if art grows heavy, revisit
  the `data:`-URI approach in favour of `manganis`/`asset!`.
- Extracted Blizzard assets are used under fan/personal terms, attributed,
  non-commercial. See `assets/ATTRIBUTION.md` (added in Plan 2).
