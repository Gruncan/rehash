pub(crate) use crate::event::{CallbackEvent, CallbackEventInit};
use crate::video::video_internal::VideoInternal;
use crate::video::video_player::{get_state_owned, Finished, Paused, Playing, Ready, SharedVideoPlayer, Uninitialized, VideoPlayer, VideoPlayerResult, VideoPlayerState, VideoPlayerTypeState};
use crate::JsResult;
use crate::{debug_console_log, log_to_tauri};
use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;


pub(crate) type EventCtxType<T> = Arc<Mutex<T>>;


#[derive(Clone)]
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
            type_id: TypeId::of::<Uninitialized>(),
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

        if self.type_id == TypeId::of::<Uninitialized>() {
            let video_uninitialised: VideoPlayer<I, Uninitialized> = get_state_owned(cell.deref())?;
            *cell = self.get_video_player_state_return(video_uninitialised.ready());
            if self.type_id == TypeId::of::<Uninitialized>() {
                return Ok(());
            }
        }

        let standard: Box<dyn VideoPlayerState>;


        match self.type_id {
            id if id == TypeId::of::<Ready>() => {
                let video_uninitialised: VideoPlayer<I, Ready> = get_state_owned(cell.deref())?;
                standard = self.get_video_player_state_return(video_uninitialised.play())
            },
            id if id == TypeId::of::<Playing>() => {
                let video_playing: VideoPlayer<I, Playing> = get_state_owned(cell.deref())?;
                standard = self.get_video_player_state_return(video_playing.pause());
            },
            id if id == TypeId::of::<Paused>() => {
                let video_paused: VideoPlayer<I, Paused> = get_state_owned(cell.deref())?;
                standard = self.get_video_player_state_return(video_paused.play());
            },
            id if id == TypeId::of::<Finished>() => {
                let video_finished: VideoPlayer<I, Finished> = get_state_owned(cell.deref())?;
                standard = self.get_video_player_state_return(video_finished.restart());
            },
            _ => {
                return Err(JsValue::from_str("Callback play event has incorrect type"))
            }
        }

        *cell = standard;

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
        Box::new(self.clone())
    }
}

impl<I> PlayPauseEvent<I>
where
    I: VideoInternal + 'static,
{
    fn get_video_player_state_return<S>(&mut self, video_result: VideoPlayerResult<I, S>) -> Box<dyn VideoPlayerState>
    where
        S: VideoPlayerTypeState + 'static,
        <S as VideoPlayerTypeState>::FallbackState: VideoPlayerTypeState,
    {
        match video_result {
            Ok(v) => {
                self.type_id = TypeId::of::<S>();
                Box::new(v) as Box<dyn VideoPlayerState>
            },
            Err(v) => {
                self.type_id = TypeId::of::<S::FallbackState>();
                Box::new(v) as Box<dyn VideoPlayerState>
            }
        }
    }

}


enum Muted {}
enum Unmuted {}


#[derive(Debug, Clone)]
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

    fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
        Box::new(self.clone())
    }
}


impl MuteUnmuteEvent {

    pub fn is_unmuted(&self) -> bool {
        self.type_id == TypeId::of::<Unmuted>()
    }
}

#[derive(Debug, Clone)]
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

    fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
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

    fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FullScreenEvent {}

impl CallbackEventInit for FullScreenEvent {
    fn new() -> Self {
        Self {}
    }
}

impl CallbackEvent<SharedVideoPlayer> for FullScreenEvent
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        debug_console_log!("Triggering fullscreen");
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RewindEvent {}

impl CallbackEventInit for RewindEvent {
    fn new() -> Self {
        Self {}
    }
}


impl CallbackEvent<SharedVideoPlayer> for RewindEvent
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        mutex.rewind();
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FastForwardEvent {}

impl CallbackEventInit for FastForwardEvent {
    fn new() -> Self {
        Self {}
    }
}


impl CallbackEvent<SharedVideoPlayer> for FastForwardEvent
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        mutex.fast_forward();
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
        Box::new(self.clone())
    }
}



