# Warrior gear path 1–30 — KB + DB sources (Track C completion)

- **Date:** 2026-10-15
- **Notebook:** WoW Forever KB — <https://notebooklm.google.com/notebook/cff6d28f-cdcc-4679-9e98-74cf2430bf2b>
- **KB sessions:** `8208cb61` (Alliance 24–30 list + 20–24 second option), `941d4fbe` (Horde 24–30 list), `29cfce97` (item-by-item corrections).
- **Purpose:** close the warrior gaps — the 20–24 band had one weapon (needs ≥2) and the 24–30 band was a "planning" placeholder (alliance) / weaponless (horde). This completes the reference class for Track C.
- **Cross-check surfaces:** **ForeverDB** `https://foreverdb.net/item/<id>` fetched with curl (no WAF; client-datamined, "in the WoW Forever beta client" + where-from lines) and **Wowhead** `/forever/item=<id>` item pages in a browser session (paced — rapid navigation hits the WAF; used for the icon names and required levels). Wowhead and ForeverDB agreed on every field they both state.

## Fidelity cross-check (KB candidate → verdict → shipped data)

| KB candidate | ForeverDB (`/item/<id>`) | Wowhead (`/forever/item=<id>`) | Verdict | Shipped |
| --- | --- | --- | --- | --- |
| Strike of the Hydra (KB: "Gelihast, BFD", 26) | 6909 — Rare **Two-Hand Sword**, ilvl 31, **req 26**, **drops from Aku'mai (boss)**, 20%, "Forever changed it from Classic Era" | req 26 ✓, icon `inv_sword_42` | Real; **boss drift: Aku'mai, not Gelihast** (DB wins) | ✅ 26, alliance + horde |
| Archeus (KB: id 1204, 28, Duskwood) | id 1204 = **"The Green Tower"** (Epic shield!) — wrong id. Real Archeus = **item 2000**, Uncommon 2H sword, **quest reward from "A Daughter's Love" (level 35)** | — | **Dropped: wrong id AND a level-35 quest (outside ≤30)** | ❌ |
| Whirlwind Axe (KB: 30, class quest) | 6975 — Rare Two-Hand Axe, **no required level**, "Quest reward from Whirlwind Weapon (level 40), choice of rewards (Classic data)" | no "Requires Level" ✓, icon `inv_axe_12` | Real; "(level 40)" is the **quest's difficulty level** — the classic level-30 warrior reward, unchanged ("Classic data"); note kept in the source line | ✅ 30, alliance + horde |
| Rod of the Sleepwalker (KB suggestion) | 1155 — Two-Hand **Staff**, Int/Spirit | — | **Dropped: caster staff** (poor warrior pick; same rule the Horde batch used for Crescent Staff / Living Root) | ❌ |
| Staff of Westfall (KB suggestion) | 2042 — Two-Hand **Staff**, +Spirit/Int | — | **Dropped: caster staff** | ❌ |
| Cruel Barb (KB: alliance 20–24 second option) | 5191 — Rare One-Hand Sword, ilvl 24, **req 19**, Edwin VanCleef, Deadmines, 20% | req 19 ✓, icon `inv_sword_24` | Real (the KB's own bracket assignment; see note) | ✅ 19, alliance 20–24 |
| Butcher's Cleaver (KB: id 6224, SFK) | **Item not found** | — | **Dropped: does not exist** (horde 24–30 needs no replacement — three verified weapons) | ❌ |
| Corpsemaker (KB list: 29; corrections said 30) | 6687 — Rare Two-Hand Axe, ilvl 34, **req 29**, Overlord Ramtusk, Razorfen Kraul, 40% | req 29 ✓, icon `inv_weapon_halbard_01` | Real; **level 29** (DB wins over the corrections answer's 30) | ✅ 29, horde |
| — | — | — | `Taskmaster Axe 5196` (offered alternative) not needed — Cruel Barb shipped | ❌ |

### Notes
- **Cruel Barb (req 19) sits in the 20–24 band** per the KB guide's bracket assignment; the `level` field keeps the verified required level (19). The band is a display grouping, not a level gate (17–19 is a deliberate gap).
- **Whirlwind Weapon quest:** ForeverDB shows the quest's level as 40 (difficulty), the item has no required level, and the reward context is "(Classic data)" — i.e., the classic level-30 warrior chain; KB sessions independently said level 30. Shipped at 30 with this record.
- **Armor lines:** alliance 24–30 = Blackfathom Deeps (24–32, Ashenvale); horde 24–30 = Blackfathom Deeps + Shadowfang Keep (18–25). Both are the ≤30 dungeon pools from the KB's leveling-weapons guide's faction routing.
- **Provenance strings in `gear.json`:** weapons `kb:leveling-weapons@2026-10-15`, armor `kb:dungeons@2026-10-15`.
- **Icons:** `inv_sword_42`, `inv_sword_24`, `inv_axe_12`, `inv_weapon_halbard_01` fetched from the `wow.zamimg.com` CDN, converted to 64 px WebP, declared in `assets/manifest.json`, embedded via `tools/assets/generate.mjs`.
- The horde batch's own artifact (`docs/kb-sources/gear-warrior-horde.md`) remains the reference for the 12–16 and 20–24 horde fills.

## Slots — warrior (2026-10-15)

- Mail/plate(40) like the paladin, so the **paladin's verified Blacksmithing set is reused wholesale** with the warrior insignias (Alliance 18854, Horde 18834 — verified) at 24–30.
- Availability: A 7/11/11 · H 7/11/11.


## Alignment fix (2026-10-16)

Three pre-Track-C weapon levels were realigned to ForeverDB's client data
(`/item/<id>`, verified 2026-10-16):

- Heavy Copper Maul (6214): level 12 → **11** (requires level 11, per ForeverDB).
- Smite's Mighty Hammer (7230): level 20 → **18** (requires level 18; also the
  Deadmines codex loot row, cross-consistent).
- Ironforge Greathammer (279895): level 14 → **1** (ForeverDB lists no level
  requirement — new in Forever, usable from level 1).
