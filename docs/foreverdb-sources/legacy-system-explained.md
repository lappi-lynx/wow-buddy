# WoW Forever Legacy system — ForeverDB source

- **Date fetched:** 2026-10-14
- **URL:** <https://foreverdb.net/article/legacy-system-explained>
- **Article date:** 2026-10-03; states it uses the Forever beta **client build 1.60.1.70205** ("the client's numbers for every rank")
- **Method:** fetched with curl (no WAF on foreverdb.net); tables extracted from the page HTML. This is the source the KB's Legacy excerpts pointed at.
- **Why:** the partner-approved query for the full Legacy perk list (design `plans/2026-10-14-ux-content-packs-design.md` §8 Q3).

## Key facts (client-verified per the article)

- **16-point seasonal cap** per character; **65 challenges**, 1 point each; most points one character can earn alone: 29.
- **3 trees — Professions, Adventure, Resourcefulness**; **21 perks, 69 ranks** (23 per tree); **1 Legacy Point per rank**; 6 locked future slots (2 per tree).
- **Unlock:** first Legacy Point — for most players when the **first character reaches level 25** (or 150 non-gathering tradeskill, or full map exploration). Opened with the shield icon / Y.
- Legacy Track rewards at levels 15 / 25 / 40 / 55 (track to 90). Talent-point track: with Talented 5/5, talents start at level 5 (see table 2 in the article).

## Perks at max rank (all 21)

| Tree | Perk | Ranks | At max rank (client) | Needs |
| --- | --- | --- | --- | --- |
| Professions | Master Chef | 5 | Cooking recipes: 50% chance for an extra result | 5 pts in Professions first |
| Professions | Working Overtime | 5 | +20% chance to gain a skill increase from any tradeskill use | nothing |
| Professions | Bartering | 2 | Vendor prices −10% | 5 pts in Professions first |
| Professions | Dedicated Study | 1 | +1 skill in your lowest tradeskill (special cases at 300/300) | — |
| Professions | Bountiful Harvest | 5 | Discover 100% more Scarce materials (Mining/Herbalism/Skinning) | nothing |
| Professions | Performance Bonus | 3 | 15% chance for 100% more Merchant's Favor on crate turn-ins | 5 pts in Professions first |
| Professions | Luremaster | 2 | With a Lure active: 50% chance to catch an extra fish | 5 pts in Professions first |
| Adventure | High Alert | 2 | Detect stealthed targets as if +2 levels (not in BGs) | 5 pts in Adventure first |
| Adventure | Well Rested | 5 | Rested XP accumulates 20% faster; rested cap +20% | nothing |
| Adventure | Talented | 5 | Talent points every level from level 5 (still max 51) | 5 pts in Adventure + Well Rested 5/5 |
| Adventure | Thrill of Adventure | 5 | Killing blow: +5% max Health/Mana over 10 s (not in dungeons/raids/BGs) | nothing |
| Adventure | Field Guide | 3 | Camp-feature cooldown −25% | 5 pts in Adventure first |
| Adventure | Frequent Flier | 1 | Flight paths −50% cost, +20% flight speed | 10 pts in Adventure + Field Guide 3/3 |
| Adventure | Field Medicine | 2 | Recently Bandaged duration −10 s (not in dungeons/raids/BGs) | 5 pts in Adventure first |
| Resourcefulness | For Great Honor | 5 | +10% Honor Points | 5 pts in Resourcefulness first |
| Resourcefulness | Gourmand | 3 | Food buff durations +100% | nothing |
| Resourcefulness | Permanence | 2 | Party/raid long-duration buffs +100% duration; camp rest benefits +100% | — |
| Resourcefulness | The Quick and the Dead | 2 | Dead movement +10%; after a rez your helpful spells cost nothing for 2 min | nothing |
| Resourcefulness | Reagent Economy | 1 | Class abilities need no vendor reagents | 10 pts in Resourcefulness + Quick and the Dead 2/2 |
| Resourcefulness | Reinforce | 5 | 40% less durability loss on death | nothing |
| Resourcefulness | Diplomat | 5 | +10% reputation gains | 5 pts in Resourcefulness first |

## FINDING — our current Legacy preset data does not match the client

`src/models/legacy.rs` ships a "16-point starter preset" of **Swift Stride (6), Cozy Camp (5), Trophy Collector (5)** — none of these three perks appear in the client catalog (or in any KB text that was not simply echoing our own names back to us). They are placeholder inventions from the original MVP plan.

Replacement (partner-approved direction, verified here): ship **one real 16-point leveling build** built from client perks with prerequisite chains satisfied:

- **Well Rested 5/5** (no prereq) — 5 pts
- **Thrill of Adventure 5/5** (no prereq) — 5 pts
- **Talented 5/5** (5 pts in Adventure ✓ + Well Rested 5/5 ✓) — 5 pts
- **Field Guide 1/3** (5 pts in Adventure ✓) — 1 pt
- = **16/16**. (Explorer / Coin Saver alternates stay Track E once costs are client-verified — they now are, via this table.)
