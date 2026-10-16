# ForeverDB — The Deadmines (`/dungeon/36-deadmines`)

Pull: 2026-10-16, `curl https://foreverdb.net/dungeon/36-deadmines`
(client-datamined). Used for: the codex entry's level range, boss roster and
order, quest list, travel info, the XP note and the loot verification.

## Dungeon facts (as stated by ForeverDB)

- Levels **17–26**, 5 players, 8 bosses, 7 quests, Westfall.
- Bosses in the client's order: 1. Rhahk'Zor · 2. Sneed · 3. Miner Johnson ·
  4. Gilnid · 5. Captain Greenskin · 6. Mr. Smite · 7. Cookie ·
  8. Edwin VanCleef.
- Quests: Red Silk Bandanas (17), Collecting Memories (18), Oh Brother… (20),
  Underground Assault (20), The Defias Brotherhood (22), The Test of
  Righteousness (22), The Unsent Letter (22).
- Getting there: entrance Westfall (38.2, 77.5) (Forever client); nearest
  Alliance flight master: Sentinel Hill. No summoning stone.
- XP note (dev podcast, cited by ForeverDB, seen 2026-10-08): a mixed-level
  group — e.g. four level 20s with one level 25 — earns very little XP per
  kill for the lower levels; run with a tight level band.
- "Worth it for" highlights: Blackened Defias Belt (top pick for Arms Warrior
  and 7 more specs at level 20), Cape of the Brotherhood (Assassination Rogue
  and 6 more specs), Ogre Loincloth (Affliction Warlock and 5 more specs).
- Unconfirmed (client clues, not player-confirmed yet): Ogre Loincloth,
  Bandsaw Wristbands, Goblin Hammer.

## Item verification

The Wowhead Forever zone page (`docs/wowhead-sources/dungeon-deadmines.json`)
lists 44 zone items; every one was checked on ForeverDB (`/item/<id>`, JSON-LD
description): **35 are Rare (blue)**, each with a source and drop rate. The
five shipped loot rows:

| id | item | slot | source (ForeverDB) |
| --- | --- | --- | --- |
| 5191 | Cruel Barb | One-Handed Sword | Edwin VanCleef (boss), 20% |
| 5193 | Cape of the Brotherhood | Cloak | Edwin VanCleef (boss), 30% |
| 10403 | Blackened Defias Belt | Waist | Captain Greenskin (boss), 30% |
| 1156 | Lavishly Jeweled Ring | Ring | Gilnid (boss), 45% |
| 7230 | Smite's Mighty Hammer | Two-Handed Mace | Mr. Smite (boss) |

## NPC verification (Wowhead Forever NPC pages)

Type / level / elite confirmed for: Rhahk'Zor 19 Humanoid · Sneed 20 · Miner
Johnson 19 · Gilnid 20 · Captain Greenskin 20 · Mr. Smite 20 · Cookie 20 ·
Edwin VanCleef 21 · Defias Watchman 16 · Goblin Engineer 18 · **Sneed's
Shredder 20 Mechanical**.

## Ripple

The epic's level range changed from the old classic 17–21 to ForeverDB's
**17–26** (this entry's `level_range` / `min_level` / `max_level`), which also
widens the readiness card's "in your level range" list.
