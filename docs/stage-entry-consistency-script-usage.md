# Stage Entry Consistency Script Usage (2026-04-02)

## Command

```bash
python3 scripts/check_stage_entry_consistency.py
```

## When to run it

- before updating the README current stage snapshot
- after changing STATUS / TODO / PROGRESS for a new stage
- before committing entry-summary-related maintenance

## Expected result

If the current control surface is aligned, the script ends with:

```text
Stage entry consistency: PASS
```

## Maintenance flow

1. update source-of-truth surfaces first (`STATUS.md`, `TODO.md`, `PROGRESS.md`)
2. run the consistency script
3. only then update README entry summary if needed
4. rerun the script before commit

## Rule in one sentence

> Entry summary updates are validated by script, not by memory.
