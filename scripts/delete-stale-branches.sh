#!/usr/bin/env bash
set -euo pipefail

cat <<'WARN'
WARNING: This script will delete EVERY non-main branch from the 'origin' remote.
It is intended to be run ONCE, after the consolidation PR has been merged into main.

If 'main' is NOT currently the default branch on GitHub, change the default branch
to 'main' first (Settings > Branches > Default branch). GitHub refuses to delete
the current default branch, and the delete of that branch will fail otherwise.

This action is DESTRUCTIVE and cannot be undone from this script.
WARN

printf 'Type DELETE to continue: '
read -r confirmation

if [ "${confirmation}" != "DELETE" ]; then
  echo "Aborted: confirmation was not 'DELETE'."
  exit 1
fi

git push origin --delete add-dhasur-readme || echo "skipped add-dhasur-readme"
git push origin --delete add-dhasur-to-readme || echo "skipped add-dhasur-to-readme"
git push origin --delete add-dhasur-to-readme-v2 || echo "skipped add-dhasur-to-readme-v2"
git push origin --delete add-go-hello-world || echo "skipped add-go-hello-world"
git push origin --delete add-perl-hello-world || echo "skipped add-perl-hello-world"
git push origin --delete add-python-hello || echo "skipped add-python-hello"
git push origin --delete add-read-json-script || echo "skipped add-read-json-script"
git push origin --delete feat/add-brainfuck-hello-world || echo "skipped feat/add-brainfuck-hello-world"
git push origin --delete feat/add-hello-world-python || echo "skipped feat/add-hello-world-python"
git push origin --delete feat/add-rust-hello-world || echo "skipped feat/add-rust-hello-world"
git push origin --delete feat/asm-hello-world || echo "skipped feat/asm-hello-world"
git push origin --delete feat/asm-hello-world-squashed || echo "skipped feat/asm-hello-world-squashed"
git push origin --delete feat/brainfuck-hello || echo "skipped feat/brainfuck-hello"
git push origin --delete feat/brainfuck-hello-world || echo "skipped feat/brainfuck-hello-world"
git push origin --delete feat/brainfuck-hello-world-v2 || echo "skipped feat/brainfuck-hello-world-v2"
git push origin --delete feat/hello-world-python || echo "skipped feat/hello-world-python"
git push origin --delete feat/print-env-script || echo "skipped feat/print-env-script"
git push origin --delete feat/python-hello-world || echo "skipped feat/python-hello-world"
git push origin --delete feat/rust-hello-world || echo "skipped feat/rust-hello-world"
git push origin --delete feat/tic-tac-toe-game || echo "skipped feat/tic-tac-toe-game"
git push origin --delete feat/tic-tac-toe-rust || echo "skipped feat/tic-tac-toe-rust"
git push origin --delete feature/java-hello-world || echo "skipped feature/java-hello-world"
git push origin --delete feature/java-hello-world-squashed || echo "skipped feature/java-hello-world-squashed"
git push origin --delete hello-world-python || echo "skipped hello-world-python"
git push origin --delete update-readme || echo "skipped update-readme"

echo
echo "Done. If any delete was skipped for 'add-dhasur-to-readme' (the previous default branch),"
echo "change the default branch on GitHub to 'main' (Settings > Branches > Default branch) and re-run this script."
