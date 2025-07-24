use crate::debug_console_log;
use crate::prelude::*;
use std::any::Any;
use web_sys::HtmlVideoElement;

pub struct VideoPlayer<S: VideoPlayerTypeState> {
    internal: HtmlVideoElement,
    marker: std::marker::PhantomData<S>,
}


#[allow(dead_code)]
pub trait VideoPlayerState {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[inline]
pub fn get_state_owned<T: 'static + Clone>(value: &Box<dyn VideoPlayerState>) -> Option<T> {
    if let Some(state_ref) = value.as_any().downcast_ref::<T>() {
        Some(state_ref.clone()) // TODO is cloning fine?
    } else {
        None
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
        T: VideoPlayerTypeState,
    {
        debug_console_log!("Transitioning from state {} to {}", std::any::type_name::<S>(), std::any::type_name::<T>());
        VideoPlayer {
            internal: self.internal,
            marker: std::marker::PhantomData,
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
        }
    }
}

impl VideoPlayer<Uninitialized> {
    pub fn new(internal: HtmlVideoElement) -> VideoPlayer<Ready> {
        VideoPlayer {
            internal,
            marker: std::marker::PhantomData
        }
    }
}

impl VideoPlayer<Ready> {

}

impl VideoPlayer<Playing> {

}

impl VideoPlayer<Paused> {
    pub(crate) fn resume(self) -> VideoPlayer<Playing> {
        self.internal.play();
        self.transition()
    }
}

pub trait VideoPlayerTypeState {

}

pub enum Uninitialized {}
pub enum Ready {}
pub enum Playing {}
pub enum Paused {}

impl VideoPlayerTypeState for Uninitialized {}
impl VideoPlayerTypeState for Ready {}
impl VideoPlayerTypeState for Playing {}
impl VideoPlayerTypeState for Paused {}