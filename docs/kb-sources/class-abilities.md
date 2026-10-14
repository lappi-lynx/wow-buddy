# Class abilities — KB validation

- **Date:** 2026-10-14
- **Notebook:** WoW Forever KB — <https://notebooklm.google.com/notebook/cff6d28f-cdcc-4679-9e98-74cf2430bf2b>
- **Question:** For WoW Forever 1.60.1: validate the following class abilities. For each class, confirm these three are real abilities learnable by that class, and flag any that are wrong or belong to a different class. Also answer: are any of these abilities race- or faction-specific — for example, do Forsaken (undead) paladins get different abilities than human paladins? Classes: warrior — Charge, Heroic Strike, Taunt; paladin — Holy Light, Seal of Righteousness, Blessing of Might; hunter — Auto Shot, Serpent Sting, Tame Beast; rogue — Stealth, Sinister Strike, Eviscerate; priest — Flash Heal, Power Word: Shield, Shadow Word: Pain; shaman — Lightning Bolt, Earthbind Totem, Healing Wave; mage — Fireball, Frost Nova, Conjure Food & Water; warlock — Corruption, Summon Imp, Life Tap; druid — Moonfire, Rejuvenation, Bear Form.
- **Trigger:** reported issue (2026-10-14) — "for undead paladin we show skills from the alliance paladin (human), which is wrong". Recorded per request; this artifact is the durable record of the re-validation.

## Verdict

**All 27 core abilities are valid and correctly class-assigned; none are race- or
faction-specific.** An Undead Paladin learns the exact same core kit as a Human
or Dwarf Paladin — races differ only in Racial Traits (Undead: Will of the
Forsaken, Cannibalism, Touch of the Grave, Underwater Breathing vs Human: Will
to Survive, Sword Specialization, The Human Spirit, Perception).

**Root of the reported confusion:** the *class tips* (not the abilities) carried
a racial clause shown to every race — e.g. the Retribution paladin tip ended
with "Humans (+2% crit with swords) and Dwarves (+1% with maces) gain racial
crit." For an Undead Paladin that reads as Human-specific advice. The advice was
softened to a clearly optional race note in `class_tips.json` (warrior ×2,
paladin, shaman, mage; all three locales) rather than the abilities being
changed.

## Answer (verbatim)

Thoughts
Class Ability Validation (WoW Forever 1.60.1)
All 27 abilities listed across all 9 classes are 100% valid and correctly assigned to their respective classes in WoW Forever
. None of them are incorrect or belong to a different class:
Warrior: Charge, Heroic Strike, Taunt — All Valid (Core Warrior abilities)
2
.
Paladin: Holy Light, Seal of Righteousness, Blessing of Might — All Valid (Core Paladin abilities)
2
3
.
Hunter: Auto Shot, Serpent Sting, Tame Beast — All Valid (Core Hunter abilities)
1
4
.
Rogue: Stealth, Sinister Strike, Eviscerate — All Valid (Core Rogue abilities).
Priest: Flash Heal, Power Word: Shield, Shadow Word: Pain — All Valid (Core Priest abilities)
2
.
Shaman: Lightning Bolt, Earthbind Totem, Healing Wave — All Valid (Core Shaman abilities)
2
.
Mage: Fireball, Frost Nova, Conjure Food & Water — All Valid (Core Mage abilities; learned as Conjure Food and Conjure Water)
1
.
Warlock: Corruption, Summon Imp, Life Tap — All Valid (Core Warlock abilities)
2
.
Druid: Moonfire, Rejuvenation, Bear Form — All Valid (Core Druid abilities)
1
.
Race- and Faction-Specificity
Are any of these abilities race- or faction-specific? No. None of the 27 listed class abilities are race- or faction-specific. They are standard, core class spells taught by class trainers to all characters of that class regardless of race or faction
Do Forsaken (Undead) Paladins get different abilities than Human Paladins? No, their core class abilities are identical. An Undead Paladin learns the exact same core spell kit (Holy Light, Seal of Righteousness, Blessing of Might, etc.) as a Human or Dwarf Paladin
The distinction between races comes purely from Racial Traits, which were overhauled in WoW Forever to feature 2 active and 2 passive abilities for every race
3
:
Undead Paladins possess Undead racials: Will of the Forsaken (charm/fear/sleep break), Cannibalism (restores 35% HP & Mana over 10s), Touch of the Grave (chance on attack/spell to drain health), and Underwater Breathing
3
7
.
Human Paladins possess Human racials: Will to Survive (stun break), Sword Specialization (+2% crit with swords for all physical and spell casts), The Human Spirit (+5% Spirit), and Perception
5
8
.
✨ Would you like to explore the full list of revised racial abilities for another race or check out class talent changes in WoW Forever?

Sources:
[1] WoW Forever Beta Development Notes – Updated October 8 - #5 by Kaivax - WoW: Forever Beta Discussion - World of Warcraft Forums — "Classes Developers' notes: There have been several bugs in the calculation of Rage gained from taking damage, and the bugs are fixed in this build. The typical Warrior or Druid should be gaining more Rage from taking damage. Fixed a bug wh…"
[2] WoW Forever Beta Development Notes – Updated October 8 - #5 by Kaivax - WoW: Forever Beta Discussion - World of Warcraft Forums — "Arcane Missiles: The base points and points per level of ranks 1-3 of this spell have been adjusted to make the growth rate of ranks 1-4 more consistent, and new ranks always an upgrade. Paladin Fixed an issue with Retribution Aura that ca…"
[3] Обзор всех расовых способностей в WoW Forever — "всем привет в этом видео предлагаю разобрать все расовые способности в World of Warcraft Forever потому что в сравнению с ванилой Blizzard довольно серьёзно их переработали что-то бафнули что-то понерфили какие-то старые способности вообще…"
[4] TAME THESE NOW! Best Hunter Pets Level 1-30 World of Warcraft — "i'm going to take you from level one all the way up to level 30 on the best pets you can have at each level when you should be looking out for new ranks and all that time codes are available you can skip to the level range that you're in l…"
[5] Обзор всех расовых способностей в WoW Forever — "всем привет в этом видео предлагаю разобрать все расовые способности в World of Warcraft Forever потому что в сравнению с ванилой Blizzard довольно серьёзно их переработали что-то бафнули что-то понерфили какие-то старые способности вообще…"
[7] Обзор всех расовых способностей в WoW Forever — "всем привет в этом видео предлагаю разобрать все расовые способности в World of Warcraft Forever потому что в сравнению с ванилой Blizzard довольно серьёзно их переработали что-то бафнули что-то понерфили какие-то старые способности вообще…"
[8] Обзор всех расовых способностей в WoW Forever — "всем привет в этом видео предлагаю разобрать все расовые способности в World of Warcraft Forever потому что в сравнению с ванилой Blizzard довольно серьёзно их переработали что-то бафнули что-то понерфили какие-то старые способности вообще…"
- **Note:** captured 2026-10-14 during the ability re-validation triggered by the undead-paladin report; `docs/wowhead-sources/` covers the icons side separately.
