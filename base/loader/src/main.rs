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
#![feature(allocator_api)]
#![test_runner(crate::test_runner)]


extern crate alloc;

#[macro_use]
mod allocator;
mod config;
mod drivers;
mod firmware;
mod uuid;

use core::panic::PanicInfo;
use config::parse_cfg;
use drivers::*;
// use debugutils::hexdump_blocks;


fn main() {
    drivers::start();
    console::clear();

    ldrprintln!("Entered main()..");
    let cfg = parse_cfg();

    extfs::find(cfg.rootfs, " ");
    
    ldrprintln!("root={}", cfg.rootfs.as_string());
    ldrprintln!("resolution={}", cfg.resolution);

    ldrprintln!("Done. Looping.");

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
