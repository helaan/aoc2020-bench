// C++ does not satisfies rust's type names
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::convert::TryInto;
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const PROGRAMS: &[unsafe extern "C" fn(*const i8, i32, *mut *mut u8)] = &[
    aoc01,
    aoc02,
    aoc03,
    aoc04,
    aoc05,
    aoc06
];

pub fn execute(id: usize, data: &[u8]) -> Option<String> {
    if id > PROGRAMS.len() {
        return None;
    }
    let mut output_buf: [u8; 1024] = [0; 1024];
    let output_ptr: *mut *mut u8 = &mut output_buf.as_mut_ptr();

    unsafe {
        // BUG: data does not end on a null terminator but this function assumes it does (which is why I use unchecked)
        let data_ptr = CStr::from_bytes_with_nul_unchecked(data).as_ptr();
        // BUG: may panic if the file is longer than 4GB long (overflows u32)
        PROGRAMS[id - 1](data_ptr, data.len().try_into().unwrap(), output_ptr);
    };

    Some(String::from_utf8_lossy(&output_buf).to_string())
}
