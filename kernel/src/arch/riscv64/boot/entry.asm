   .section .text.entry
   .globl _start
_start:
   la sp, boot_stack       # Set the stack pointer
   call main

   .section .bss.stack
boot_stack:
   .space 4096 * 16        # 64KB, 16 pages

   .global start_boot_heap
start_boot_heap:
   .space 4096 * 16        # 64KB, 16 pages
   .global end_boot_heap
end_boot_heap:
