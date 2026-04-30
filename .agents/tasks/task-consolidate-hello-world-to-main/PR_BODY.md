# Consolidate Hello World programs onto main

## Summary

This PR consolidates every hello-world-in-a-language variant scattered across sibling branches onto `main` so that `main` becomes the single source of truth for this repository. After this PR merges, all seven hello-world programs (Rust, Assembly, Java, Go, Python, Perl, Brainfuck) live at the top level of `main`, documented uniformly in a single `README.md`.

This PR also ships `scripts/delete-stale-branches.sh`, a helper the user runs locally after merge to clean up the 25 now-redundant branches on `origin`.

## Consolidated

Each language and the source branch it was taken from:

| Language | Source branch |
| --- | --- |
| Rust (`main.rs`) | `add-dhasur-to-readme` |
| Assembly, x86-64 NASM (`hello.asm`, `Makefile`) | `add-dhasur-to-readme` |
| Java (`HelloWorld.java`) | `add-dhasur-to-readme` |
| Go (`hello.go`) | `add-go-hello-world` |
| Python (`hello.py`) | `add-python-hello` (chosen for the idiomatic `def main()` + `if __name__ == "__main__":` guard) |
| Perl (`hello.pl`) | `add-perl-hello-world` |
| Brainfuck (`hello.bf`) | already on `main` (from PR #6) |

The `README.md` has been rewritten so every language has the same shape: a short description, a `Prerequisites` bullet list linking to the toolchain, a `How to compile and run` (or `How to run`) section with fenced `sh` code blocks, and the expected output. Brainfuck's expected output is documented as `Hello World! Kiro is doing this`, matching the actual output of `hello.bf` on `main`.

## Discarded (off-topic, not hello-world)

These branches are not hello-world exercises and are not being merged. They can be deleted after this PR lands:

- `feat/tic-tac-toe-game`: a JavaScript/HTML tic-tac-toe game. Not a hello-world program.
- `feat/tic-tac-toe-rust`: a Rust tic-tac-toe CLI game. Not a hello-world program.
- `feat/print-env-script`: a `print_env.sh` environment-dump script. Not a hello-world program.
- `add-read-json-script`: a `read_json.py` JSON reader. Not a hello-world program.

If the user wants any of these preserved, they should be moved to their own repositories before running the cleanup script.

## Duplicate branches superseded by this PR

The following branches contain earlier, partial, or redundant versions of content that is now fully represented on `main` via this PR. They are safe to delete after merge:

- `add-dhasur-readme` (3-line README stub)
- `add-dhasur-to-readme-v2` (3-line README stub)
- `update-readme` (older README without the Java section)
- `feat/add-hello-world-python` (print-only Python variant)
- `feat/hello-world-python` (`hello_world.py` variant; we kept the flat `hello.py` filename)
- `feat/python-hello-world` (print-only Python variant)
- `hello-world-python` (print-only Python variant)
- `feat/brainfuck-hello` (same Brainfuck content as `main`; already merged via PR #6)
- `feat/brainfuck-hello-world` (partial Brainfuck plus unrelated files)
- `feat/brainfuck-hello-world-v2` (partial Brainfuck)
- `feat/add-brainfuck-hello-world` (partial Brainfuck)
- `feat/asm-hello-world` (superseded by the squashed variant, already merged)
- `feat/asm-hello-world-squashed` (already merged via PR #9)
- `feat/rust-hello-world` (already merged via PR #5)
- `feat/add-rust-hello-world` (`hello.rs` variant, superseded by `main.rs`)
- `feature/java-hello-world` (superseded by the squashed variant, already merged)
- `feature/java-hello-world-squashed` (already merged via PR #13 into `add-dhasur-to-readme`)

## Post-merge cleanup

After this PR is merged into `main`, run the bundled cleanup script locally to delete every non-main branch from `origin`:

```sh
./scripts/delete-stale-branches.sh
```

The script prints a warning, prompts for `DELETE` confirmation, and only then issues `git push origin --delete <branch>` for each of the 25 stale branches listed above (plus the discarded off-topic branches). It never deletes `main`.

Important: `add-dhasur-to-readme` is currently the GitHub default branch for this repository. GitHub will refuse to delete the default branch, so before running the cleanup script the user must change the default branch to `main` in the repository's GitHub settings (Settings > Branches > Default branch). Once `main` is the default, the script will succeed in removing every other branch.
