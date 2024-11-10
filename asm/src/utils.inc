
%macro	exit	1
; exit(exit-code)

	mov	rax,	60
%if %1 == 0
	xor	rdi,	rdi
%else
	mov	rdi,	%1
%endif
	syscall

%endmacro


; read(fd, char*, len)
%macro	read	3

	mov	rax,	0
	mov	rdi,	%1
	mov	rsi,	%2
	mov	rdx,	%3
	syscall

%endmacro


; write(fd, char*, len)
%macro	write	3

	mov	rax,	1
	mov	rdi,	%1
	mov	rsi,	%2
	mov	rdx,	%3
	syscall

%endmacro


; print(char*, len)
%macro	print 2

	write	1,	%1, %2

%endmacro


%macro	mksyscall	1

	mov	rax,	%1
	syscall

%endmacro

%macro	fopen	3

	mov	rdi,	%1
	mov	rsi,	%2
	mov	rdx,	%3
	mksyscall	2

%endmacro

; vim: ft=asm
