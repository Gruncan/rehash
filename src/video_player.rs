use crate::prelude::*;
use crate::{debug_console_log, JsResult};
use std::any::Any;
use wasm_bindgen::JsValue;
use web_sys::HtmlVideoElement;

pub struct VideoPlayer<S: VideoPlayerTypeState> {
    internal: HtmlVideoElement,
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

impl<S> VideoPlayerState for VideoPlayer<S>
where
    S: VideoPlayerTypeState + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl<S> VideoPlayer<S>
where
    S: VideoPlayerTypeState,
{
    pub(self) fn transition<T>(self) -> VideoPlayer<T>
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

impl<S> Clone for VideoPlayer<S>
where
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

impl VideoPlayer<Uninitialized> {
    pub fn new(internal: HtmlVideoElement) -> VideoPlayer<Ready> {
        debug_console_log!("VideoPlayer initializing");
        VideoPlayer {
            internal,
            marker: std::marker::PhantomData,
            type_id: std::any::TypeId::of::<Ready>(),
        }
    }
}


impl VideoPlayer<Ready> {
    pub(crate) fn play(self) -> VideoPlayer<Playing> {
        // should probably return a 'future' type state e.g. WaitingToPlay
        let _ = self.internal.play().expect("Failed to play");
        self.transition()
    }
}

impl VideoPlayer<Playing> {
    pub(crate) fn pause(self) -> VideoPlayer<Paused> {
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
