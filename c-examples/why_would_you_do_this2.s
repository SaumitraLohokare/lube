	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 14, 0	sdk_version 14, 4
	.globl	_why_would_you_do_this          ; -- Begin function why_would_you_do_this
	.p2align	2
_why_would_you_do_this:                 ; @why_would_you_do_this
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	ldr	w9, [sp, #48]
	ldr	w8, [sp, #52]
	str	w0, [sp, #44]
	str	w1, [sp, #40]
	str	w2, [sp, #36]
	str	w3, [sp, #32]
	str	w4, [sp, #28]
	str	w5, [sp, #24]
	str	w6, [sp, #20]
	str	w7, [sp, #16]
	str	w9, [sp, #12]
	str	w8, [sp, #8]
	add	sp, sp, #48
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w0, #0
	str	w0, [sp, #8]                    ; 4-byte Folded Spill
	stur	wzr, [x29, #-4]
	mov	x9, sp
	mov	w8, #8
	str	w8, [x9]
	mov	w8, #9
	str	w8, [x9, #4]
	mov	w1, #1
	mov	w2, #2
	mov	w3, #3
	mov	w4, #4
	mov	w5, #5
	mov	w6, #6
	mov	w7, #7
	bl	_why_would_you_do_this
	ldr	w0, [sp, #8]                    ; 4-byte Folded Reload
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
