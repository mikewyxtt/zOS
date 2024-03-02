// sys/lib/debugtools/src/reg_probe.rs

/// Sets 'EAX' register to 'value'
pub unsafe fn set_eax(value: u32) {
    core::arch::asm!(
        "mov eax, {}",
        in(reg) value
        );
}
