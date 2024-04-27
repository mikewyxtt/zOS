/*  console.rs - UEFI console driver
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


use crate::libuefi::protocol::simple_text_output::SimpleTextOutputProtocol;


pub fn putc(c: char) {
    // UEFI requires UTF-16 string literals.
    let mut utf16c = [0; 2];
    char::encode_utf16(c, &mut utf16c);

    SimpleTextOutputProtocol::output_string(utf16c.as_ptr());

    // UEFI is similar to serial in that you have to write the carriage return as well as the newline to reset the cursor
    if c == '\n' {
        let mut utf16c = [0; 2];
        char::encode_utf16('\r', &mut utf16c);

        SimpleTextOutputProtocol::output_string(utf16c.as_ptr());
    }
}


pub fn clear() {
    SimpleTextOutputProtocol::reset();
}