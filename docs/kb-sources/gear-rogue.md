# Rogue gear path 1–30 — KB + DB sources (Track C)

- **Date:** 2026-10-15
- **KB sessions:** `135e3b6c` (Alliance list), `343cf9b5` (Horde list), `dae31f1f` (corrections).
- **Forever-specific mechanic confirmed by the KB:** rogues can train and **dual-wield one-hand axes** in 1.60.1 (the guide's rogue weapon track lists axes alongside swords and maces). Also confirmed in passing: Undead paladins and Dwarf shamans exist — both-faction classes, as `content.json` already encodes.
- **Cross-check surfaces:** ForeverDB item pages (curl) + Wowhead search/item pages (paced) for names → ids and icons.

## Fidelity cross-check (KB candidate → verdict → shipped data)

| KB candidate | ForeverDB / Wowhead | Verdict | Shipped |
| --- | --- | --- | --- |
| Poniard (KB: 14, vendors) | 2208 — Common Dagger, **req 14**, sold by Gunther Weller | Real | ✅ 14, both |
| Big Bronze Knife (KB: 16) | 3848 — Uncommon Dagger, **req 15**, Blacksmithing | Real (level 15) | ✅ 15, both |
| Tail Spike (KB: 20, Lord Pythas) | 6448 — Rare Dagger, **req 16**, drops from **Skum**, WC | Real; **level 16 + boss drift** → 12–16 band | ✅ 16, horde |
| Rhahk'Zor's Hammer (KB: 16) | Already verified as a warrior 2H mace — **wrong slot for a rogue** | **Dropped: off-slot** | ❌ |
| Cruel Barb (19) | Already verified | Real | ✅ 19, alliance |
| Blackfang (found via search) | 2236 — Rare Dagger, **req 20**, world drop | Real | ✅ 20, both |
| Meteor Shard (KB: 24) | 6220 — Rare Dagger, **req 23**, Archmage Arugal, SFK | Real; **23** | ✅ 23, both (24–30 bracket note) |
| Prison Shank (found via search) | 2941 — Rare Dagger, **req 23**, Bruegal Ironknuckle, Stockades | Real | ✅ 23, alliance |
| Toxic Revenger (KB: 30, Viscous Fallout) | 9453 — Rare Dagger, **req 27**, Gnomeregan | Real (level 27) | ✅ 27, alliance |
| Outlaw Sabre (KB: "The Stockade Unrest quest", Alliance) | **16886 — Uncommon 1H sword, quest "Baron Aquanis" (level 30), a choice of rewards, HORDE** | Real but **faction flipped: it is a Horde quest reward** | ✅ 30, horde |
| Mechanic's Pipehammer (KB: rogue pick) | 9604 — **Two-Hand Mace**, quest "Data Rescue", Alliance | **Dropped: off-slot (rogues can't use 2H)** | ❌ |
| Defias Rapier (found via search) | 1925 — Main Hand sword, **req 11** | Dropped: req 11 sits in the 7–11 gap, no band | ❌ |
| Butcher's Cleaver (6224, pushed again) | ForeverDB: item not found (third check) | **Dropped: does not exist** | ❌ |
| Subterranean Axe (14, repeated) | Previously dropped as non-existent | **Dropped: repeated fabrication** | ❌ |

## Notes

- **Bracket note (acc. to review follow-up):** Meteor Shard (req 23) is filed in the 24–30 band because the KB guide routes Arugal/Shadowfang Keep in its late-bracket pool; unlike Cruel Barb (19, which sits in the 17–19 gap), 23 is the top of the 20–24 range — it is a late-20s pickup and stays the best dagger until Toxic Revenger. The `level` field keeps the verified 23.
- **Alliance 24–30 second weapon:** Mechanic's Pipehammer was the KB's suggestion but is a 2H mace; Toxic Revenger + Meteor Shard are the two verified daggers.
- **Alliance armor pool** now reads BFD / Gnomeregan (the guide routes Alliance through Gnomeregan); horde stays BFD / SFK.
- **Icons imported:** `inv_sword_12` (Poniard), `inv_weapon_shortblade_04` (Big Bronze Knife), `inv_weapon_shortblade_10` (Tail Spike), `inv_weapon_shortblade_02` (Blackfang & Toxic Revenger — shared art), `inv_weapon_shortblade_25` (Meteor Shard), `inv_weapon_shortblade_18` (Prison Shank), `inv_sword_33` (Outlaw Sabre).
