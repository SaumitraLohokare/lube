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

.align 2
_main:
    sub sp, sp, #16
    stp x29, x30, [sp]
    add x29, sp, #16
    mov w12, #0
    mov w9, #1
    mov w8, #2
    mov w14, #3
    mov w15, #4
    mov w11, #5
    mov w10, #6
    mov w13, #7
    mov w19, #8
    mov w20, #9
    mov w0, w12
    mov w1, w9
    mov w2, w8
    mov w3, w14
    mov w4, w15
    mov w5, w11
    mov w6, w10
    mov w7, w13
    str w19, [sp]
    str w20, [sp, #4]
    bl _why_would_you_do_this
    mov w8, #0
    mov w0, w8
    b label_1
label_1:
    ldp x29, x30, [sp]
    add sp, sp, #16
    ret

