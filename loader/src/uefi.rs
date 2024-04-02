use core::ffi::c_void;

/*
EFI Data types:
EFI_GUID: 128 bit buffer
EFI_STATUS: usize
EFI_HANDLE: void*
EFI_EVENT: void*
EFI_LBA: u64
EFI_TPL: usize
EFI_MAC_ADDRESS: 32 byte buffer
EFI_IPv4_ADDRESS: 4 byte buffer
EFI_IPv6_ADDRESS: 16 byte buffer
EFI_IP_ADDRESS: 16 byte buffer
*/

static mut SYSTEM_TABLE: *const SystemTable = core::ptr::null();

#[allow(dead_code)]
#[repr(C)]
pub struct EFITableHeader {
    pub signature:      u64,
    pub revision:       u32,
    pub header_size:    u32,
    pub crc32:          u32,
    pub reserved:       u32
}

#[allow(dead_code)]
#[repr(C)]
pub struct SystemTable {
    pub header:                     EFITableHeader,
    pub firmware_vendor:            *const u16,
    pub firmware_revision:          u32,
    pub console_in_handle:          *const c_void,
    pub console_in:                 *const c_void,
    pub console_out_handle:         *const c_void,
    pub console_out:                *const SimpleTextOutputProtocol,
    pub standard_error_handle:      *const c_void,
    pub std_error:                  *const c_void,
    pub runtime_services:           *const c_void,
    pub boot_services:              *const EFIBootServices,
    pub number_of_table_entries:    usize,
    pub configuration_table:        *const c_void
}

#[allow(dead_code)]
#[repr(C)]
pub struct EFIBootServices {
    pub header:                                     EFITableHeader,
    pub raise_tpl:                                  *const c_void,
    pub restore_tpl:                                *const c_void,
    pub allocate_pages:                             *const c_void,
    pub free_pages:                                 *const c_void,
    pub get_memory_map:                             *const c_void,
    pub allocate_pool:                              *const c_void,
    pub free_pool:                                  *const c_void,
    pub create_event:                               *const c_void,
    pub set_timer:                                  *const c_void,
    pub wait_for_event:                             *const c_void,
    pub signal_event:                               *const c_void,
    pub close_event:                                *const c_void,
    pub check_event:                                *const c_void,
    pub install_protocol_interface:                 *const c_void,
    pub reinstall_protocol_interface:               *const c_void,
    pub uninstall_protocol_interface:               *const c_void,
    pub handle_protocol:                            *const c_void,
    pub reserved:                                   *const c_void,
    pub register_protocol_notify:                   *const c_void,
    pub locate_handle:                              *const c_void,
    pub locate_device_path:                         *const c_void,
    pub install_configuration_table:                *const c_void,
    pub load_image:                                 *const c_void,
    pub start_image:                                *const c_void,
    pub exit:                                       *const c_void,
    pub unload_image:                               *const c_void,
    pub exit_boot_services:                         *const c_void,
    pub get_next_monotonic_count:                   *const c_void,
    pub stall:                                      *const c_void,
    pub set_watchdog_timer:                         *const c_void,
    pub connect_controller:                         *const c_void,
    pub disconnect_controller:                      *const c_void,
    pub open_protocol:                              *const c_void,
    pub close_protocol:                             *const c_void,
    pub open_protocol_information:                  *const c_void,
    pub protocols_per_handle:                       *const c_void,
    pub locate_handle_buffer:                       *const c_void,
    pub locate_protocol:                            *const c_void,
    pub install_multiple_protocol_interfaces:       *const c_void,
    pub uninstall_multiple_protocol_interfaces:     *const c_void,
    pub calculate_crc32:                            *const c_void,
    pub copy_mem:                                   *const c_void,
    pub set_mem:                                    *const c_void,
    pub create_event_ex:                            *const c_void
}

#[allow(dead_code)]
#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset:                  unsafe extern "efiapi" fn(*const SimpleTextOutputProtocol, bool),
    pub output_string:          unsafe extern "efiapi" fn(*const SimpleTextOutputProtocol, *const u16),
    pub test_string:            *const c_void,
    pub query_mode:             *const c_void,
    pub set_mode:               *const c_void,
    pub set_attribute:          *const c_void,
    pub clear_screen:           unsafe extern "efiapi" fn(*const SimpleTextOutputProtocol),
    pub set_cursor_position:    *const c_void,
    pub enable_cursor:          *const c_void,
    pub mode:                   *const c_void
}


pub fn initialize(system_table: *const SystemTable) {
    unsafe { SYSTEM_TABLE = system_table };
}

pub fn get_system_table() -> &'static SystemTable {
    unsafe { &*(SYSTEM_TABLE) }
}