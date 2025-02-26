/*
Basic Cortex-A linker script.

You must supply a file called `memory.x` which defines the memory regions 'CODE' and 'DATA'.

The stack pointer(s) will be (near) the top of the DATA region by default.

Based upon the linker script from https://github.com/rust-embedded/cortex-m
*/

INCLUDE memory.x

ENTRY(_vector_table);
EXTERN(_vector_table);

SECTIONS {
    .text : {
        /* The vector table must come first */
        *(.vector_table)
        /* Our exception handling routines */
        *(.text.handlers)
        /* Now the rest of the code */
        *(.text .text*)
    } > CODE

    .rodata : {
        *(.rodata .rodata*)
    } > CODE

    .data : ALIGN(4) {
        . = ALIGN(4);
        __sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
    } > DATA AT>CODE
    /*
     * Allow sections from user `memory.x` injected using `INSERT AFTER .data` to
     * use the .data loading mechanism by pushing __edata. Note: do not change
     * output region or load region in those user sections!
     */
    . = ALIGN(4);
    __edata = .;

    /* LMA of .data */
    __sidata = LOADADDR(.data);

    .bss (NOLOAD) : ALIGN(4) {
        . = ALIGN(4);
        __sbss = .;
        *(.bss .bss* COMMON)
        . = ALIGN(4);
    } > DATA
    /*
     * Allow sections from user `memory.x` injected using `INSERT AFTER .bss` to
     * use the .bss zeroing mechanism by pushing __ebss. Note: do not change
     * output region or load region in those user sections!
     */
    __ebss = .;

    .uninit (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        __suninit = .;
        *(.uninit .uninit.*);
        . = ALIGN(4);
        __euninit = .;
    } > DATA

    /DISCARD/ : {
        *(.note .note*)
    }
}

/*
We reserve some space at the top of the RAM for our stacks. We have an IRQ stack
and a FIQ stack, plus the remainder is our system stack.

You must keep _stack_top and the stack sizes aligned to eight byte boundaries.
*/
PROVIDE(_stack_top = ORIGIN(DATA) + LENGTH(DATA));
PROVIDE(_fiq_stack_size = 0x400);
PROVIDE(_irq_stack_size = 0x1000);
PROVIDE(_abt_stack_size = 0x400);
PROVIDE(_und_stack_size = 0x400);
PROVIDE(_svc_stack_size = 0x1000);

ASSERT(_stack_top % 8 == 0, "ERROR(cortex-a-rt): top of stack is not 8-byte aligned");
ASSERT(_fiq_stack_size % 8 == 0, "ERROR(cortex-a-rt): size of FIQ stack is not 8-byte aligned");
ASSERT(_irq_stack_size % 8 == 0, "ERROR(cortex-a-rt): size of IRQ stack is not 8-byte aligned");
ASSERT(_fiq_stack_size % 8 == 0, "ERROR(cortex-a-rt): size of FIQ stack is not 8-byte aligned");
ASSERT(_abt_stack_size % 8 == 0, "ERROR(cortex-a-rt): size of ABT stack is not 8-byte aligned");
ASSERT(_und_stack_size % 8 == 0, "ERROR(cortex-a-rt): size of UND stack is not 8-byte aligned");
ASSERT(_svc_stack_size % 8 == 0, "ERROR(cortex-a-rt): size of SVC stack is not 8-byte aligned");

PROVIDE(_asm_undefined_handler =_asm_default_handler);
PROVIDE(_asm_prefetch_handler  =_asm_default_handler);
PROVIDE(_asm_abort_handler     =_asm_default_handler);
PROVIDE(_asm_fiq_handler       =_asm_default_fiq_handler);
PROVIDE(_irq_handler           =_default_handler);
PROVIDE(_svc_handler           =_default_handler);
PROVIDE(_start                 =_default_start);
