# WoW Buddy: Developer & Testing Guide

This is the maintained guide to running, testing, and extending **WoW Buddy** (version `0.1.0`).
The app targets **World of Warcraft: Forever 1.60.1** and runs on desktop and the web; the Dioxus
project also supports iOS and Android targets.

For the product overview, screenshots, and the data-provenance rules, see
[`README.md`](README.md).

---

## 1. Quickstart

WoW Buddy is a Rust 2021 application using **Dioxus 0.7**.

```bash
# Native desktop window
cargo run

# Full automated suite
cargo test
```

The suite currently reports **46 green test binaries** (185 test functions). It covers content
schemas, provenance artifacts, gear coverage, i18n, asset embedding, routing, and navigation. The
tests do not require a network connection.

No environment variables are needed for a normal local run.

---

## 2. Using the Dioxus CLI (`dx`)

Install the CLI once for hot reload and cross-platform builds:

```bash
cargo install dioxus-cli --locked
dx --version
```

---

## 3. Platform-by-platform

### A. Desktop (macOS / Windows / Linux)

Desktop is the primary target, using the platform WebView.

```bash
# Hot reload
dx serve --platform desktop

# Or a plain release build
cargo build --release
```

- **macOS:** native WKWebView.
- **Windows:** WebView2 (Edge Chromium).
- **Linux:** requires WebKit2GTK (`sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev` on
  Ubuntu/Debian).

### B. Web / local browser preview

```bash
# Dev server with auto-reload, usually http://localhost:8080
dx serve --platform web

# Release build
dx build --platform web --release
```

The release artifacts are written to `target/dx/wow-buddy/release/web/public`. For a manual pass,
serve that directory with any static server, for example:

```bash
cd target/dx/wow-buddy/release/web/public
python3 -m http.server 8092
```

> **Always test the release build.** A debug static build (`dx build --platform web` without
> `--release`) embeds the dev-server toast/hot-reload client and shows “Your app is being rebuilt”
> when served by a plain static server. It is only usable under `dx serve`.

To produce a shippable web bundle:

```bash
dx bundle --platform web --release
```

The bundle is generated under `dist/` (`dist/public` for web).

### C. iOS

**Prerequisites:** macOS with Xcode and an iOS Simulator.

```bash
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
dx serve --platform ios
```

### D. Android

