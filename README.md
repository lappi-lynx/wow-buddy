# WoW Buddy

> Personal companion for *World of Warcraft: Forever 1.60.1* built with Rust & Dioxus 0.7.

**WoW Buddy** is a cross-platform offline companion app (Desktop & Mobile) designed to help newcomers and returning players navigate their first 10 hours in *World of Warcraft: Forever* without FOMO or complexity.

## Features
- **Two Majestic Shields:** Interactive faction selection between Alliance and Horde, with dual-faction support for the new Skyborne race.
- **Race Lore & Philosophy:** Faction-filtered races with authentic lore in RU, EN, and FI, racial abilities, and class badges.
- **3D Dressing Room:** Full-body model showcase with WebGL / 2D fallback, 4 character animations (Stand, Attack, Cast, Dance), and gear evolution slider (Lvl 1, 20, 40, 60).
- **Progression Guide (1–20 Lvl):** Starter zone checklists (Northshire, Coldridge, Shadowglen, Valley of Trials, Deathknell, Camp Narache, Zephras Isle), compass orientation, and 1-Click 16-point Legacy allocation preset.
- **Dungeon Survival Codex:** Pocket etiquette guide for The Deadmines & Ragefire Chasm (tank pulls, healer mana, bag prep, interactive Need vs Greed test).
- **Extensible i18n:** English default with instant Russian and Finnish support, persisted to local storage cache.
- **Persistent Bottom Navigation:** Bottom bar (`[🛡️ Dressing Room] [🧭 Progression] [🏰 Dungeons]`) maintaining character state across views.

## Running the App

For detailed setup across macOS, Windows, Linux, Web, iOS, and Android, see **[DEV_GUIDE.md](./DEV_GUIDE.md)**.

### Quick Start
```bash
cargo run
```

### Running Tests
```bash
cargo test
```
