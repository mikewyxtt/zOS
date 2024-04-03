#![allow(dead_code)]

use core::ffi::c_void;

const DISK_IO_PROTOCOL_GUID: GUID = GUID::new(0xCE345171, 0xBA0B, 0x11d2, [0x8e, 0x4F, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

// Pointer to the EFI System Table
static mut SYSTEM_TABLE: *const SystemTable = core::ptr::null();


#[repr(C)]
pub struct TableHeader {
    pub signature:      u64,
    pub revision:       u32,
    pub header_size:    u32,
    pub crc32:          u32,
    reserved:           u32
}


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
    _allocate_pages:                                *const c_void,
    _free_pages:                                    *const c_void,
    _get_memory_map:                                *const c_void,
    _allocate_pool:                                 *const c_void,
    _free_pool:                                     *const c_void,
    _create_event:                                  *const c_void,
    _set_timer:                                     *const c_void,
    _wait_for_event:                                *const c_void,
    _signal_event:                                  *const c_void,
    _close_event:                                   *const c_void,
    _check_event:                                   *const c_void,
    _install_protocol_interface:                    *const c_void,
    _reinstall_protocol_interface:                  *const c_void,
    _uninstall_protocol_interface:                  *const c_void,
    _handle_protocol:                               *const c_void,
    _reserved:                                      *const c_void,
    _register_protocol_notify:                      *const c_void,
    _locate_handle:                                 *const c_void,
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
    _open_protocol:                                 *const c_void,
    _close_protocol:                                *const c_void,
    _open_protocol_information:                     *const c_void,
    _protocols_per_handle:                          *const c_void,
    _locate_handle_buffer:                          *const c_void,
    _locate_protocol:                               unsafe extern "efiapi" fn (GUID, *const c_void) -> *const c_void,
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

    /// Returns a pointer to the requested protocol
    pub fn locate_protocol(protocol: GUID, registration: *const c_void) -> *const c_void {
        unsafe { (Self::get()._locate_protocol)(protocol, registration) }
    }
}


#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub _reset:                 unsafe extern "efiapi" fn (*const Self, bool),
    _output_string:             unsafe extern "efiapi" fn (*const Self, *const u16),
    _test_string:               *const c_void,
    _query_mode:                *const c_void,
    _set_mode:                  *const c_void,
    _set_attribute:             *const c_void,
    _clear_screen:              *const c_void,
    _set_cursor_position:       *const c_void,
    _enable_cursor:             *const c_void,
    _mode:                      *const c_void
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
}


#[repr(C)]
pub struct DiskIOProtocol {
    pub revision:       u64,
    _read_disk:         unsafe extern "efiapi" fn (*const Self, u32, u64, usize) -> *const c_void,
    _write_disk:        *const c_void
}


impl DiskIOProtocol {
    /// Returns a reference to DiskIOProtocol
    fn get() -> &'static Self {
        unsafe { &*(BootServices::locate_protocol(DISK_IO_PROTOCOL_GUID, core::ptr::null()) as *const Self) as &Self}
    }

    /// Reads from the disk
    pub fn read_disk(media_id: u32, offset: u64, buffer_size: usize) -> *const c_void {
        unsafe { (Self::get()._read_disk)(Self::get(), media_id, offset, buffer_size) }
    }
}


pub fn initialize(system_table: *const SystemTable) {
    unsafe { SYSTEM_TABLE = system_table };
}
