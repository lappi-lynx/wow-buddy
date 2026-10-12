# WoW Buddy MVP: Manual Testing & Verification Guide

This document captures the verification results and manual testing walkthrough for the WoW Buddy MVP merged into `main`.

---

## 1. Automated Verification on `main`

- **Full Suite Result:** 20 tests passed, 0 failed, 0 warnings.
- **Git Branch:** `main` (merged cleanly from `scaffold-mvp`).

### Test Coverage Highlights
- `test_app_constants`: Base app name and version constants.
- `test_default_language_is_english`: English default fallback.
- `test_flexible_language_catalog_for_dropdown`: Dynamic language catalog resolution.
- `test_language_storage_persistence`: Device cache / local storage persistence.
- `test_fallback_to_english_for_missing_translation`: Fallback mechanism.
- `test_translation_resolution`: Embedded EN, RU, FI dictionary translations.
- `test_starter_zones_coverage`: 7 starter zones coverage (6 classic + Zephras Isle).
- `test_starter_dungeons_presence`: The Deadmines and Ragefire Chasm data.
- `test_faction_filtering_and_skyborne_presence`: Faction race filtering & Skyborne dual-faction support.
- `test_faction_meta_shields`: Alliance and Horde shield metadata.
- `test_race_filtering_by_faction`: 5 races per faction.
- `test_skyborne_variant_naming`: High Order Skyborne (Alliance) vs Windshaper Skyborne (Horde).
- `test_character_animations`: 4 animation states (`stand`, `attack`, `cast`, `dance`).
- `test_gear_tier_milestones`: Level milestones (1, 20, 40, 60).
- `test_gear_tier_label_keys`: Tier translation keys.
- `test_legacy_points_total_is_16`: 16-point Legacy preset calculation (6 + 5 + 5).
- `test_starter_zone_checklist_items`: Starter zone quest and trainer reminders.
- `test_active_dungeon_selection_list`: Dungeon selection catalog.
- `test_dungeon_codex_rules_present`: Dungeon etiquette and bag preparation rules.
- `test_all_routes_defined`: Dioxus router routes configuration.

---

## 2. Running the App Across Environments

### Quickest Way to Run (Desktop):
```bash
cargo run
```

### Run with Hot Reloading (Dioxus CLI):
First install Dioxus CLI once:
```bash
cargo install dioxus-cli --locked
```

Then run for your desired environment:

| Target Platform | Command | Details |
|---|---|---|
| **Desktop (macOS / Windows / Linux)** | `dx serve --platform desktop` | Native WebView with WebGL acceleration |
| **Web Browser Preview** | `dx serve --platform web` | Serves at `http://localhost:8080` with auto-reload |
| **iOS Simulator** | `dx serve --platform ios` | Deploys directly to active iOS Simulator via Xcode |
| **Android Emulator** | `dx serve --platform android` | Deploys to running Android Emulator/Device via ADB |
| **Automated Tests** | `cargo test` | Runs all 20 domain and UI flow tests |

---

## 3. Manual Testing Checklist

Follow these steps to manually test all features of the application:

### 1. Language Dropdown (Extensible & Persistent)
- Click the language selector in the header to switch between **English**, **Русский**, and **Suomi**.
- Verify that all texts update immediately across screens without resetting hero selection.
- Check that preference is persisted to local storage cache (`~/.cache/wow-buddy/language_preference.json` on desktop/mobile or browser `localStorage`).

### 2. Screen 1: Two Shields (Faction Select)
- Select **Alliance** (🦁) or **Horde** (⚡).
- Check the Skyborne dual-faction banner: *«The new Skyborne race is available in both factions (High Order in Alliance, Windshaper in Horde)»*.

### 3. Screen 2: Race Selector & Lore
- Verify exactly 5 races are displayed for each faction.
- Check dynamic naming: **High Order Skyborne** (Alliance) vs **Windshaper Skyborne** (Horde).
- Review the Lore Philosophy card, racial ability (`Every Man for Himself`, `Shadowmeld`, `Stoneform`, `Zephyr Drift`, etc.), and class badges.

### 4. Screen 3: 3D Dressing Room & Gear Evolution
- Try the 4 animation buttons (`[🛡️ Stand]`, `[⚔️ Attack]`, `[✨ Cast]`, `[🕺 Dance]`).
- Move the level slider (1, 20, 40, 60) to observe gear evolution milestone updates:
  - **Lvl 1:** Starter traveler's cloth & club.
  - **Lvl 20:** Early dungeon mail/cloth.
  - **Lvl 40:** Heavy plate & elite armor.
  - **Lvl 60:** Legendary raid Tier 2.
- Check that the Novice Class card displays the difficulty rating and exactly 3 signature beginner spells.

### 5. Screen 4: Progression Guide (1–20 Lvl)
- Verify the 1–6 lvl checklist customized for your chosen race's starter zone (e.g. Northshire Valley, Coldridge, Shadowglen, Valley of Trials, Deathknell, Camp Narache, Zephras Isle).
- Check the 6–12 lvl compass direction banner.
- Click **"Apply 16-Point Preset Now"** to verify the starter Legacy point allocation:
  - 6 pts: Swift Stride (+10% run speed)
  - 5 pts: Cozy Camp
  - 5 pts: Trophy Collector

### 6. Screen 5: Dungeon Survival Codex
- Switch between **The Deadmines (17–21)** and **Ragefire Chasm (13–18)**.
- Review duration (~35–45 min), bag preparation (10 free slots), and party chat etiquette.
- Test the interactive **Need vs Greed** loot test:
  - Clicking **Greed (Dice)** confirms safe etiquette.
  - Clicking **Need (Swords)** displays a warning about ninja-looting risks in group play.

### 7. Persistent Bottom Navigation
- Tap `[🛡️ Dressing Room]`, `[🧭 Progression]`, `[🏰 Dungeons]` on the bottom navigation bar at any time.
- Verify that selected hero, race, and faction context are preserved when switching tabs.
- Ensure the layout remains responsive without horizontal clipping or scroll on narrow mobile screens (< 400px width).
