# Hunter pets — KB source

- **Date:** 2026-10-14
- **Notebook:** WoW Forever KB — <https://notebooklm.google.com/notebook/cff6d28f-cdcc-4679-9e98-74cf2430bf2b>
- **Used for:** `assets/data/pets.json` (basics → `kb_pets` source id, notable pets, warlock minions, companion card) and the conflicts recorded in `docs/wowhead-sources/pets-families.json`.
- **Fresh sessions for the shipped data:** `9f2a4341` (families/basics/upkeep), `9a7c9a9c` (notable pets/minions/companions/name).

## Confirmation table (fresh sessions, every value the UI ships)

| Claim | Result | Note |
| --- | --- | --- |
| Tame Beast learned at level 10, via the racial hunter trainer's quest chain | Confirm | Unlocks Tame Beast, Call Pet, Dismiss Pet, Feed Pet, Revive Pet, pet trainer, stable master, Beast Training Manual |
| Feed Pet effect | Confirm (behavior) | "The client tooltip and buff icon state a duration of 15 seconds" (session `9f2a4341`); the rendered Wowhead tooltip shows no duration; the UI claims none |
| Happiness multipliers | Confirm | Happy 125% / Content 100% / Unhappy 75%; neglected pets bleed loyalty and can abandon you |
| Loyalty levels | Confirm | Six: Rebellious, Unruly, Submissive, Dependable, Faithful, Best Friend |
| Mend Pet | Confirm | Level 12 from the hunter trainer |
| Stable slots | Confirm | 3 max (1 active + 2 stabled); slots cost 5 silver then 5 gold — the UI states only "silver at first, then gold" |
| Notable pets 10–30 | Confirm | The Rake (Mulgore, rare, 10), Savannah Patriarch (The Barrens, 15), Greater Flesh Ripper (Westfall, 16–19), Giant Grizzled Bear (Silverpine, 12–13), Black Bear Patriarch (16), Leech Widow / Giant Moss Creeper (24), Cloud Serpent (Thousand Needles, 25) |
| Ability ranks taught | Confirm | Screech 1 (Flesh Ripper), Swipe 1 at 12 (bears), Claw 3 at 16 (Black Bear Patriarch), Bite 4 (level-24 spiders), Lightning Breath 3 (Cloud Serpent) — ranks are artifact-only; the UI says "a stronger rank" |
| Attack speeds | Confirm | The Rake 1.2, Savannah Patriarch 1.3 — artifact-only; the UI says "unusually fast attacks" |
| Warlock demons | Confirm | Imp 1 (ranged fire, Blood Pact), Voidwalker 10 (tank: Torment/Suffering/Sacrifice), Succubus 20 (melee burst, Seduction), Felhunter 30 (Spell Lock, Devour Magic, Paranoia) |
| Companion pets for every class | Confirm | City vendors (cats/owls/snakes/rabbits/parrots), quests (Ancona Chicken, Westfall), Engineering (Mechanical Squirrel), rare world drops/chests |
| **Fishing yields companion pets** | **No — corrected** | Fishing awards fish, gems, recipes and locked trunks; no vanity pets. The UI does not mention fishing |
| Family name for owls | **Conflict (KB: `Owl`)** | The Wowhead Forever table says `Bird of Prey` with an `UpdatedRenamed` beta marker; all three KB sessions read the client DB as `Owl`. The DB ships — evidence (live client table + changelog marker) over consensus; see the `conflicts` block in `docs/wowhead-sources/pets-families.json` |
| Ferocity/Tenacity/Cunning system | **Conflict (KB: does not exist)** | The Forever DB `/forever/hunter-pets` table has a Type column with exactly those values; the DB ships. Same pointer as above |

## Session `9f2a4341` — answer (verbatim)

