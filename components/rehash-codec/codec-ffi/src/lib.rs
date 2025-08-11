pub mod codec;
pub mod codec_ffi;

pub use rehash_codec_library::*;


#[cfg(target_os = "windows")]
pub mod rehash_codec_library {
    use libloading::os::windows::Library;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;
    use windows_sys::Win32::System::LibraryLoader::{AddDllDirectory, SetDefaultDllDirectories, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR};

    pub struct RehashCodecLibrary {
        pub lib: Library,
    }

    impl RehashCodecLibrary {
        #[cfg(target_os = "windows")]
        fn convert_to_wide(s: &OsStr) -> Vec<u16> {
            s.encode_wide().chain(Some(0)).collect::<Vec<u16>>()
        }

        #[cfg(target_os = "windows")]
        pub fn new<T: AsRef<str>>(path: &T) -> Self {
            let path = Path::new(path.as_ref());
            let parent = path.parent().unwrap();

            let lib = unsafe {
                SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);
                let directory = Self::convert_to_wide(parent.as_os_str());
                AddDllDirectory(directory.as_ptr());

                Library::load_with_flags(path.as_os_str(), LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS).expect("Failed to load windows library")
            };

            Self {
                lib
            }
        }
    }
}

#[cfg(target_os = "linux")]
pub mod rehash_codec_library {
    use libloading::Library;

    pub struct RehashCodecLibrary {
        pub lib: Library,
    }


    impl RehashCodecLibrary {
        #[cfg(target_os = "linux")]
        pub fn new<T: AsRef<str>>(path: &T) -> Self {
            let lib = unsafe {
                Library::new(path.as_ref()).expect("Failed to loaded library")
            };

            Self {
                lib
            }
        }
    }
}
