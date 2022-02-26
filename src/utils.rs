use windows::core::PCWSTR;

/// Converts rgb values into a single unsigned 32bit integer
/// Individual values must be below or equal to 255
/// Can be expressed as a constant value
#[inline]
pub const fn rgb(r: u32, g: u32, b: u32) -> u32 {
    ((b & 0x0ff) << 16) | ((g & 0x0ff) << 8) | (r & 0x0ff)
}

/// rgb value but includes an alpha value
#[inline]
pub const fn rgba(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (a << 24) | rgb(r, g, b)
}

/// Modified from the example in rust docs
/// https://doc.rust-lang.org/std/iter/trait.Iterator.html
#[inline]
pub fn str_to_utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Creates a wide-pointer-string
#[inline]
pub fn str_to_pcwstr(s: &str) -> PCWSTR {
    PCWSTR(str_to_utf16(s).as_mut_ptr())
}

/// The lower/last 16 bits of 32 bits of information
#[inline]
#[allow(non_snake_case)]
pub fn LOWORD(dword: u32) -> u16 {
    // Just drop the upper bits
    dword as u16
}

/// The upper/first 16 bits of 32 bits of information
#[inline]
#[allow(non_snake_case)]
pub fn HIWORD(dword: u32) -> u16 {
    // Shift all of the bits 16 places and drop
    (dword >> 16) as u16
}

#[inline]
#[allow(non_snake_case)]
pub fn GET_X_LPARAM(dword: u32) -> i32 {
    LOWORD(dword as u32) as i16 as i32
}

#[inline]
#[allow(non_snake_case)]
pub fn GET_Y_LPARAM(dword: u32) -> i32 {
    HIWORD(dword as u32) as i16 as i32
}
