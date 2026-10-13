# KB source artifacts

For every class in `assets/data/class_tips.json`, this directory holds the **raw
NotebookLM KB answer** that class's tips were authored from.

- **Filename:** `<class>.md` — the class id, e.g. `druid.md`.
- **Contents:** the date, the exact question asked, and the verbatim answer
  (including its citation markers).
- **Not embedded in the app.** It lives in the repo only, so the shipped dataset
  stays lean. It exists so a claim can be **checked**, not merely attributed —
  each spec's `provenance` in the JSON points here.

## Why this exists

The class-tip tests verify **shape** (three locales non-empty, provenance
present, coverage counted) — they cannot verify **truth**. A plausible-but-wrong
tip passes every test we have. Keeping the source next to the claim lets a
reviewer (or the Plan 5 KB sync) diff the shipped text against the text it came
from.

## Workflow

1. Query the KB — **fresh session per query** (a reused session can fail on a UI
   overlay timeout).
2. Save the verbatim answer here as `<class>.md`.
3. Author `assets/data/class_tips.json` from it — never from memory.
4. **Fidelity check:** every concrete claim (item, level, percentage, mechanic)
   must trace back to this file. Drop or soften anything that does not; if the
   source is silent on a spec, leave that spec empty.

`tests/class_tips_data_test.rs::test_every_class_has_a_kb_source_artifact`
enforces the presence of these files.
