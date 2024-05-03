/*  config.rs - Bootloader configuration stuff
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

use alloc::string::{String, ToString};
use crate::{fs::{self, File}, ldrprintln, libuefi::GUID};


pub struct Config {
    pub rootfs:         GUID,
    pub resolution:     String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rootfs:     GUID::new(0,0,0, [0; 8]),
            resolution: String::from("native"),
        }
    }
}



/// Reads and parses the cfg file from the ESP
pub fn parse_cfg() -> Config {
    let mut config = Config::default();

    let f = File::open(fs::get_esp_guid(), "/EFI/BOOT/ZOS/LOADER.CFG");
    let s = f.read_to_string();

    for line in s.lines() {
        let (key, value) = parse_key_value_pair(line);

        match key.as_str() {
            "root" => {
                config.rootfs = GUID::new_from_string(&value);
            }

            "resolution" => {
                config.resolution=value;
            }

            _ => { ldrprintln!("WARNING: Unknown configuration option \"{}\". Ignoring.", key); }
        }
    }

    config
}



/// Parses a key="value" pair
pub fn parse_key_value_pair(line: &str) -> (String, String) {

    if !line.contains('=') {
        ldrprintln!("WARNING: Unknown configuration line \"{}\". Ignoring.", line);
        return (String::new(), String::new())
    }

    let (key, value) = line.split_once('=').unwrap();
    let key = key.trim();
    let value = value.trim();
    let value = value.trim_matches('"');

    (key.to_string(), value.to_string())
}