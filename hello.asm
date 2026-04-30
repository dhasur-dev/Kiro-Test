; hello.asm - Hello World in x86-64 Linux assembly (NASM syntax)
;
; Build and run:
;   nasm -f elf64 hello.asm -o hello.o
;   ld hello.o -o hello
;   ./hello
;
; Or simply: make hello && ./hello

section .data
    msg db "Hello, World!", 10    ; 10 = newline
    msg_len equ $ - msg

section .text
    global _start

_start:
    ; write(stdout, msg, msg_len)
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; fd: stdout
    mov rsi, msg        ; buffer
    mov rdx, msg_len    ; length
    syscall

    ; exit(0)
    mov rax, 60         ; syscall: exit
    xor rdi, rdi        ; status: 0
    syscall
