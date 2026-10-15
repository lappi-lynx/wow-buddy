# Shaman gear path 1–30 — KB + DB sources (Track C)

- **Date:** 2026-10-15
- **KB sessions:** `7e5cbc32` (Alliance/Dwarf list), `e00089a1` (Horde list).
- **Context:** this server has **Dwarf shamans** (Alliance) alongside the classic Horde races — both faction paths ship.

## Fidelity cross-check (KB candidate → verdict → shipped data)

| KB candidate | ForeverDB / Wowhead | Verdict | Shipped |
| --- | --- | --- | --- |
| Heavy Copper Maul / Ironforge Greathammer / Rhahk'Zor's Hammer | Already verified (warrior/paladin batches) | Real, shaman-appropriate (2H maces / 1H) | ✅ 12–16, alliance |
| Smite's Mighty Hammer (20) | Already verified | Real | ✅ 20–24, alliance |
| Emberstone Staff (20, "Taskmaster Fizzule") | 5201 — Rare 2H staff, **req 17**, drops from **Captain Greenskin**, Deadmines, 40% | Real; **level 17 + boss drift** (bracket note) | ✅ 17, alliance (20–24) |
| Reef Axe (22) | Already verified | Real (2H axe) | ✅ both bands (alliance 20–24, horde 20–24) |
| Mechanic's Pipehammer (28, "Electrocutioner 6000") | 9604 — Uncommon **2H mace**, quest "Data Rescue" (30), Alliance — previously verified and dropped from the **rogue** list as off-slot | Real — **correct for a shaman** | ✅ 30, alliance |
| Corpsemaker (29) | Already verified (2H axe) | Real | ✅ both 24–30 |
| Rod of the Sleepwalker (30) | Already verified (1155, req 24, caster staff — dropped for warrior, correct for priests) | Real; shaman-usable staff | ✅ 24, horde |
| Hammer of Orgrimmar (13) | Already verified (1H mace) | Real | ✅ 12–16, horde |
| Axe of Orgrimmar (13) | Already verified (2H axe) | Real | ✅ 12–16, horde |
| Crescent Staff (20) | Already verified (6505, quest, Horde) | Real | ✅ 22, horde |
| Subterranean Axe (14) / Cavedweller's Mace (14) | Previously dropped as non-existent | **Dropped (repeated fabrications)** | ❌ |
| Butcher's Cleaver (24) | ForeverDB: not found (fourth check) | **Dropped: does not exist** | ❌ |
| Commander's Crest (24, SFK shield) | Not verified — and shields are not part of the weapons dataset | **Dropped: out of model** | ❌ |

## Notes

- **Armor pools:** alliance reads Gnomeregan / RFK (Dwarf shaman path); horde reads BFD / RFK. Both 24–30 lines record today's provenance.
- **Rod of the Sleepwalker** appears here as a *melee-capable staff filler* for horde 24–30 — it is the KB's own suggestion for the band and DB-verified; if a better 2H is found later, it replaces first.
- **Bracket note:** Emberstone Staff (req 17) sits in the 20–24 band per the KB guide assignment (Cruel Barb precedent).
- **Icons imported:** `inv_mace_04` (Mechanic's Pipehammer), `inv_staff_13` (Emberstone Staff); everything else reuses already-embedded icons.

## Slots — shaman (2026-10-15)

- Mail, so the **paladin's verified Blacksmithing set is reused wholesale** (Runed Copper → Silvered Bronze/Patterned Bronze → Shining Silver/Elite/Silvered Bronze Leggings) with the faction cloaks/necks/rings/PvP items and the **class insignia** (Horde 18845 verified; **no Alliance shaman insignia exists in the DB** — a classic-era gap — so the Alliance 24–30 set has 10 slots with no trinket).
- Availability: A 7/11/10 · H 7/11/11.
