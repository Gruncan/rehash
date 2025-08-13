use rehash_codec_proc_macro::rehash_codec_ffi;
use std::ffi::{c_char, c_uchar, CStr};
use std::fs::File;
use std::io::Read;

#[rehash_codec_ffi]
fn print_codec_version() {
    println!("Codec version: {}", env!("CARGO_PKG_VERSION"));
}


fn get_video_bytes(path: &str) -> Result<Vec<u8>, String> {
    println!("Reading video file: {}", path);
    let mut file = File::open(path).ok().ok_or("Failed to open video file")?;
    let metadata = file.metadata().ok().ok_or("Failed to read metadata")?;
    let mut buffer = Vec::with_capacity(metadata.len() as usize);

    match file.read_to_end(&mut buffer) {
        Ok(size) => {
            assert_eq!(size, metadata.len() as usize);
            println!("Successfully read {} bytes", size);
            Ok(buffer)
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(e.to_string())
        }
    }
}

#[rehash_codec_ffi]
fn get_bytes_from_video(path: *const c_char, out_len: *mut usize) -> *mut c_uchar {
    let c_str = unsafe {
        assert!(!path.is_null());
        CStr::from_ptr(path)
    };

    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(), // invalid UTF-8
    };

    let bytes = match get_video_bytes(path_str) {
        Ok(bytes) => bytes,
        Err(_) => return std::ptr::null_mut(),
    };

    let len = bytes.len();

    unsafe {
        if !out_len.is_null() {
            *out_len = len;
        }
    }
    let ptr = bytes.as_ptr() as *mut c_uchar;
    std::mem::forget(bytes);
    ptr
}

#[rehash_codec_ffi]
fn free_file_bytes(ptr: *mut c_uchar, len: usize) {
    if !ptr.is_null() && len > 0 {
        unsafe {
            drop(Vec::from_raw_parts(ptr, len, len));
        }
    }
}