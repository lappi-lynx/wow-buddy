# Leveling milestones — KB source

- **Date:** 2026-10-14
- **Notebook:** WoW Forever KB — <https://notebooklm.google.com/notebook/cff6d28f-cdcc-4679-9e98-74cf2430bf2b>
- **Question:** For WoW Forever 1.60.1, confirm these leveling milestones with levels/ranges, and say 'not known' where the beta data is silent: (1) At what level does a brand-new character first learn abilities from their class trainer? (2) At what level do talents become available in the base game (without Legacy perks)? (3) What is the first dungeon a new character can run and its level range? (4) Around what level does group content (dungeons) become a good way to level?
- **Used for:** `src/models/milestones.rs` (the "What unlocks around here" card under the gear slider).

## Corrections to my draft rows

- Trainer abilities start at **level 2**, not 4 → `ms.trainer` = 2 (verified).
- Group leveling becomes viable at **13–15**, not 20 → `ms.group_leveling` = 15 (verified).
- Talents base = 10 ✓; first dungeons = The Hall of Thanes (A, 13–20) / Ragefire Chasm (H, 13–18) ✓ (both verified). Legacy at 25 already client-verified (`docs/foreverdb-sources/legacy-system-explained.md`).

## Answer (verbatim)

Thoughts
Here are the confirmed leveling milestones for WoW Forever 1.60.1:
1. First Class Trainer Abilities (Level 2): Brand-new characters first begin learning new spell ranks and abilities from their class trainer starting at Level 2
1
2
.
2. Base Game Talent Availability (Level 10): Without Legacy perks, character talent trees open and begin awarding talent points at Level 10
3
. (Note: The "Early Talent Access" Legacy perk can unlock your first talent point at level 5, but level 10 remains the base game standard
3
.)
3. First Dungeons & Level Ranges (Levels 13–20): The earliest dungeons a new character can enter are The Hall of Thanes for Alliance characters in Ironforge (level 13–20) and Ragefire Chasm for Horde characters in Orgrimmar (level 13–18)
4. Dungeon Group Leveling Viability (Levels 13–15+): Group dungeon content becomes a viable way to gain experience starting around Level 13–15, aligned with the entry brackets of The Hall of Thanes (13–20), Ragefire Chasm (13–18), Ruins of Lordaeron (15–22), and Wailing Caverns (15–22)
. Following recent 1.60.1 updates, developers increased dungeon creature kill XP by ~20% and fixed party member level-scaling bugs so that well-organized dungeon groups running dungeon quests achieve leveling speeds on par with optimal open-world questing
8
9
.
🗺️ Would you like to examine the dungeon quest chains for The Hall of Thanes or look into the best open-world questing routes in the level 10–20 bracket?

Sources:
[1] TAME THESE NOW! Best Hunter Pets Level 1-30 World of Warcraft — "i'm going to take you from level one all the way up to level 30 on the best pets you can have at each level when you should be looking out for new ranks and all that time codes are available you can skip to the level range that you're in l…"
[2] WoW Forever Guides & News - Warcraft Tavern — "Recent Guides Blacksmithing Overview: Weapons, Armor & Bonuses Class Trainer Locations Razorfen Downs Dungeon: Map, Location & Bosses Razorfen Downs Quests Fishing Overview: Equipment, Lures & Bonuses Razorfen Kraul Dungeon: Map, Location…"
[3] TAME THESE NOW! Best Hunter Pets Level 1-30 World of Warcraft — "i'm going to take you from level one all the way up to level 30 on the best pets you can have at each level when you should be looking out for new ranks and all that time codes are available you can skip to the level range that you're in l…"
[4] ForeverDB - World of Warcraft: Forever database & tools — "i'm going to take you from level one all the way up to level 30 on the best pets you can have at each level when you should be looking out for new ranks and all that time codes are available you can skip to the level range that you're in l…"
[8] WoW Forever Beta Development Notes – Updated October 8 - #5 by Kaivax - WoW: Forever Beta Discussion - World of Warcraft Forums — "Changes and Updates Now available for testing: the new City of Dalaran dungeon. Players will once again be able to swap multiple weapons within the same GCD, for example via a macro to swap from a two-handed weapon, to a sword and shield.…"
[9] Бафф опыта, нерфы классов, новая PvP-система и другие новости WoW Forever — "всем привет это третий даджест новостей по Wolf Forever сегодня разберём последний подкаст разработчиков про данжи и Пачнота от 8 октября про изменение опыта классов экипировки ещё поговорим о PvP-системе прогрессии новых моделях и в этот…"
