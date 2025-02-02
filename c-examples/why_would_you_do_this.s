	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 14, 0	sdk_version 14, 4
	.globl	_why_would_you_do_this          ; -- Begin function why_would_you_do_this
	.p2align	2
_why_would_you_do_this:                 ; @why_would_you_do_this
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	ldr	x9, [sp, #64]
	ldrsh	w8, [sp, #72]
	strh	w0, [sp, #62]
	str	w1, [sp, #56]
	str	x2, [sp, #48]
	strh	w3, [sp, #46]
	str	w4, [sp, #40]
	str	x5, [sp, #32]
	strh	w6, [sp, #30]
	str	w7, [sp, #24]
	str	x9, [sp, #16]
	strh	w8, [sp, #14]
	add	sp, sp, #64
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
