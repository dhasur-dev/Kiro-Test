# Hello World in Many Languages

A small collection of "Hello, World!" programs, one per language. Each file lives at the repo root and can be run or built with the command shown beside it.

## Programs

| Language | File | Description | How to run |
| --- | --- | --- | --- |
| Brainfuck | `hello.bf` | Prints `Hello World!` followed by a second line `Kiro is doing this`. | Requires a Brainfuck interpreter, e.g. `bf hello.bf` |
| Python | `hello.py` | Uses a `main()` function with the `if __name__ == "__main__"` guard. | `python3 hello.py` |
| Rust | `hello.rs` | Uses `println!` from the Rust standard library. | `rustc hello.rs -o hello_rs && ./hello_rs` |
| Go | `hello.go` | Uses `fmt.Println` from the standard library. | `go run hello.go` |
| Java | `HelloWorld.java` | Standard `public class HelloWorld` with a `static main`. | `javac HelloWorld.java && java HelloWorld` |
| x86-64 Assembly (NASM, Linux) | `hello.asm` | Uses the Linux `write` and `exit` syscalls directly. | `make hello && ./hello` |
| Perl | `hello.pl` | Uses `use strict;` and `use warnings;`. | `perl hello.pl` |

## Building the assembly target

The `Makefile` at the repo root builds only the assembly program:

```sh
make hello     # assemble with nasm + link with ld
make clean     # remove hello and hello.o
```

Prerequisites: `nasm`, `ld` (from binutils), and a 64-bit Linux environment.

## Notes

- Every runnable hello world prints `Hello, World!`, except Rust (which prints the idiomatic `Hello, world!` with a lower-case `w`) and Brainfuck (which prints `Hello World!` — no comma, capital `W` — followed by a second line: `Kiro is doing this`).
- Files are deliberately independent so each language can be demonstrated on its own without any shared project configuration.
