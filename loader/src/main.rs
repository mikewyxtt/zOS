#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(crate::test_runner)]

extern crate alloc;

#[macro_use]
mod uefi;
mod allocator;
mod drivers;

use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use drivers::*;


const ROOT_PARTITION: &str = "disk0s2";

#[no_mangle]
extern "win64" fn efi_main(efi_image_handle: *const usize, efi_system_table: *const uefi::SystemTable) -> ! {
    uefi::init(efi_image_handle, efi_system_table);
    console::reset();
    disk::init().unwrap();
    get_cfg();

    println!("Hello, World!");

    let mut buffer: Vec<u8> = vec!(0; 512);
    disk::read_bytes_vec(ROOT_PARTITION, 2,512, &mut buffer).expect("Block IO error");
    let buffer = buffer;

    let mut ptr = buffer.as_ptr() as usize;
    ptr += 56;
    let magic: u16 = unsafe { *(ptr as *const u16) as u16};
    println!("EXT2 magic: 0x{:X}", magic);

    loop {}
}

fn get_cfg() {
    //
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}