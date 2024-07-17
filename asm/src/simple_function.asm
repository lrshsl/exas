; Simple function call


section .text
global simple_function

add:
	push rbp
	mov rbp, rsp
	mov rax, rdi
	add rax, rsi
	leave
	ret
