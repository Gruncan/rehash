use crate::video::event::CallbackEvent;
use crate::JsResult;
use js_sys::Reflect;
use rehash_utils::utils::tauri_invoke;
use std::sync::{Arc, Mutex};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Blob, BlobPropertyBag, HtmlVideoElement, Url};

use crate::prelude::*;
use crate::tauri::tauri_events::file_open_event::FileOpenEventCtx;

pub(crate) type FileOpenEventCtxType = Arc<Mutex<FileOpenEventCtx>>;

pub(crate) mod file_open_event {
    use super::*;
    use crate::html::html_ui::HtmlLoadBar;
    use wasm_bindgen::JsValue;

    #[derive(Debug, Clone)]
    pub(crate) struct FileOpenEvent {}

    #[derive(Debug, Clone)]
    pub(crate) struct FileOpenEventCtx {
        pub(crate) video_element: HtmlVideoElement,
        pub(crate) video_path: Option<String>,
        pub(crate) load_bar: HtmlLoadBar,
    }


    impl CallbackEvent<FileOpenEventCtxType> for FileOpenEvent {
        fn trigger(&mut self, ctx: &mut FileOpenEventCtxType) -> JsResult<()> {
            #[cfg(debug_assertions)]
            {
                let mutex = ctx.lock().unwrap();
                let video_element = &mutex.video_element;
                video_element.set_onloadstart(Some(&js_sys::Function::new_no_args("console.log('Video load started')")));
                video_element.set_oncanplay(Some(&js_sys::Function::new_no_args("console.log('Video can play')")));
                // video_element.set_onloadeddata(Some(&js_sys::Function::new_no_args("console.log('Video data loaded')")));
                video_element.set_onerror(Some(&js_sys::Function::new_no_args("console.log('Video error:', this.error)")));
            }
            let arc_ctx = ctx.clone();
            spawn_local(async move {
                let mutex = arc_ctx.lock().unwrap();
                if let Some(string) = &mutex.video_path {
                    load_video_blob(&mutex.video_element, string, &mutex.load_bar).await;
                }
            });

            Ok(())
        }

    }

    impl FileOpenEvent {
        pub fn new() -> Self {
            Self {}
        }
    }

    async fn load_video_blob(video_element: &HtmlVideoElement, file_path: &String, load_bar: &HtmlLoadBar) {
        let args = js_sys::Object::new();
        Reflect::set(&args, &"path".into(), &file_path.into()).unwrap();


        match JsFuture::from(tauri_invoke("get_video", args.into())).await {
            Ok(js_length) => {
                load_bar.show_loader();
                let const_length = js_length.as_f64().expect("Expected a number") as u32;
                debug_console_log!("File frontend length: {}", const_length);
                let mut vec = Vec::with_capacity(const_length as usize);
                let mut i = 0;
                let mut length = 0;
                while length < const_length {
                    match JsFuture::from(tauri_invoke("get_video_chunk", JsValue::NULL)).await {
                        Ok(js_chunk) => {
                            let uint8_array = js_sys::Uint8Array::new(&js_chunk);
                            let chunk_length = uint8_array.length();
                            let mut chunk_vec = vec![0u8; chunk_length as usize];
                            uint8_array.copy_to(&mut chunk_vec);
                            vec.extend_from_slice(&chunk_vec);
                            i += 1;
                            length += chunk_length;
                            load_bar.update_progress(length as f64, const_length as f64);
                        },
                        Err(e) => {
                            debug_console_log!("Failed to load read chunk: {}", i+1);
                            debug_console_log!("{:?}", e);
                            break
                        }
                    }
                }

                let final_array = js_sys::Uint8Array::new_with_length(vec.len() as u32);
                final_array.copy_from(&vec);

                let array = js_sys::Array::new();
                array.push(&final_array);

                let blob_options = BlobPropertyBag::new();
                blob_options.set_type("video/mp4");

                match Blob::new_with_u8_array_sequence_and_options(&array, &blob_options) {
                    Ok(blob) => {
                        match Url::create_object_url_with_blob(&blob) {
                            Ok(blob_url) => {
                                debug_console_log!("Created blob URL: {}", blob_url);
                                video_element.set_src(&blob_url);
                                video_element.load();
                            }
                            Err(e) => { debug_console_log!("Failed to create blob URL: {:?}", e); }
                        }
                    }
                    Err(e) => { debug_console_log!("Failed to create blob: {:?}", e); }
                }
            }
            Err(e) => {
                debug_console_log!("Backend error: {:?}", e);
            }
        }
    }
}

pub(crate) mod onload_callback {
    use crate::html::html_ui::HtmlLoadBar;
    use crate::log_to_tauri;
    use crate::CallbackClosureWrapper;
    use rehash_utils::console_log;


    #[derive(Debug, Clone)]
    pub(crate) struct OnLoadCallback {
        pub(crate) ctx: HtmlLoadBar,

    }

    impl OnLoadCallback {
        pub(crate) fn new(ctx: &HtmlLoadBar) -> Self {
            Self {
                ctx: ctx.clone()
            }
        }
    }

    impl CallbackClosureWrapper<web_sys::Event> for OnLoadCallback {
        fn closure(&mut self, _: web_sys::Event) {
            console_log!("Loaded data");
            self.ctx.hide_loader();
        }
    }
}