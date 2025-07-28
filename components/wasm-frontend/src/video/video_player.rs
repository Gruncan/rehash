use crate::prelude::*;
use crate::video::event::EventCtxType;
use crate::video::video_internal::VideoInternal;
pub(crate) use crate::video::video_ui::VideoUIController;
use crate::{debug_console_log, JsResult};
use std::any::Any;
use std::rc::Rc;
use wasm_bindgen::JsValue;

pub type VideoPlayerResult<I, S: VideoPlayerTypeState> = Result<VideoPlayer<I, S>, VideoPlayer<I, S::FallbackState>>;
pub(crate) type SharedVideoPlayer = EventCtxType<Box<dyn VideoPlayerState>>;

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

    fn get_progress(&self) -> f64;

    fn get_video_length(&self) -> f64;

    fn set_video_time(&self);

    fn set_video_progress(&self, progress: f64);

    fn set_volume(&self, volume: f64);
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

    fn get_progress(&self) -> f64 {
        self.internal.get_progress().expect("Video player failed to get progress")
    }

    fn get_video_length(&self) -> f64 {
        self.internal.get_video_length().expect("Failed to get video length")
    }

    fn set_video_time(&self) {
        let progress = self.get_progress();
        let duration = self.get_video_length();
        self.video_controller.update_progress(progress, duration)
    }

    fn set_video_progress(&self, progress: f64) {
        self.internal.set_video_progress(progress);
    }

    fn set_volume(&self, volume: f64) {
        self.internal.set_volume(volume);
        self.video_controller.update_volume(volume);
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
    I: VideoInternal + 'static,
{
    pub fn new(internal: I, video_controller: Rc<dyn VideoUIController<I>>) -> VideoPlayer<I, Uninitialized> {
        debug_console_log!("VideoPlayer initializing");
        VideoPlayer {
            internal,
            marker: std::marker::PhantomData,
            type_id: std::any::TypeId::of::<Paused>(),
            video_controller,
        }
    }


    pub(crate) fn ready(self) -> VideoPlayerResult<I, Ready> {
        if self.internal.ready() {
            Ok(self.transition())
        } else {
            Err(self.transition())
        }
    }
}


impl<I, T> VideoPlayer<I, T>
where
    I: VideoInternal + 'static,
    T: Playable + 'static,
{
    pub(crate) fn play(self) -> VideoPlayerResult<I, Playing> {
        // should probably return a 'future' type state e.g. WaitingToPlay
        if let Ok(playing) = self.internal.play() {
            self.video_controller.swap_play_button();
            Ok(self.transition())
        } else {
            Err(self.transition())
        }

    }
}

impl<I> VideoPlayer<I, Playing>
where
    I: VideoInternal + 'static,
{
    pub(crate) fn pause(self) -> VideoPlayerResult<I, Paused> {
        let _ = self.internal.pause().expect("Failed to pause");
        self.video_controller.swap_pause_button();
        Ok(self.transition())
    }

    pub(crate) fn finish(self) -> VideoPlayerResult<I, Finished> {
        Ok(self.transition())
    }

}


impl<I> VideoPlayer<I, Finished>
where
    I: VideoInternal + 'static,
{
    pub(crate) fn restart(self) -> VideoPlayerResult<I, Ready> {
        self.internal.set_video_progress(0f64);
        Ok(self.transition())
    }
}


pub(crate) trait VideoPlayerTypeState {
    type FallbackState;
}

pub enum Uninitialized {}
pub enum Ready {}
pub enum Paused {}
pub enum Playing {}
pub enum Finished {}


pub(crate) trait Playable: VideoPlayerTypeState {}

impl Playable for Ready {}

impl Playable for Paused {}

impl VideoPlayerTypeState for Uninitialized {
    type FallbackState = Uninitialized;
}

impl VideoPlayerTypeState for Ready {
    type FallbackState = Uninitialized;
}


impl VideoPlayerTypeState for Paused {
    type FallbackState = Playing;
}

impl VideoPlayerTypeState for Playing {
    type FallbackState = Paused;
}

impl VideoPlayerTypeState for Finished {
    type FallbackState = Paused;
}


