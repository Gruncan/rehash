use crate::callback_event;
use crate::tauri::tauri_events::file_open_event::{FileOpenEvent, FileOpenEventCtx};
use crate::video::event::{CallbackController, CallbackEvent};
use js_sys::Reflect;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindings_lib::tauri_listen;
use web_sys::HtmlVideoElement;

pub(crate) type FileOpenEventCtxType = Arc<Mutex<FileOpenEventCtx>>;

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