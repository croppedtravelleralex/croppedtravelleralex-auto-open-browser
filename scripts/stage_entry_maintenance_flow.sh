#!/usr/bin/env bash
set -euo pipefail

python3 scripts/check_stage_entry_consistency.py

echo
echo "Next maintenance order:"
echo "1. Update STATUS.md / TODO.md / PROGRESS.md first"
echo "2. Re-run python3 scripts/check_stage_entry_consistency.py"
echo "3. Update README snapshot last"
echo "4. Re-run python3 scripts/check_stage_entry_consistency.py before commit"
