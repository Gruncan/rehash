use crate::video::video_internal::VideoInternal;
use std::fmt::Debug;
use wasm_bindgen::closure::{Closure, WasmClosure};

pub(crate) trait VideoUIController<I>: Debug
where
    I: VideoInternal,
{
    fn swap_play_button(&self);

    fn swap_pause_button(&self);

    fn swap_mute_button(&self);

    fn swap_unmute_button(&self);

    fn update_progress(&self, progress: f64, duration: f64);

    fn update_volume(&self, volume: f64);
}

pub(crate) trait VideoUIRegister {
    fn register_global_event_listener<T: ?Sized + WasmClosure>(&self, closure: Box<Closure<T>>);

    fn register_element_event_listener<T: ?Sized + WasmClosure>(&self, ids: Vec<String>, closure: Box<Closure<T>>);

    fn register_video_global_event_listener_specific<T: ?Sized + WasmClosure>(&self, string: &str, closure: Box<Closure<T>>);

    fn register_element_event_listener_specific<T: ?Sized + WasmClosure>(&self, string: &str, id: &str, closure: Box<Closure<T>>);

    fn register_doc_global_event_listener_specific<T: ?Sized + WasmClosure>(&self, string: &str, closure: Box<Closure<T>>);
}