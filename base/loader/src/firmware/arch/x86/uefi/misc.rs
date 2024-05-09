/*  misc.rs - Misc. UEFI functions
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

use super::libuefi::{bootservices::BootServices, protocol::{device_path::{DevicePathProtocol, HardDriveDevicePath}, loaded_image::LoadedImageProtocol}};
use crate::uuid::GUID;


/// Returns the GUID partition signature of the ESP
pub fn get_esp_guid() -> GUID {
    let handle = BootServices::handle_protocol::<LoadedImageProtocol>(super::libuefi::IMAGE_HANDLE.load(core::sync::atomic::Ordering::SeqCst)).device_handle;
    let mut node = BootServices::handle_protocol::<DevicePathProtocol>(handle as *const usize);

    while (node._type, node.subtype) != (0x7F, 0xFF)  {
                match (node._type, node.subtype, node.length[0] + node.length[1]) {
                    // Hard drive device path
                    (4, 1, 42) => {
                        // Cast the current node as HardDriveDevicePath so we can read the GUID
                        #[allow(invalid_reference_casting)]
                        let hddp: &HardDriveDevicePath = unsafe { &*((node as *const DevicePathProtocol).cast()) };
                        let guid = hddp.partition_sig;

                        return guid;
                    }
    
                    _ => {}
                }
    
                node = node.next();
            }

    panic!("FS error: Could not find EFI System Partition. Halting.");
}