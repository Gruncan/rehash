use crate::video::video_internal::VideoPlaybackSpeed::{DoubleHalf, Half};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type VideoResultUnit = Result<(), InternalVideoError>;
pub type VideoResult<T> = Result<T, InternalVideoError>;


const PLAYBACK_CONVERSION: f64 = 10f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum VideoPlaybackSpeed {
    Half = 5,
    Normal = 10,
    NormalHalf = 15,
    Double = 20,
    DoubleHalf = 25,
}


impl VideoPlaybackSpeed {
    pub fn new() -> Self {
        Self::Normal
    }

    pub fn get_playback_speed(&self) -> f64 {
        let num = *self as u8;
        num as f64 / PLAYBACK_CONVERSION
    }

    pub fn increment(self) -> Self {
        match self {
            VideoPlaybackSpeed::Half => VideoPlaybackSpeed::Normal,
            VideoPlaybackSpeed::Normal => VideoPlaybackSpeed::NormalHalf,
            VideoPlaybackSpeed::NormalHalf => VideoPlaybackSpeed::Double,
            _ => DoubleHalf
        }
    }

    pub fn decrement(self) -> Self {
        match self {
            VideoPlaybackSpeed::NormalHalf => VideoPlaybackSpeed::Normal,
            VideoPlaybackSpeed::Double => VideoPlaybackSpeed::NormalHalf,
            VideoPlaybackSpeed::DoubleHalf => VideoPlaybackSpeed::Double,
            _ => Half
        }
    }
}



pub trait VideoInternal: Clone {
    fn mute(&self, should_be_muted: bool) -> VideoResultUnit;

    fn fast_forward(&self) -> VideoResultUnit;

    fn rewind(&self) -> VideoResultUnit;

    fn pause(&self) -> VideoResultUnit;

    fn play(&self) -> VideoResult<js_sys::Promise>;

    fn get_volume(&self);

    fn get_playback_time(&self);

    fn get_progress(&self) -> VideoResult<f64>;

    fn get_video_length(&self) -> f64;

    fn set_video_progress(&self, time: f64);

    fn ready(&self) -> bool;

    fn set_volume(&self, volume: f64);

    fn set_min_progress(&mut self, percent: f64);

    fn get_min_progress(&self) -> f64;

    fn set_max_progress(&mut self, percent: f64);

    fn get_max_progress(&self) -> f64;

    fn set_playback_speed(&self, speed: VideoPlaybackSpeed);

    fn increment_video_speed(&mut self);

    fn decrement_video_speed(&mut self);

}

pub trait VideoNotInit {}


pub(crate) struct InternalVideoError(pub String);

impl<'a> From<&'a str> for InternalVideoError {
    fn from(s: &'a str) -> Self {
        InternalVideoError(s.to_string())
    }
}

impl<'a> From<String> for InternalVideoError {
    fn from(s: String) -> Self {
        InternalVideoError(s)
    }
}


impl Debug for InternalVideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for InternalVideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for InternalVideoError {}