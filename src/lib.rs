
mod utils;
mod prelude;
mod web_video_player;
mod callback_handler;
mod video;

use crate::callback_handler::*;
use crate::prelude::*;
use crate::video::html_video::{HtmlVideoController, HtmlVideoPlayerInternal};
use crate::video::video_player::{SharedVideoPlayer, VideoPlayer};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, HtmlVideoElement};

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

#[inline]
fn create_shared_video_player(document: &Document, html_video_element: HtmlVideoElement) -> SharedVideoPlayer {
    let html_controller = Rc::new(HtmlVideoController::new(document));
    Arc::new(
        Mutex::new(
            Box::new(
                VideoPlayer::new(
                    HtmlVideoPlayerInternal::new(html_video_element),
                    html_controller
                )
            )
        )
    )
}

async fn init() -> JsResult<()> {
    let window = web_sys::window().ok_or("Failed to get window")?;
    let document = window.document().ok_or("Failed to get document")?;
    let video_element = document.get_element_by_id("video-player")
        .ok_or("Failed to get video player")?
        .dyn_into::<HtmlVideoElement>()?;

    video_element.set_src("../pkg/66WithFacesV6Audio.mp4");
    let video_player = create_shared_video_player(&document, video_element);

    let _callback_handler = CallbackHandler::new(video_player, document)?;

    Ok(())
}