
;%include "sys/types.h"
;%include "sys/stat.h"

section .data
    filename db "file.txt", 0

section .text
    global _start

_start:
    ; Open the file
    mov rax, 2                    ; sys_open
    mov rdi, filename
    mov rsi, 0                    ; O_RDONLY
    syscall
    mov rbx, rax                  ; save file descriptor in rbx

    ; Check if open was successful
    cmp rax, 0
    js  _error                    ; if rax < 0, an error occurred

    ; File operations would go here (read, write, etc.)

    ; Close the file
    mov rax, 3
    mov rdi, rbx
    syscall

    ; Exit program
    mov rax, 60
    xor rdi, rdi
    syscall

_error:
    mov rax, 60
    mov rdi, 1
    syscall
