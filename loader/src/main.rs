#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(strict_provenance)]
#![test_runner(crate::test_runner)]

extern crate alloc;

#[macro_use]
mod allocator;
mod drivers;
mod libuefi;

use core::panic::PanicInfo;
use drivers::*;


#[no_mangle]
extern "win64" fn efi_main(efi_image_handle: *const usize, efi_system_table: *const libuefi::SystemTable) -> ! {
    libuefi::init(efi_image_handle, efi_system_table);
    console::clear();
    disk::init();

    drivers::fat32::read();
    //parse_conf();

    println!("Hello, World!");
    // println!("Boot disk: {}", find_efi_slice().unwrap());


    loop {}
}

// use libuefi::bootservices::BootServices;
// use libuefi::protocol::{device_path::DevicePathProtocol, loaded_image::LoadedImageProtocol};

// use crate::disk::find_efi_slice;

// fn read_cfg() {
//     // Find the path to the EFI boot partition
//     let loaded_image_protocol: *mut *mut LoadedImageProtocol = core::ptr::dangling_mut();
//     BootServices::handle_protocol(libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst), &(LoadedImageProtocol::guid()), loaded_image_protocol.cast());
//     let loaded_image_protocol: &LoadedImageProtocol = unsafe { &mut (**loaded_image_protocol)};

//     // let file_path = loaded_image_protocol.file_path;
//     // println!("Dev handle: 0x{:X}", file_path as usize);
    

//     // let device_path_protocol_ptr: *mut *mut DevicePathProtocol = core::ptr::dangling_mut();
//     // BootServices::handle_protocol(file_path.cast(), &(DevicePathProtocol::guid()), device_path_protocol_ptr.cast());
    
//     let node: &DevicePathProtocol = unsafe { &(*loaded_image_protocol.file_path)};
//     println!("type: {} subtype:{} length: {}", node._type, node.subtype, node.length[0]);
//     println!("path loc: 0x{:X}", node as *const DevicePathProtocol as usize);
// }

// fn parse_conf() {
//     read_cfg();
//     // get the values from the config file
// }


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



struct BootInfo {
    pub version:    u16,
    pub size:       usize,

    pub arch:       AMD64BootInfo,
    pub end:        u8,
}

impl BootInfo {
    pub fn new() -> Self {
        Self {
            version:    0,
            size:       core::mem::size_of::<Self>(),
            arch:       unsafe { core::mem::zeroed() },
            end:        0xFAFA,
        }
    }
}

struct AMD64BootInfo {
    cpuid:      u32,
}