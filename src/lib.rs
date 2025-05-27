
mod utils;
mod video_player;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlElement, HtmlVideoElement};
use crate::video_player::*;


#[wasm_bindgen]
pub struct WebVideoPlayer{
    video_player: VideoPlayer<Uninitialized>
}

#[wasm_bindgen]
impl WebVideoPlayer {

    #[wasm_bindgen(constructor)]
    pub fn new(video_element: HtmlVideoElement) -> Result<Self, JsValue> {
        utils::set_panic_hook();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let video_element = document.get_element_by_id("video-player")
            .unwrap()
            .dyn_into::<HtmlVideoElement>()?;

        let raw = VideoPlayer::<Uninitialized>::new();

        Ok(
            WebVideoPlayer{
                video_player: raw,
            }
        )

    }
}