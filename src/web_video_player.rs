use crate::prelude::*;

use crate::video_player::{Uninitialized, VideoPlayer};
use crate::console_log;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::HtmlVideoElement;

#[wasm_bindgen]
pub struct WebVideoPlayer {
    video_player: VideoPlayer<Uninitialized>,
}

#[wasm_bindgen]
impl WebVideoPlayer {
    #[wasm_bindgen(constructor)]
    pub fn new(video_element: HtmlVideoElement) -> Result<Self, JsValue> {
        let raw = VideoPlayer::<Uninitialized>::new(video_element);
        console_log!("VideoPlayer initializing");
        Ok(
            WebVideoPlayer {
                video_player: raw,
            }
        )
    }
}