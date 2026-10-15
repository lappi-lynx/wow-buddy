# WoW Buddy: Developer & Testing Guide

This guide describes how to run and manually test **WoW Buddy** across desktop, web, and mobile environments.

---

## 1. Quickstart (Immediate Desktop Execution)

WoW Buddy is written in Rust using **Dioxus 0.7**. You can build and run it directly with standard Cargo:

```bash
cargo run
```

### Running All Automated Tests
```bash
cargo test
```
All 20 unit and integration tests will run and pass, verifying data integrity, 16-point Legacy calculations, dungeon survival rules, and multilingual i18n resolution.

---

## 2. Using Dioxus CLI (`dx`) for Multiplatform & Hot-Reload

For hot reloading, mobile emulators, and full cross-platform builds, install the Dioxus CLI:

```bash
cargo install dioxus-cli --locked
```

Verify installation:
```bash
dx --version
```

---

## 3. Platform-by-Platform Instructions

### A. Desktop (macOS / Windows / Linux)

Desktop is the primary target with native WebGL WebView support.

```bash
# Run with hot reloading enabled
dx serve --platform desktop

# Or standard release build
cargo build --release
```

- **macOS:** Uses native WebKit WKWebView.
- **Windows:** Uses WebView2 (Edge Chromium).
- **Linux:** Requires WebKit2GTK (`sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev` on Ubuntu/Debian).

---

### B. Web / Local Browser Preview

You can quickly test the entire UI in your web browser:

```bash
# Serves at http://localhost:8080 with auto-reload
dx serve --platform web
```

To build the site (artifacts under `target/dx/wow-buddy/release/web/public`):
```bash
dx build --platform web --release
```
> Serve the **release** build for manual passes. A *debug* static build
> (`dx build --platform web` without `--release`) embeds the dev-server
> toast/hot-reload client and shows "Your app is being rebuilt" when served
> by a static file server; it is only usable under `dx serve`.

To produce a shippable web bundle:
```bash
dx bundle --platform web --release
```
The bundle will be generated in `dist/` (`dist/public` for the web platform).

---

### C. Mobile: iOS (Simulator & Physical Device)

**Prerequisites:** macOS with Xcode and iOS Simulator installed.

```bash
# Add iOS Rust targets
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

# Serve to active iOS Simulator
dx serve --platform ios
```

---

### D. Mobile: Android (Emulator & Physical Device)

**Prerequisites:** Android Studio with Android SDK and NDK installed, plus `ANDROID_HOME` configured in environment.

```bash
# Add Android Rust targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

# Serve to running Android Emulator or connected device via ADB
dx serve --platform android
```

---

## 4. Manual Verification & Feature Walkthrough Checklist

When testing manually, verify the following core features:

### 1. Language Dropdown & Local Storage Cache
- In the top right header, switch between **English**, **Русский**, and **Suomi**.
- Verify all labels, mottos, and buttons update instantly without resetting state.
- Check that preference is persisted to `~/.cache/wow-buddy/language_preference.json` (or browser `localStorage`).

### 2. Screen 1: Two Shields (Alliance vs Horde)
- Click the **Alliance** shield (lion crest art) with royal blue aura or **Horde** shield (red emblem art) with crimson aura.
- Notice the special banner explaining that the new **Skyborne** race is playable in both factions.

### 3. Screen 2: Race Selector & Lore
- Verify exactly 5 races are present for the chosen faction.
- Check that Skyborne is displayed as **High Order Skyborne** (Alliance) or **Windshaper Skyborne** (Horde).
- Review the Lore Philosophy card, racial ability (`Every Man for Himself`, `Shadowmeld`, `Stoneform`, `Walk on Air`, etc.), and class badges.

### 4. Screen 3: Dressing Room & Gear Evolution
- Test the 4 animation buttons: `[🛡️ Stand]`, `[⚔️ Attack]`, `[✨ Cast]`, `[🕺 Dance]`.
- The viewer badge must tell the truth: **Preview** (paper-doll art) today — never "3D" until a real model exists. "Drag to rotate" only shows in 3D mode.
- Move the gear slider between 1, 20, 40, and 60 levels:
  - **Lvl 1:** Starter traveler's cloth & club.
  - **Lvl 20:** Early dungeon set.
  - **Lvl 40:** Heavy plate & elite armor.
  - **Lvl 60:** Top-level raid armor.
- Off-anchor levels (e.g. 38) show a snap label — "Level 38 (Level 40: Heavy Plate)" — and the viewer caption uses the same anchors (ties snap upward). The slider also exposes the same text as `aria-valuetext`.
- **What unlocks around here:** milestone chips within ±5 levels (first dungeon, talents, Legacy at 25...) plus a "Next unlock" line; rows past 30 carry the "Beta, may change" tag.
- **Novice Card:** exactly 3 signature class spells. Spells above the selected level show an "Unlocks at level N" chip — a level-1 warrior has only Heroic Strike.
- **Terminology:** tap any jargon chip (Tank, Aggro, Respec, Need / Greed...) to open the explanation popover; check it in EN, RU, and FI.

