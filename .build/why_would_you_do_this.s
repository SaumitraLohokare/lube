.align 2
_why_would_you_do_this:
    sub sp, sp, #48
    str w0, [sp, #44]
    str w1, [sp, #40]
    str w2, [sp, #36]
    str w3, [sp, #32]
    str w4, [sp, #28]
    str w5, [sp, #24]
    str w6, [sp, #20]
    str w7, [sp, #16]
    ldr w9, [sp, #48]
    str w9, [sp, #12]
    ldr w9, [sp, #52]
    str w9, [sp, #8]
    b label_0
label_0:
    add sp, sp, #48
    ret

