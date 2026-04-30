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

The `README.md` has been rewritten so every language has the same shape: a short description, a `Prerequisites` bullet list linking to the toolchain, a `How to compile and run` (or `How to run`) section with fenced `sh` code blocks, and the expected output. Brainfuck's expected output is documented as the two-line string `Hello World!` / `Kiro is doing this`, matching the verified output of `hello.bf` on `main`.

## Discarded (off-topic, not hello-world)

These branches are not hello-world exercises and are not being merged. They are deleted by the cleanup script after this PR lands:

- `feat/tic-tac-toe-game`: a JavaScript/HTML tic-tac-toe game. Not a hello-world program.
- `feat/tic-tac-toe-rust`: a Rust tic-tac-toe CLI game. Not a hello-world program.
- `feat/print-env-script`: a `print_env.sh` environment-dump script. Not a hello-world program.
- `add-read-json-script`: a `read_json.py` JSON reader. Not a hello-world program.

If the user wants any of these preserved, they should be moved to their own repositories before running the cleanup script.

### Source branches (fully absorbed by this PR)

The following branches are the source-of-truth branches whose content has been fully pulled into this PR. They will also be deleted by the cleanup script because their content now lives on `main`:

- `add-dhasur-to-readme`: supplied `main.rs`, `hello.asm`, `Makefile`, and `HelloWorld.java`; all four files are now on `main` via this PR.
- `add-go-hello-world`: supplied `hello.go`; now on `main` via this PR.
- `add-python-hello`: supplied `hello.py` (with the `def main()` + `__main__` guard); now on `main` via this PR.
- `add-perl-hello-world`: supplied `hello.pl` (with `use strict;` / `use warnings;`); now on `main` via this PR.

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

After this PR is merged into `main`, run the bundled cleanup script locally to delete every non-main branch from the GitHub remote:

```sh
./scripts/delete-stale-branches.sh
```

If your GitHub remote is named something other than `origin`, pass its name as the first argument:

```sh
./scripts/delete-stale-branches.sh my-remote
```

The script prints a warning, verifies that the remote's default branch is `main` (and aborts with a clear error if it isn't), prompts for `DELETE` confirmation, and only then issues `git push "<remote>" --delete <branch>` for each of the 25 stale branches listed above, which includes the 4 off-topic branches above. It never deletes `main`.

The arithmetic is: 4 off-topic + 4 source + 17 superseded = 25.

Important: `add-dhasur-to-readme` is currently the GitHub default branch for this repository. GitHub refuses to delete the default branch, so before running the cleanup script the user must change the default branch to `main` in the repository's GitHub settings (Settings > Branches > Default branch). The script performs this check itself and refuses to run until the default is `main`.
