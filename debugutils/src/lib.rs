/*  lib.rs - Debugging utils library
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

pub mod hextools {

    #[macro_export]
    macro_rules! hexdump {
        ($length:expr, $split_by:expr, $buffer:expr) => {
            println!("[HEXDUMP]:");
            let mut x = 0;

            for i in 0..$length {
                if x == $split_by {
                    print!("  ");
                }

                if x == $split_by * 2 {
                    print!("\n");
                    x = 0;
                }

                let byte = core::ptr::read($buffer.offset(i as isize));
                print!("0x{:02X} ", byte);

                x+=1;
            }
            print!("\n");
        }
    }

    #[macro_export]
    macro_rules! hexdump_blocks {
        ($length:expr, $split_by:expr, $block_size:expr, $buffer:expr) => {
            println!("[HEXDUMP]:");
            println!("Block 0");
            let mut x = 0;

            for i in 0..$length {
                if x == $split_by {
                    print!("  ");
                }

                if x == $split_by * 2 {
                    print!("\n");
                    x = 0;
                }

                if i % $block_size == 0 && i >= $block_size {
                    print!("\n");
                    println!("Block {}:", i / $block_size);
                }

                let byte = core::ptr::read($buffer.offset(i as isize));
                print!("0x{:02X} ", byte);

                x+=1;
            }
            print!("\n");
        }
    }
}