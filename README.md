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


## Java

A classic Java "Hello, World!" program — famously verbose for such a simple task.

### Prerequisites

- [Java JDK](https://www.oracle.com/java/technologies/downloads/) installed on your system

### How to compile and run

Compile the program:

```sh
javac HelloWorld.java
```

Run the compiled class:

```sh
java HelloWorld
```

You should see the following output:

```
Hello, World!
```