### 5. Screen 4: Progression Guide & 16 Legacy Points
- Check the 1–6 level checklist for your race's starter zone (e.g. Northshire Valley, Valley of Trials, Zephras Isle).
- Check the 6–12 level compass direction.
- Click **"Apply 16-Point Preset Now"** to allocate the starter Legacy points:
  - 5 pts: Well Rested (rested XP accumulates faster)
  - 5 pts: Thrill of Adventure (killing blows restore Health/Mana)
  - 5 pts: Talented (talents from level 5 instead of 10)
  - 1 pt: Field Guide (camp feature cooldown)

### 6. Screen 5: Dungeon Survival Codex
- Toggle between **The Deadmines (17–21)** and **Ragefire Chasm (13–18)**.
- Review estimated duration (~35–45 min) and the reminder to clear 10 bag slots.
- Test the interactive **Need vs Greed** loot test:
  - Choosing **Greed (Dice)** confirms safe etiquette.
  - Choosing **Need (Swords)** alerts about ninja-looting risks in group play.

### 7. Persistent Bottom Navigation Bar
- Tap `[🛡️ Gear]`, `[🧭 Progression]`, `[🏰 Dungeons]` on the bottom bar at any time.
- Verify your selected hero, race, and faction context are preserved when switching tabs.
- Verify UI does not horizontally scroll or clip on narrow screen viewports (< 400px width).

### 8. Reference Popovers & About/Legal
- On the Progression screen, tap the first-quest NPC name (e.g. **Marshal McBride**) — a panel opens with the KB-verified zone/subzone; the NPC name stays English in all locales (only the sentence around it is translated).
- In the Gear Planner, tap a weapon name — the panel shows its icon (when one exists) and the source line; generic items like "starter club" open without a picture.
- Gear coverage: all **9 classes × both factions** ship level bands **1–6 / 12–16 / 20–24 / 24–30** (Track C). 30–60 arrives with the post-Nov-4 sync. Coverage is enforced by `tests/gear_coverage_test.rs` (`all_classes_complete`, `every_gear_icon_resolves_to_embedded_bytes`); per-class provenance lives in `docs/kb-sources/gear-<class>.md`.
- Close popovers via the ✕ or by tapping the backdrop.
- Open **ⓘ About & Legal** from the footer of the first screen — app version, asset sources, and the verbatim Blizzard disclaimer (mirrors `assets/ATTRIBUTION.md`).
- Every footer reads **Works offline** (not "100% offline"): the app is offline-first and may fetch optional updates.

### 9. Pets (`/pets`)
- Hunter: dressing room → "Hunter pets" card → guide opens with "Tame basics" first
  (Tame Beast at level 10, Mend Pet at level 12), an "Unlocks at level 10" chip on the
  card while under level 10, "Safe first picks" (Cat / Bear / Boar), the full family
  list behind "Show all 17 families", and "Notable pets" 10–30 with level chips.
- Warlock: "Warlock minions" card → four demons with level chips and roles, plus the
  companion section.
- Other classes: companion-only view (no hunter or warlock claims).
- Deep link /pets without a hero shows the companion view.
- Sources: `docs/wowhead-sources/pets-families.json` (raw table + conflicts) and
  `docs/kb-sources/pets.md` (KB answers + confirmations).

### 10. Guides (Track E)
- Progression guide: the Legacy preset picker (Balanced / Explorer / Coin Saver tabs; "Copy allocation"
  copies the summary — the app never changes your in-game build), the Fishing & Cooking card
  (3 KB-verified steps + the Forever-only +5% XP from cooked food), and the Comfort tips deck
  (10 cards, ←/→, `{current} / {total}` counter).
- Dungeon codex: the readiness card — tick the 8 checklist items to move the
  Not ready / Almost ready / Ready meter (5/8 threshold), set your level to see the dungeons in
  range, and read the KB-verified "How do I find a group?" card (premade Group Finder).
- Sources: `docs/foreverdb-sources/legacy-system-explained.md` (21 perks), KB sessions `542bc6e7`
  (grouping) and `e136eda7` (fishing).

### 11. Gear inventory sheet (Track "UI polish & armor")
- All **9 classes × both factions** ship the paper-doll slot sheet: one weapon row
  (primary + dashed alternates) and four slot columns (weapon/chest/feet ·
  head/shoulders/legs · back/hands/finger · neck/waist/wrist) + a bottom row
  with the trinket and the alternate weapon tiles. Dragging the slider swaps
  the whole set per band; tapping any tile opens its source popover.
- Slot availability is honest per band (empty "—" where the game has nothing:
  no trinkets before ~28, no cloth wrist ≥20 ≤30, no Alliance shaman insignia).
- Gate: `tests/gear_coverage_test.rs` — every class/faction band meets the
  per-band floor (6/10/10), every present slot carries id+icon+provenance, and
  every slot icon resolves to embedded bytes. Per-class artifacts:
  `docs/kb-sources/gear-<class>.md`.
