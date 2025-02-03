.global _variables
.align 2
_variables:
    sub sp, sp, #16
    mov w8, #33
    strb w8, [sp, #15]
    mov w8, #69
    strh w8, [sp, #12]
    mov w8, #64870
    movk w8, #65535, lsl #16
    str w8, [sp, #8]
    mov x8, #5866
    movk x8, #19632, lsl #16
    movk x8, #2, lsl #32
    str x8, [sp]
label_0:
    add sp, sp, #16
    ret


