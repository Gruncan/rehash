use std::error::Error;
use std::ffi::{c_char, c_void, CStr, CString};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const EXTERNAL_DEP_URL: &str = "https://github.com/Gruncan/rehash/releases/download/ffmpeg-win64-dep/rehash-win64-dep.zip";

#[link(name = "msi")]
unsafe extern "system" {
    #[allow(non_snake_case)]
    fn MsiGetPropertyA(hInstall: *mut c_void, szName: *const c_char, szValueBuf: *mut c_char, pchValueBuf: *mut u32) -> u32;
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn DownloadRehashDependency(_session: *mut c_void) -> u32 {
    match run(_session) {
        Ok(_) => 1,
        Err(e) => {
            eprintln!("rehash-desktop failed to download rehash dependencies: {e:?}");
            3
        }
    }
}

fn get_property(session: *mut c_void, name: &str) -> Result<String, Box<dyn Error>> {
    let cname = CString::new(name)?;
    unsafe {
        let mut buf_len: u32 = 0;
        MsiGetPropertyA(session, cname.as_ptr(), std::ptr::null_mut(), &mut buf_len);
        let mut buf = vec![0u8; buf_len as usize];
        let ret = MsiGetPropertyA(session, cname.as_ptr(),
                                  buf.as_mut_ptr() as *mut c_char, &mut buf_len);
        if ret != 0 {
            return Err(format!("MsiGetPropertyA failed: {ret}").into());
        }
        let s = CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned();
        Ok(s)
    }
}


fn run(session: *mut c_void) -> Result<(), Box<dyn Error>> {
    let rehash_install_dir = get_property(session, "RehashInstallDirectory")?;
    let dependency_dir = Path::new(&rehash_install_dir).join("codec");
    let dep_file = dependency_dir.join("rehash_dependency.zip");

    println!("Downloading {}...", EXTERNAL_DEP_URL);
    let bytes = reqwest::blocking::get(EXTERNAL_DEP_URL)?.bytes()?;
    println!("Downloaded {} bytes", bytes.len());

    println!("Extracting download to {}", dependency_dir.display());
    fs::create_dir_all(&dependency_dir)?;

    // let reader = Cursor::new(bytes);
    let mut file = File::create(&dep_file)?;
    file.write_all(&bytes)?;
    println!("Unpacked {}", dep_file.display());

    Ok(())
}