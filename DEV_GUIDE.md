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

### 4. Screen 3: 3D Dressing Room & Gear Evolution
- Test the 4 animation buttons: `[🛡️ Stand]`, `[⚔️ Attack]`, `[✨ Cast]`, `[🕺 Dance]`.
- Move the gear slider between 1, 20, 40, and 60 levels:
  - **Lvl 1:** Starter traveler's cloth & club.
  - **Lvl 20:** Early dungeon set.
  - **Lvl 40:** Heavy plate & elite armor.
  - **Lvl 60:** Legendary raid Tier 2.
- Verify the Novice Card displays exactly 3 signature class spells with beginner-friendly explanations.

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
- Tap `[🛡️ Dressing Room]`, `[🧭 Progression]`, `[🏰 Dungeons]` on the bottom bar at any time.
- Verify your selected hero, race, and faction context are preserved when switching tabs.
- Verify UI does not horizontally scroll or clip on narrow screen viewports (< 400px width).

### 8. Reference Popovers & About/Legal
- On the Progression screen, tap the first-quest NPC name (e.g. **Marshal McBride**) — a panel opens with the KB-verified zone/subzone; the NPC name stays English in all locales (only the sentence around it is translated).
- In the Gear Planner, tap a weapon name — the panel shows its icon (when one exists) and the source line; generic items like "starter club" open without a picture.
- Close popovers via the ✕ or by tapping the backdrop.
- Open **ⓘ About & Legal** from the footer of the first screen — app version, asset sources, and the verbatim Blizzard disclaimer (mirrors `assets/ATTRIBUTION.md`).
