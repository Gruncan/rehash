use crate::console_log;
pub(crate) use crate::html::html_ui::HtmlVideoUIController;
use crate::prelude::*;
use crate::video::video_callback::*;
use crate::video::video_internal::{VideoInternal, VideoPlaybackSpeed, VideoResult, VideoResultUnit};
use crate::video::video_player::{SharedVideoPlayer, VideoPlayer, VideoPlayerState};
use std::cell::RefCell;
use std::cmp::PartialOrd;
use std::rc::Rc;
use web_sys::HtmlVideoElement;

const SKIP_INCREMENT: f64 = 5.0;

pub(crate) type HtmlVideoPlayer<S> = VideoPlayer<HtmlVideoPlayerInternal, S>;
pub(crate) type Event = Rc<RefCell<dyn CallbackEvent<SharedVideoPlayer>>>;
pub(crate) type EventT<T> = Rc<RefCell<dyn CallbackEvent<T>>>;

#[derive(Debug)]
pub(crate) struct HtmlVideoPlayerInternal {
    video_element: HtmlVideoElement,
    min_video_progress: f64,
    max_video_progress: f64,
    video_playback_speed: VideoPlaybackSpeed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub(crate) enum InternalVideoReadiness {
    Nothing,
    MedaData,
    CurrentData,
    FutureData,
    AllData,
    Unknown(u16),
}

impl From<u16> for InternalVideoReadiness {
    fn from(value: u16) -> Self {
        match value {
            0 => InternalVideoReadiness::Nothing,
            1 => InternalVideoReadiness::MedaData,
            2 => InternalVideoReadiness::CurrentData,
            3 => InternalVideoReadiness::FutureData,
            4 => InternalVideoReadiness::AllData,
            other => InternalVideoReadiness::Unknown(other),
        }
    }
}




impl VideoInternal for HtmlVideoPlayerInternal {
    fn mute(&self, should_be_muted: bool) -> VideoResultUnit {
        self.video_element.set_muted(should_be_muted);
        Ok(())
    }

    fn fast_forward(&self) -> VideoResultUnit {
        let to_move = (self.video_element.current_time() + SKIP_INCREMENT).min(self.video_element.duration());
        console_log!("Fast forwarding to: {}", to_move);
        self.video_element.set_current_time(to_move);
        Ok(())
    }

    fn rewind(&self) -> VideoResultUnit {
        let current_time = (self.video_element.current_time() - SKIP_INCREMENT).max(0f64);
        console_log!("Rewinding to: {}", current_time);
        self.video_element.set_current_time(current_time);
        Ok(())
    }

    fn pause(&self) -> VideoResultUnit {
        match self.video_element.pause() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.as_string().unwrap().into()),
        }
    }

    fn play(&self) -> VideoResult<::js_sys::Promise> {
        match self.video_element.play() {
            Ok(p) => Ok(p),
            Err(err) => Err(err.as_string().unwrap().into()),
        }
    }

    fn get_volume(&self) {
        todo!()
    }

    fn get_playback_time(&self) {
        todo!()
    }

    fn get_progress(&self) -> VideoResult<f64> {
        Ok(self.video_element.current_time())
    }

    fn get_video_length(&self) -> f64 {
        self.video_element.duration()
    }

    fn set_video_progress(&self, time: f64) {
        self.video_element.set_current_time(time);
    }

    fn ready(&self) -> bool {
        let state: InternalVideoReadiness = self.video_element.ready_state().into();
        state >= InternalVideoReadiness::CurrentData
    }

    fn set_volume(&self, volume: f64) {
        self.video_element.set_volume(volume)
    }

    fn set_min_progress(&mut self, percent: f64) {
        let length = self.get_video_length();
        let time = length * percent;
        let max_progress = length * self.max_video_progress;
        if time + 1f64 < max_progress {
            self.min_video_progress = percent;
        }
    }

    fn get_min_progress(&self) -> f64 {
        todo!()
    }

    fn set_max_progress(&mut self, percent: f64) {
        let length = self.get_video_length();
        let time = length * percent;
        let min_progress = length * self.min_video_progress;
        if time - 1f64 > min_progress {
            self.max_video_progress = percent;
        }
    }

    fn get_max_progress(&self) -> f64 {
        todo!()
    }

    fn set_playback_speed(&self, speed: VideoPlaybackSpeed) {
        self.video_element.set_playback_rate(speed.get_playback_speed());
    }

    fn increment_video_speed(&mut self) {
        self.video_playback_speed = self.video_playback_speed.increment();
        self.set_playback_speed(self.video_playback_speed);
    }

    fn decrement_video_speed(&mut self) {
        self.video_playback_speed = self.video_playback_speed.decrement();
        self.set_playback_speed(self.video_playback_speed);
    }
}

impl Clone for HtmlVideoPlayerInternal {
    fn clone(&self) -> Self {
        Self {
            video_element: self.video_element.clone(),
            min_video_progress: self.max_video_progress,
            max_video_progress: self.max_video_progress,
            video_playback_speed: self.video_playback_speed,
        }
    }
}

impl HtmlVideoPlayerInternal {
    pub fn new(video_element: HtmlVideoElement) -> Self {
        Self { video_element, min_video_progress: 0f64, max_video_progress: 1f64, video_playback_speed: VideoPlaybackSpeed::Normal }
    }
}

