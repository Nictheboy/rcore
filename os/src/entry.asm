   .section .text.entry
   .globl _start
_start:
   la sp, boot_stack       # Set the stack pointer
   call main

   .section .bss.stack
boot_stack:
   .space 4096 * 16        # 64KB, 16 pages
