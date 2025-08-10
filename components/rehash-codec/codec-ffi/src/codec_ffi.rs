use crate::RehashCodecLibrary;
use libloading::Symbol;

impl RehashCodecLibrary {
    pub fn add(&self, left: u64, right: u64) -> u64 {
        unsafe {
            let func: Symbol<unsafe extern "C" fn(u64, u64) -> u64> =
                self.lib.get(b"add").expect("Failed to load symbol");
            func(left, right)
        }
    }
    pub fn sub(&self, left: u64, right: u64) -> u64 {
        unsafe {
            let func: Symbol<unsafe extern "C" fn(u64, u64) -> u64> =
                self.lib.get(b"sub").expect("Failed to load symbol");
            func(left, right)
        }
    }
    pub fn sub2(&self, left: u64, right: u64) -> u64 {
        unsafe {
            let func: Symbol<unsafe extern "C" fn(u64, u64) -> u64> =
                self.lib.get(b"sub2").expect("Failed to load symbol");
            func(left, right)
        }
    }
}
