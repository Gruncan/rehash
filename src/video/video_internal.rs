use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type VideoResultUnit = Result<(), InternalVideoError>;
pub type VideoResult<T> = Result<T, InternalVideoError>;

pub trait VideoInternal: Clone {
    fn mute(&self, should_be_muted: bool) -> VideoResultUnit;

    fn fast_forward(&self) -> VideoResultUnit;

    fn rewind(&self) -> VideoResultUnit;

    fn pause(&self) -> VideoResultUnit;

    fn play(&self) -> VideoResult<js_sys::Promise>;

    fn get_volume(&self);

    fn get_playback_time(&self);

    fn get_progress(&self) -> VideoResult<f64>;

    fn get_video_length(&self) -> VideoResult<f64>;
}


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