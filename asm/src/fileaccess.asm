
;%
%include "defines/fcntl.h.inc"

section .data
	msg		db		"nice",0x0a,0
	msglen	equ	$ - msg
	filename db		"out/fileaccess",0

section .text
	global _start

_start:
	; Open the file
	mov rax, 2			; sys_open
	mov rdi, filename
	mov rsi, O_CREAT | O_WRONLY
	mov rdx,	777o
	syscall
	mov r12, rax

	; Check if open was successful
	cmp rax, 0
	js  _error			; if rax < 0, an error occurred

	mov rax, 1			; sys_write
	mov rdi, r12
	mov rsi, msg
	mov rdx, msglen
	syscall

	; Close the file
	mov rax, 3
	mov rdi, r12
	syscall

	; Exit program
	mov rax, 60
	xor rdi, rdi
	syscall

_error:
	mov rax, 60
	mov rdi, 1
	syscall
