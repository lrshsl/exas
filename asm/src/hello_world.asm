; Hello world in x86_64 assembly (nasm)

%include "sys/types.h"
%include "sys/stat.h"
%include "fcntl.h"
%include "unistd.h"

section .data

msg: dw "Hello, World!", 10
msg_len: equ $ - msg

section .text
global _start

_start:
	mov rax, 2
	mov rdi, msg 
	mov rsi, O_CREAT
	mov rdx, S_IRWXU
	syscall

	jmp exit

exit:
	mov rax, 60
	xor rdi, rdi
	syscall
