use rehash_codec_ffi::RehashCodecLibrary;
use std::sync::Mutex;


const CHUNK_SIZE: usize = 500_000;

pub struct VideoState {
    pub(crate) codec: Mutex<RehashCodecLibrary>,
    pub(crate) bytes: Mutex<Option<Vec<u8>>>,
    pos: Mutex<usize>,
}


impl VideoState {
    pub fn new(codec: RehashCodecLibrary) -> VideoState {
        Self {
            codec: Mutex::new(codec),
            bytes: Mutex::new(None),
            pos: Mutex::new(0),
        }
    }

    pub fn set_bytes(&self, bytes: Vec<u8>) {
        self.bytes.lock().unwrap().replace(bytes);
        *self.pos.lock().unwrap() = 0;
    }

    pub fn get_bytes(&self) -> Option<Vec<u8>> {
        let bytes = self.bytes.lock().unwrap();
        let mut p = self.pos.lock().unwrap();
        let bytes = bytes.as_ref()?;
        let upper = if *p + CHUNK_SIZE >= bytes.len() {
            bytes.len()
        } else {
            *p + CHUNK_SIZE
        };
        println!("{}..{}", p, bytes.len());

        let slice = bytes[*p..upper].to_vec();
        *p = upper;
        Some(slice)
    }
}