#![no_std]
#![no_main]


struct SystemTable {
    _abc: u32,
    //
}

#[no_mangle]
extern "win64" fn efi_main(_efi_handle: &usize, _system_table: &'static SystemTable) -> ! {
    loop {}
}



use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // use debugtools::*;
    // serial_log!("{}", _info);
    // unsafe { chimera::debug::debugtools::set_eax(0xBadDeed); }
    loop {}
}