**Prerequisites:** Android Studio with the SDK and NDK, plus `ANDROID_HOME`.

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
dx serve --platform android
```

Mobile targets have not yet been packaged for release; treat them as development targets until a
build is verified.

---

## 4. Manual verification & feature walkthrough

Use a fresh profile (or clear the language preference) for the first pass. The screenshots in
`docs/screenshots/` show the expected current state.

### 4.1 Language selector, footer, and persistence

- Every screen ends with a **one-line footer**: the guide name on the left and the compact
  **EN / FI / RU** selector on the right. There is no language dropdown in the header.
- Switch the language and verify that labels, lore, button text, and prose update immediately without
  resetting the selected hero, race, or class.
- Check the choice survives a reload:
  - desktop: `~/.cache/wow-buddy/language_preference.json`;
  - web: browser `localStorage`.

### 4.2 Faction picker

- The first screen offers **Alliance** (lion crest) and **Horde** (red emblem) as two large shields.
- Confirm the Skyborne notice: the race is available to both factions — **High Order Skyborne**
  (Alliance) and **Windshaper Skyborne** (Horde).
- Open **About & Legal** from the top right and check the version, asset-source summary, offline
  statement, and the verbatim Blizzard disclaimer.

### 4.3 Race selector and class choice

- Each faction lists **exactly five races**.
- Selecting a race updates the lore panel, racial ability, effect line, and the available classes.
- **Clicking a class confirms the choice and navigates to the dressing room.** The old “Choose
  <Race>” button is retired.
- Check the class row on a narrow viewport: it is centred and wraps without clipping.
- Race and class names remain English in every locale; only the prose around them is translated.

### 4.4 Dressing room

- Check the viewer badge: it must read **Preview**, never “3D”, until a real model ships. The central
  art is the class icon (paper-doll mode), and “Drag to rotate” must not appear.
- The four animation buttons (**Stand / Attack / Cast / Dance**) currently only change the
  `ANIMATION: …` label; there is no 3D playback in this build. Do not describe them as rendered
  animations in release notes.
- The gear card reads **“Gear at level N”**. Drag the slider:
  - set-change dots are amber; leveling-milestone dots are blue;
  - off-anchor levels show a snap label such as “Level 38 (Level 40: …)”;
  - “What unlocks around here” shows milestone chips within ±5 levels and the next unlock;
  - rows past the verified range carry **Beta, may change**.
- Confirm the starter-spell card shows exactly three core spells; spells above the selected level
  carry an **Unlocks at level N** chip.
- Open a terminology chip (Tank, Aggro, Need / Greed, …) and verify the popover in all three
  locales; close it with ✕ or the backdrop.
- **Copy my character plan** should copy the shareable build summary to the clipboard.
- The **Open Gear Planner** button opens the inventory sheet. The pets card appears **only for
  hunters and warlocks**.

### 4.5 Gear planner

- Toggle **Alliance / Horde** and move the level slider through the bands: **1–6, 12–16, 20–24,
  24–30**.
- The sheet shows one weapon row plus armor slots for weapon/chest/feet, head/shoulders/legs,
  back/hands/finger, and neck/waist/wrist, with the trinket and alternate weapons in the bottom row.
- Empty slots are shown honestly as “—” (for example, trinkets are thin on the ground before the
  late 20s).
- Tap any item tile to open its source popover; the panel shows the icon when one exists and the
  item's provenance line.
- Coverage is enforced by `tests/gear_coverage_test.rs`: all 9 classes × both factions, every band
  meets its floor, and every gear icon resolves to embedded bytes.
- **Known limit:** coverage currently stops at 30. The 30–60 sync is planned and must not be
  presented as shipped.

### 4.6 Progression guide

- The title is race-specific. Verify the **Starter Glade Checklist (Lvl 1–6)** for the chosen race's
  zone (Northshire, Coldridge, Shadowglen, Valley of Trials, Deathknell, Camp Narache, or Zephras
  Isle).
- Tap the first-quest NPC name (e.g. Marshal McBride) to open the KB-verified zone/subzone popover.
- Check the **Next Horizon Direction (Levels 6–12)** compass banner.
- **Legacy presets:** three tabs (Balanced / Explorer / Coin Saver), each a 16/16 build. **Copy
  allocation** copies the summary; the app never changes the in-game build.
- **Fishing guide card:** three steps plus the Forever-only +5% XP cooking tie-in, and
  **Read the full 1–300 guide →**.
- **Comfort tips:** one random tip from the deck per visit; no arrows and no counter.
- **Open Dungeon Survival Codex →** moves to the dungeon screen (also available from the bottom
  nav).

### 4.7 Dungeon codex

- Five dungeon tabs: **Ragefire Chasm (13–18), Hall of Thanes (13–20), Wailing Caverns (15–24),
  The Deadmines (17–26), Shadowfang Keep (20–30)**. Ranges follow ForeverDB, not Classic Era.
- **Dungeon readiness:** tick the eight compact pills; the meter moves through Not ready / Almost
  ready / Ready (5/8 threshold). Checked pills fill green and have no checkbox glyph.
- Set the level input and confirm the “In your level range” chips update. Follow **How do I find a
  group?** to verify the premade Group Finder guidance.
- Confirm the info cards: estimated duration, bag preparation, and communication.
- **Recommended group** always carries **Beta, may change**, even when it comes from the KB.
- Read the sections in the intended order: **What's inside → Loot → Prep → Watch out**.
  - **What's inside:** bosses in verified order with levels and abilities, plus notable trash.
  - **Loot:** top blue items with boss and drop rate; tap a row for its source popover.
  - **Prep:** potions, campfire, cooked food.
  - **Watch out:** Forever-specific hazards and loot caveats (e.g. unconfirmed new Deadmines
    items).
- Test the interactive **Need vs Greed** scenario: Greed confirms safe etiquette; Need warns about
  ninja-looting.
- Repeat the pass in RU and FI; headings and body text must be translated while dungeon, boss, and
  item names stay English.

### 4.8 Fishing guide (`/fishing`)

- Reachable from the progression card and from the fourth bottom-nav tab (**🎣 Fishing**).
- The guide is marked **Beta, may change** at the page level and on sections that depend on the beta.
- **How to fish:** seven steps, including the hammer-button gotcha (the Professions window, not the
  Fishing tab).
- **What to buy:** pole and lures with icons; the shop strip should render all five entries.
- **Trainers:** capital cities grouped by faction, starting-zone trainers, and the two Skyborne
  trainers on Zephras Isle.
- **Leveling route:** four tier cards (1–75 / 75–150 / 150–225 / 225–300) with zones, lures, and
  tips; the artisan card shows Nat Pagle, quest 6607, and four quest fish.
- Back returns to progression via the floating ←, and the bottom nav remains usable.
- Narrow-screen check: tier cards and trainer rows reflow without clipping.
- Gates: `tests/fishing_test.rs` (sections, tier chain, source paths, item icons, trilingual purity,
  the hammer fact).

### 4.9 Pets & companions (`/pets`)

- Only hunters and warlocks see the pets card in the dressing room.
- **Hunter:** “Tame basics” first (Tame Beast at level 10, Mend Pet at level 12), an
  **Unlocks at level 10** chip while below level 10, safe first picks (Cat / Bear / Boar), the full
  **17-family grid** (4 columns on desktop, 2 on phones), and notable pets 10–30 with level chips.
- **Warlock:** four minions with level chips and roles, plus the companion section.
- Other classes and a direct `/pets` visit without a hero show the companion-only view.
- Confirm every family and minion icon renders; this is gated by `tests/pets_test.rs`.
- Sources: `docs/wowhead-sources/pets-families.json` and `docs/kb-sources/pets.md`.

### 4.10 Navigation, popovers, and back behaviour

- The bottom bar has four tabs: **🛡️ Gear · 🧭 Progression · 🏰 Dungeons · 🎣 Fishing**. The active
  tab is highlighted, and hero/race/faction context is preserved when switching.
- The bottom bar is hidden on the faction picker, race selector, gear planner, pets, and About
  screens; those screens use the floating back button / swipe.
- Sub-screens have no header bar. Go back with the floating **←** button, a horizontal swipe on
  mobile, or a two-finger trackpad swipe on desktop.
- Popovers close via ✕ or the backdrop. Slider dragging must never trigger the back swipe.
- The footer language choice and bottom nav must both remain visible at the bottom of long pages
  without overlapping content in a way that blocks interaction.

### 4.11 Responsive and offline checks

- Test at a narrow width (≤ 400 px): no horizontal scrolling, clipped cards, or hidden primary
  buttons.
- Confirm the four-tab bottom nav still fits and each target is comfortably tappable.
- In DevTools, set the network to **Offline**, reload, and repeat a quick pass. The app should render
  fully — content and images are embedded and no runtime requests are made.

---

## 5. Content sources and provenance

Every shipped gameplay fact should be traceable. Source artifacts live outside the app so the bundle
stays lean:

| Folder | What it holds |
| --- | --- |
| `docs/kb-sources/` | Verbatim answers from the project NotebookLM KB, with date and question. Class tips, dungeons, gear, pets, fishing, milestones. |
| `docs/wowhead-sources/` | Wowhead Forever checks (item/NPC/dungeon pages, family tables), including conflicts. |
| `docs/foreverdb-sources/` | ForeverDB client-datamined pulls for items, dungeons, and NPCs. |
| `docs/wow-professions-sources/` | Profession reference material (currently the fishing guide). |

Rules:

1. Never author a concrete gameplay claim from memory. Save the source first.
2. Point the shipped JSON at the artifact (dungeons carry `kb`, `wowhead`, and `foreverdb` source
   paths; fishing carries four).
3. If sources conflict, record both and choose the more current client-verified value. ForeverDB
   ranges override classic ones.
4. If nothing confirms the claim, set `verified: false`; the UI must print **Beta, may change**.
5. A test can enforce the presence of provenance, not its truth. Review the raw artifact when
   changing text.

Useful entry points:

- `assets/data/dungeons.json` → `docs/kb-sources/dungeon-<id>.md`,
  `docs/wowhead-sources/dungeon-<id>.json`, `docs/foreverdb-sources/dungeon-<id>.md`.
- `assets/data/class_tips.json` → `docs/kb-sources/<class>.md` (presence enforced by
  `tests/class_tips_data_test.rs`).
- `assets/data/gear.json` → per-item provenance plus `docs/kb-sources/gear-<class>.md`.
- `assets/data/fishing.json` → the three fishing artifacts plus
  `docs/wow-professions-sources/fishing.md`.
- `assets/data/pets.json` → `docs/wowhead-sources/pets-families.json`,
  `docs/kb-sources/pets.md`.

When adding content, update the JSON, the provenance artifact, and the relevant gate; then run
`cargo test` and rebuild the release bundle.

---

## 6. Embedded asset pipeline

All artwork is compiled into the app as `data:` URIs. There is no runtime file loader and no
hotlinking.

1. Add the image under `assets/` and register it in `assets/manifest.json` with `path`, `kind`,
   `source`, and `license`.
2. Run the generator from the repository root:

   ```bash
   node tools/assets/generate.mjs
   ```

   It writes `src/assets/generated.rs` (icons in `ICONS`, all other art in `ARTS`). Missing files are
   reported; a declared entry with neither a source file nor previous bytes fails the run.
3. Update `assets/ATTRIBUTION.md` if the asset introduces a new source or license.
4. Run `cargo test`; `tests/asset_registry_test.rs`, `tests/asset_art_registry_test.rs`, and the
   icon-resolution tests must stay green.
5. `tests/asset_urls_test.rs` fails if `wowhead.com` or `zamimg.com` appears in `src/`.

The current manifest holds **195 assets** (118 item icons, 47 spell icons, 18 race renders, 9 class
renders, 2 faction crests, 1 zone painting).

---

## 7. Release checklist

Before calling a cycle done:

```bash
cargo test
dx build --platform web --release
```

Then:

- serve `target/dx/wow-buddy/release/web/public` and complete the relevant section of the manual
  checklist above (always test the release bundle, not a stale one);
- if `assets/manifest.json` or any source image changed, run `node tools/assets/generate.mjs` before
  the build;
- update `DEV_GUIDE.md` and `README.md` when behaviour, routes, or content scope changes;
- capture fresh screenshots for `docs/screenshots/` when the affected screen's layout changes;
- commit only the intended paths and keep the work local unless a push/PR is explicitly requested.

---

## 8. Test map

The suite is organised by concern rather than by module. The most useful gates:

| Test | Guards |
| --- | --- |
| `data_integrity_test.rs`, `game_data_matrix_test.rs` | Core game data and faction/race/class matrix. |
| `i18n_test.rs`, `ui_translation_test.rs` | Locale parity, nav labels, fallback behaviour, translation coverage. |
| `asset_urls_test.rs`, `asset_registry_test.rs`, `asset_art_registry_test.rs` | Offline asset policy, registry integrity, icon bytes. |
| `gear_coverage_test.rs`, `gear_planner_test.rs` | 9 classes × both factions, band floors, item provenance, planner logic. |
| `dungeons_data_test.rs` | Five-dungeon schema, source citations, loot icons, section completeness. |
| `fishing_test.rs` | Fishing sections, tier chain, artifact paths, trilingual purity. |
| `pets_test.rs`, `class_tips_data_test.rs`, `class_tips_card_test.rs` | Pet art/sections, KB artifacts, class tip shape. |
| `full_navigation_flow_test.rs`, `smoke_test.rs`, `about_view_test.rs` | Routes, navigation, and the About screen. |
| `css_test.rs` | Precompiled Tailwind stylesheet is embedded. |
| `render_art_test.rs`, `spell_icons_test.rs`, `npc_reference_test.rs` | Render/icon references and popovers. |

`cargo test` is the gate; do not commit with a red suite.

---

## 9. Troubleshooting

- **“Your app is being rebuilt” toast in a static preview:** you are serving a debug build. Use
  `dx build --platform web --release`.
- **The UI looks stale after data or source changes:** rebuild the release bundle; the static server
  keeps serving the previous files.
- **The preview server stopped:** restart `python3 -m http.server 8092` from
  `target/dx/wow-buddy/release/web/public`.
- **A generated asset ID is missing:** run `node tools/assets/generate.mjs` and check the manifest
  entry; the generator reports entries with no source file and no previous embedded bytes.
- **Language preference looks reset:** clear it deliberately via the footer, or remove the cache file
  / `localStorage` key when testing a fresh profile.
