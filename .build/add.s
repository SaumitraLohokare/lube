.global _add
.align 2
_add:
    sub sp, sp, #16
    str w0, [sp, #12]
    str w1, [sp, #8]
    ldr w8, [sp, #12]
    ldr w9, [sp, #8]
    add w0, w8, w9
    b label_0
label_0:
    add sp, sp, #16
    ret

.data
