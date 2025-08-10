pub mod codec;
pub mod codec_ffi;

use libloading::Library;

pub struct RehashCodecLibrary {
    lib: Library,
}


impl RehashCodecLibrary {
    pub fn new<T: AsRef<str>>(path: &T) -> Self {
        let lib = unsafe {
            Library::new(path.as_ref()).expect("Failed to loaded library")
        };
        println!("Loaded {}", path.as_ref());
        Self {
            lib
        }
    }
}