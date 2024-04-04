// disk.rs

use crate::uefi;
use uefi::{LocateSearchType, BlockIOProtocol, GUID};
use core::ptr;
use alloc::vec;
use alloc::vec::Vec;


pub fn read_blocks() {
    let buffer_size: usize = 0;
    let handles: *mut usize = core::ptr::NonNull::<usize>::dangling().as_ptr();

    // 
    uefi::BootServices::locate_handle(LocateSearchType::ByProtocol, BlockIOProtocol::guid(), ptr::null(), &buffer_size, handles);
    let handles: Vec<usize> = vec![0; ((buffer_size as u64 * 8) / 64) as usize];
    uefi::BootServices::locate_handle(LocateSearchType::ByProtocol, BlockIOProtocol::guid(), ptr::null(), &buffer_size, handles.as_ptr() as *const usize);


    // Open the Block IO protocol
    let block_io_protocol: *const BlockIOProtocol = core::ptr::NonNull::<BlockIOProtocol>::dangling().as_ptr();
    let ptr = &block_io_protocol as *const *const BlockIOProtocol as *const *const usize;
    let guid = BlockIOProtocol::guid();

    // uefi::BootServices::handle_protocol(handles[0] as *const usize, &guid as *const GUID, ptr);
    // let block_io_protocol: &BlockIOProtocol = unsafe { &*(*ptr as *const BlockIOProtocol) };

    for entry in 0..3 {
        uefi::BootServices::handle_protocol(handles[entry] as *const usize, &guid as *const GUID, ptr);
        let block_io_protocol: &BlockIOProtocol = unsafe { &*(*ptr as *const BlockIOProtocol) };
        println!("Revision: 0x{:X}", block_io_protocol.revision);
        println!("Entry: {}", entry);
        unsafe { println!("\tMediaID: {}", (*block_io_protocol.media).media_id); }
        unsafe { println!("\tRemovable?: {}", (*block_io_protocol.media).removable_media); }
        unsafe { println!("\tPresent?: {}", (*block_io_protocol.media).media_present); }
        unsafe { println!("\tLogical partition: {}", (*block_io_protocol.media).logical_partition); }
        unsafe { println!("\tRead-Only?: {}", (*block_io_protocol.media).read_only); }
        unsafe { println!("\tWrite Caching?: {}", (*block_io_protocol.media).write_caching); }
        unsafe { println!("\tBlock size: {}", (*block_io_protocol.media).block_size); }
        unsafe { println!("\tIO Align: {}", (*block_io_protocol.media).io_align); }
        unsafe { println!("\tLast block: {}", (*block_io_protocol.media).last_block); }
        

        println!("\tfirst 10 bytes: ");
        let buffer: Vec<u8> = vec![0; 10];
        block_io_protocol.read_blocks(0, 10, buffer.as_ptr() as *const usize);
        for byte in 0..10 {
            print!("\t\t{}", buffer[byte]);
        }
        println!("");
    }
    // Load some blocks
    // 
    // block_io_protocol.read_blocks(0, 512, buffer.as_ptr());
    // println!("buffer address: 0x{}", buffer.as_ptr() as usize);

}
// 0x108310552