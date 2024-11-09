
%macro	exit	1
; exit(exit-code)

	mov	rax,	60
	mov	rdi,	%1
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

