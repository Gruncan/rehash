pub(crate) use crate::video::event::CallbackEvent;
pub(crate) use crate::video::video_player::{SharedVideoPlayer, VideoPlayer, VideoPlayerState, VideoPlayerTypeState};
use std::fmt::Debug;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

pub(crate) trait ClosureWrapperEventType {}

pub(crate) trait CallbackClosureWrapper<T>: Debug
where
    T: ClosureWrapperEventType + JsCast + wasm_bindgen::convert::FromWasmAbi + Debug + 'static,
    Self: 'static,
{
    fn closure(&mut self, event: T);

    fn create_callback(mut this: Box<Self>) -> Box<Closure<dyn FnMut(T)>> {
        // debug_console_log!("{:#}", this);
        let closure: Box<Closure<dyn FnMut(T)>> = Box::new(Closure::new(Box::new(move |event: T| {
            this.closure(event.dyn_into().unwrap())
        })));
        closure
    }
}

impl ClosureWrapperEventType for web_sys::Event {}
impl ClosureWrapperEventType for web_sys::MouseEvent {}
impl ClosureWrapperEventType for web_sys::KeyboardEvent {}
impl ClosureWrapperEventType for JsValue {}
