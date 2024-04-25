/*  allocator.rs - Basic allocator
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


use core::alloc::{GlobalAlloc, Layout};
use crate::libuefi::bootservices::{BootServices, MemoryType};

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
