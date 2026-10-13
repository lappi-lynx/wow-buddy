# Asset attribution

WoW Buddy bundles game artwork and item icons for a free, non-commercial fan
guide. This file records where each asset comes from and the terms it is used
under. Every entry in `assets/manifest.json` carries a `source` and a
`license`.

## Required disclaimer (World of Warcraft Fan Site Kit)

> World of Warcraft® and Blizzard Entertainment® are all trademarks or registered trademarks of Blizzard Entertainment in the United States and/or other countries. These terms and all related materials, logos, and images are copyright © Blizzard Entertainment. This site is in no way associated with or endorsed by Blizzard Entertainment®.

## Sources

| Source | Used for | Terms |
| --- | --- | --- |
| World of Warcraft Fan Site Kit (2004–2007), archived at `johnwhitegm.github.io/wowfansitekit` | faction, race, class and zone artwork | Blizzard Fan Site Kit License — personal, non-commercial, display; notices retained; modification only as needed for a fan site |
| Game client assets extracted with `wow.export` | item icons (`inv_*`), race renders | © Blizzard Entertainment — used for a free, non-commercial fan guide |

## Rules

- **Non-commercial release only.**
- **Never hotlink** third-party hosts (`wow.zamimg.com`, Wowhead screenshot
  stores, etc.); all assets are embedded offline as `data:` URIs.
- Keep the copyright/trademark notices above intact.
- Blizzard may revoke fan use at any time; every asset is swappable through
  `assets/manifest.json` without code changes.
