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


    ldrprintln!("Hello, World!");

    // unsafe { drivers::uefi::disk::read_bytes_raw("", 0, 0, core::ptr::null_mut()) };


    loop {}
}



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    ldrprintln!("{}", _info);
    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    ldrprintln!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
