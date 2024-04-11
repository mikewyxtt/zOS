// allocator.rs

use core::alloc::{GlobalAlloc, Layout};
use crate::uefi::bootservices::{BootServices, MemoryType};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let buffer: *mut *mut usize = core::ptr::NonNull::<usize>::dangling().as_ptr() as *mut *mut usize;

        let efi_status = BootServices::allocate_pool(MemoryType::LoaderData, layout.size(), buffer);
        if efi_status != 0 {
            panic!("Could not allocate heap memory.\nEFI_STATUS: {}", efi_status);
        }

        (*buffer) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let efi_status = BootServices::free_pool(ptr as *const usize);
        if efi_status != 0 {
            panic!("Could not deallocate heap memory.\nEFI_STATUS: {}", efi_status);
        }
    }
}
