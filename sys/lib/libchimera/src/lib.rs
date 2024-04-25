/*  sys/lib/libzOS/src/lib.rs - zOS main lib
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

#![no_std]

/* Main zOS system api */



pub mod hal {
    //#[cfg(feature = "boot_components")]
    pub mod boot { pub use hal::boot::*; }

    //#[cfg(feature = "io_access")]
    pub mod io { pub use hal::io::*; }
}

pub mod log {
    pub use sys_log::*;
}


//#[cfg(feature = "debugtools")]
pub mod debug { 
    pub mod debugtools { 
        pub use debugtools::serial_log::*;
        pub use debugtools::reg_probe::*; 
    } 
}