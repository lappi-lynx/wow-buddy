# Hunter gear path 1–30 — KB + DB sources (Track C)

- **Date:** 2026-10-15
- **KB sessions:** `20bddf9d` (Alliance list), `8194e327` (Horde list), `30a46bba` (corrections).
- **Cross-check surfaces:** ForeverDB `https://foreverdb.net/item/<id>` (curl; client-datamined) + Wowhead `/forever/item=<id>` (browser, paced) for icons and required levels; Wowhead `/forever/search?q=` to resolve names → ids.

## Fidelity cross-check (KB candidate → verdict → shipped data)

| KB candidate | ForeverDB / Wowhead | Verdict | Shipped |
| --- | --- | --- | --- |
| Venomstrike (KB: 19, Lord Cobrahn) | 6469 — Rare Bow, req 19; drops from **Lord Serpentis**, WC, 25% (Wowhead: req 19, `inv_weapon_bow_10`) | Real; **boss drift: Serpentis** (DB wins) | ✅ 19, both factions |
| Master Hunter's Bow (KB id 2981/17686) | id 2981 = **Seer's Robe** (KB wrong again); real = **17686** — Uncommon Bow, quest "Big Game Hunter" (Nesingwary chain; quest difficulty 43 = the classic level-30 chain), no item req (icon `inv_weapon_bow_11`); a duplicate page exists at 4110 | Real; DB id | ✅ 30, both |
| Heavy Recurve Bow (KB: 14, id 2505) | id 2505 = Polished Shortbow (ilvl 4 junk); real = **3027** — req **20**, sold by bow vendors (Wowhead req 20) | Real; **level 20, not 14** → moved to the 20–24 band | ✅ 20 |
| Silver-plated Shotgun (KB: 15, id 3082) | id 3082 = Dargol's Skull; real = **4379** — req **21**, crafted (Engineering) (Wowhead req 21) | Real; **level 21, not 15** → 20–24 band | ✅ 21 |
| Steelarrow Crossbow (6315, 21) | Search-confirmed id 6315 | Real — **not shipped** (two better ranged picks already in the band; artifact-only) | ❌ |
| Piercing Axe (6094, KB: 16) | Real — Uncommon 2H Axe, quest "Red Silk Bandanas" (**quest level 17**) | **Dropped: quest at 17 falls in the 17–19 gap, outside both bands** | ❌ |
| Balanced Fighting Stick (6215) | Real — Uncommon 2H Staff, quest "Wanted: 'Hogger'" (level 11, Alliance) (icon `inv_staff_08`) | Real | ✅ 12, alliance |
| Heavy Copper Broadsword (3487) | Real — Uncommon 2H Sword, **req 14**, crafted (icon `inv_sword_14`) | Real | ✅ 14, alliance |
| Axe of Orgrimmar / Cursed Felblade | Already verified (warrior/paladin batches) | Real, hunter-usable (2H axe / 1H sword) | ✅ 12–16, horde |
| Reef Axe / Corpsemaker / Strike of the Hydra | Already verified | Real, hunter-usable stat sticks | ✅ (Reef 22 horde, Corpsemaker 29 both) |

## Notes

- **No DB-verified 12–16 ranged weapon exists** beyond vendor/basic bows (Polished Shortbow is ilvl 4 junk); the alliance 12–16 band therefore ships two melee stat sticks, and the horde band two reused melee items. The artifact records this rather than padding with unverifiable KB names.
- **KB level drift was severe this pass** (14→20, 15→21, both wrong ids); the corrections session itself produced three more wrong ids. Every shipped id here is ForeverDB-verified; icons from Wowhead item pages.
- **Master Hunter's Bow quest level**: ForeverDB shows the quest's difficulty level 43; it is the classic level-30 Hemet Nesingwary chain reward with no item required level — shipped at 30 with this note (same pattern as Whirlwind Axe).
- **Starter band:** "starter bow" + "Quest leather pieces (…)" hints (Elwynn/Durotar), exempt from provenance (deliberate).
- **Icons imported:** `inv_weapon_bow_10`, `inv_weapon_bow_11`, `inv_weapon_bow_01`, `inv_weapon_rifle_07`, `inv_staff_08`, `inv_sword_14`.

## Slot pilot — Alliance hunter armor (Track "UI polish & armor", 2026-10-15)

- **KB session:** `3a979e48` (per-slot armor/jewelry, 12–16 / 20–24 / 24–30) — **ForeverDB-verified** item by item (`foreverdb.net/item/<id>`): existence, armor type, real equipment slot, required level.
- **Shipped slots** (alliance): 12–16 → back/chest/hands/waist/legs/feet (6); 20–24 → + head/neck/shoulders/wrist/finger (11); 24–30 → + trinket (11; no verified neck upgrade ≤30 exists — the 20–24 Medallion keeps serving).
- **Fabrications caught by the fidelity pass** (dropped): "Embossed Leather Bracers", "Embossed Leather Belt", "Blackened Defias Bracers", "Barkhide Shoulders", "River Pride Amulet", "Triprunner Pants", "Deviate Scale Boots" — none exist in the Forever DB. Replacements came from the real families (Wrangler's, Fine/Embossed Leather, Deviate Scale, Dark Leather, Fang set).
- **Wrong-slot/wrong-class catches:** Chausses of Westfall is **mail** (hunters cannot wear mail before 40 in 1.60.1) → dropped; "Band of the Tiger" → the real item is **Tiger Band** (quest: Willix the Importer); "Sentinel's Medallion"/"Caretaker's Cape" have **req-58 variants** (19538/19530) — the **req-18** versions (20444/20428) shipped.
- **Icons:** 24 new item icons imported (list in `assets/manifest.json`, source lines name each item).
