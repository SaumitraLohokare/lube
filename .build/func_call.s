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


.global _main
.align 2
_main:
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    add x29, sp, #16
    mov w8, #2
    str w8, [sp, #12]
    mov w8, #65535
    movk w8, #65535, lsl #16
    str w8, [sp, #8]
    ldr w0, [sp, #12]
    ldr w1, [sp, #8]
    bl _add
    b label_1
label_1:
    ldp x29, x30, [sp, #16]
    add sp, sp, #32
    ret


