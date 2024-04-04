#![no_std]
#![no_main]

extern crate alloc;


#[macro_use]
mod console;
mod uefi;
mod disk;
mod allocator;

use core::panic::PanicInfo;

#[no_mangle]
extern "win64" fn efi_main(efi_image_handle: *const usize, efi_system_table: *const uefi::SystemTable) -> ! {
    uefi::initialize(efi_image_handle, efi_system_table);
    console::reset();

    println!("Hello, World!");

    disk::read_blocks();

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
