.global _main
.align 2
_main:
    sub sp, sp, #16
    stp x29, x30, [sp]
    add x29, sp, #16
    adrp x0, data_0@PAGE
    add x0, x0, data_0@PAGEOFF
    bl _printf
    mov w0, #0
    b label_0
label_0:
    ldp x29, x30, [sp]
    add sp, sp, #16
    ret

.data
data_0:
    .asciz "Hello, World!\n"
