use crate::prelude::*;
use crate::tauri::tauri_events::file_open_event::FileOpenEventCtx;
use crate::video::event::CallbackEvent;
use crate::JsResult;
use js_sys::Reflect;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_bindings_lib::{debug_console_log, tauri_invoke};
use web_sys::MediaSource;
use web_sys::{HtmlVideoElement, Url};

pub(crate) type FileOpenEventCtxType = Arc<Mutex<FileOpenEventCtx>>;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoStreamMeta {
    file_path: String,
    current_position: u64,
    total_size: u64,
    chunk_size: usize,
}


pub(crate) mod file_open_event {
    use super::*;
    use wasm_bindgen::JsValue;

    #[derive(Debug, Clone)]
    pub(crate) struct FileOpenEvent {}

    #[derive(Debug, Clone)]
    pub(crate) struct FileOpenEventCtx {
        pub(crate) video_element: HtmlVideoElement,
        pub(crate) video_path: Option<String>,
    }


    impl CallbackEvent<FileOpenEventCtxType> for FileOpenEvent {
        fn trigger(&mut self, ctx: &mut FileOpenEventCtxType) -> JsResult<()> {
            #[cfg(debug_assertions)]
            {
                let mutex = ctx.lock().unwrap();
                let video_element = &mutex.video_element;
                video_element.set_onloadstart(Some(&js_sys::Function::new_no_args("console.log('Video load started')")));
                video_element.set_oncanplay(Some(&js_sys::Function::new_no_args("console.log('Video can play')")));
                video_element.set_onloadeddata(Some(&js_sys::Function::new_no_args("console.log('Video data loaded')")));
                video_element.set_onerror(Some(&js_sys::Function::new_no_args("console.log('Video error:', this.error)")));
            }

            let arc_ctx = ctx.clone();
            spawn_local(async move {
                let mutex = arc_ctx.lock().unwrap();
                if let Some(string) = &mutex.video_path {
                    load_video_blob(&mutex.video_element, string).await;
                }
            });

            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<FileOpenEventCtxType>> {
            Box::new(self.clone())
        }
    }

    impl FileOpenEvent {
        pub fn new() -> Self {
            Self {}
        }
    }

    async fn start_stream(html_video_element: HtmlVideoElement, stream_meta: VideoStreamMeta) -> Result<(), JsValue> {
        let media_source = MediaSource::new()?;
        let url = Url::create_object_url_with_source(&media_source)?;
        html_video_element.set_src(url.as_str());


        // media_source.add_event_listener_with_callback("sourceopen")

        Ok(())
    }

    async fn load_video_blob(video_element: &HtmlVideoElement, file_path: &String) {
        let args = js_sys::Object::new();
        Reflect::set(&args, &"path".into(), &file_path.into()).unwrap();

        debug_console_log!("{:?}", args);
        match JsFuture::from(tauri_invoke("create_video_stream", args.into())).await {
            Ok(result) => {
                let object: VideoStreamMeta = serde_wasm_bindgen::from_value(result).unwrap();
                debug_console_log!("create_video_stream returned {:?}", object);
                // start_stream().await;
            },
            Err(e) => {
                error_log!("create_video_stream returned error {:?}", e);
            }
        }

        // match JsFuture::from(tauri_invoke("get_video_bytes", args.into())).await {
        //     Ok(result) => {
        //         let uint8_array = js_sys::Uint8Array::new(&result);
        //         debug_console_log!("Received {} bytes", uint8_array.length());
        //
        //         let array = Array::new();
        //         array.push(&uint8_array);
        //
        //         let blob_options = BlobPropertyBag::new();
        //         blob_options.set_type("video/mp4");
        //
        //         match Blob::new_with_u8_array_sequence_and_options(&array, &blob_options) {
        //             Ok(blob) => {
        //                 match Url::create_object_url_with_blob(&blob) {
        //                     Ok(blob_url) => {
        //                         debug_console_log!("Created blob URL: {}", blob_url);
        //                         video_element.set_src(&blob_url);
        //                         video_element.load();
        //                     }
        //                     Err(e) => { debug_console_log!("Failed to create blob URL: {:?}", e); }
        //                 }
        //             }
        //             Err(e) => { debug_console_log!("Failed to create blob: {:?}", e); }
        //         }
        //     }
        //     Err(e) => {
        //         debug_console_log!("Backend error: {:?}", e);
        //     }
        // }
    }
}