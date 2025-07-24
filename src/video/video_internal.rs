use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type VideoResult = Result<(), InternalVideoError>;

pub trait VideoInternal: Clone {
    fn mute(&self, should_be_muted: bool) -> VideoResult;

    fn fast_forward(&self) -> VideoResult;

    fn rewind(&self) -> VideoResult;

    fn pause(&self) -> VideoResult;

    fn play(&self) -> VideoResult;
}


pub(crate) struct InternalVideoError {}

impl Debug for InternalVideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for InternalVideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for InternalVideoError {}