# Horde warrior gear path — KB source

- **Date:** 2026-10-14
- **Notebook:** WoW Forever KB — <https://notebooklm.google.com/notebook/cff6d28f-cdcc-4679-9e98-74cf2430bf2b>
- **Purpose:** finish the seeded-but-unfilled Horde warrior bands in `assets/data/gear.json` (12–16 Ragefire Chasm, 20–24 Wailing Caverns). Each item was cross-checked against its Wowhead Forever item page (existence, weapon type, level, source) before shipping.

## Question 1 (weapon lists)

For WoW Forever leveling, for a HORDE warrior from level 12 to 24, list concrete weapon upgrades that drop in Ragefire Chasm (13–18) and Wailing Caverns (15–22): give each weapon's name, required level, and which boss or source it comes from. Match this style: 'level 12 Heavy Copper Maul — crafted or early zones', 'level 16 Rhahk'Zor's Hammer — Rhahk'Zor, The Deadmines', 'level 20 Smite's Mighty Hammer — Mr. Smite, The Deadmines'. Give 2–4 weapons per dungeon.

## Answer 1 (verbatim)

Thoughts
Ragefire Chasm (Levels 13–18)
level 13 Orgrimmar Axe — "Hidden Enemies" quest reward, Ragefire Chasm
1
2
level 14 Subterranean Axe — Taragaman the Hungerer, Ragefire Chasm
2
3
level 14 Cavedweller's Mace — Maur Grimtotem / Orebore, Ragefire Chasm
2
3
Wailing Caverns (Levels 15–22)
level 15 Crescent Staff — "Leaders of the Fang" quest reward, Wailing Caverns
2
3
level 18 Living Root — Verdan the Everliving, Wailing Caverns
4
5
level 19 Venomstrike — Lord Cobrahn, Wailing Caverns
2
3
level 22 Reef Axe — Dungeon boss drop, Wailing Caverns
6
7
💡 Would you like to review the next weapon upgrade options in the level 24–30 bracket (such as Corpsemaker from Razorfen Kraul or Strike of the Hydra from Blackthom Deeps)?

Sources:
[1] WoW Forever Beta Development Notes – Updated October 8 - #5 by Kaivax - WoW: Forever Beta Discussion - World of Warcraft Forums — "Mulgore The turn-in marker for “Grim Tidings” has been updated for accuracy. Creatures in Gloomrise now respawn more evenly, and Gloomrise Hatchlings give appropriate XP for their difficulty. Orgrimmar Neeru Fireblade now has a map marker…"
[2] WoW Forever leveling weapons for every class, 1 to 60 — "Every path sticks to its own faction's quests. Horde paths also skip the Deadmines and Gnomeregan, which sit deep in Alliance land, and use Ragefire Chasm, Wailing Caverns and Shadowfang Keep instead. How weapons are compared Melee weapons…"
[3] Forever Database - Warcraft DB — (database navigation excerpt)
[4] WoW Forever leveling weapons for every class, 1 to 60 — "Priest 4 weapon types"
[5] WoW Forever leveling weapons for every class, 1 to 60 — "Priest 4 weapon types"
[6] WoW Forever leveling weapons for every class, 1 to 60 — "Theme Modern Beta live Level 30 cap Launch 4 Nov Home Weapon progression WoW Forever leveling weapons Compare weapon options and their sources for every class, two levels at a time."
[7] WoW Forever leveling weapons for every class, 1 to 60 — "Shaman 7 weapon types"

## Question 2 (corrections)

In WoW Forever 1.60.1: (1) Which dungeon and which boss drops the two-hand axe Reef Axe (item id 6905), and at what level do players typically get it? (2) Which quest chain rewards the one-hand sword Wingblade (item id 6504), and at what level should a player expect it? (3) Confirm that Cursed Felblade (item id 14145) drops from Taragaman the Hungerer in Ragefire Chasm, and that Axe of Orgrimmar (15424) and Hammer of Orgrimmar (15445) are rewards from the 'Hidden Enemies' quest chain in Ragefire Chasm.

