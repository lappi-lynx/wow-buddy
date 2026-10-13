#!/usr/bin/env node
// tools/assets/generate.mjs
//
// Regenerates src/assets/generated.rs from assets/manifest.json.
//
// For every manifest entry it embeds the file at `path` as a `data:` URI:
//   - kind === "item_icon"  -> ICONS
//   - every other kind      -> ARTS
//
// It FAILS (non-zero exit) and lists every missing source file instead of
// silently dropping entries — so running it before the curator has placed the
// real images can never wipe the hand-seeded placeholders.
//
// Run from the repository root:  node tools/assets/generate.mjs

import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { extname } from "node:path";

const ROOT = process.cwd();
const MANIFEST = `${ROOT}/assets/manifest.json`;
const OUT = `${ROOT}/src/assets/generated.rs`;

const MIME = {
  ".png": "image/png",
  ".webp": "image/webp",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".gif": "image/gif",
};

const manifest = JSON.parse(readFileSync(MANIFEST, "utf8"));
const entries = Object.entries(manifest.assets ?? {});

const missing = [];
const icons = [];
const arts = [];

for (const [id, entry] of entries) {
  const abs = `${ROOT}/${entry.path}`;
  if (!existsSync(abs)) {
    missing.push(`${id} (${entry.path})`);
    continue;
  }
  const mime = MIME[extname(entry.path).toLowerCase()] ?? "application/octet-stream";
  const uri = `data:${mime};base64,${readFileSync(abs).toString("base64")}`;
  (entry.kind === "item_icon" ? icons : arts).push([id, uri]);
}

if (missing.length > 0) {
  console.error("generate.mjs: missing source files for declared assets:");
  for (const m of missing) console.error(`  - ${m}`);
  console.error("\nPlace the files (see tools/assets/README.md) or remove the manifest entry.");
  process.exit(1);
}

const render = (name, doc, pairs) =>
  `/// ${doc}\npub const ${name}: &[(&str, &str)] = &[\n` +
  pairs.map(([id, uri]) => `    ("${id}", "${uri}"),`).join("\n") +
  `\n];\n`;

writeFileSync(
  OUT,
  `//! GENERATED FILE — produced by \`tools/assets/generate.mjs\`. Do not edit by hand.\n\n` +
    render("ICONS", "Item icons (`kind: \"item_icon\"`).", icons) +
    "\n" +
    render("ARTS", "Art assets (every other `kind`).", arts),
);

console.log(`generate.mjs: wrote ${OUT} — ${icons.length} icon(s), ${arts.length} art asset(s).`);
