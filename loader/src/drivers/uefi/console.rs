use crate::libuefi::protocol::simple_text_output::SimpleTextOutputProtocol;


pub fn putc(c: char) {
    let mut c_u16: [u16; 2] = [0,0];
    c.encode_utf16(&mut c_u16);
    
    // UEFI requires UTF-16 string literals. So, we simply create a 16 bit array with the char and '\0' so it thinks it is the end of the string, then call the UEFI output_string function with a pointer to the array. It works...
    let utf16_str: [[u16; 2]; 2] = [c_u16, [0,0]];
    SimpleTextOutputProtocol::output_string(utf16_str[0].as_ptr());

    // UEFI is similar to serial in that you have to write the carriage return as well as the newline to reset the cursor
    if c == '\n' {
        let c = '\r';
        let mut c_u16: [u16; 2] = [0,0];
        c.encode_utf16(&mut c_u16);

        let utf16_str: [[u16; 2]; 2] = [c_u16, [0,0]];
        SimpleTextOutputProtocol::output_string(utf16_str[0].as_ptr());
    }
}


pub fn clear() {
    SimpleTextOutputProtocol::reset();
}