.align 2
_dummy:
    b label_0
label_0:
    ret

.align 2
_why_would_you_do_this:
    sub sp, sp, #64
    stp x29, x30, [sp, #48]
    add x29, sp, #16
    str w0, [sp, #44]
    str w1, [sp, #40]
    str w2, [sp, #36]
    str w3, [sp, #32]
    str w4, [sp, #28]
    str w5, [sp, #24]
    str w6, [sp, #20]
    str w7, [sp, #16]
    ldr w9, [sp, #64]
    str w9, [sp, #12]
    ldr w9, [sp, #68]
    str w9, [sp, #8]
    bl _dummy
    b label_1
label_1:
    ldp x29, x30, [sp, #48]
    add sp, sp, #64
    ret

.align 2
_main:
    sub sp, sp, #16
    stp x29, x30, [sp]
    add x29, sp, #16
    mov w0, #0
    mov w1, #1
    mov w2, #2
    mov w3, #3
    mov w4, #4
    mov w5, #5
    mov w6, #6
    mov w7, #7
    mov w9, #8
    mov w8, #9
    str w9, [sp]
    str w8, [sp, #4]
    bl _why_would_you_do_this
    mov w0, #0
    b label_2
label_2:
    ldp x29, x30, [sp]
    add sp, sp, #16
    ret

.data
