use crate::prelude::*;
use crate::video::video_internal::VideoInternal;
use crate::{debug_console_log, JsResult};
use std::any::Any;
use std::sync::{Arc, Mutex};
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
}


#[allow(dead_code)]
pub trait VideoPlayerState {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[inline]
pub fn get_state_owned<T: 'static + Clone>(value: &Box<dyn VideoPlayerState>) -> JsResult<T> {
    if let Some(state_ref) = value.as_any().downcast_ref::<T>() {
        Ok(state_ref.clone()) // TODO is cloning fine?
    } else {
        Err(JsValue::from_str("Invalid downcasting"))
    }
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
        }
    }
}

impl<I, S> VideoPlayer<I, S>
where
    I: VideoInternal,
    S: VideoPlayerTypeState,
{
    pub fn mute(&self) {
        self.internal.mute(true).expect("TODO: panic message");
    }

    pub fn fast_forward(&self) {
        self.internal.fast_forward().expect("TODO: panic message");
    }

    pub fn rewind(&self) {
        self.internal.rewind().expect("TODO: panic message");
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
        }
    }
}

impl<I> VideoPlayer<I, Uninitialized>
where
    I: VideoInternal,
{
    pub fn new(internal: I) -> VideoPlayer<I, Ready> {
        debug_console_log!("VideoPlayer initializing");
        VideoPlayer {
            internal,
            marker: std::marker::PhantomData,
            type_id: std::any::TypeId::of::<Ready>(),
        }
    }
}


impl<I> VideoPlayer<I, Ready>
where
    I: VideoInternal + 'static,
{
    pub(crate) fn play(self) -> VideoPlayer<I, Playing> {
        // should probably return a 'future' type state e.g. WaitingToPlay
        let _ = self.internal.play().expect("Failed to play");
        self.transition()
    }
}

impl<I> VideoPlayer<I, Playing>
where
    I: VideoInternal + 'static,
{
    pub(crate) fn pause(self) -> VideoPlayer<I, Paused> {
        let _ = self.internal.pause().expect("Failed to pause");
        self.transition()
    }

}

pub trait VideoPlayerTypeState {

}

pub enum Uninitialized {}
pub enum Ready {}
pub enum Playing {}

pub type Paused = Ready;

impl VideoPlayerTypeState for Uninitialized {}
impl VideoPlayerTypeState for Ready {}
impl VideoPlayerTypeState for Playing {}