## Answer 2 (verbatim)

Thoughts
1. Reef Axe (Item ID: 6905)
Dungeon: Wailing Caverns
1
.
Boss / Source: Drops as a boss loot item inside Wailing Caverns (notably from Verdan the Everliving)
1
.
Level Requirement: Required level 22 (players typically acquire and equip it at level 22 during Wailing Caverns runs)
1
.
2. Wingblade (Item ID: 6504)
Quest Chain: "Leaders of the Fang" (the primary Wailing Caverns dungeon quest chain given by Nara Wildmane in Thunder Bluff, requiring the defeat of the four Druid of the Fang leaders).
Expected Level: Expected around level 18–20 (the quest requires level 15 to pick up, but is typically turned in after completing the dungeon near level 18–20).
3. Confirmation: Ragefire Chasm Drops & Rewards
Cursed Felblade (Item ID: 14145): Confirmed. This one-handed sword drops directly from Taragaman the Hungerer, the level 16 elite demon boss in Ragefire Chasm.
Axe of Orgrimmar (Item ID: 15424) & Hammer of Orgrimmar (Item ID: 15445): Confirmed. Both weapons are choice rewards upon completing "Hidden Enemies", the iconic Horde quest chain in Ragefire Chasm involving Thrall and Neeru Fireblade
2
.
💡 Would you like to look at the next set of weapon upgrades in the level 24–30 bracket (such as Blackfathom Deeps or Razorfen Kraul), or check out the quest chains for another dungeon?

Sources:
[1] WoW Forever leveling weapons for every class, 1 to 60 — "Theme Modern Beta live Level 30 cap Launch 4 Nov Home Weapon progression WoW Forever leveling weapons Compare weapon options and their sources for every class, two levels at a time. The beta reaches level 30; higher brackets are planning f…"
[2] WoW Forever Beta Development Notes – Updated October 8 - #5 by Kaivax - WoW: Forever Beta Discussion - World of Warcraft Forums — "Mulgore The turn-in marker for “Grim Tidings” has been updated for accuracy. Creatures in Gloomrise now respawn more evenly, and Gloomrise Hatchlings give appropriate XP for their difficulty. Forgrimmar Neeru Fireblade now has a map marker and mouseover tooltip during the quest “Hidden Enemies”. Ironic."

## Fidelity cross-check (Wowhead Forever item pages + dungeon guides)

| KB candidate | Verdict | Shipped data |
| --- | --- | --- |
| "Orgrimmar Axe" (13, Hidden Enemies) | Real — actual name **Axe of Orgrimmar** (15424, Two-Hand Axe, ilvl 18) | ✅ shipped, level 13 |
| "Subterranean Axe" (14, Taragaman) | **Does not exist** (DB has "Subterranean Cape" 14149); the real Taragaman weapon is **Cursed Felblade** (14145, sword, req 13) | ✅ shipped as Cursed Felblade |
| "Cavedweller's Mace" (14) | **Does not exist** (DB has "Cavedweller Bracers" 14147) | ❌ dropped |
| "Crescent Staff" (15, quest) | Exists but is a caster staff | ❌ dropped for warrior |
| "Living Root" (18) | Exists but is a caster staff (+Spirit) | ❌ dropped for warrior |
| "Venomstrike" (19) | Exists but is a bow | ❌ dropped for warrior |
| "Reef Axe" (22) | Real — Two-Hand Axe, req 22, +8 Str; dropper per KB = Verdan the Everliving | ✅ shipped, level 22 |
| — | Added from the RFC/WC dungeon guides (verified item pages): **Hammer of Orgrimmar** (15445, 1H mace, Hidden Enemies reward) and **Taskmaster Axe** (5194, Two-Hand Axe, req 18, "Deviate Eradication" quest) | ✅ shipped | 
| — | **Wingblade** (6504, sword, "Leaders of the Fang" quest, expected ~18–20) | ✅ shipped, level 18 |
