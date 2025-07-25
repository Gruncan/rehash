use crate::video::video_internal::VideoInternal;
use crate::video::video_player::{get_state_owned, Paused, Playing, SharedVideoPlayer, VideoPlayer, VideoPlayerState};
use crate::JsResult;
use std::any::TypeId;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub(crate) type VideoCallbackEventType<T> = Rc<RefCell<T>>;


pub(crate) trait CallbackController {
    fn register_events(&self);
}


pub(crate) trait VideoCallbackEvent<I>
where
    I: VideoInternal,
{
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()>;
}


pub(crate) struct PlayPauseEvent {
    type_id: TypeId,
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
    pub fn new() -> Self {
        PlayPauseEvent {
            type_id: TypeId::of::<Paused>(),
        }
    }

    pub fn is_paused(&self) -> bool {
        self.type_id == TypeId::of::<Paused>()
    }
}

#[macro_export]
macro_rules! callback_event {
    ($t:ty) => {
        std::rc::Rc::new(std::cell::RefCell::new(<$t>::new()))
    };
    ($t:ty, $($args:expr),*) => {
        std::rc::Rc::new(std::cell::RefCell::new(<$t>::new($($args),*)))
    }
}


