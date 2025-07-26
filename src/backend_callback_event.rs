use crate::event::{CallbackController, CallbackEvent, CallbackEventInit};
use crate::log_to_tauri;
use crate::prelude::{tauri_invoke, tauri_listen};
use crate::{callback_event, debug_console_log, JsResult};
use js_sys::{Array, Reflect};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Blob, BlobPropertyBag, HtmlVideoElement, Url};


type FileOpenEventCtxType = Arc<Mutex<FileOpenEventCtx>>;

#[derive(Clone)]
pub(crate) struct FileOpenCallbackController {
    ctx: FileOpenEventCtxType,
    open_event: Rc<RefCell<dyn CallbackEvent<FileOpenEventCtxType>>>,
}


impl FileOpenCallbackController {
    pub fn new(video_element: HtmlVideoElement) -> Self {
        let open_event = callback_event!(FileOpenEvent);

        Self {
            ctx: Arc::new(Mutex::new(FileOpenEventCtx { video_element, video_path: None })),
            open_event,
        }
    }
}

impl CallbackController for FileOpenCallbackController {
    fn register_events(&mut self) {
        let mut ctx: FileOpenEventCtxType = self.ctx.clone();
        let open_event = self.open_event.clone();

        let open_closure: Box<Closure<dyn FnMut(JsValue)>> = Box::new(Closure::new(move |event: JsValue| {
            {
                let mut mutex = ctx.lock().unwrap();
                let payload = Reflect::get(&event, &JsValue::from_str("payload")).expect("Failed to get payload");

                (*mutex).video_path = payload.as_string();
            }

            let mut callback = open_event.borrow_mut();
            callback.trigger(&mut ctx).expect("Failed callback");
        }));

        tauri_listen("select-video-event", open_closure.as_ref().as_ref().unchecked_ref());

        open_closure.forget();
    }
}


#[derive(Debug)]
pub(crate) struct FileOpenEvent {}

#[derive(Clone)]
pub(crate) struct FileOpenEventCtx {
    video_element: HtmlVideoElement,
    video_path: Option<String
    >,
}


impl CallbackEventInit for FileOpenEvent {
    fn new() -> Self {
        Self {}
    }
}

impl CallbackEvent<FileOpenEventCtxType> for FileOpenEvent
{
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
}

async fn load_video_blob(video_element: &HtmlVideoElement, file_path: &String) {
    let args = js_sys::Object::new();
    Reflect::set(&args, &"path".into(), &file_path.into()).unwrap();

    match JsFuture::from(tauri_invoke("get_video_bytes", args.into())).await {
        Ok(result) => {
            let uint8_array = js_sys::Uint8Array::new(&result);
            debug_console_log!("Received {} bytes", uint8_array.length());

            let array = Array::new();
            array.push(&uint8_array);

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