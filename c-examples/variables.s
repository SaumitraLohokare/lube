	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 14, 0	sdk_version 14, 4
	.globl	_variables                      ; -- Begin function variables
	.p2align	2
_variables:                             ; @variables
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	mov	w8, #33
	strb	w8, [sp, #15]
	mov	w8, #69
	strh	w8, [sp, #12]
	mov	w8, #-666
	str	w8, [sp, #8]
	mov	x8, #5866
	movk	x8, #19632, lsl #16
	movk	x8, #2, lsl #32
	str	x8, [sp]
	add	sp, sp, #16
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