> (a) Tameable Pet Families, Spec System, Diets, and Exact Count
> Role/Spec System: The modern Ferocity / Tenacity / Cunning pet specialization system does not exist in WoW Forever 1.60.1. In 1.60.1, "Ferocity" refers strictly to a Beast Mastery class talent that increases pet critical strike chance. Pets use the classic Vanilla pet family system with distinct base stats, armor/health multipliers, diets, and family-specific abilities.
> Exact Count: There are 17 tameable pet families: Bat: Fruit, Fungus; Bear: Meat, Fish, Fruit, Fungus, Bread, Cheese (Omnivore); Boar: Meat, Fish, Fruit, Fungus, Bread, Cheese (Omnivore); Carrion Bird: Meat, Fish; Cat: Meat, Fish; Crab: Fish, Bread, Fungus; Crocolisk: Meat, Fish; Gorilla: Fruit, Fungus; Hyena: Meat, Fruit; Owl: Meat; Raptor: Meat; Scorpid: Meat; Spider: Meat; Tallstrider: Fruit, Fungus, Cheese; Turtle: Fruit, Fungus; Wind Serpent: Bread, Cheese, Fish; Wolf: Meat.
> (b) Tame Beast Level, Source & Unlocks — Level Learned: Level 10. Source: Unlocked by completing the multi-part Level 10 Hunter class quest chain given by racial Hunter Trainers in starting zones/hubs. Unlocks: Tame Beast, Call Pet, Dismiss Pet, Feed Pet, and Revive Pet; access to the Pet Trainer; the Beast Training Manual (General tab, press P); Stable Masters.
> (c) Feed Pet Behavior & Duration — Casting Feed Pet applies a periodic happiness restoration buff over time; feeding items back-to-back wastes food — let the full effect run its course. Duration: the client tooltip and buff icon state a duration of 15 seconds.
> (d) Happiness States & Damage Multipliers — Unhappy (Red): 75% base damage, constantly loses loyalty and will eventually abandon you permanently if neglected. Content (Yellow): 100%. Happy (Green): 125% and earns loyalty at the maximum rate.
> (e) Loyalty Levels & Names — 6 Loyalty Levels: Rebellious, Unruly, Submissive, Dependable, Faithful, Best Friend.
> (f) Mend Pet — Level 12 from your Hunter Trainer.
> (g) Stable Slots & Costs — 3 pets maximum (1 active + 2 stabled). First stable slot: 5 silver. Second: 5 gold.

## Session `9a7c9a9c` — answer (verbatim)

> (a) Recommended Hunter Pets (Levels 10–30): CONFIRMED — The Rake (Mulgore, Level 10 Rare): top early-game Cat. Savannah Patriarch (The Barrens, Level 15): accessible Cat alternative. Greater Flesh Ripper (Westfall, Level 16–19): teaches Screech Rank 1. Giant Grizzled Bear (Silverpine Forest, Level 12–13): teaches Swipe Rank 1. Black Bear Patriarch (Level 16): teaches Claw Rank 3. Leech Widow (Wetlands, Level 24) & Giant Moss Creeper (Hillsbrad Foothills, Level 24): teach Bite Rank 4. Cloud Serpent (Thousand Needles, Level 25): teaches Lightning Breath Rank 3.
> (b) Attack Speeds: CONFIRMED — The Rake 1.2; Savannah Patriarch 1.3.
> (c) Warlock Demon Unlock Levels & Roles: CONFIRMED — Imp (Level 1): automatic at creation; ranged Fire damage, party Stamina via Blood Pact. Voidwalker (Level 10 quest): frontline tanking, Torment/Suffering/Sacrifice. Succubus (Level 20 quest): single-target Shadow burst (Lash of Pain), Seduction. Felhunter (Level 30 quest): anti-caster utility (Spell Lock, Devour Magic, Paranoia).
> (d) Companion / Vanity Pets: CONFIRMED — Can fishing yield companion pets? No. Real methods: city vendors (Cats in Elwynn, Owls in Darnassus, Snakes in Orgrimmar, Rabbits in Dun Morogh, Parrots in Booty Bay); quests (Ancona Chicken from Westfall); crafting (Mechanical Squirrel, Engineering); rare world drops and open-world chests.
> (e) Database Family Name — In the WoW Forever 1.60.1 client database, the pet family is officially listed as Owl (not 'Bird of Prey').

## Cross-checks

- The rendered Wowhead `/forever/hunter-pets` table (browser, WAF blocks plain HTTP) is the source of the shipped families, types and diets; its raw rows and the KB-conflict resolution live in `docs/wowhead-sources/pets-families.json`.
- The Wowhead `/forever/spell=6991/feed-pet` tooltip states no duration; recorded in the same artifact's `spell_checks`.
