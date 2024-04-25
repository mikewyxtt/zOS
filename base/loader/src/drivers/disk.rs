// disk.rs
#![allow(dead_code)]


use core::ops::AddAssign;
use core::ptr;
use core::mem::size_of;
use alloc::boxed::Box;
use alloc::{vec, vec::Vec};
use alloc::string::String;


static mut BLOCK_DEVICES: Vec<BlockDevice> = Vec::new();




pub struct BlockDevice {
    pub name: String,
    pub is_slice: bool,
    pub removable: bool,
    pub block_size: u16,
    pub read_bytes_raw: unsafe fn (dev: &str, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String>,
}





impl BlockDevice {
    pub fn new(name: String, is_slice: bool, removable: bool, block_size: u16, read_bytes_raw: unsafe fn (dev: &str, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String>) -> Self {
        Self {
            name,
            is_slice,
            removable,
            block_size,
            read_bytes_raw,
        }
    }
}




/// Reads data from the disk as Box<T>
pub fn read_bytes_into_box<T>(dev: &str, lba: u64, count: usize) -> Box<T> {
    assert!(count <= size_of::<T>());

    let mut t: Box<T> = unsafe { Box::new(core::mem::zeroed()) };

    let buffer = read_bytes(dev, lba, count).unwrap();
    unsafe { core::ptr::copy(buffer.as_ptr(), (t.as_mut() as *mut T).cast(), count); }

    t
}






/// Reads data from the disk into T
pub fn read_bytes_into<T>(dev: &str, lba: u64, count: usize, buffer: &mut T) {
    assert!(count <= core::mem::size_of_val(buffer));
    unsafe { read_bytes_raw(dev, lba, count, (buffer as *mut T).cast()).unwrap(); }
}







pub fn read_bytes(dev: &str, lba: u64, count: usize) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = vec![0; count];

    unsafe {
        match read_bytes_raw(dev, lba, count, buffer.as_mut_ptr()) {
            Ok(_) => Ok(buffer),
            Err(error) => Err(error)
        }
    }
}








/// Reads bytes from the disk into a buffer. 'buffer' is a raw ptr, and is unsafe as the boundaries cannot be checked.
/// 
/// count: Number of bytes to read
/// lba: Logical block address, which logical block to start reading from
/// device: Deivce to read from. e.g disk0s1
/// buffer: Buffer to fill with bytes
pub unsafe fn read_bytes_raw(device: &str, lba: u64, count: usize, buffer: *mut u8) -> Result<(), String> {
    // If the count is smaller than the block size, UEFI error out. To mitigate this we create our own buffer can hold at least one block, read the block into it, then copy the requested amount of bytes into the callers buffer
    let dev = find_device(device).ok().unwrap();
    if count < dev.block_size.into() {
        let mut tmp: Vec<u8> = vec![0; dev.block_size.into()];

        match ((dev.read_bytes_raw))(device, lba, dev.block_size.into(), tmp.as_mut_ptr()) {
            Ok(_) => {
                ptr::copy(tmp.as_ptr(), buffer.cast(), count);
                Ok(())
            }

            Err(error) => Err(error)
        }
    }

    else {
        ((dev.read_bytes_raw))(device, lba, dev.block_size.into(), buffer)
    }
}



/// Searches for block devices and returns a Vector of BlockDevice structs
fn probe_disks() -> Vec<BlockDevice> {
    super::uefi::disk::probe_disks()
}







pub fn name_device(_removable: bool, is_slice: bool, devices: &Vec<BlockDevice> ) -> String {
    // name a slice
    if is_slice {
        let mut slice_num = 1;

        let mut parent_disk = &String::new();    
        for dev in devices.iter().rev() {
            if !dev.is_slice {
                parent_disk = &dev.name;
                break;
            }
        }
    
        loop {
            let mut available = true;
            let name = String::from(alloc::format!("{}s{}", parent_disk, slice_num));
    
            for dev in devices.iter() {
                if dev.name.eq(&name) {
                    available = false;
                }
            }
    
            if available {
                return name
            }

            slice_num.add_assign(1);
        }

    }
    // name a full disk
    else {
        let mut disk_num = 0;

        loop {
            let mut available = true;
            let name = String::from(alloc::format!("disk{}", disk_num));

            for dev in devices.iter() {
                available = true;
                if dev.name == name {
                    available = false;
                }
            }

            if available {
                return name
            }

            disk_num.add_assign(1);
        }
    }
    
}




fn find_device(name: &str) -> Result<&'static BlockDevice, ()> {
    unsafe {
        if !BLOCK_DEVICES.is_empty() {
            for device in BLOCK_DEVICES.iter() {
                if device.name == name {
                    return Ok(device)
                }
            }
        }
    }

    Err(())
}




pub fn init(){
    unsafe { 
        assert!(BLOCK_DEVICES.is_empty(), "Disk driver cannot be initialized more than once.");
        BLOCK_DEVICES = probe_disks();
    }
}