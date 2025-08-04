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
pub(crate) use drag_events::*;
pub(crate) use drag_events_exit::*;
pub(crate) use fast_forward_event::*;
pub(crate) use fullscreen_event::*;
pub(crate) use mute_unmute_event::*;
pub(crate) use play_pause_event::*;
pub(crate) use playback_speed_event::*;
pub(crate) use progress_bar_change_event::*;
pub(crate) use rewind_event::*;
pub(crate) use settings_event::*;


fn get_video_player_state_return<I, S>(video_result: VideoPlayerResult<I, S>) -> Box<dyn VideoPlayerState>
where
    I: VideoInternal + 'static + Debug,
    S: VideoPlayerTypeState + 'static + Debug,
    <S as VideoPlayerTypeState>::FallbackState: VideoPlayerTypeState,
    <S as VideoPlayerTypeState>::FallbackState: Debug,
{
    match video_result {
        Ok(v) => {
            Box::new(v) as Box<dyn VideoPlayerState>
        }
        Err(v) => {
            Box::new(v) as Box<dyn VideoPlayerState>
        }
    }
}


pub(crate) mod play_pause_event {
    use super::*;


    #[derive(Debug, Clone)]
    pub(crate) struct PlayPauseEvent<I>
    where
        I: VideoInternal + 'static,
    {
        marker: PhantomData<I>,
    }


