use crate::prelude::*;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::video_player::{get_state_owned, Paused, Playing, Uninitialized, VideoPlayer, VideoPlayerState};
use crate::{console_log, JsResult};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::HtmlVideoElement;

#[wasm_bindgen]
pub struct WebVideoPlayer {
    video_player: Arc<Mutex<Box<dyn VideoPlayerState>>>,
}

#[wasm_bindgen]
impl WebVideoPlayer {

    #[wasm_bindgen(constructor)]
    pub fn new(video_element: HtmlVideoElement) -> Result<Self, JsValue> {
        video_element.set_src("./pkg/66WithFacesV6Audio.mp4");
        video_element.set_controls(true);
        let raw = VideoPlayer::<Uninitialized>::new(video_element);
        console_log!("VideoPlayer initializing");

        // let closure = Closure::new(move |event: web_sys::KeyboardEvent| {
        //     if event.key() == "k" {
        //         event.prevent_default();
        //     }
        // });

        Ok(
            WebVideoPlayer {
                video_player: Arc::new(Mutex::new(Box::new(raw))),
            }
        )
    }

    #[wasm_bindgen]
    pub fn play(&mut self) -> JsResult<()> {
        let mut mutex = self.video_player.lock().unwrap();
        let video_paused: VideoPlayer<Paused> = get_state_owned(mutex.deref()).unwrap();
        let video: VideoPlayer<Playing> = video_paused.resume();
        *mutex = Box::new(video);
        Ok(())
    }


}