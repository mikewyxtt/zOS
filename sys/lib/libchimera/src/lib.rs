/*  sys/lib/libchimera/src/lib.rs - chimera main lib
 *
 *  chimera  --  Advanced *NIX System
 *  Copyright (C) 2024  Free Software Foundation, Inc.
 *
 *  chimera is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  chimera is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with GRUB. If not, see <http://www.gnu.org/licenses/>.
 */

#![no_std]


pub mod boot {
    // chimera::boot::bootinfo
    pub use sys_boot_bootinfo as bootinfo;

    // chimera::boot::early_log
    pub use sys_boot_early_log as early_log;
}


pub mod hal {
    // chimera::hal::io
    pub use hal::io as io;
}

// chimera::debugtools
//pub use sys_debugtools as debugtools;
