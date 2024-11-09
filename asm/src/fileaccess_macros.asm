
%include "defines/fcntl.h.inc"
%include "src/utils.asm"

section .data
	msg		db		"42",0x0a,0
	msglen	equ	$ - msg
	filename db		"file.out",0

section .text
	global _start

_start:
	; Open the file
	mov	rax,	2			; sys_open
	mov	rdi,	filename
	mov	rsi,	O_CREAT | O_WRONLY
	mov	rdx,	666o
	syscall
	mov	r12,	rax

	; Check if open was successful
	cmp	rax,	0
	js		_exit_error		; if rax < 0, an error occurred

	write	r12, msg, msglen

	; Close the file
	mov	rax,	3
	mov	rdi,	r12
	syscall

	; Exit program
_exit:
	exit	0

_exit_error:
	exit	1

; vim: et! ts=3 sts=3 sw=3
