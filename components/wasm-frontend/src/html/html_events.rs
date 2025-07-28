use crate::video::event::CallbackEvent;
use crate::video::video_internal::VideoInternal;
use crate::video::video_player::{get_state_owned, Finished, Paused, Playing, Ready, SharedVideoPlayer, Uninitialized, VideoPlayer, VideoPlayerResult, VideoPlayerState, VideoPlayerTypeState};
use crate::JsResult;
use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use wasm_bindgen::JsValue;
use wasm_bindings_lib::debug_console_log;

pub use crate::prelude::*;


pub(crate) mod play_pause_event {
    use super::*;
    use crate::video::video_player;


    #[derive(Debug, Clone)]
    pub(crate) struct PlayPauseEvent<I>
    where
        I: VideoInternal + 'static,
    {
        marker: PhantomData<I>,
        type_id: TypeId,
    }


    impl<I> CallbackEvent<SharedVideoPlayer> for PlayPauseEvent<I>
    where
        I: VideoInternal + 'static + Debug,
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
                }
                id if id == TypeId::of::<Playing>() => {
                    let video_playing: VideoPlayer<I, Playing> = get_state_owned(cell.deref())?;
                    standard = self.get_video_player_state_return(video_playing.pause());
                }
                id if id == TypeId::of::<Paused>() => {
                    let video_paused: VideoPlayer<I, Paused> = get_state_owned(cell.deref())?;
                    standard = self.get_video_player_state_return(video_paused.play());
                }
                id if id == TypeId::of::<Finished>() => {
                    let video_finished: VideoPlayer<I, Finished> = get_state_owned(cell.deref())?;
                    standard = self.get_video_player_state_return(video_finished.restart());
                }
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
        I: VideoInternal + 'static + std::fmt::Debug,
    {
        pub(crate) fn new() -> Self {
            Self {
                marker: PhantomData,
                type_id: TypeId::of::<Uninitialized>(),
            }
        }

        fn get_video_player_state_return<S>(&mut self, video_result: VideoPlayerResult<I, S>) -> Box<dyn VideoPlayerState>
        where
            S: VideoPlayerTypeState + 'static + Debug,
            <S as VideoPlayerTypeState>::FallbackState: VideoPlayerTypeState,
            <S as video_player::VideoPlayerTypeState>::FallbackState: std::fmt::Debug
        {
            match video_result {
                Ok(v) => {
                    self.type_id = TypeId::of::<S>();
                    Box::new(v) as Box<dyn VideoPlayerState>
                }
                Err(v) => {
                    self.type_id = TypeId::of::<S::FallbackState>();
                    Box::new(v) as Box<dyn VideoPlayerState>
                }
            }
        }
    }
}

pub(crate) mod mute_unmute_event {
    use super::*;
    enum Muted {}
    enum Unmuted {}


    #[derive(Debug, Clone)]
    pub(crate) struct MuteUnmuteEvent {
        type_id: TypeId,
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
        pub fn new() -> Self {
            Self {
                type_id: TypeId::of::<Unmuted>(),
            }
        }

        fn is_unmuted(&self) -> bool {
            self.type_id == TypeId::of::<Unmuted>()
        }
    }
}

pub(crate) mod progress_bar_change_event {
    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) struct ProgressBarChangeEvent {}


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

    impl ProgressBarChangeEvent {
        pub fn new() -> Self {
            Self {}
        }
    }
}

pub(crate) mod settings_event {
    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) struct SettingsEvent {}

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

    impl SettingsEvent {
        pub fn new() -> Self {
            Self {}
        }
    }
}

pub(crate) mod fullscreen_event {
    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) struct FullScreenEvent {}

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

    impl FullScreenEvent {
        pub fn new() -> Self {
            Self {}
        }
    }
}

pub(crate) mod rewind_event {
    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) struct RewindEvent {}


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

    impl RewindEvent {
        pub fn new() -> Self {
            Self {}
        }
    }
}

pub(crate) mod fast_forward_event {
    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) struct FastForwardEvent {}

    impl CallbackEvent<SharedVideoPlayer> for FastForwardEvent {
        fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
            let mutex = ctx.lock().unwrap();
            mutex.fast_forward();
            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
            Box::new(self.clone())
        }
    }

    impl FastForwardEvent {
        pub fn new() -> Self {
            Self {}
        }
    }
}

pub(crate) mod drag_events {
    use super::*;
    use crate::video::event::EventCtxType;

    #[derive(Debug, Clone)]
    pub(crate) struct ProgressBarClickEvent {}

    #[derive(Debug, Clone)]
    pub(crate) struct VolumeBarClickEvent {}


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


    #[derive(Debug)]
    pub(crate) struct BarDragEventEventCtx<T>
    where
        T: BarDraggable + 'static,
    {
        video_player: SharedVideoPlayer,
        pub(crate) percent: f64,
        marker: PhantomData<T>,
        action_id: TypeId,
    }

    impl<T> BarDragEventEventCtx<T>
    where
        T: BarDraggable + 'static,
    {
        pub(crate) fn new<A>(video_player: SharedVideoPlayer) -> Self
        where
            A: DragAction + 'static,
        {
            Self {
                video_player,
                percent: Default::default(),
                marker: PhantomData,
                action_id: TypeId::of::<A>(),
            }
        }
    }


    #[derive(Debug, Clone)]
    pub(crate) struct BarDragEvent {
        is_dragging: bool,
    }


    impl CallbackEvent<EventCtxType<BarDragEventEventCtx<ProgressBarClickEvent>>> for BarDragEvent
    {
        fn trigger(&mut self, ctx: &mut EventCtxType<BarDragEventEventCtx<ProgressBarClickEvent>>) -> JsResult<()> {
            let mutex = ctx.lock().unwrap();
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
                    let percent = mutex.percent;
                    debug_console_log!("Mouse down volume Percent: {}%", percent);
                    let video_mutex = mutex.video_player.lock().unwrap();
                    video_mutex.set_volume(percent);
                    self.is_dragging = true;
                }
                id if id == TypeId::of::<MouseUp>() => {
                    debug_console_log!("Triggering progress volume mouse up");
                    self.is_dragging = false;
                }
                id if id == TypeId::of::<MouseMove>() => {
                    let percent = mutex.percent;
                    debug_console_log!("Mouse move volume Percent: {}%", percent);
                    let video_mutex = mutex.video_player.lock().unwrap();
                    video_mutex.set_volume(percent);
                }
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

    impl BarDragEvent {
        pub fn new() -> Self {
            Self { is_dragging: false }
        }
    }
}