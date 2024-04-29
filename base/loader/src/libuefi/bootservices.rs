/*  bootservices.rs - UEFI BootServices implementation
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

#![allow(dead_code)]

use core::{mem::size_of, sync::atomic::Ordering};
use core::ffi::c_void;
use core::ptr;
use alloc::vec;
use alloc::vec::Vec;

use super::{protocol::EFIProtocol, TableHeader, GUID, IMAGE_HANDLE, SYSTEM_TABLE_PTR};

#[repr(C)]
pub struct BootServices {
    pub header:                                     TableHeader,
    _raise_tpl:                                     *const c_void,
    _restore_tpl:                                   *const c_void,
    _allocate_pages:                                unsafe extern "efiapi" fn (AllocateType, MemoryType, usize, *mut u64) -> u32,
    _free_pages:                                    unsafe extern "efiapi" fn (*const c_void, usize) -> u32,
    _get_memory_map:                                *const c_void,
    _allocate_pool:                                 unsafe extern "efiapi" fn (MemoryType, usize, *mut *mut c_void) -> u32,
    _free_pool:                                     unsafe extern "efiapi" fn (*const c_void) -> u32,
    _create_event:                                  *const c_void,
    _set_timer:                                     *const c_void,
    _wait_for_event:                                *const c_void,
    _signal_event:                                  *const c_void,
    _close_event:                                   *const c_void,
    _check_event:                                   *const c_void,
    _install_protocol_interface:                    *const c_void,
    _reinstall_protocol_interface:                  *const c_void,
    _uninstall_protocol_interface:                  *const c_void,
    _handle_protocol:                               unsafe extern "efiapi" fn (*const c_void, &GUID, *const *const c_void) -> u32,
    _reserved:                                      *const c_void,
    _register_protocol_notify:                      *const c_void,
    _locate_handle:                                 unsafe extern "efiapi" fn (LocateSearchType, &GUID, *const c_void, *const usize, *mut usize) -> u32,
    _locate_device_path:                            *const c_void,
    _install_configuration_table:                   *const c_void,
    _load_image:                                    *const c_void,
    _start_image:                                   *const c_void,
    _exit:                                          *const c_void,
    _unload_image:                                  *const c_void,
    _exit_boot_services:                            *const c_void,
    _get_next_monotonic_count:                      *const c_void,
    _stall:                                         *const c_void,
    _set_watchdog_timer:                            *const c_void,
    _connect_controller:                            *const c_void,
    _disconnect_controller:                         *const c_void,
    _open_protocol:                                 unsafe extern "efiapi" fn (*const c_void, &GUID, *const *const c_void, *const c_void, *const c_void, u32) -> u32,
    _close_protocol:                                unsafe extern "efiapi" fn (*const c_void, &GUID, *const c_void, *const c_void) -> u32,
    _open_protocol_information:                     *const c_void,
    _protocols_per_handle:                          *const c_void,
    _locate_handle_buffer:                          *const c_void,
    _locate_protocol:                               *const c_void,
    _install_multiple_protocol_interfaces:          *const c_void,
    _uninstall_multiple_protocol_interfaces:        *const c_void,
    _calculate_crc32:                               *const c_void,
    _copy_mem:                                      *const c_void,
    _set_mem:                                       *const c_void,
    _create_event_ex:                               *const c_void
}

impl BootServices {
    /// Returns a reference to BootServices
    fn get() -> &'static Self {
        unsafe { &*(*(SYSTEM_TABLE_PTR.load(Ordering::SeqCst))).boot_services }
    }
}



/* Protocol Handler Services */

#[repr(C)]
#[derive(Clone, Copy)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol
}

impl BootServices {

    /// Returns a vector of handles that support protocol <T>
    pub fn locate_handle_by_protocol<T: EFIProtocol>() -> Vec<usize> {
        // Setting the buffer size to 0 allows the firmware to replace the value of 'buffer_size' to the array size needed to hold the list.
        let mut buffer_size = 0;

        unsafe { (Self::get()._locate_handle)(LocateSearchType::ByProtocol, &T::guid(), core::ptr::null(), &mut buffer_size, core::ptr::dangling_mut()); }
        let mut handles: Vec<usize> = vec![0; buffer_size / size_of::<usize>()];
        unsafe { (Self::get()._locate_handle)(LocateSearchType::ByProtocol, &T::guid(), core::ptr::null(), &mut buffer_size, handles.as_mut_ptr()); }

        handles
    }

    /// Returns a protocol interface
    pub fn handle_protocol<T: EFIProtocol>(handle: *const usize) -> &'static T {
        let proto: *mut *mut T = core::ptr::dangling_mut();
        unsafe { (Self::get()._handle_protocol)(handle.cast(), &T::guid(), proto as *const *const c_void); }
        unsafe { &mut (**proto ) }
    }

    /// Opens aprotocol
    pub fn open_protocol(handle: *const usize, protocol: &GUID) -> u32 {
        unsafe { (Self::get()._open_protocol)(handle.cast(), protocol, ptr::null(), IMAGE_HANDLE.load(Ordering::SeqCst).cast(), ptr::null(), 0x00000004) }
    }

    /// Closes a protocol
    pub fn close_protocol(handle: *const usize, protocol: &GUID) -> u32 {
        unsafe { (Self::get()._close_protocol)(handle.cast(), protocol, IMAGE_HANDLE.load(Ordering::SeqCst).cast(), ptr::null()) }
    }
}



/* Memory Allocation Services */
#[repr(C)]
pub enum MemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemoryType,
    MaxMemoryType
}

#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType
}

impl BootServices {
    /// Allocate memory pages from the system.
    /// Returns the starting address of a free memory page
    pub fn allocate_pages(_type: AllocateType, memory_type: MemoryType, pages: usize, memory: *mut u64) -> u32 {
        unsafe { (Self::get()._allocate_pages)(_type, memory_type, pages, memory) }
    }

    /// Frees memory pages.
    pub fn free_pages(memory: *const usize, pages: usize) {
        unsafe { (Self::get()._free_pages)(memory as *const c_void, pages) };
    }

    /// Allocate pool memory
    pub fn allocate_pool(_type: MemoryType, size: usize, buffer: *mut *mut usize) -> u32 {
        unsafe { (Self::get()._allocate_pool)(_type, size, buffer as *mut *mut c_void) }
    }

    /// Free pool memory.
    pub fn free_pool(buffer: *const usize) -> u32 {
        unsafe { (Self::get()._free_pool)(buffer as *const c_void) }
    }
}