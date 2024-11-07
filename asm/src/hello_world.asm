
;%include "sys/types.h"
;%include "sys/stat.h"

section .data
	msg		db		"heyy from asm",0xa
	msglen	equ	$ - msg

section .text
	global _start
	extern printf

_start:
	; asm print
	mov rax, 1
	mov rdi, 1
	mov rsi, msg
	mov rdx, msglen
	syscall

	; printf
	mov rdi, msg
	xor rax, rax
	call printf

	jmp _exit

_exit:
	mov rax, 60
	xor rdi, rdi
	syscall

_exit_error:
	mov rax, 60
	mov rdi, 1
	syscall

