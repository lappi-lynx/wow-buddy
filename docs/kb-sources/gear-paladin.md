# Paladin gear path 1–30 — KB + DB sources (Track C)

- **Date:** 2026-10-15
- **KB sessions:** `41ed665e` (Alliance 12–30 list + level-1 starter), `5ca47e02` (Horde list), `8d81f658` (item corrections), `e7e7acb1` (Cobalt Crusher follow-up).
- **Cross-check surfaces:** ForeverDB `https://foreverdb.net/item/<id>` (curl; client-datamined) + Wowhead `/forever/item=<id>` (browser, paced) for the icon of the one new item.
- **Reuse note:** several items were already DB-verified in the warrior batches (Heavy Copper Maul, Ironforge Greathammer, Rhahk'Zor's Hammer, Smite's Mighty Hammer, Hammer/Axe of Orgrimmar, Cursed Felblade, Wingblade, Reef Axe, Strike of the Hydra, Corpsemaker) — ids and icons reused, provenance refreshed to today's KB sessions.

## Fidelity cross-check (KB candidate → verdict → shipped data)

| KB candidate | ForeverDB / Wowhead | Verdict | Shipped |
| --- | --- | --- | --- |
| Worn Mace + Wooden Shield (level-1 starter) | — | Real (starter band; null item fields by design) | ✅ "starter mace", 1–6 |
| Heavy Copper Maul (12) / Ironforge Greathammer (14) / Rhahk'Zor's Hammer (16) | Already verified (warrior batches) | Real, paladin-appropriate (2H maces / 1H mace + shield) | ✅ 12–16, alliance |
| Verigan's Fist (20, "The Test of Righteousness") | 6953 — Rare **Two-Hand Mace**; quest reward ("The Test of Righteousness (level 22)" = quest difficulty, classic level-20 class quest); Wowhead shows **no required level** (icon `inv_hammer_05`) | Real | ✅ 20, alliance |
| Smite's Mighty Hammer (20) | Already verified (7230) | Real | ✅ 20–24, alliance |
| Strike of the Hydra (26) | 6909 — **Aku'mai**, BFD (KB again said Gelihast — DB wins) | Real | ✅ 24–30, both |
| Archeus (28) | Real Archeus = item **2000**, quest "A Daughter's Love" **(level 35)**; KB's id 1204 is a shield | **Dropped: level-35 quest; wrong id** | ❌ |
| Cobalt Crusher (26, BFD) | KB gave id 6907 = **Tortoise Armor**; KB then stated it cannot confirm the item | **Dropped: unverifiable id** | ❌ |
| Corpsemaker (KB: 30) | 6687 — **req 29**, Overlord Ramtusk, RFK | Real; **29** (DB) | ✅ 24–30, both (KB-endorsed for Alliance too) |
| Butcher's Cleaver (24, SFK) | KB claims it exists (id 6224); ForeverDB: **item not found** (checked twice) | **Dropped: does not exist** | ❌ |
| Subterranean Axe (14) / Cavedweller's Mace (14) | Both previously dropped as non-existent (warrior batch) | **Dropped: repeated fabrications** | ❌ |
| Rod of the Sleepwalker (BFD) | Caster staff; KB again inconsistent on its level (24 vs 30) | **Dropped: caster staff, unstable facts** | ❌ |

## Notes

- **Horde paladin = Undead** (client-verified in `content.json`): starter armor hint "Tirisfal Glades"; Alliance paladin (Human/Dwarf) hint "Elwynn Forest".
- **Axe of Orgrimmar (15424) is a Two-Hand Axe** per ForeverDB; the corrections session's "one-hand" note was wrong (same conflict the warrior artifact records).
- **Armor lines** reuse the verified dungeon pools: Hall of Thanes (13–20), Deadmines (18–23), RFC (13–18), WC (15–22) with the 2026-09-17 dungeon provenance; 24–30 lines use BFD (24–32) and SFK (18–25) with today's provenance.
- **Icons:** `inv_hammer_05` (Verigan's Fist) imported 2026-10-15; every other icon was already embedded.
- The KB's per-class lists remain a *candidate* source only — three fabrications and two wrong ids were caught here by the DB check, the same failure mode the warrior batches documented.
