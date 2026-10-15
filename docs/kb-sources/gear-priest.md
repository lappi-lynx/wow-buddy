# Priest gear path 1–30 — KB + DB sources (Track C)

- **Date:** 2026-10-15
- **KB sessions:** `cfa93405` (Alliance list), `d8e9e0fa` (Horde list).
- **Cross-check surfaces:** Wowhead search (names → ids, paced) + ForeverDB item pages (curl) for existence/level/source; Wowhead item pages for icons.

## Fidelity cross-check (KB candidate → verdict → shipped data)

| KB candidate | ForeverDB / Wowhead | Verdict | Shipped |
| --- | --- | --- | --- |
| Oakthrush Staff (12, vendor/drop) | 15397 — Uncommon 2H staff, quest "The Fragments Within" (14), **Alliance** | Real; source corrected (quest, not vendor) | ✅ 14, alliance |
| Greater Magic Wand (13, Enchanting) | 11288 — Uncommon wand, **req 13**, Enchanting | Real | ✅ 13, both |
| Golemheart Stave (14) | 270228 — Rare staff, **req 13**, "new in WoW Forever" with **no documented source** | **Dropped: undocumented source** (guessing one would violate the rules) | ❌ |
| Smoldering Staff (horde suggestion) | No such item in search | Dropped (name likely invented) | ❌ |
| Staff of Westfall (20, Defias quest) | 2042 — Rare 2H staff, quest "The Defias Brotherhood" (22), Alliance | Real — **the same item the warrior artifact dropped as a caster staff; correct for a priest** | ✅ 20, alliance |
| Cookie's Stirring Rod (20, Cookie) | 5198 — Rare **wand**, **req 17**, Cookie, Deadmines, 35% | Real; level 17 (bracket note) | ✅ 17, alliance (20–24 band) |
| Living Root (18, Verdan) | 6631 — Rare 2H staff, **req 18**, Verdan the Everliving, WC | Real | ✅ 18, horde |
| Crescent Staff (20, quest) | 6505 — Rare 2H staff, quest "Leaders of the Fang" (22), **Horde** | Real | ✅ 22, horde |
| "Gravestone Rod" (30, BFD quest) | Real name = **Gravestone Scepter** (7001) — Rare **wand**, quest "Blackfathom Villainy" (27), **Alliance** | Real, name mangled by the KB | ✅ 27, alliance |
| Lesser Mystic Wand (26, Enchanting) | 11289 — Uncommon wand, **req 26**, Enchanting | Real | ✅ 26, both |
| Odo's Ley Staff (30, Odo) | 6318 — Rare 2H staff, **req 21** (KB said 30), Odo the Blindwatcher, SFK | Real; not shipped (tighter verified set already fills the bands) | ❌ |
| — (found via DB) | **Rod of the Sleepwalker** (1155) — Rare 2H staff, **req 24**, Lady Sarevess, BFD — previously dropped for warrior as a caster staff | Correct for a priest | ✅ 24, both |
| Tail Spike (horde 12–16 filler) | 6448 — req 16 dagger, Skum, WC (verified in the rogue pass) | Priest-usable stat stick | ✅ 16, horde |

## Notes

- **Wands matter:** Greater Magic Wand (13) → Cookie's Stirring Rod (17) → Lesser Mystic Wand (26) → Gravestone Scepter (27) is the leveling wand line for priests, exactly as the guide describes.
- **Bracket notes:** Cookie's Stirring Rod (req 17) sits in the 20–24 band per the KB guide's assignment (same pattern as Cruel Barb).
- **Two "warrior drops" return as priest ships:** Staff of Westfall and Rod of the Sleepwalker — both were dropped from the warrior lists as caster staves and are correct picks here.
- **Armor hints are cloth** ("Quest cloth pieces (…)") unlike the warrior mail hint.
- **Icons imported:** `inv_staff_07`, `inv_staff_10`, `inv_staff_02` (Cookie's/Lesser Mystic — shared art), `inv_staff_25`, `inv_staff_04`, `inv_wand_04`, `inv_staff_06`; Oakthrush reuses `inv_staff_08` (hunter's Balanced Fighting Stick).
