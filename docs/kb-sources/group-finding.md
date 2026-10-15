# Group finding — KB source

- **Date:** 2026-10-15
- **KB session:** `542bc6e7` — <https://notebooklm.google.com/notebook/cff6d28f-cdcc-4679-9e98-74cf2430bf2b>
- **Used for:** `assets/data/readiness.json` → `group_finding` (`verified: true`, provenance `kb:542bc6e7@2026-10-15`).

## Question (verbatim)

> In WoW Forever 1.60.1, how do players form dungeon groups? Is there an in-game group finder / looking-for-group tool (what is it called and how is it opened), or do players use the world chat channel, /who, or a Discord? Describe exactly what exists in 1.60.1 — if the group finder is not implemented in the beta, say that plainly.

## Answer (verbatim)

> In WoW Forever 1.60.1, an in-game Group Finder tool is fully implemented in the beta. **Manual Premade Listing System:** There is no automated matchmaking queue or automatic teleportation tool. Instead, the game features a manual premade Group Finder board (similar to modern Retail's Premade Group Finder). **Posting & Browsing:** Group Leaders can post an open listing specifying the dungeon, quest line, raid, or PvP activity. Listings support custom filters and tags (such as beginner-friendly, casual, or speedrun focus). Solo Players can browse active listings or post themselves onto the board to be invited by party leaders. **Voice Chat Integration:** Under the UI settings, group leaders can set the Group Finder posting to prompt joining members to join an in-game voice channel or connect directly via Discord Voice. **Manual World Travel:** Once the party is formed, players are not teleported to the dungeon. Developers intentionally omitted meeting stone summons and automatic teleportation, requiring all group members to walk or travel manually to the dungeon entrance.

## What shipped

- The card text: listings with tags, solo players can post themselves, **no auto-queue and no teleport — the party walks to the entrance** (all three locales).
- `verified: true` + the provenance string; the card's "Beta, may change" tag renders only when `!verified` (kept for future unverified facts).
- Corroboration: ForeverDB's site navigation lists a "Group finder" tool (the KB's first source line points at the same ForeverDB nav), matching an implemented system rather than a plan.
