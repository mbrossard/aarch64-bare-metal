ENTRY(_start)

start_address = 0x40080000;
# start_address = 0x40100000;
# start_address = 0x0;

SECTIONS
{
    . = start_address;
    .start : {
        start_begin = .;
        *(.start._start)
        *(.start.*)
        start_end = .;
    }
    .text : {
        text_begin = .;
        *(.text.*)
        text_end = .;
        rodata_begin = .;
        *(.rodata.*)
        rodata_end = .;
    }
    .got : {
        # got_begin = .;
        *(.got.*)
        # got_end = .;
    }
    .data : ALIGN(16) {
        data_begin = .;
        *(.data.*)
        . = ALIGN(16);
        data_end = .;
    }
    # bin_end = .;
    .bss (NOLOAD) : ALIGN(16)  {
        bss_begin = .;
        *(.bss.*)
        *(COMMON)
        . = ALIGN(16);
        bss_end = .;
    } # >image
    .stack (NOLOAD) : ALIGN(4096) {
        stack_begin = .;
        . += 16 * 4096;
        . = ALIGN(4096);
        stack_end = .;
    } # >image

    . = ALIGN(4K);
    PROVIDE(free_memory = .);
}
