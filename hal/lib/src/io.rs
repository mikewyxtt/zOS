/*  hal/lib/src/io.rs - port i/o
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

 
 /// Writes a single byte to 'port'
 #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
 #[inline(always)]
 pub unsafe fn write_byte(port: u16, data: u8) {
     core::arch::asm!(   "out dx, al",
             in("al") data,
             in("dx") port);
 }
 
 // Reads a single byte from 'port'
 #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
 #[allow(unreachable_code)]
 #[inline(always)]
 pub unsafe fn read_byte(__port: u16) -> u8{
     panic!("read_byte() not implemented.");
     0
 }