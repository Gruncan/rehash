use crate::video::video_internal::VideoInternal;
use crate::video::video_player::{get_state_owned, Paused, Playing, SharedVideoPlayer, VideoPlayer, VideoPlayerState};
use crate::JsResult;
use std::any::TypeId;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

pub(crate) type VideoCallbackEventType<T> = Rc<RefCell<T>>;


pub(crate) trait CallbackController {
    fn register_events(&self);
}


pub(crate) trait VideoCallbackEvent<I>: Debug
where
    I: VideoInternal,
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()>;
}

pub(crate) trait VideoCallbackEventInit {
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



#[derive(Debug)]
pub(crate) struct PlayPauseEvent {
    type_id: TypeId,
}

impl VideoCallbackEventInit for PlayPauseEvent {
    fn new() -> Self {
        Self {
            type_id: TypeId::of::<Paused>(),
        }
    }
}

impl<I> VideoCallbackEvent<I> for PlayPauseEvent
where
    I: VideoInternal + 'static,
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        let mut cell = mutex;

        let standard: Box<dyn VideoPlayerState>;
        if self.is_paused() {
            let video_paused: VideoPlayer<I, Paused> = get_state_owned(cell.deref())?;
            let video: VideoPlayer<I, Playing> = video_paused.play();
            self.type_id = TypeId::of::<Playing>();
            standard = Box::new(video);
        } else {
            let video_playing: VideoPlayer<I, Playing> = get_state_owned(cell.deref())?;
            let video: VideoPlayer<I, Paused> = video_playing.pause();
            self.type_id = TypeId::of::<Paused>();
            standard = Box::new(video);
        }
        *cell = standard;

        Ok(())
    }
}

impl PlayPauseEvent {

    pub fn is_paused(&self) -> bool {
        self.type_id == TypeId::of::<Paused>()
    }
}


enum Muted {}
enum Unmuted {}

#[derive(Debug)]
pub(crate) struct MuteUnmuteEvent {
    type_id: TypeId,
}


impl VideoCallbackEventInit for MuteUnmuteEvent {
    fn new() -> Self {
        Self {
            type_id: TypeId::of::<Unmuted>(),
        }
    }
}

impl<I> VideoCallbackEvent<I> for MuteUnmuteEvent
where
    I: VideoInternal + 'static,
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        let video_player_state = mutex.deref();

        if self.is_unmuted() {
            video_player_state.mute();
            self.type_id = TypeId::of::<Muted>();
        } else {
            video_player_state.unmute();
            self.type_id = TypeId::of::<Unmuted>();
        }

        Ok(())
    }
}


impl MuteUnmuteEvent {

    pub fn is_unmuted(&self) -> bool {
        self.type_id == TypeId::of::<Unmuted>()
    }
}

#[derive(Debug)]
pub(crate) struct ProgressBarEvent {}

impl VideoCallbackEventInit for ProgressBarEvent {
    fn new() -> Self {
        Self {}
    }
}


impl<I> VideoCallbackEvent<I> for ProgressBarEvent
where
    I: VideoInternal + 'static,
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        let cell = mutex.deref();

        cell.set_video_time();

        Ok(())
    }
}

impl ProgressBarEvent {}


