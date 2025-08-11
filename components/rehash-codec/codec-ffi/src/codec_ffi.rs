#[cfg(unix)]
use libloading::Symbol;

#[cfg(target_os = "windows")]
use libloading::os::windows::Symbol;

use crate::RehashCodecLibrary;

impl RehashCodecLibrary {
    pub fn print_codec_version(&self) {
        unsafe {
            let func: Symbol<unsafe extern "C" fn()> = self.lib.get(b"print_codec_version").expect("Failed to load symbol");
            func()
        }
    }
    pub fn run_codec_test(&self) {
        unsafe {
            let func: Symbol<unsafe extern "C" fn()> = self.lib.get(b"run_codec_test").expect("Failed to load symbol");
            func()
        }
    }
}
