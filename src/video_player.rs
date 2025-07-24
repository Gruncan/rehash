use crate::debug_console_log;
use crate::prelude::*;
use web_sys::HtmlVideoElement;

pub struct VideoPlayer<S: VideoPlayerState> {
    internal: HtmlVideoElement,
    marker: std::marker::PhantomData<S>,
}


impl<S> VideoPlayer<S>
where
    S: VideoPlayerState,
{
    pub(crate) fn transition<T>(self) -> VideoPlayer<T>
    where
        T: VideoPlayerState,
    {
        debug_console_log!("Transitioning from state {} to {}", std::any::type_name::<S>(), std::any::type_name::<T>());
        VideoPlayer {
            internal: self.internal,
            marker: std::marker::PhantomData,
        }
    }
}

impl VideoPlayer<Uninitialized> {
    pub fn new(internal: HtmlVideoElement) -> Self {
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

}

trait VideoPlayerState {

}

pub enum Uninitialized {}
pub enum Ready {}
pub enum Playing {}
pub enum Paused {}

impl VideoPlayerState for Uninitialized {}
impl VideoPlayerState for Ready {}
impl VideoPlayerState for Playing {}
impl VideoPlayerState for Paused {}