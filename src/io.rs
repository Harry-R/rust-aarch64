use core::ptr;

pub fn write_byte(port_address: u32, byte: u8) {
    /// write a single byte to port address
    let port = port_address as *mut u8;

    // LF newline characters need to be extended to CRLF over a real serial port.
    if byte == b'\n' {
        unsafe { ptr::write_volatile(port, b'\r'); }
    }

    unsafe { ptr::write_volatile(port, byte); }
}