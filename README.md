# Hello World Programs

A collection of "Hello, World!" programs in different languages.

## Rust

A simple Rust program that prints "Hello, world!" to the console.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system

### How to compile and run

Compile the program:

```sh
rustc main.rs
```

Run the compiled binary:

```sh
./main
```

You should see the following output:

```
Hello, world!
```

## Assembly (x86-64 Linux, NASM)

A hello world program written in x86-64 assembly using NASM syntax. It uses Linux syscalls to write to stdout and exit.

### Prerequisites

- [NASM](https://www.nasm.us/) assembler
- Linux x86-64 environment
- `ld` linker (part of GNU binutils)
- (Optional) `make`

### How to build and run

Using make:

```sh
make hello && ./hello
```

Or manually:

```sh
nasm -f elf64 hello.asm -o hello.o
ld hello.o -o hello
./hello
```

To clean build artifacts:

```sh
make clean
```

You should see the following output:

```
Hello, World!
```

## Brainfuck

A hello world program written in Brainfuck, an esoteric programming language that uses only eight commands.

### Prerequisites

- A Brainfuck interpreter (e.g., [beef](https://github.com/andrewarchi/beef), `bf`, or any other interpreter)

### How to run

Using a Brainfuck interpreter:

```sh
bf hello.bf
```

You should see the following output:

```
Hello World!
```
