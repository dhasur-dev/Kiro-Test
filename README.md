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

## Go

A concise Go program that does exactly what it says on the tin.

### Prerequisites

- [Go](https://go.dev/dl/) installed on your system

### How to compile and run

Run directly:

```sh
go run hello.go
```

Or build a binary and execute it:

```sh
go build hello.go
./hello
```

You should see the following output:

```
Hello, World!
```

## Python

A tiny Python script with a proper `main()` function and an `if __name__ == "__main__":` guard, because even hello-world deserves good habits.

### Prerequisites

- [Python 3](https://www.python.org/downloads/) installed on your system

### How to run

```sh
python3 hello.py
```

You should see the following output:

```
Hello, World!
```

## Perl

Perl greets the world with strict and warnings turned on, the way a responsible script should.

### Prerequisites

- [Perl](https://www.perl.org/get.html) installed on your system

### How to run

```sh
perl hello.pl
```

You should see the following output:

```
Hello, World!
```

## Brainfuck

A Brainfuck script that prints a custom greeting using only eight cryptic characters. It is intentionally unreadable, as tradition demands.

### Prerequisites

- A [Brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter such as `bf`, or any online Brainfuck interpreter

### How to run

With a local interpreter (for example `bf`):

```sh
bf hello.bf
```

You should see the following output:

```
Hello World!
Kiro is doing this
```
