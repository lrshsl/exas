
%include "src/utils.inc"

%define		p1		'x'
%define		p2		'o'

; place(position, player_symbol)
%macro		place 2
%endmacro

section .rodata
	field_indices	db	2,2+4,2+8, 2+25,2+25+4,2+25+8, 2+50,2+50+4,2+50+8
	nl			db		0xa
	players	db		"xo"

section .data
	cur_p		db		0

	t1			db		"Player 0's turn",0xa
	t1len		equ	$ - t1

	t2			db		"Input: 0",0xa
	t2len		equ	$ - t2

	field		db		"    |   |  ",0xa," ---|---|---",0xa,"    |   |  ",0xa," ---|---|---",0xa,"    |   |  ",0xa,0xa
	fieldlen	equ	$ - field


section .bss
	buf		resb	4


section .text
	global _start

_start:
	xor		r14,			r14

_nextturn:
	; change player
	xor		r14,			1

	; prompt + input
	mov		al,				byte [players + r14]
	mov		byte [t1 + 7],	al
	print		t1, t1len
	read		1, buf, 2

	; check
	movzx		r12,			byte [buf]
	cmp		r12,			'9'
	jg			_error
	cmp		r12,			'1'
	jl			_error
	sub		r12,			'1'

	; place
	mov		rdi,			r12
	call		_place

	; print field
	print		field,		fieldlen
	jmp		_nextturn

	mov		al,			[buf]
	mov		[t2 + 7],	al
	print		t2,			t2len

	exit		0

_error:
	exit		9


; _place
;		rdi	index
;		r14	player symbol

; messed up afterwards: rax,rbx
_place:
	movzx			rax,						byte [field_indices + rdi]
	mov			rbx,						[players + r14]
	mov			byte [field + rax],	bl
	ret


