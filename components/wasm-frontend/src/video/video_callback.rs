pub(crate) use crate::video::event::CallbackEvent;
pub(crate) use crate::video::video_player::{SharedVideoPlayer, VideoPlayer, VideoPlayerState, VideoPlayerTypeState};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;


pub(crate) trait ClosureWrapperEventType {}

pub(crate) trait CallbackClosureWrapper<T>
where
    T: ClosureWrapperEventType + 'static + JsCast,
    Self: 'static,
{
    fn closure(&mut self, event: T);

    fn create_callback(this: Rc<RefCell<Self>>) -> Box<Closure<dyn FnMut(web_sys::Event)>> {
        let closure: Box<Closure<dyn FnMut(web_sys::Event)>> = Box::new(Closure::new(Box::new(move |event: web_sys::Event| {
            let mut instance = this.borrow_mut();
            instance.closure(event.dyn_into().unwrap())
        })));
        closure
    }
}

impl ClosureWrapperEventType for web_sys::MouseEvent {}
impl ClosureWrapperEventType for web_sys::Event {}
