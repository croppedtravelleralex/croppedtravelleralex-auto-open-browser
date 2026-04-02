# Dual-Entry Snapshot Maintenance Example (2026-04-02)

## Scenario

A future stage changes the active project mainline.
Both `README.md` and `AI.md` contain Current Stage Snapshot blocks, so both must stay aligned.

## Correct order

1. update `STATUS.md`
2. update `TODO.md`
3. update `PROGRESS.md`
4. run:

```bash
python3 scripts/check_stage_entry_consistency.py
```

5. update `README.md` Current Stage Snapshot
6. update `AI.md` Current Stage Snapshot
7. rerun:

```bash
python3 scripts/check_stage_entry_consistency.py
```

8. commit only after the script passes again

## Rule

> README and AI.md are twin stage-entry surfaces; neither should drift ahead of the other.
