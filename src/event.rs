use crate::JsResult;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub(crate) trait CallbackController {
    fn register_events(&mut self);
}

pub(crate) type CallbackEventType<T> = Rc<RefCell<T>>;


pub(crate) trait CallbackEvent<T>: Debug
{
    fn trigger(&mut self, ctx: &mut T) -> JsResult<()>;
}

pub(crate) trait CallbackEventInit {
    // Forces callback_event marco to work correctly
    fn new() -> Self;
}

#[macro_export]
macro_rules! callback_event {
    ($t:ty) => {
        std::rc::Rc::new(std::cell::RefCell::new(<$t>::new()))
    };

    // Maybe needed at some point
    // ($t:ty, $($args:expr),*) => {
    //     std::rc::Rc::new(std::cell::RefCell::new(<$t>::new($($args),*)))
    // }
}
