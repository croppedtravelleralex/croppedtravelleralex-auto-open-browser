# Entry Summary Update Example (2026-04-02)

## Scenario

A future stage changes the active mainline.
Before editing the README entry summary, check:

1. `STATUS.md` mainline line already reflects the new stage
2. `TODO.md` only contains still-live next actions
3. `PROGRESS.md` already records the newly landed milestone
4. deferred freeze lines are still correct

## Example update flow

### Step 1: update source-of-truth surfaces first
- update `STATUS.md`
- update `TODO.md`
- update `PROGRESS.md`

### Step 2: run the entry-summary checklist mentally
- does README still match current stage?
- is anything in README now stale?
- is a frozen line being accidentally reopened by wording drift?

### Step 3: update README snapshot last
Only after the above is aligned.

## Rule in one sentence

> README entry summary is the last surface to update, not the first.
