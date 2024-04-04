#![allow(dead_code)]

use core::ffi::c_void;
use core::ptr;

// Pointers to the EFI System Table and EFI Image handle
static mut SYSTEM_TABLE: *const SystemTable = ptr::null();
static mut IMAGE_HANDLE: *const c_void = ptr::null();


#[repr(C)]
pub struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8]
}

impl GUID {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        GUID {
            data1,
            data2,
            data3,
            data4
        }
    }
}

#[repr(C)]
pub struct TableHeader {
    pub signature:      u64,
    pub revision:       u32,
    pub header_size:    u32,
    pub crc32:          u32,
    reserved:           u32
}

#[repr(C)]
pub struct SystemTable {
    pub header:                                     TableHeader,
    pub firmware_vendor:                            *const u16,
    pub firmware_revision:                          u32,
    pub console_in_handle:                          *const c_void,
    pub simple_text_input_protocol:                 *const c_void,
    pub console_out_handle:                         *const c_void,
    pub simple_text_output_protocol:                *const SimpleTextOutputProtocol,
    pub standard_error_handle:                      *const c_void,
    pub std_error:                                  *const c_void,
    pub runtime_services:                           *const c_void,
    pub boot_services:                              *const BootServices,
    pub number_of_table_entries:                    usize,
    pub configuration_table:                        *const c_void
}


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
    _handle_protocol:                               unsafe extern "efiapi" fn (*const c_void, *const GUID, *const *const c_void) -> u32,
    _reserved:                                      *const c_void,
    _register_protocol_notify:                      *const c_void,
    _locate_handle:                                 unsafe extern "efiapi" fn (LocateSearchType, GUID, *const c_void, *const usize, *const usize) -> u32,
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
    _open_protocol:                                 unsafe extern "efiapi" fn (*const c_void, *const GUID, *const *const c_void, *const c_void, *const c_void, u32) -> u32,
    _close_protocol:                                unsafe extern "efiapi" fn (*const c_void, *const GUID, *const c_void, *const c_void) -> u32,
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
        unsafe { &*(*(SYSTEM_TABLE)).boot_services }
    }
}



/* Protocol Handler Services */

#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol
}

impl BootServices {
    pub fn locate_handle(search_type: LocateSearchType, protocol: GUID, search_key: *const c_void, buffer_size: *const usize, buffer: *const usize) -> u32{
        unsafe { (Self::get()._locate_handle)(search_type, protocol, search_key, buffer_size, buffer) }
    }

    /// Returns a protocol interface
    pub fn handle_protocol(handle: *const usize, guid: *const GUID, interface: *const *const usize) -> u32 {
        unsafe { (Self::get()._handle_protocol)(handle as *const c_void, guid, interface as *const *const c_void) }
    }

    /// Opens aprotocol
    pub fn open_protocol(handle: *const usize, protocol: *const GUID) -> u32 {
        unsafe { (Self::get()._open_protocol)(handle as *const c_void, protocol, ptr::null(), IMAGE_HANDLE, ptr::null(), 0x00000004) }
    }

    /// Closes a protocol
    pub fn close_protocol(handle: *const usize, protocol: *const GUID) -> u32 {
        unsafe { (Self::get()._close_protocol)(handle as *const c_void, protocol, IMAGE_HANDLE, ptr::null()) }
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

/* Console Support */

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    _reset:                     unsafe extern "efiapi" fn (*const Self, bool),
    _output_string:             unsafe extern "efiapi" fn (*const Self, *const u16),
    _test_string:               *const c_void,
    _query_mode:                *const c_void,
    _set_mode:                  unsafe extern "efiapi" fn (*const Self, usize) -> u32,
    _set_attribute:             *const c_void,
    _clear_screen:              *const c_void,
    _set_cursor_position:       *const c_void,
    _enable_cursor:             *const c_void,
    _mode:                      *const c_void
}

#[repr(C)]
pub struct SimpleTextOutputMode {
    pub max_mode:               i32,
    pub mode:                   i32,
    pub attribute:              i32,
    pub cursor_column:          i32,
    pub cursor_row:             i32,
    pub cursor_visible:         bool
}


impl SimpleTextOutputProtocol {
    /// Returns a reference to SimpleTextOutputProtocol
    fn get() -> &'static Self {
        unsafe { &*(*(SYSTEM_TABLE)).simple_text_output_protocol }
    }

    /// Resets the console
    pub fn reset() {
        unsafe { (Self::get()._reset)(Self::get(), false) };
    }

    /// Outputs a UTF-16 string to the UEFI console
    pub fn output_string(string: *const u16) {
        unsafe { (Self::get()._output_string)(Self::get(), string) };
    }

    /// Sets the output mode
    pub fn set_mode(mode_number: usize) -> u32 {
        unsafe { (Self::get()._set_mode)(Self::get(), mode_number) }
    }
}

/* Media Access */

#[repr(C)]
pub struct BlockIOProtocol {
    pub revision:       u64,
    pub media:          *const BlockIOMedia,
    _reset:             *const c_void,
    _read_blocks:       unsafe extern "efiapi" fn (*const Self, u32, u64, usize, *const c_void) -> u32,
    _write_blocks:      *const c_void,
    _flush_blocks:      *const c_void,
}

impl BlockIOProtocol {
    /// Returns the BlockIOProtocol GUID
    pub fn guid() -> GUID {
        GUID::new(0x964e5b21, 0x6459, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b])
    }

    // /// Reads from the disk
    pub fn read_blocks(&self, lba: u64, buffer_size: usize, buffer: *const usize) -> u32 {
        unsafe { (self._read_blocks)(self, (*self.media).media_id, lba, buffer_size, buffer as *const c_void) }
    }
}

#[repr(C)]
pub struct BlockIOMedia {
    pub media_id:                               u32,
    pub removable_media:                        bool,
    pub media_present:                          bool,
    pub logical_partition:                      bool,
    pub read_only:                              bool,
    pub write_caching:                          bool,
    pub block_size:                             u32,
    pub io_align:                               u32,
    pub last_block:                             u64,
    pub lowest_aligned_lba:                     u64,
    pub logical_blocks_per_physical_block:      u32,
    pub optimal_transfer_length_granularity:    u32
}


/// Initializes the pointer to the system table
pub fn initialize(image_handle: *const usize, system_table: *const SystemTable) {
    unsafe { 
        SYSTEM_TABLE = system_table;
        IMAGE_HANDLE = image_handle as *const c_void;
    }
}
