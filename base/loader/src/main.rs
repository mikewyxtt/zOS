/*  main.rs - UEFI loader main
 *
 *  zOS  --  Advanced *NIX System
 *  Copyright (C) 2024  Free Software Foundation, Inc.
 *
 *  zOS is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  zOS is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with zOS. If not, see <http://www.gnu.org/licenses/>.
 */


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