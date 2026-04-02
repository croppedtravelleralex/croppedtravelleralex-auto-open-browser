# Stage Entry Maintenance Example for a Future Stage (2026-04-02)

## Example scenario

A future stage decides to reopen a previously frozen line because its reopen condition is explicitly met.

## Correct maintenance order

1. update `STATUS.md` to reflect the new active mainline
2. update `TODO.md` so only still-live next actions remain
3. update `PROGRESS.md` only with actually landed milestones
4. run:

```bash
python3 scripts/check_stage_entry_consistency.py
```

5. update README current stage snapshot last
6. rerun the script before commit

## What not to do

- do not update README first
- do not reopen a frozen line just because it feels like the next logical thing
- do not let TODO keep already-landed items after the stage switches

## One-sentence rule

> Stage entry maintenance is a controlled sequence, not a free-form doc edit.
