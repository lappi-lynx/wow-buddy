# Core spell learn levels — Wowhead Forever DB + KB source

- **Date:** 2026-10-14
- **Method:** each class's database page `https://www.wowhead.com/forever/spells/abilities/<class>` was read in a browser session (AWS WAF blocks plain HTTP); every row was extracted as `[name+rank, level, spell id]` from the listview table (`td[2]` name, `td[3]` level). The learn level is the **first rank's** level.
- **Used for:** `assets/data/content.json` core-spell `learned_at` (+ `verified: true`) and the Novice Card "Unlocks at level N" chip.

## Extracted levels (first rank)

| Class | Spell | Level | Base-rank ID |
| --- | --- | --- | --- |
| warrior | Charge | 4 | 100 |
| warrior | Heroic Strike | 1 | 78 |
| warrior | Taunt | 10 | 355 |
| paladin | Holy Light | 1 (KB-confirmed; rank-1 row not in the 100-row slice) | 635 (classic) |
| paladin | Seal of Righteousness | 1 | 20154 / 21084 |
| paladin | Blessing of Might | 4 | 19740 |
| hunter | Auto Shot | 1 | 75 |
| hunter | Serpent Sting | 4 | 1978 |
| hunter | Tame Beast | 10 | 1515 |
| rogue | Stealth | 1 | 1784 |
| rogue | Sinister Strike | 1 | 1752 |
| rogue | Eviscerate | 1 | 2098 |
| priest | Flash Heal | 20 | 2061 |
| priest | Power Word: Shield | 6 | 17 |
| priest | Shadow Word: Pain | 4 | 589 |
| shaman | Lightning Bolt | 1 | 403 |
| shaman | Earthbind Totem | 6 | 2484 |
| shaman | Healing Wave | 1 | 331 |
| mage | Fireball | 1 | 133 |
| mage | Frost Nova | 10 | 122 |
| mage | Conjure Water | 4 | — |
| mage | Conjure Food | 6 | — |
| warlock | Corruption | 4 | 172 |
| warlock | Summon Imp | 1 | 688 |
| warlock | Life Tap | 6 | 1454 |
| druid | Moonfire | 4 | 8921 |
| druid | Rejuvenation | 4 | 774 |
| druid | Bear Form | 10 | 5487 |

## Notes

- **Conjure Food & Water** (combined entry): the entry stores the earliest unlock, `learned_at = 4` (Conjure Water). Conjure Food arrives at 6.
- **Level 1** means learned at character creation, not from the class trainer — this is why the milestones card says trainer abilities start at **level 2** (first trainer visit).
- Each class DB page caps at 100 rows, which hid the paladin Holy Light rank-1 row.

## KB confirmation (fresh session)

The full 27-level table was submitted; the KB answered:

> "Every single one of the 27 core class abilities and their level learn requirements listed in your prompt is 100% correct for WoW Forever 1.60.1. None of the levels require correction. […] Paladin Holy Light = 1 (Confirmed: Rank 1 is learned at creation / Level 1). […]"

(NotebookLM session `54d9d696`; the KB also noted the "Early Talent Access" Legacy perk can give a first talent point at level 5 — base remains 10.)
