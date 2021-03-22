    .section .text.entry
    .globl _start
_start:
    li t1, 32768
    mul t1, a0, t1
    la sp, boot_stack_top
    sub sp, sp, t1
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 64
    .globl boot_stack_top
boot_stack_top: