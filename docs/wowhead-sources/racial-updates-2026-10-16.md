# Wowhead Forever — October 2026 beta development notes: racial update (verification)

Pull: **2026-10-25**, browser session (Wowhead blocks plain HTTP scrapers).
Article: <https://www.wowhead.com/forever/news/massive-racial-updates-wow-forever-beta-development-notes-383454>
Primary source: Blizzard forum post #6, *WoW Forever Beta Development Notes – Updated October 15*:
<https://us.forums.blizzard.com/en/wow/t/2360696/6> (created `2026-10-16T00:15Z`, updated
`2026-10-16T20:50Z`). Wowhead posted it as “9 days ago” on the pull date.

This is the October 15/16 beta build notes. The beta has since closed (the app's Beta markers are a
separate review); no later racial patch notes were posted in this thread as of the pull date.

## Verbatim racial section

> Racial abilities have been updated.
>
> **Tauren**
> Endurance: Now grants 2% increased critical damage and healing (was 1% hit chance).
> Plainsrunning: Also grants 3% increased mounted movement speed. This increase stacks with other mounted movement speed increases.
> §Plainsrunning now requires the Tauren to move a minimum distance per second to keep the effect.
> Developers’ notes: Stutter-stepping is no longer sufficient to maintain the effect. This minimum does make some allowance for movement speed penalties.
> §Cultivation: Cooldown reduced to 15 minutes (was 60 minutes).
> §War Stomp: Cooldown reduced to 90 seconds (was 2 minutes).
>
> **Dwarf**
> Mace Specialization: Replaced with Earth Fury: 2% increased critical damage and healing.
> Find Treasure: Now also grants a chance to find additional items in most chests.
>
> **Skyborne**
> Wind Blessed: Haste bonus increased to 3% (was 1%).
> Skysight: Now castable while moving.
> Read Leyline: Now castable while moving. Minimum duration increased to 30 seconds (was 20 seconds).
> Walk on Air: Cooldown reduced to 90 seconds (was 2 minutes).
> Increased the number of spawn locations of Elemental Convergences and Leylines in most Eastern Kingdoms and Kalimdor zones.
>
> **Human**
> Sword Specialization: Replaced with Ruthless: 2% increased critical strike chance.
>
> **Orc**
> Axe Specialization: Replaced with Savage: 1% increased critical strike chance.
> Blood Fury: Cooldown increased to 3 minutes (was 2 minutes).
> Shatter Curse: Now dispels Curses and Banes (was granting immunity). Magic damage taken reduced by 5% (was 15%).
>
> **Undead**
> Touch of the Grave: Proc chance and cooldown adjusted to make benefit more consistent. Improvements made to visuals.
> Undead will no longer have Will of the Forsaken available to them while under the effects of Scatter Shot.
> Touch of the Dead now has a 30% chance to activate for casters and a 20% chance to activate for Paladins/Rogues/Warriors, as well as a 25 second internal cooldown.
> Touch of the Dead no longer scales with Shadow increases on the target or the caster.
> Touch of the Dead no longer suffers from a decreased multiplier when a lower ranked spell is used.
> Developers’ notes: This shouldn’t have been in place because it’s a racial, not a Class spell or ability.
>
> **Gnome**
> Eureka!: Now an 8 second buff with no charges. Same effect on damage and ability costs. Now also benefits Power Word: Shield.
> § Eureka cannot correctly be used while in stealth or invisibility.
>
> **Night Elf**
> Quickness: Movement speed increase now stacks with other movement speed increases.
> § Night Elves no longer display their shields while in Wisp Form.
>
> **Troll**
> Regeneration: Health regeneration bonuses increased to 20% (was 10%).

## What this changed in the app

The race card intentionally displays **one signature racial per race**. The update therefore did not
require editing every race:

| App surface | Old value | New value | Why |
|---|---|---|---|
| Orc race card | Axe Specialization — +1% crit while wielding an axe | **Savage** — 1% increased critical strike chance | The displayed passive was replaced. |
| Skyborne race card | Walk on Air — 2-minute cooldown | Walk on Air — **90-second cooldown** | The displayed cooldown changed. |
| Human / Dwarf race cards | Every Man for Himself / Stoneform | unchanged | The replaced Sword/Mace Specializations are not the displayed ability. |
| Tauren race card | War Stomp — 2s stun | unchanged effect line | The changed cooldown (90s) is not displayed by the card. |
| Undead / Gnome / Night Elf / Troll cards | Will of the Forsaken / Expansive Mind / Shadowmeld / Berserking | unchanged | Those displayed abilities were not changed. |

### Class tips

Five specialization tips recommended the retired weapon-specific crit racials. They now name the flat
replacements, in EN/RU/FI:

- Warrior → Arms and Fury (which two-hander / dual-wield advice)
- Paladin → Retribution
- Shaman → Enhancement
- Mage → Fire

Values used: **Human Ruthless +2% critical strike chance**, **Orc Savage +1% critical strike chance**,
**Dwarf Earth Fury +2% critical damage and healing**.

Unchanged passives (Endurance, Plainsrunning, Cultivation, Find Treasure, Wind Blessed, Skysight, Read
Leyline, Blood Fury, Shatter Curse, Eureka, Quickness, Regeneration, etc.) are not displayed by the
app's one-racial race card, so no UI copy was invented for them. Other patch-note sections (class
talents, pets, quests, UI) were scanned against shipped claims; no other displayed numeric value was
contradicted by this build.
