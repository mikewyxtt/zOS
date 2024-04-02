use crate::uefi;

pub fn reset() {
    let efi_system_table = uefi::get_system_table();
    unsafe { ((*efi_system_table.console_out).reset)((*efi_system_table).console_out, false); }
}

pub fn putc(c: char) {
    let efi_system_table = uefi::get_system_table();
    let utf16_char: [u16; 2] = [(c as u8).into(), b'\0'.into()];
    unsafe { ((*efi_system_table.console_out).output_string)((*efi_system_table).console_out, utf16_char.as_ptr()); }
}