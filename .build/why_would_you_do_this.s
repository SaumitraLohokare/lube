.align 2
_why_would_you_do_this:
    sub sp, sp, #64
    strh w0, [sp, #62]
    str w1, [sp, #56]
    str x2, [sp, #48]
    strh w3, [sp, #46]
    str w4, [sp, #40]
    str x5, [sp, #32]
    strh w6, [sp, #30]
    str w7, [sp, #24]
    ldr x9, [sp, #64]
    str x9, [sp, #16]
    ldrsh w9, [sp, #72]
    strh w9, [sp, #14]
    b label_0
label_0:
    add sp, sp, #64
    ret

