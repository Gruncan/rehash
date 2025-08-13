// use std::ffi::CStr;
// use std::os::raw::{c_char, c_int};
// use ffmpeg_next::codec::traits::Encoder;
// use ffmpeg_next::codec::Context;
// use ffmpeg_next::format::context::Output;
// use ffmpeg_next::format::Pixel;
// use ffmpeg_next::media::Type;
// use ffmpeg_next::{codec, format, frame};
// use rehash_codec_proc_macro::rehash_codec_ffi;
// use std::path::Path;
// use std::ptr;
// use crate::codec::Mp4Fragmenter;
//
// #[rehash_codec_ffi]
// fn print_codec_version() {
//     println!("Codec version: {}", env!("CARGO_PKG_VERSION"));
// }
//
//
// #[repr(C)]
// pub struct CMp4Fragmenter {
//     fragmenter: *mut Mp4Fragmenter,
// }
//
// #[rehash_codec_ffi]
// fn mp4_fragmenter_new(path: *const c_char, fragment_duration: f64) -> *mut CMp4Fragmenter {
//     if path.is_null() {
//         return ptr::null_mut();
//     }
//
//     let c_str = unsafe { CStr::from_ptr(path) };
//     let path_str = match c_str.to_str() {
//         Ok(s) => s,
//         Err(_) => return ptr::null_mut(),
//     };
//
//     let fragmenter = match Mp4Fragmenter::new(path_str, fragment_duration) {
//         Ok(f) => Box::into_raw(Box::new(f)),
//         Err(_) => return ptr::null_mut(),
//     };
//
//     let c_fragmenter = CMp4Fragmenter { fragmenter };
//     Box::into_raw(Box::new(c_fragmenter))
// }
//
//
// #[rehash_codec_ffi]
// fn mp4_fragmenter_create_fragment(fragmenter: *mut crate::codec::CMp4Fragmenter, start_time: f64, data_out: *mut *mut u8, size_out: *mut usize) -> c_int {
//     if fragmenter.is_null() || data_out.is_null() || size_out.is_null() {
//         return -1;
//     }
//
//     let fragmenter_ref = unsafe { &mut *(*fragmenter).fragmenter };
//
//     match fragmenter_ref.create_fragment(start_time) {
//         Ok(data) => {
//             let size = data.len();
//             let data_ptr = data.as_ptr() as *mut u8;
//
//             // Allocate C memory and copy data
//             let c_data = unsafe { libc::malloc(size) as *mut u8 };
//             if c_data.is_null() {
//                 return -1;
//             }
//
//             unsafe {
//                 ptr::copy_nonoverlapping(data_ptr, c_data, size);
//                 *data_out = c_data;
//                 *size_out = size;
//             }
//
//             0
//         }
//         Err(_) => -1,
//     }
// }
//
//
// #[rehash_codec_ffi]
// fn mp4_fragmenter_free(fragmenter: *mut crate::codec::CMp4Fragmenter) {
//     if !fragmenter.is_null() {
//         unsafe {
//             let fragmenter = Box::from_raw(fragmenter);
//             let _ = Box::from_raw(fragmenter.fragmenter);
//         }
//     }
// }
//
// #[rehash_codec_ffi]
// fn mp4_free_data(data: *mut u8) {
//     if !data.is_null() {
//         unsafe { libc::free(data as *mut libc::c_void) };
//     }
// }