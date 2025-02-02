.global _signed_add
.align 2
_signed_add:
    sub sp, sp, #16
    strh w0, [sp, #14]
    strh w1, [sp, #12]
    ldrsh w8, [sp, #14]
    ldrsh w10, [sp, #12]
    add w9, w8, w10
    strh w9, [sp, #10]
    ldrsh w8, [sp, #10]
    mov w0, w8
    b label_0
label_0:
    add sp, sp, #16
    ret

