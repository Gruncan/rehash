use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoStreamMeta {
    pub file_path: String,
    pub current_position: u64,
    pub total_size: u64,
    pub chunk_size: usize,
}


impl VideoStreamMeta {
    pub fn new(file_path: String, current_position: u64, total_size: u64, chunk_size: usize) -> Self {
        Self { file_path, current_position, total_size, chunk_size }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VideoStreamChunk {
    pub bytes: Vec<u8>,
    pub position: u64,
    pub is_final: bool,
}