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
        I: VideoInternal + 'static + Debug,
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
            <S as VideoPlayerTypeState>::FallbackState: Debug
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
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Ctx = BarDragEventCtx<ProgressBarClickEvent>;

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
    pub(crate) struct BarDragEventCtx<T>
    where
        T: BarDraggable + 'static,
    {
        video_player: Rc<RefCell<Box<dyn VideoPlayerState>>>,
        pub(crate) percent: f64,
        marker: PhantomData<T>,
        action_id: TypeId,
        is_dragging: Rc<Cell<bool>>,
    }

    impl<T> BarDragEventCtx<T>
    where
        T: BarDraggable + 'static,
    {
        pub(crate) fn new<A>(video_player: Rc<RefCell<Box<dyn VideoPlayerState>>>, is_dragging: Rc<Cell<bool>>) -> Self
        where
            A: DragAction + 'static,
        {
            Self {
                video_player,
                percent: Default::default(),
                marker: PhantomData,
                action_id: TypeId::of::<A>(),
                is_dragging,
            }
        }
    }


    #[derive(Debug, Clone)]
    pub(crate) struct BarDragEvent {
        is_dragging: bool,
    }


    impl CallbackEvent<Ctx> for BarDragEvent
    {
        fn trigger(&mut self, ctx: &mut Ctx) -> JsResult<()> {
            debug_console_log!("Percent: {}", ctx.percent);

            match ctx.action_id {
                id if id == TypeId::of::<MouseDown>() => {
                    let percent = ctx.percent;
                    debug_console_log!("Progress mouse down Percent: {}%", percent);
                    let video_mutex = ctx.video_player.borrow();
                    video_mutex.set_video_progress(percent);
                    // self.is_dragging = true;
                    ctx.is_dragging.set(true);
                    debug_console_log!("Setting is dragging to true");
                }
                id if id == TypeId::of::<MouseUp>() => {
                    debug_console_log!("Triggering progress volume mouse up");
                    ctx.is_dragging.set(false);
                    // self.is_dragging = false;
                }
                id if id == TypeId::of::<MouseMove>() => {
                    let percent = ctx.percent;
                    debug_console_log!("Mouse move progress Percent: {}%", percent);
                    if ctx.is_dragging.get() {
                        debug_console_log!("Dragging mouse move is dragging is true");
                        let video_mutex = ctx.video_player.borrow();
                        video_mutex.set_video_progress(percent);
                    } else {
                        debug_console_log!("Dragging mouse move is dragging is false");
                    }
                }
                _ => {
                    error_log!("ctx.action_id is unknown, {:?}", std::any::type_name_of_val(&ctx.action_id));
                    return Err(JsValue::from_str("Callback play event has incorrect type"))
                }
            }

            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<Ctx>> {
            Box::new(self.clone())
        }
    }

    impl CallbackEvent<BarDragEventCtx<VolumeBarClickEvent>> for BarDragEvent
    {
        fn trigger(&mut self, ctx: &mut BarDragEventCtx<VolumeBarClickEvent>) -> JsResult<()> {
            debug_console_log!("Trying to acquire mutex for: {:?}", self);
            // debug_console_log!("Triggering volume bar drag event");
            // let contex = ctx.borrow();

            match ctx.action_id {
                id if id == TypeId::of::<MouseDown>() => {
                    let percent = ctx.percent;
                    debug_console_log!("Mouse down volume Percent: {}%", percent);
                    let video_mutex = ctx.video_player.borrow();
                    video_mutex.set_volume(percent);
                    self.is_dragging = true;
                }
                id if id == TypeId::of::<MouseUp>() => {
                    debug_console_log!("Triggering progress volume mouse up");
                    self.is_dragging = false;
                }
                id if id == TypeId::of::<MouseMove>() && self.is_dragging => {
                    let percent = ctx.percent;
                    debug_console_log!("Mouse move volume Percent: {}%", percent);
                    let video_mutex = ctx.video_player.borrow();
                    video_mutex.set_volume(percent);
                }
                _ => {
                    return Err(JsValue::from_str("Callback play event has incorrect type"))
                }
            }

            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<BarDragEventCtx<VolumeBarClickEvent>>> {
            Box::new(self.clone())
        }
    }

    impl BarDragEvent {
        pub fn new() -> Self {
            Self { is_dragging: false }
        }
    }
}