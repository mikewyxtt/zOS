// lib.rs

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