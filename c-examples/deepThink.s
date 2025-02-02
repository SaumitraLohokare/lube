	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 14, 0	sdk_version 14, 4
	.globl	_deepThink                      ; -- Begin function deepThink
	.p2align	2
_deepThink:                             ; @deepThink
	.cfi_startproc
; %bb.0:
	mov	w0, #42
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
