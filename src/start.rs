use log::debug;

core::arch::global_asm!("
.section .start, \"ax\"
.global _start
_start:
    adr x30, stack_end
    mov sp, x30

    // Save x0-x3 on stack
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!

    /* Zero out the bss section. */
    adr x29, bss_begin
    adr x30, bss_end
0:  cmp x29, x30
    b.hs 1f
    stp xzr, xzr, [x29], #16
    b 0b
1:

    /* Disable trapping floating point access in EL1. */
    mrs x30, cpacr_el1
    orr x30, x30, #(0x3 << 20)
    msr cpacr_el1, x30
    isb

    /* Call into init() */
    bl init

    // Restore x0-x3 from stack
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16

    /* Call into main() */
    bl main
");

#[no_mangle]
unsafe extern "C" fn init(x0: u64, x1: u64, x2: u64, x3: u64) -> u64 {
    crate::logger::init(log::LevelFilter::Debug).unwrap();
    debug!("init({:#010x}, {:#010x}, {:#010x}, {:#010x})", x0, x1, x2, x3);

    0
}