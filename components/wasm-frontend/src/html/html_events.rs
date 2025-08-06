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
    }

    impl FastForwardEvent {
        pub fn new() -> Self {
            Self {}
        }
    }
}

pub(crate) mod drag_events {
    use crate::video::event::CallbackEvent;
    use crate::video::video_callback::{SharedVideoPlayer, VideoPlayerState};
    use crate::JsResult;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const PROGRESS_BAR_ID: &'static str = "progress-bar";
    const VOLUME_ID: &'static str = "volume-slider";
    const START_DOT_ID: &'static str = "start-dot";
    const END_DOT_ID: &'static str = "end-dot";



    #[derive(Debug, Copy, Clone)]
    pub(crate) enum MoveState {
        Nothing,
        ProgressBar,
        StartClipDot,
        EndClipDot,
        VolumeBar,
    }

    impl From<&str> for MoveState {
        fn from(value: &str) -> Self {
            match value {
                PROGRESS_BAR_ID => MoveState::ProgressBar,
                START_DOT_ID => MoveState::StartClipDot,
                END_DOT_ID => MoveState::EndClipDot,
                VOLUME_ID => MoveState::VolumeBar,
                _ => MoveState::Nothing,
            }
        }
    }

    impl TryFrom<MoveState> for &str {
        type Error = ();

        fn try_from(value: MoveState) -> Result<Self, Self::Error> {
            match value {
                MoveState::ProgressBar => Ok(PROGRESS_BAR_ID),
                MoveState::StartClipDot => Ok(START_DOT_ID),
                MoveState::EndClipDot => Ok(END_DOT_ID),
                MoveState::VolumeBar => Ok(VOLUME_ID),
                _ => Err(())
            }
        }
    }


    type MovingCtx = Rc<Cell<MoveState>>;

    pub(crate) type DragEventCtxType = Rc<RefCell<DragEventCtx>>;

    #[derive(Debug, Clone)]
    pub(crate) struct DragEventCtx {
        currently_moving: MovingCtx,
        video_player: SharedVideoPlayer,
        percent: f64,
        clicked: MoveState,
    }

    impl DragEventCtx {
        pub fn new(video_player: SharedVideoPlayer) -> Self {
            Self {
                video_player,
                currently_moving: Rc::new(Cell::new(MoveState::Nothing)),
                percent: 0f64,
                clicked: MoveState::Nothing,
            }
        }

        pub fn set_percent(&mut self, percent: f64) {
            self.percent = percent
        }

        pub fn set_moving(&self, moving_type: MoveState) {
            self.currently_moving.set(moving_type)
        }

        pub fn set_clicked(&mut self, clicked: MoveState) {
            self.clicked = clicked
        }
    }

    type Ctx = DragEventCtxType;

    #[derive(Debug, Clone)]
    pub(crate) struct DragMoveEvent {}

    impl CallbackEvent<Ctx> for DragMoveEvent {
        fn trigger(&mut self, ctx: &mut Ctx) -> JsResult<()> {
            let ctx = ctx.borrow();
            match ctx.currently_moving.get() {
                MoveState::Nothing => {},
                MoveState::ProgressBar => {
                    ctx.video_player.borrow_mut().set_video_progress(ctx.percent);
                },
                MoveState::StartClipDot => {
                    ctx.video_player.borrow_mut().set_min_progress(ctx.percent);
                },
                MoveState::EndClipDot => {
                    ctx.video_player.borrow_mut().set_max_progress(ctx.percent);
                },
                MoveState::VolumeBar => {
                    let video_mutex = ctx.video_player.borrow();
                    video_mutex.set_volume(ctx.percent);
                },
            }

            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct DragClickEvent {}


    impl CallbackEvent<Ctx> for DragClickEvent {
        fn trigger(&mut self, ctx: &mut Ctx) -> JsResult<()> {
            let ctx = ctx.borrow();
            ctx.set_moving(ctx.clicked);
            match ctx.currently_moving.get() {
                MoveState::Nothing => {},
                MoveState::ProgressBar => {
                    ctx.video_player.borrow_mut().set_video_progress(ctx.percent);
                },
                MoveState::StartClipDot => {
                    ctx.video_player.borrow_mut().set_min_progress(ctx.percent);
                },
                MoveState::EndClipDot => {
                    ctx.video_player.borrow_mut().set_max_progress(ctx.percent);
                },
                MoveState::VolumeBar => {
                    let video_mutex = ctx.video_player.borrow();
                    video_mutex.set_volume(ctx.percent);
                },
            }
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct DragExitEvent {}

    impl CallbackEvent<Ctx> for DragExitEvent {
        fn trigger(&mut self, ctx: &mut Ctx) -> JsResult<()> {
            let ctx = ctx.borrow();
            ctx.set_moving(MoveState::Nothing);
            Ok(())
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