use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct VideoStreamHandler {
    video_stream_contex: Arc<Mutex<Option<VideoStreamContext>>>,
}

#[derive(Debug)]
pub struct VideoStreamContext {
    file: File,
    meta: VideoStreamMeta,
}

impl VideoStreamContext {
    pub fn new(file: File, meta: VideoStreamMeta) -> Self {
        Self { file, meta }
    }

    pub fn update_current_position(&mut self, increment: u64) {
        self.meta.current_position += increment;
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoStreamMeta {
    file_path: String,
    current_position: u64,
    total_size: u64,
    chunk_size: usize,
}

impl VideoStreamMeta {
    pub fn new(file_path: String, current_position: u64, total_size: u64, chunk_size: usize) -> Self {
        Self { file_path, current_position, total_size, chunk_size }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VideoStreamChunk {
    bytes: Vec<u8>,
    position: u64,
    is_final: bool,
}


impl VideoStreamHandler {
    pub fn new() -> Self {
        Self {
            video_stream_contex: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn create_stream(&self, file_path: String, chunk_size: usize) -> Result<VideoStreamMeta, String> {
        let file = File::open(file_path.clone()).await.map_err(|e| e.to_string())?;
        let metadata = file.metadata().await.map_err(|e| e.to_string())?;
        let len = metadata.len();


        let meta = VideoStreamMeta::new(file_path, 0, len, chunk_size);
        let context = VideoStreamContext::new(file, meta.clone());

        let mut stream_context = self.video_stream_contex.lock().await;
        stream_context.replace(context);

        Ok(meta)
    }

    pub async fn read_chunk(&self) -> Result<Option<VideoStreamChunk>, String> {
        let mut stream_context = self.video_stream_contex.lock().await;
        if stream_context.is_none() {
            return Ok(None);
        }

        let mut context = stream_context.take().unwrap();

        let mut buffer = vec![0u8; context.meta.chunk_size];
        match context.file.read(&mut buffer).await {
            Ok(0) => {
                Ok(None)
            }
            Ok(size) => {
                let stream_chunk = VideoStreamChunk {
                    bytes: buffer[..size].to_vec(),
                    position: context.meta.current_position,
                    is_final: context.meta.current_position + size as u64 >= context.meta.total_size,
                };

                context.update_current_position(size as u64);
                stream_context.replace(context);
                Ok(Some(stream_chunk))
            }
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    pub async fn close_stream(&self) -> Result<(), String> {
        Ok(())
    }
}


#[derive(serde::Deserialize)]
struct RangeObject {
    start: u64,
    end: Option<u64>,
}
