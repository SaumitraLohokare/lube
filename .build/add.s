.global _add
.align 2
_add:
    sub sp, sp, #16
    str w0, [sp, #12]
    str w1, [sp, #8]
    ldr w9, [sp, #12]
    ldr w10, [sp, #8]
    add w8, w9, w10
    mov w0, w8
    b label_0
label_0:
    add sp, sp, #16
    ret