pub(crate) struct ProgressBarClickEventCtx {
    pub(crate) video_player: SharedVideoPlayer,
    pub(crate) time_to_seek: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ProgressBarClickEvent {}

impl CallbackEventInit for ProgressBarClickEvent {
    fn new() -> Self {
        Self {}
    }
}


impl CallbackEvent<EventCtxType<ProgressBarClickEventCtx>> for ProgressBarClickEvent {
    fn trigger(&mut self, ctx: &mut EventCtxType<ProgressBarClickEventCtx>) -> JsResult<()> {
        debug_console_log!("Triggering progress bar click event");
        let progress_mutex = ctx.lock().unwrap();
        let percent = progress_mutex.time_to_seek;
        debug_console_log!("Percent: {}%", percent);
        let video_mutex = progress_mutex.video_player.lock().unwrap();
        let new_video_time = video_mutex.get_video_length() * percent;
        video_mutex.set_video_progress(new_video_time);

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CallbackEvent<EventCtxType<ProgressBarClickEventCtx>>> {
        Box::new(self.clone())
    }
}


pub(crate) struct VolumeBarClickEventCtx {
    pub(crate) video_player: SharedVideoPlayer,
    pub(crate) volume_to_set: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct VolumeBarClickEvent {}

impl CallbackEventInit for VolumeBarClickEvent {
    fn new() -> Self {
        Self {}
    }
}


impl CallbackEvent<EventCtxType<VolumeBarClickEventCtx>> for VolumeBarClickEvent {
    fn trigger(&mut self, ctx: &mut EventCtxType<VolumeBarClickEventCtx>) -> JsResult<()> {
        debug_console_log!("Triggering volume bar click event");
        let progress_mutex = ctx.lock().unwrap();
        let percent = progress_mutex.volume_to_set;
        debug_console_log!("Volume Percent: {}%", percent);
        let video_mutex = progress_mutex.video_player.lock().unwrap();
        video_mutex.set_volume(percent);

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CallbackEvent<EventCtxType<VolumeBarClickEventCtx>>> {
        Box::new(self.clone())
    }
}


pub(crate) enum MouseDown {}
pub(crate) enum MouseUp {}
pub(crate) enum MouseMove {}

pub(crate) trait DragAction {}
pub(crate) trait BarDraggable {}

impl DragAction for MouseMove {}
impl DragAction for MouseUp {}
impl DragAction for MouseDown {}

impl BarDraggable for VolumeBarClickEvent {}
impl BarDraggable for ProgressBarClickEvent {}


pub(crate) struct BarDragEventEventCtx<T>
where
    T: BarDraggable + 'static,
{
    video_player: SharedVideoPlayer,
    percent: f64,
    marker: PhantomData<T>,
    action_id: TypeId,
}

impl<T> BarDragEventEventCtx<T>
where
    T: BarDraggable + 'static,
{
    pub(crate) fn new<A>(video_player: SharedVideoPlayer, percent: f64) -> Self
    where
        A: DragAction + 'static,
    {
        Self {
            video_player,
            percent,
            marker: PhantomData,
            action_id: TypeId::of::<A>(),
        }
    }
}


#[derive(Debug, Clone)]
pub(crate) struct BarDragEvent {}

impl CallbackEventInit for BarDragEvent
{
    fn new() -> Self {
        Self {}
    }
}


impl CallbackEvent<EventCtxType<BarDragEventEventCtx<ProgressBarClickEvent>>> for BarDragEvent
{
    fn trigger(&mut self, ctx: &mut EventCtxType<BarDragEventEventCtx<ProgressBarClickEvent>>) -> JsResult<()> {
        // let mutex = ctx.lock().unwrap();
        debug_console_log!("Triggering progress bar drag event");

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CallbackEvent<EventCtxType<BarDragEventEventCtx<ProgressBarClickEvent>>>> {
        Box::new(self.clone())
    }
}

impl CallbackEvent<EventCtxType<BarDragEventEventCtx<VolumeBarClickEvent>>> for BarDragEvent
{
    fn trigger(&mut self, ctx: &mut EventCtxType<BarDragEventEventCtx<VolumeBarClickEvent>>) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        // debug_console_log!("Triggering volume bar drag event");

        match mutex.action_id {
            id if id == TypeId::of::<MouseDown>() => {
                debug_console_log!("Triggering progress volume mouse down");
                // let percent = mutex.percent;
                // debug_console_log!("Volume Percent: {}%", percent);
                // let video_mutex = mutex.video_player.lock().unwrap();
                // video_mutex.set_volume(percent);
            },
            id if id == TypeId::of::<MouseUp>() => {
                debug_console_log!("Triggering progress volume mouse up");
            },
            id if id == TypeId::of::<MouseMove>() => {},
            _ => {
                return Err(JsValue::from_str("Callback play event has incorrect type"))
            }
        }

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CallbackEvent<EventCtxType<BarDragEventEventCtx<VolumeBarClickEvent>>>> {
        Box::new(self.clone())
    }
}



