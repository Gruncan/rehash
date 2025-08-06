use crate::tauri::tauri_events::file_open_event::FileOpenEventCtx;
use crate::video::event::CallbackEvent;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use wasm_bindgen_futures::spawn_local;
use web_sys::MediaSource;
use web_sys::{HtmlVideoElement, Url};

pub use crate::prelude::*;

pub(crate) type FileOpenEventCtxType = Arc<Mutex<FileOpenEventCtx>>;


pub(crate) mod file_open_event {
    use super::*;
    use crate::tauri::tauri_callback::source_open_closure::SourceOpenClosure;
    use crate::tauri::tauri_events::source_open_event::{SourceOpenEvent, SourceOpenEventCtx};
    use crate::video::video_callback::CallbackClosureWrapper;
    use rehash_utils::codec::VideoStreamMeta;
    use rehash_utils::utils::tauri_invoke;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::{JsCast, JsValue};

    #[derive(Debug, Clone)]
    pub(crate) struct FileOpenEvent {}

    #[derive(Debug, Clone)]
    pub(crate) struct FileOpenEventCtx {
        pub(crate) video_element: HtmlVideoElement,
        pub(crate) video_path: Option<String>,
    }


    impl CallbackEvent<FileOpenEventCtxType> for FileOpenEvent {
        fn trigger(&mut self, ctx: &mut FileOpenEventCtxType) -> RehashResultUnit {
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

    async fn start_stream(html_video_element: &HtmlVideoElement, stream_meta: VideoStreamMeta) -> RehashResultUnit {
        let media_source = MediaSource::new()?;
        let url = Url::create_object_url_with_source(&media_source)?;
        debug_console_log!("url: {}", url);
        html_video_element.set_src(url.as_str());

        let ctx = Arc::new(Mutex::new(SourceOpenEventCtx { media_source: media_source.clone(), video_element: html_video_element.clone() }));
        let event = Rc::new(RefCell::new(SourceOpenEvent {}));

        let open_source_closure = CallbackClosureWrapper::create_callback(Box::new(SourceOpenClosure::new(ctx, event)));

        media_source.add_event_listener_with_callback("sourceopen", open_source_closure.as_ref().as_ref().unchecked_ref()).expect("Failed to create sourceopen callback");
        open_source_closure.forget();
        Ok(())
    }

    async fn load_video_blob(video_element: &HtmlVideoElement, file_path: &String) {
        let args = into_object!("path" => file_path).unwrap();

        debug_console_log!("{:?}", args);
        match into_async!(tauri_invoke("create_video_stream", args.into())).await {
            Ok(result) => {
                let object: VideoStreamMeta = serde_wasm_bindgen::from_value(result).unwrap();
                debug_console_log!("create_video_stream returned {:?}", object);
                start_stream(video_element, object).await.expect("Failed to start stream");

            },
            Err(e) => {
                error_log!("create_video_stream returned error {:?}", e);
            }
        }

    }
}

pub(crate) mod source_open_event {
    use super::*;
    use rehash_utils::codec::VideoStreamChunk;
    use rehash_utils::utils::tauri_invoke;
    use wasm_bindgen::JsValue;

    pub type SourceEventCtxType = Arc<Mutex<SourceOpenEventCtx>>;

    #[derive(Debug)]
    pub(crate) struct SourceOpenEventCtx {
        pub(crate) video_element: HtmlVideoElement,
        pub(crate) media_source: MediaSource,
    }

    #[derive(Debug, Clone)]
    pub struct SourceOpenEvent {}

    impl CallbackEvent<SourceEventCtxType> for SourceOpenEvent {
        fn trigger(&mut self, ctx: &mut SourceEventCtxType) -> RehashResultUnit {
            let source_buffer = {
                let mutex = ctx.lock().unwrap();

                mutex.media_source.add_source_buffer("video/mp4; codecs=\"avc1.42E01E, mp4a.40.2\"")
                    .or_else(|e| Err(format!("Failed to create source buffer: {:?}", e)))?
            };

            // let event = Rc::new(RefCell::new(UpdateEndEvent {}));
            // let update_end_closure = CallbackClosureWrapper::create_callback(Box::new(UpdateEndClosure::new((), event)));
            // source_buffer.add_event_listener_with_callback("updateend", update_end_closure.as_ref().as_ref().unchecked_ref()).expect("Failed to create updateend callback");
            // update_end_closure.forget();

            let ctx = ctx.clone();
            debug_console_log!("source buffer created: {:?}", source_buffer);
            spawn_local(async move {
                match into_async!(tauri_invoke("get_chunk", JsValue::NULL)).await {
                    Ok(obj) => {
                        let mutex = ctx.lock().unwrap();
                        let chunk: VideoStreamChunk = serde_wasm_bindgen::from_value(obj).unwrap();
                        debug_console_log!("get_chunk returned {:?}", chunk);
                        let uint8_array = js_sys::Uint8Array::from(&chunk.bytes[..]);
                        debug_console_log!("Received chunk, length {}", uint8_array.length());
                        source_buffer.append_buffer_with_array_buffer(&uint8_array.buffer()).expect("Failed to append source buffer");
                        // mutex.media_source.end_of_stream().expect("Failed to end source stream");
                        mutex.video_element.play().expect("Failed to play video");
                    },
                    Err(e) => {
                        error_log!("Failed to get chunk {}", e.as_string().unwrap());
                    }
                }
            });
            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<SourceEventCtxType>> {
            Box::new(self.clone())
        }
    }
}

pub(crate) mod update_end_event {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct UpdateEndEvent {}


    impl CallbackEvent<()> for UpdateEndEvent {
        fn trigger(&mut self, _data: &mut ()) -> RehashResultUnit {
            debug_console_log!("Trigger UpdateEndEvent");
            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<()>> {
            Box::new(self.clone())
        }
    }
}
