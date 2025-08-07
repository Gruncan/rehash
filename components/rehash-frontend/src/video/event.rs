use rehash_utils::errors::RehashResultUnit;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub(crate) type CallbackEventType<T> = Rc<RefCell<T>>;
pub(crate) type EventCtxType<T> = Arc<Mutex<T>>;

pub(crate) trait CallbackController {
    fn register_events(&self);
}


pub(crate) trait CallbackEvent<T>: Debug
{
    fn trigger(&mut self, ctx: &mut T) -> RehashResultUnit;

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
