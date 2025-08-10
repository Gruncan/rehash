pub mod codec;
pub mod codec_ffi;

use libloading::{Library, Symbol};

pub struct RehashCodecLibrary<'a>(&'a str);


impl<'a> RehashCodecLibrary<'a> {
    pub fn new<T: AsRef<str>>(path: &'a T) -> Self {
        println!("Loaded rehashcodec!");
        unsafe {
            let lib = Library::new(path.as_ref()).expect("Failed to loaded library");
            let add: Symbol<unsafe extern "C" fn(i32, i32) -> i32> = lib.get(b"add").expect("Failed to load symbol");
            let result = add(10, 20);
            println!("Result {}", result);
        }
        Self {
            0: path.as_ref()
        }
    }
}