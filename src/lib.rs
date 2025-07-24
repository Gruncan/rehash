
mod utils;
mod video_player;
mod prelude;
mod web_video_player;

use crate::prelude::*;
use crate::web_video_player::WebVideoPlayer;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlVideoElement;

type JsResult<T> = Result<T, JsValue>;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = init().await {
            error_log!("{}", e.as_string().unwrap());
        }
    })
}

async fn init() -> JsResult<()> {
    let window = web_sys::window().ok_or("Failed to get window")?;
    let document = window.document().ok_or("Failed to get document")?;
    let video_element = document.get_element_by_id("video-player")
        .ok_or("Failed to get video player")?
        .dyn_into::<HtmlVideoElement>()?;

    WebVideoPlayer::new(video_element);

    Ok(())
}