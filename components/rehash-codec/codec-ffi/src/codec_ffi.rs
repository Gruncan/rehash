use crate::RehashCodecLibrary;
use libloading::Symbol;

impl RehashCodecLibrary {
    pub fn print_codec_version(&self) {
        unsafe {
            let func: Symbol<unsafe extern "C" fn()> = self.lib.get(b"print_codec_version").expect("Failed to load symbol");
            func()
        }
    }
}
