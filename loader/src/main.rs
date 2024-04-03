#![no_std]
#![no_main]

mod uefi;
mod console;

use core::panic::PanicInfo;

#[no_mangle]
extern "win64" fn efi_main(_efi_image_handle: *const usize, efi_system_table: *const uefi::SystemTable) -> ! {
    uefi::initialize(efi_system_table);
    console::reset();

    println!("Yoooo");
    println!("15 in hex: {:X}", 15);
    
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { core::arch::asm!("MOV EAX, 0xBadDeed"); }
    loop {}
}