    impl<I> CallbackEvent<SharedVideoPlayer> for PlayPauseEvent<I>
    where
        I: VideoInternal + 'static + Debug,
    {
        fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
            let mut cell = ctx.borrow_mut();

            if cell.get_type_id() == TypeId::of::<Uninitialized>() {
                let video_uninitialised: VideoPlayer<I, Uninitialized> = get_state_owned(cell.deref())?;
                *cell = get_video_player_state_return(video_uninitialised.ready());
                if cell.get_type_id() == TypeId::of::<Uninitialized>() {
                    return Ok(());
                }
            }

            let standard: Box<dyn VideoPlayerState>;

            match cell.get_type_id() {
                id if id == TypeId::of::<Ready>() => {
                    let video_uninitialised: VideoPlayer<I, Ready> = get_state_owned(cell.deref())?;
                    standard = get_video_player_state_return(video_uninitialised.play())
                }
                id if id == TypeId::of::<Playing>() => {
                    let video_playing: VideoPlayer<I, Playing> = get_state_owned(cell.deref())?;
                    standard = get_video_player_state_return(video_playing.pause());
                }
                id if id == TypeId::of::<Paused>() => {
                    let video_paused: VideoPlayer<I, Paused> = get_state_owned(cell.deref())?;
                    standard = get_video_player_state_return(video_paused.play());
                }
                id if id == TypeId::of::<Finished>() => {
                    let video_finished: VideoPlayer<I, Finished> = get_state_owned(cell.deref())?;
                    standard = get_video_player_state_return(video_finished.restart());
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
            let video_player_state = ctx.borrow();

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
            let cell = ctx.borrow();

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
            let video_player = ctx.borrow();
            video_player.rewind();
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
            let video_player = ctx.borrow();
            video_player.fast_forward();
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


    #[derive(Debug, Clone)]
    pub(crate) struct StartClipDot {}

    #[derive(Debug, Clone)]
    pub(crate) struct EndClipDot {}


    pub(crate) enum MouseDown {}
    pub(crate) enum MouseMove {}

    pub(crate) trait DragAction {}
    pub(crate) trait BarDraggable {}

    impl DragAction for MouseMove {}
    impl DragAction for MouseDown {}

    impl BarDraggable for VolumeBarClickEvent {}
    impl BarDraggable for ProgressBarClickEvent {}
    impl BarDraggable for StartClipDot {}
    impl BarDraggable for EndClipDot {}


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
    pub(crate) struct BarDragEvent<I>
    where
        I: VideoInternal + 'static,
    {
        marker: PhantomData<I>,
    }

    impl<I> CallbackEvent<Ctx> for BarDragEvent<I>
    where
        I: VideoInternal + 'static + Debug,
    {
        fn trigger(&mut self, ctx: &mut Ctx) -> JsResult<()> {
            debug_console_log!("Percent: {}", ctx.percent);

            let mut video_player = ctx.video_player.borrow_mut();

            if video_player.get_type_id() == TypeId::of::<Uninitialized>() {
                debug_console_log!("Not changing state as video not initialized");
                let video_uninitialised: VideoPlayer<I, Uninitialized> = get_state_owned(video_player.deref())?;
                *video_player = get_video_player_state_return(video_uninitialised.ready());
                if video_player.get_type_id() == TypeId::of::<Uninitialized>() {
                    return Ok(());
                }
            }

            match ctx.action_id {
                id if id == TypeId::of::<MouseDown>() => {
                    let percent = ctx.percent;
                    debug_console_log!("Progress mouse down Percent: {}%", percent);
                    video_player.set_video_progress(percent);
                    ctx.is_dragging.set(true);
                    debug_console_log!("Setting is dragging to true");
                }
                id if id == TypeId::of::<MouseMove>() => {
                    let percent = ctx.percent;
                    debug_console_log!("Mouse move progress Percent: {}%", percent);
                    if ctx.is_dragging.get() {
                        video_player.set_video_progress(percent);
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

    impl<I> CallbackEvent<BarDragEventCtx<VolumeBarClickEvent>> for BarDragEvent<I>
    where
        I: VideoInternal + 'static + Debug,
    {
        fn trigger(&mut self, ctx: &mut BarDragEventCtx<VolumeBarClickEvent>) -> JsResult<()> {

            match ctx.action_id {
                id if id == TypeId::of::<MouseDown>() => {
                    let percent = ctx.percent;
                    debug_console_log!("Mouse down volume Percent: {}%", percent);
                    debug_console_log!("Volume drag state: {}", ctx.is_dragging.get());
                    let video_mutex = ctx.video_player.borrow();
                    video_mutex.set_volume(percent);
                    ctx.is_dragging.set(true);
                }
                id if id == TypeId::of::<MouseMove>() => {
                    if ctx.is_dragging.get() {
                        let percent = ctx.percent;
                        debug_console_log!("Mouse move volume Percent: {}%", percent);
                        let video_mutex = ctx.video_player.borrow();
                        video_mutex.set_volume(percent);
                    }
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

    impl<I> CallbackEvent<BarDragEventCtx<EndClipDot>> for BarDragEvent<I>
    where
        I: VideoInternal + 'static + Debug,
    {
        fn trigger(&mut self, ctx: &mut BarDragEventCtx<EndClipDot>) -> JsResult<()> {
            debug_console_log!("EndClipDot drag state: {}", ctx.is_dragging.get());
            match ctx.action_id {
                id if id == TypeId::of::<MouseDown>() => {
                    let percent = ctx.percent;
                    debug_console_log!("Mouse down End dot clip Percent: {}%", percent);
                    ctx.is_dragging.set(true);
                    ctx.video_player.borrow_mut().set_max_progress(percent);
                }
                id if id == TypeId::of::<MouseMove>() => {
                    if ctx.is_dragging.get() {
                        let percent = ctx.percent;
                        debug_console_log!("End dot Mouse move volume Percent: {}%", percent);
                    }
                }
                _ => {
                    return Err(JsValue::from_str("Callback play event has incorrect type"))
                }
            }
            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<BarDragEventCtx<EndClipDot>>> {
            Box::new(self.clone())
        }
    }

    impl<I> CallbackEvent<BarDragEventCtx<StartClipDot>> for BarDragEvent<I>
    where
        I: VideoInternal + 'static + Debug,
    {
        fn trigger(&mut self, ctx: &mut BarDragEventCtx<StartClipDot>) -> JsResult<()> {
            debug_console_log!("StartClipDot drag state: {}", ctx.is_dragging.get());
            match ctx.action_id {
                id if id == TypeId::of::<MouseDown>() => {
                    let percent = ctx.percent;
                    debug_console_log!("Mouse down Start dot clip Percent: {}%", percent);
                    ctx.is_dragging.set(true);
                    ctx.video_player.borrow_mut().set_min_progress(percent);
                }
                id if id == TypeId::of::<MouseMove>() => {
                    if ctx.is_dragging.get() {
                        let percent = ctx.percent;
                        debug_console_log!("Start dot Mouse move volume Percent: {}%", percent);
                        ctx.video_player.borrow_mut().set_min_progress(percent);
                    }
                }
                _ => {
                    return Err(JsValue::from_str("Callback play event has incorrect type"))
                }
            }

            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<BarDragEventCtx<StartClipDot>>> {
            Box::new(self.clone())
        }
    }



    impl<I> BarDragEvent<I>
    where
        I: VideoInternal + 'static + Debug,
    {
        pub fn new() -> Self {
            Self {
                marker: PhantomData
            }
        }
    }
}

pub(crate) mod drag_events_exit {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type Ctx = DragEventExitCtx;

    #[derive(Debug)]
    pub(crate) struct DragEventExitCtx {
        drag_event_cells: Vec<Rc<Cell<bool>>>,
    }

    impl DragEventExitCtx {
        pub fn new(drag_event_cells: Vec<Rc<Cell<bool>>>) -> Self {
            Self {
                drag_event_cells
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct DragEventExit {}

    impl CallbackEvent<Ctx> for DragEventExit {
        fn trigger(&mut self, ctx: &mut Ctx) -> JsResult<()> {
            debug_console_log!("Mouse up setting to false");
            for event_exit_cell in &ctx.drag_event_cells {
                event_exit_cell.set(false);
            }
            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<Ctx>> {
            Box::new(self.clone())
        }
    }

    impl DragEventExit {
        pub fn new() -> Self {
            Self {}
        }
    }
}

pub(crate) mod playback_speed_event {
    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) enum PlaybackIncreaseAction {}

    #[derive(Debug, Clone)]
    pub(crate) enum PlaybackDecreaseAction {}

    pub(crate) trait PlaybackSpeedAction {}

    impl PlaybackSpeedAction for PlaybackIncreaseAction {}
    impl PlaybackSpeedAction for PlaybackDecreaseAction {}

    #[derive(Debug, Clone)]
    pub(crate) struct PlaybackSpeedEvent<A>
    where
        A: PlaybackSpeedAction + 'static,
    {
        marker: PhantomData<A>,
        type_id: TypeId,
    }


    impl<A> CallbackEvent<SharedVideoPlayer> for PlaybackSpeedEvent<A>
    where
        A: PlaybackSpeedAction + Debug + Clone + 'static,
    {
        fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
            let mut video_player = ctx.borrow_mut();
            if self.type_id == TypeId::of::<PlaybackIncreaseAction>() {
                debug_console_log!("Increasing playback");
                video_player.increment_video_speed();
            } else if self.type_id == TypeId::of::<PlaybackDecreaseAction>() {
                debug_console_log!("Decreasing playback");
                video_player.decrement_video_speed();
            } else {
                return Err(JsValue::from_str("Callback play event has incorrect type"))
            }
            Ok(())
        }

        fn clone_box(&self) -> Box<dyn CallbackEvent<SharedVideoPlayer>> {
            Box::new(self.clone())
        }
    }

    impl<A> PlaybackSpeedEvent<A>
    where
        A: PlaybackSpeedAction + Debug + Clone + 'static,
    {
        pub fn new() -> Self
        {
            Self {
                marker: PhantomData,
                type_id: TypeId::of::<A>(),
            }
        }
    }
}