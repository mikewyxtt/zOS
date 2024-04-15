use alloc::{string::ToString, vec, vec::Vec};

use crate::libuefi::{bootservices::BootServices, protocol::block_io::BlockIOProtocol};

unsafe fn read_bytes_raw(dev: &str, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {
    let block_io_protocol: *mut *mut BlockIOProtocol = core::ptr::dangling_mut();
    BootServices::handle_protocol(find_device(dev).expect("Device not found.").handle, &(BlockIOProtocol::guid()), block_io_protocol.cast());
    let block_io_protocol: &BlockIOProtocol = unsafe { &(**block_io_protocol) };

    if count < BLOCK_SIZE {
        let mut tmp: Vec<u8> = vec![0; BLOCK_SIZE];
        let status = block_io_protocol.read_blocks(lba, BLOCK_SIZE, tmp.as_mut_ptr());
        if status == 0 {
            unsafe { ptr::copy(tmp.as_ptr(), buffer, count) };
            Ok(())
        }
        else {
            Err(alloc::format!("EFI ERROR: {}", status).to_string())
        }
    }

    else {
        let status = block_io_protocol.read_blocks(lba, count, buffer);
        if status == 0 {
            Ok(())
        }
        else {
            Err(alloc::format!("EFI ERROR: {}", status).to_string())
        }
    }
}