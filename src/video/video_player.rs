use crate::prelude::*;
use crate::video::video_internal::VideoInternal;
use crate::{debug_console_log, JsResult};
use std::any::Any;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::closure::{Closure, WasmClosure};
use wasm_bindgen::JsValue;

pub type SharedVideoPlayer = Arc<Mutex<Box<dyn VideoPlayerState>>>;


pub struct VideoPlayer<I, S>
where
    I: VideoInternal,
    S: VideoPlayerTypeState,
{
    internal: I,
    marker: std::marker::PhantomData<S>,
    type_id: std::any::TypeId,
    video_controller: Rc<dyn VideoUIController<I>>,
}


#[allow(dead_code)]
pub trait VideoPlayerState {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn mute(&self);

    fn unmute(&self);

    fn fast_forward(&self);

    fn rewind(&self);
}

impl<I, S> VideoPlayerState for VideoPlayer<I, S>
where
    I: VideoInternal + 'static,
    S: VideoPlayerTypeState + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn mute(&self) {
        self.internal.mute(true).expect("Video player failed to mute");
        self.video_controller.swap_mute_button();
    }

    fn unmute(&self) {
        self.internal.mute(false).expect("Video player failed to unmute");
        self.video_controller.swap_unmute_button();
    }

    fn fast_forward(&self) {
        self.internal.fast_forward().expect("Video player failed to fast forward");
    }

    fn rewind(&self) {
        self.internal.rewind().expect("Video player failed to rewind");
    }
}

#[inline]
pub fn get_state_owned<T: 'static + Clone>(value: &Box<dyn VideoPlayerState>) -> JsResult<T> {
    if let Some(state_ref) = value.as_any().downcast_ref::<T>() {
        Ok(state_ref.clone()) // TODO is cloning fine?
    } else {
        Err(JsValue::from_str("Invalid downcasting"))
    }
}


impl<I, S> VideoPlayer<I, S>
where
    I: VideoInternal + 'static,
    S: VideoPlayerTypeState,
{
    pub(self) fn transition<T>(self) -> VideoPlayer<I, T>
    where
        T: VideoPlayerTypeState + 'static,
    {
        debug_console_log!("Transitioning from state {} to {}", std::any::type_name::<S>(), std::any::type_name::<T>());
        VideoPlayer {
            internal: self.internal,
            marker: std::marker::PhantomData,
            type_id: std::any::TypeId::of::<T>(),
            video_controller: self.video_controller,
        }
    }
}


impl<I, S> Clone for VideoPlayer<I, S>
where
    I: VideoInternal,
    S: VideoPlayerTypeState,
{
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
            marker: self.marker,
            type_id: self.type_id,
            video_controller: self.video_controller.clone(),
        }
    }
}

impl<I> VideoPlayer<I, Uninitialized>
where
    I: VideoInternal,
{
    pub fn new(internal: I, video_controller: Rc<dyn VideoUIController<I>>) -> VideoPlayer<I, Paused> {
        debug_console_log!("VideoPlayer initializing");
        VideoPlayer {
            internal,
            marker: std::marker::PhantomData,
            type_id: std::any::TypeId::of::<Paused>(),
            video_controller,
        }
    }
}


impl<I> VideoPlayer<I, Paused>
where
    I: VideoInternal + 'static,
{
    pub(crate) fn play(self) -> VideoPlayer<I, Playing> {
        // should probably return a 'future' type state e.g. WaitingToPlay
        let _ = self.internal.play().expect("Failed to play");
        self.video_controller.swap_play_button();
        self.transition()
    }
}

impl<I> VideoPlayer<I, Playing>
where
    I: VideoInternal + 'static,
{
    pub(crate) fn pause(self) -> VideoPlayer<I, Paused> {
        let _ = self.internal.pause().expect("Failed to pause");
        self.video_controller.swap_pause_button();
        self.transition()
    }

}

pub trait VideoPlayerTypeState {

}

pub enum Uninitialized {}
pub enum Paused {}
pub enum Playing {}

impl VideoPlayerTypeState for Uninitialized {}
impl VideoPlayerTypeState for Paused {}
impl VideoPlayerTypeState for Playing {}


pub trait VideoUIController<I>
where
    I: VideoInternal,
{

    fn swap_play_button(&self);

    fn swap_pause_button(&self);

    fn swap_mute_button(&self);

    fn swap_unmute_button(&self);
}

pub trait VideoUIRegister {
    fn register_global_event_listener<T: ?Sized + WasmClosure>(&self, closure: Box<Closure<T>>);

    fn register_element_event_listener<T: ?Sized + WasmClosure>(&self, ids: Vec<String>, closure: Box<Closure<T>>);
}
