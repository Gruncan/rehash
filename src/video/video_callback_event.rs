pub(crate) use crate::event::{CallbackEvent, CallbackEventInit};
use crate::video::video_internal::VideoInternal;
use crate::video::video_player::{get_state_owned, Paused, Playing, SharedVideoPlayer, VideoPlayer, VideoPlayerState};
use crate::JsResult;
use crate::{debug_console_log, log_to_tauri};
use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

pub(crate) struct PlayPauseEvent<I>
where
    I: VideoInternal + 'static,
{
    marker: std::marker::PhantomData<I>,
    type_id: TypeId,
}

impl<I> CallbackEventInit for PlayPauseEvent<I>
where
    I: VideoInternal + 'static,
{
    fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
            type_id: TypeId::of::<Paused>(),
        }
    }
}

impl<I> Debug for PlayPauseEvent<I>
where
    I: 'static + VideoInternal,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<I> CallbackEvent<SharedVideoPlayer> for PlayPauseEvent<I>
where
    I: VideoInternal + 'static,
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        let mut cell = mutex;

        let standard: Box<dyn VideoPlayerState>;
        if self.is_paused() {
            let video_paused: VideoPlayer<I, Paused> = get_state_owned(cell.deref())?;
            match video_paused.play() {
                Ok(video) => {
                    self.type_id = TypeId::of::<Playing>();
                    standard = Box::new(video);
                },
                Err(video) => {
                    standard = Box::new(video);
                }
            }
        } else {
            let video_playing: VideoPlayer<I, Playing> = get_state_owned(cell.deref())?;
            match video_playing.pause() {
                Ok(video) => {
                    self.type_id = TypeId::of::<Paused>();
                    standard = Box::new(video);
                },
                Err(video) => {
                    standard = Box::new(video);
                }
            }
        }
        *cell = standard;

        Ok(())
    }
}

impl<I> PlayPauseEvent<I>
where
    I: VideoInternal + 'static,
{
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


impl CallbackEventInit for MuteUnmuteEvent {
    fn new() -> Self {
        Self {
            type_id: TypeId::of::<Unmuted>(),
        }
    }
}


impl CallbackEvent<SharedVideoPlayer> for MuteUnmuteEvent {
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
pub(crate) struct ProgressBarChangeEvent {

}

impl CallbackEventInit for ProgressBarChangeEvent {
    fn new() -> Self {
        Self {}
    }
}


impl CallbackEvent<SharedVideoPlayer> for ProgressBarChangeEvent {
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        let cell = mutex.deref();

        cell.set_video_time();

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct SettingsEvent {}

impl CallbackEventInit for SettingsEvent {
    fn new() -> Self {
        Self {}
    }
}


impl CallbackEvent<SharedVideoPlayer> for SettingsEvent
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        debug_console_log!("Triggering settings");
        Ok(())
    }
}


pub(crate) struct FullScreenEvent {}

impl CallbackEventInit for FullScreenEvent {
    fn new() -> Self {
        Self {}
    }
}

impl Debug for FullScreenEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl CallbackEvent<SharedVideoPlayer> for FullScreenEvent
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        debug_console_log!("Triggering fullscreen");
        Ok(())
    }
}


pub(crate) type ProgressBarClickEventCtxType = Arc<Mutex<ProgressBarClickEventCtx>>;

pub(crate) struct ProgressBarClickEventCtx {
    pub(crate) video_player: SharedVideoPlayer,
    pub(crate) time_to_seek: f64,
}

#[derive(Debug)]
pub(crate) struct ProgressBarClickEvent {}

impl CallbackEventInit for ProgressBarClickEvent {
    fn new() -> Self {
        Self {}
    }
}


impl CallbackEvent<ProgressBarClickEventCtxType> for ProgressBarClickEvent {
    fn trigger(&mut self, ctx: &mut ProgressBarClickEventCtxType) -> JsResult<()> {
        debug_console_log!("Triggering progress bar click event");
        let progress_mutex = ctx.lock().unwrap();
        let percent = progress_mutex.time_to_seek;
        debug_console_log!("Percent: {}%", percent);
        let video_mutex = progress_mutex.video_player.lock().unwrap();
        let new_video_time = video_mutex.get_video_length() * percent;
        video_mutex.set_video_progress(new_video_time);

        Ok(())
    }
}




