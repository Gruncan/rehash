use crate::callback_event;
use crate::html::html_ui::HtmlLoadBar;
use crate::tauri::tauri_callback::file_open_closure::FileOpenClosure;
use crate::tauri::tauri_events::file_open_event::{FileOpenEvent, FileOpenEventCtx};
use crate::tauri::tauri_events::onload_callback::OnLoadCallback;
use crate::video::event::{CallbackController, CallbackEvent};
use crate::video::video_callback::CallbackClosureWrapper;
use js_sys::Reflect;
use rehash_utils::utils::tauri_listen;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlVideoElement;

pub(crate) type FileOpenEventCtxType = Arc<Mutex<FileOpenEventCtx>>;

#[derive(Clone)]
pub(crate) struct FileOpenCallbackController {
    ctx: FileOpenEventCtxType,
    open_event: Rc<RefCell<dyn CallbackEvent<FileOpenEventCtxType>>>,
}


impl FileOpenCallbackController {
    pub fn new(video_element: HtmlVideoElement, load_bar: HtmlLoadBar) -> Self {
        let open_event = callback_event!(FileOpenEvent);
        let load_callback = Box::new(OnLoadCallback { ctx: load_bar.clone() });
        let closure = CallbackClosureWrapper::create_callback(load_callback);
        video_element.set_onloadeddata(Some(closure.as_ref().as_ref().unchecked_ref()));

        closure.forget();

        Self {
            ctx: Arc::new(Mutex::new(FileOpenEventCtx { video_element, video_path: None, load_bar })),
            open_event,
        }
    }
}

impl CallbackController for FileOpenCallbackController {
    fn register_events(&self) {
        let file_open = Box::new(FileOpenClosure::new(self.ctx.clone(), self.open_event.clone()));
        let file_open_closure = CallbackClosureWrapper::create_callback(file_open);

        tauri_listen("select-video-event", file_open_closure.as_ref().as_ref().unchecked_ref());

        file_open_closure.forget();
    }
}

mod file_open_closure {
    use super::*;

    type Ctx = FileOpenEventCtxType;
    type Callback = Rc<RefCell<dyn CallbackEvent<FileOpenEventCtxType>>>;

    #[derive(Debug)]
    pub(crate) struct FileOpenClosure {
        ctx: Ctx,
        callback: Callback,
    }

    impl FileOpenClosure {
        pub(crate) fn new(ctx: Ctx, callback: Callback) -> Self {
            Self {
                ctx,
                callback,
            }
        }
    }

    impl CallbackClosureWrapper<JsValue> for FileOpenClosure {
        fn closure(&mut self, event: JsValue) {
            {
                let mut mutex = self.ctx.lock().unwrap();
                let payload = Reflect::get(&event, &JsValue::from_str("payload")).expect("Failed to get payload");

                (*mutex).video_path = payload.as_string();
            }

            let mut callback = self.callback.borrow_mut();
            callback.trigger(&mut self.ctx).expect("Failed callback");
        }
    }
}