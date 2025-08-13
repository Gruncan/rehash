/* THIS FILE IS GENERATED DO NOT EDIT */
use crate::RehashCodecLibrary;
#[cfg(target_os = "windows")]
use libloading::os::windows::Symbol;
#[cfg(target_os = "linux")]
use libloading::Symbol;
use std::ffi::{c_char, c_uchar};

impl RehashCodecLibrary {
    pub fn print_codec_version(&self) {
        unsafe {
            let func: Symbol<unsafe extern "C" fn()> = self
                .lib
                .get(b"print_codec_version")
                .expect("Failed to load symbol");
            func()
        }
    }
    pub fn get_bytes_from_video(&self, path: *const c_char, out_len: *mut usize) -> *mut c_uchar {
        unsafe {
            let func: Symbol<unsafe extern "C" fn(*const c_char, *mut usize) -> *mut c_uchar> =
                self.lib
                    .get(b"get_bytes_from_video")
                    .expect("Failed to load symbol");
            func(path, out_len)
        }
    }
    pub fn free_file_bytes(&self, ptr: *mut c_uchar, len: usize) {
        unsafe {
            let func: Symbol<unsafe extern "C" fn(*mut c_uchar, usize)> = self
                .lib
                .get(b"free_file_bytes")
                .expect("Failed to load symbol");
            func(ptr, len)
        }
    }
}
