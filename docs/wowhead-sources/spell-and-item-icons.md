# Spell and item icons — Wowhead Forever sources

- **Date:** 2026-10-13
- **Method:** pages loaded in a browser session (AWS WAF blocks plain HTTP
  scrapers); icon names extracted from the rendered DOM and downloaded from
  the `wow.zamimg.com/images/wow/icons/large/` CDN (56px, converted to 64px WebP).
- **Used for:** `assets/data/content.json` core-spell `icon_id`s and
  `assets/data/gear.json` weapon `item_id`/`icon_id`s.

## Spells (per-class abilities pages)

Source pattern: `https://www.wowhead.com/forever/guide/classes/<class>/<spec>/<role>-abilities`
(one page per class was enough; hunter additionally used the Auto Shot spell page).

| Class | Spell | Wowhead id | Page | Icon |
| --- | --- | --- | --- | --- |
| warrior | Charge | 11578 | warrior/arms/dps-abilities | ability_warrior_charge |
| warrior | Heroic Strike | 25286 | warrior/arms/dps-abilities | ability_rogue_ambush |
| warrior | Taunt | 355 | warrior/arms/dps-abilities | spell_nature_reincarnation |
| paladin | Holy Light | 10329 | paladin/holy/healer-abilities | spell_holy_holybolt |
| paladin | Seal of Righteousness | 20293 | paladin/holy/healer-abilities | ability_thunderbolt |
| paladin | Blessing of Might | 25291 | paladin/holy/healer-abilities | spell_holy_fistofjustice |
| hunter | Auto Shot | 75 | spell=75/auto-shot | ability_whirlwind |
| hunter | Serpent Sting | 25295 | hunter/beast-mastery/dps-abilities | ability_hunter_quickshot |
| hunter | Tame Beast | 1515 | hunter/beast-mastery/dps-abilities | ability_hunter_beasttaming |
| rogue | Stealth | 1784 | rogue/assassination/dps-abilities | ability_stealth |
| rogue | Sinister Strike | 1752 | rogue/assassination/dps-abilities | spell_shadow_ritualofsacrifice |
| rogue | Eviscerate | 2098 | rogue/assassination/dps-abilities | ability_rogue_eviscerate |
| priest | Flash Heal | 10917 | priest/discipline/healer-abilities | spell_holy_flashheal |
| priest | Power Word: Shield | 10901 | priest/discipline/healer-abilities | spell_holy_powerwordshield |
| priest | Shadow Word: Pain | 10894 | priest/discipline/healer-abilities | spell_shadow_shadowwordpain |
| shaman | Lightning Bolt | 15208 | shaman/elemental/dps-abilities | spell_nature_lightning |
| shaman | Earthbind Totem | 2484 | shaman/elemental/dps-abilities | spell_nature_strengthofearthtotem02 |
| shaman | Healing Wave | 25357 | shaman/elemental/dps-abilities | spell_nature_magicimmunity |
| mage | Fireball | 25306 | mage/arcane/dps-abilities | spell_fire_flamebolt |
| mage | Frost Nova | 10230 | mage/arcane/dps-abilities | spell_frost_frostnova |
| mage | Conjure Food & Water | 28612 | mage/arcane/dps-abilities | inv_misc_food_73cinnamonroll |
| warlock | Corruption | 25311 | warlock/affliction/dps-abilities | spell_shadow_abominationexplosion |
| warlock | Summon Imp | 688 | warlock/affliction/dps-abilities | spell_shadow_summonimp |
| warlock | Life Tap | 11689 | warlock/affliction/dps-abilities | spell_shadow_burningspirit |
| druid | Moonfire | 9835 | druid/balance/dps-abilities | spell_nature_starfall |
| druid | Rejuvenation | 25299 | druid/balance/dps-abilities | spell_nature_rejuvenation |
| druid | Bear Form | 5487 | druid/balance/dps-abilities | ability_racial_bearform |

## Items (gear-guide weapons)

Item pages: `https://www.wowhead.com/forever/item=<id>/<slug>` (found via
`/forever/search?q=<name>`); icon = the page's main item icon.

| Weapon | Wowhead id | Icon |
| --- | --- | --- |
| Heavy Copper Maul | 6214 | inv_hammer_18 |
| Ironforge Greathammer | 279895 | inv_hammer_21 |
| Rhahk'Zor's Hammer | 5187 | inv_hammer_09 |
| Smite's Mighty Hammer | 7230 | inv_hammer_01 (imported earlier) |

## Cross-check 2026-10-14 — Wowhead Forever abilities DB + KB (ID-level)

**Trigger:** user request — re-validate every mapped spell before launch ("make sure we don't bring error at this stage").

**Method (this pass):** each class's database page `https://www.wowhead.com/forever/spells/abilities/<class>` was read in a browser session (WAF); every row for our spell names was extracted with its rank IDs and its icon (inline `background-image` on the row's `ins` element).

**Result: all 27 mappings verified, zero errors.**
- Every spell appears on its class's abilities page (`missing: []` for all 9 classes).
- Every icon matches the mapping exactly, including the non-obvious classic ones: Charge `ability_warrior_charge`, Heroic Strike `ability_rogue_ambush`, Taunt `spell_nature_reincarnation`, Auto Shot `ability_whirlwind`, Sinister Strike `spell_shadow_ritualofsacrifice`, Healing Wave `spell_nature_magicimmunity`, Moonfire `spell_nature_starfall`, Bear Form `ability_racial_bearform`.
- Rank-ID notes (no action needed): our recorded IDs are specific rank variants (e.g. Holy Light `10329` = Rank 9, verified via its spell page; the abilities page slice is capped at 100 rows). Mage `Conjure Food` has two rank icon variants (`inv_misc_food_10`, `inv_misc_food_73cinnamonroll`); our mapping uses the latter, a valid rank icon.
- KB annotation (no action): the KB reads ID `28612` as *Conjure Water Rank 7* while both Wowhead pages list it under *Conjure Food* — our combined "Conjure Food & Water" entry covers either reading; the icon is unaffected.

**KB second pass (MCP, fresh session, 2026-10-14):** the full 27 spell-ID/name/class triples were submitted; the KB confirmed every ID+name corresponds to the listed class — "no misassigned classes or incorrect IDs in the set" (ranks noted per spell, e.g. Charge Rank 3, Taunt base rank).

