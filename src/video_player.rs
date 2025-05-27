

pub struct VideoPlayer<S: VideoPlayerState> {

    marker: std::marker::PhantomData<S>,
}


impl VideoPlayer<Uninitialized> {

    pub fn new() -> Self {
        VideoPlayer {
            marker: std::marker::PhantomData
        }
    }
}

impl VideoPlayer<Ready> {

}

impl VideoPlayer<Playing> {

}

impl VideoPlayer<Paused> {

}

trait VideoPlayerState {

}

pub enum Uninitialized {}
pub enum Ready {}
pub enum Playing {}
pub enum Paused {}

impl VideoPlayerState for Uninitialized {}
impl VideoPlayerState for Ready {}
impl VideoPlayerState for Playing {}
impl VideoPlayerState for Paused {}