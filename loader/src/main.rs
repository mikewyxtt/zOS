#![no_std]
#![no_main]


struct EFISystemTable {
    // EFISystemTable stuff goes here
}

#[no_mangle]
extern "win64" fn efi_main(_efi_handle: &usize, _system_table: &'static EFISystemTable) -> ! {
    unsafe { core::arch::asm!("MOV EAX, 0xFACE"); }
    loop {}
}



use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { core::arch::asm!("MOV EAX, 0xBadDeed"); }
    loop {}
}