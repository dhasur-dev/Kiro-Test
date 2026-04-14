hello: hello.asm
	nasm -f elf64 hello.asm -o hello.o
	ld hello.o -o hello

clean:
	rm -f hello hello.o

.PHONY: clean
