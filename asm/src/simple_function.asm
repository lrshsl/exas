; Simple function call

section .text
	global _start

_start:
	mov rdi, 4
	mov rsi, 5
	call add

	mov rdi, rax
	mov rax, 0x3c
	syscall

add:
	push rbp
	mov rbp, rsp

	mov rax, rdi
	add rax, rsi

	leave
	ret
