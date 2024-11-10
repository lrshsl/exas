
%include "src/utils.inc"

%define		p1		'x'
%define		p2		'o'

%define		nl		print [t1 + t1len - 1], 1

;section .rodata

section .data
	t1			db		"Player 1's turn",0x0a
	t1len		equ	$ - t1
	t2			db		"Input: 0",0x0a
	t2len		equ	$ - t2

	field		db		"   |  | ",0xa," --|--|-- ",0xa,"   |  | ",0xa," --|--|-- ",0xa,"   |  | ",0xa
	fieldlen	equ	$ - field


section .bss
	buf		resb	1


section .text
	global _start

_start:
	print		t1,			t1len
	print		field,		fieldlen
	read		1, buf, 2

	inc word	[t1 + 7]
	print		t1,			t1len
	print		field,		fieldlen
	read		1, buf, 2

	mov		al,			[buf]
	mov		[t2 + 7],	al
	print		t2,			t2len

	exit	0


