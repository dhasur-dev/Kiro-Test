#!/usr/bin/env bash
set -euo pipefail

remote="${1:-origin}"

cat <<WARN
WARNING: This script will delete EVERY non-main branch from the '${remote}' remote.
It is intended to be run ONCE, after the consolidation PR has been merged into main.

If 'main' is NOT currently the default branch on GitHub, change the default branch
to 'main' first (Settings > Branches > Default branch). GitHub refuses to delete
the current default branch.

This action is DESTRUCTIVE and cannot be undone from this script.
WARN

# Pre-flight: verify the remote's default branch is 'main' before prompting for
# confirmation, so the user is not asked to confirm a run that will partially fail.
default_ref="$(git ls-remote --symref "${remote}" HEAD | awk '/^ref:/ {print $2; exit}')"
if [ -z "${default_ref}" ]; then
  echo "ERROR: Could not determine the default branch on remote '${remote}'." >&2
  echo "Check that '${remote}' exists and is reachable: git remote -v" >&2
  exit 2
fi

default_branch="${default_ref#refs/heads/}"
if [ "${default_branch}" != "main" ]; then
  echo "ERROR: The default branch on remote '${remote}' is '${default_branch}', not 'main'." >&2
  echo "GitHub refuses to delete the default branch, so this script would fail on that branch." >&2
  echo "Change the default branch to 'main' in GitHub Settings > Branches > Default branch," >&2
  echo "then re-run this script." >&2
  exit 3
fi

printf 'Type DELETE to continue: '
read -r confirmation

if [ "${confirmation}" != "DELETE" ]; then
  echo "Aborted: confirmation was not 'DELETE'."
  exit 1
fi

git push "$remote" --delete add-dhasur-readme || echo "skipped add-dhasur-readme"
git push "$remote" --delete add-dhasur-to-readme || echo "skipped add-dhasur-to-readme"
git push "$remote" --delete add-dhasur-to-readme-v2 || echo "skipped add-dhasur-to-readme-v2"
git push "$remote" --delete add-go-hello-world || echo "skipped add-go-hello-world"
git push "$remote" --delete add-perl-hello-world || echo "skipped add-perl-hello-world"
git push "$remote" --delete add-python-hello || echo "skipped add-python-hello"
git push "$remote" --delete add-read-json-script || echo "skipped add-read-json-script"
git push "$remote" --delete feat/add-brainfuck-hello-world || echo "skipped feat/add-brainfuck-hello-world"
git push "$remote" --delete feat/add-hello-world-python || echo "skipped feat/add-hello-world-python"
git push "$remote" --delete feat/add-rust-hello-world || echo "skipped feat/add-rust-hello-world"
git push "$remote" --delete feat/asm-hello-world || echo "skipped feat/asm-hello-world"
git push "$remote" --delete feat/asm-hello-world-squashed || echo "skipped feat/asm-hello-world-squashed"
git push "$remote" --delete feat/brainfuck-hello || echo "skipped feat/brainfuck-hello"
git push "$remote" --delete feat/brainfuck-hello-world || echo "skipped feat/brainfuck-hello-world"
git push "$remote" --delete feat/brainfuck-hello-world-v2 || echo "skipped feat/brainfuck-hello-world-v2"
git push "$remote" --delete feat/hello-world-python || echo "skipped feat/hello-world-python"
git push "$remote" --delete feat/print-env-script || echo "skipped feat/print-env-script"
git push "$remote" --delete feat/python-hello-world || echo "skipped feat/python-hello-world"
git push "$remote" --delete feat/rust-hello-world || echo "skipped feat/rust-hello-world"
git push "$remote" --delete feat/tic-tac-toe-game || echo "skipped feat/tic-tac-toe-game"
git push "$remote" --delete feat/tic-tac-toe-rust || echo "skipped feat/tic-tac-toe-rust"
git push "$remote" --delete feature/java-hello-world || echo "skipped feature/java-hello-world"
git push "$remote" --delete feature/java-hello-world-squashed || echo "skipped feature/java-hello-world-squashed"
git push "$remote" --delete hello-world-python || echo "skipped hello-world-python"
git push "$remote" --delete update-readme || echo "skipped update-readme"

echo
echo "Done."
