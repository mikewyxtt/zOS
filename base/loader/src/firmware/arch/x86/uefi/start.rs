/*  start.rs - UEFI loader entry point
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



#[no_mangle]
extern "win64" fn efi_main(efi_image_handle: *const usize, efi_system_table: *const super::libuefi::SystemTable) -> ! {
    super::libuefi::init(efi_image_handle, efi_system_table);
    super::console::init();
    super::disk::init();
    super::fb::init();

    crate::main();
    unreachable!()
}