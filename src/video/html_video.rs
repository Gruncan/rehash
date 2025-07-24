use crate::video::video_internal::{VideoInternal, VideoResult};
use crate::video::video_player::VideoPlayer;
use web_sys::HtmlVideoElement;

pub(crate) type HtmlVideoPlayer<S> = VideoPlayer<HtmlVideoPlayerInternal, S>;

#[derive(Clone)]
pub(crate) struct HtmlVideoPlayerInternal {
    video_element: HtmlVideoElement,
}

impl VideoInternal for HtmlVideoPlayerInternal {
    fn mute(&self, should_be_muted: bool) -> VideoResult {
        todo!()
    }

    fn fast_forward(&self) -> VideoResult {
        todo!()
    }

    fn rewind(&self) -> VideoResult {
        todo!()
    }

    fn pause(&self) -> VideoResult {
        todo!()
    }

    fn play(&self) -> VideoResult {
        todo!()
    }
}

impl HtmlVideoPlayerInternal {
    pub fn new(video_element: HtmlVideoElement) -> Self {
        Self { video_element }
    }
}