	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 14, 0	sdk_version 14, 4
	.globl	_signed_add                     ; -- Begin function signed_add
	.p2align	2
_signed_add:                            ; @signed_add
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	strh	w0, [sp, #14]
	strh	w1, [sp, #12]
	ldrsh	w8, [sp, #14]
	ldrsh	w9, [sp, #12]
	add	w8, w8, w9
	strh	w8, [sp, #10]
	ldrsh	w0, [sp, #10]
	add	sp, sp, #16
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
