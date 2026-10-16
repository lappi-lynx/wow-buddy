# Dungeon codex — The Deadmines (KB + verification)

Pipeline: KB (NotebookLM) → Wowhead Forever page (browser tool) → ForeverDB item checks.
Cycle: `plans/2026-10-16-codex-readiness-pets-plan.md`, Task 5 (codex pilot).

## KB session `27d92bbb` (2026-10-16)

Question: for WoW Forever 1.60.1, The Deadmines (Alliance, 17–21): recommended
average group level and difficulty; what to expect/avoid; notable mobs and
bosses; consumables.

Key answers:

- **Level:** the sources show a **level-20 party clearing The Deadmines**
  (within the 17–21 range); no explicit difficulty rating is stated.
- **Travel:** no summoning stone or auto-teleport — the group walks through
  Westfall to the entrance.
- **Bosses referenced:** Rhahk'Zor, *"Taskmaster Fizzule"*, Mr. Smite,
  Edwin VanCleef.
- **Consumables:** no checklist in the sources; only general tips (profession
  campfire buffs; cooked food +5% XP — already covered by the fishing/cooking
  card). Any potion/buff list ships as authored guidance with the Beta marker.

### Verification flags

- **"Taskmaster Fizzule" must be checked against Wowhead/ForeverDB — it does not
  match the classic Deadmines boss list; treat as a suspected KB error until
  proven otherwise.**
- Expected boss list (to confirm on the Wowhead page): Rhahk'Zor, Sneed,
  Gilnid, Mr. Smite, Captain Greenskin, Edwin VanCleef (+ Cookie).
- Loot candidates come from the Wowhead page; **every item id is checked on
  ForeverDB** (existence, rare quality, sensible level) before shipping.
- Recommended average level: KB supports ~20; if Wowhead states a group level,
  use it; otherwise apply the design's fallback rule
  `max(level_range.max − 2, min_level)` → 19, marked `verified: false`.

## Wowhead pull

Recorded in `docs/wowhead-sources/dungeon-deadmines.json` (raw rows + URL).

## ForeverDB verification

Recorded below once the loot candidates are picked.

## Resolution (2026-10-16)

- **"Taskmaster Fizzule" refuted** — he appears in neither the Wowhead Forever
  boss list nor ForeverDB's boss order; treated as a KB error, not shipped.
- **Boss roster** = ForeverDB's client order: Rhahk'Zor, Sneed, Miner Johnson,
  Gilnid, Captain Greenskin, Mr. Smite, Cookie, Edwin VanCleef (all Humanoid
  elites, levels 19–21, verified on the Wowhead Forever NPC pages).
- **Recommended average level:** no source states one directly; the KB's
  "level-20 party" example is an anecdote, and ForeverDB only documents the
  17–26 range. Fallback rule (`max(level_range.max − 2, min_level)`) against
  the classic range that was live at authoring time (17–21) gives 19; the
  wider ForeverDB range (26) was deliberately not used to inflate the number.
  Shipped: "Average 19+ for a 5-man", `verified: false` (Beta).
- **Level range ripple:** the old classic range 17–21 was replaced by
  ForeverDB's **17–26** in the dungeon entry.
- **Prep:** the KB has no consumable checklist; the shipped prep is the
  KB-verified campfire buff line plus the established cooked-food +5% XP fact
  (see the fishing/cooking card) and a potion-stack reminder.
- **Watch-outs:** all four ship verified — Forever loot differences (Wowhead
  Forever changelog), the walk-in/no-summoning-stone note (KB + ForeverDB
  "Getting there"), the mixed-level XP penalty (ForeverDB citing the dev
  podcast), and the unconfirmed new-item list (ForeverDB "client clue"
  language).
- Full verification tables: `docs/foreverdb-sources/dungeon-deadmines.md`
  and `docs/wowhead-sources/dungeon-deadmines.json`.

## Addendum (2026-10-16, Task 6)

- The global Forever dungeon XP rules also apply here (quest XP -50%,
  creature XP +20%, October 8 beta build) — the entry's watch-out list carries them
  from the Task 6 pass (KB sessions `6b4fb703` / `a33f9deb`).
