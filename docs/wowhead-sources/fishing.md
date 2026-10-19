# Wowhead Forever — trainers, vendor and the artisan quest (verification)

Pulls: 2026-10-19, browser (search rows + the quest page), paced.

## NPCs (all found on wowhead.com/forever/search)

Capitals: Lumak `npc=3332` · Armand Cromwell `npc=4573` · Kah Mistrunner `npc=3028` ·
Arnold Leland `npc=5493` · Grimnur Stonebrand `npc=5161` · Astaia `npc=4156`.
Vendor: Old Man Heming `npc=2626`.

Starting zones: Lau'Tiki `npc=5941` · Uthan Stillwater `npc=5938` · Clyde Kellen `npc=5690` ·
Lee Brown `npc=1651` · Paxton Ganter `npc=1700` · Androl Oakhand `npc=3607`.
Skyborne (Forever ids): Baelann Swiftcurrent `npc=257008` · Fenn Fairweather `npc=251992`.

**Nat Pagle's NPC page did not surface in the Forever search** ("Nat Pagle" and "Pagle");
the quest page below is the primary confirmation of the quest giver's role.

## Quest 6607 — "Nat Pagle, Angler Extreme" (on wowhead.com/forever/quest=6607)

- Title confirmed: *Nat Pagle, Angler Extreme - Quest - Forever*.
- Quest level 45, **"Requires level 35"** shown on the page.
- All four quest fish named on the page: Feralas Ahi, Misty Reed Mahi Mahi, Sar'theris
  Striker, Savage Coast Blue Sailfin.
- The **Fishing 225** requirement is not rendered in the page's visible text; it is stated by
  the wow-professions source **and** KB session `e136eda7` — two sources, so the artisan
  section ships `verified: true` while the guide as a whole keeps the Beta marker.

## Item icon spot-check

Savage Coast Blue Sailfin (16969) icon confirmed via its Wowhead item page/search row during
the icon import (recorded in `assets/manifest.json`).
