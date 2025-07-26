
mod utils;
mod prelude;
mod video;

use crate::prelude::*;
use crate::video::html_video::{HtmlVideoCallbackController, HtmlVideoPlayerInternal, HtmlVideoUIController};
use crate::video::video_player::{SharedVideoPlayer, VideoPlayer, VideoUIController};
use js_sys::Reflect;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use video::video_callback_event::*;
use wasm_bindgen::closure::Closure;
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

#[inline]
fn create_shared_video_player(html_controller: Rc<dyn VideoUIController<HtmlVideoPlayerInternal>>, html_video_element: HtmlVideoElement) -> SharedVideoPlayer {
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

    // video_element.set_src("/static/pkg/66WithFacesV6Audio.mp4");


    let html_controller = HtmlVideoUIController::new(document.clone());
    let video_player = create_shared_video_player(Rc::new(html_controller), video_element.clone());

    let html_controller = HtmlVideoUIController::new(document.clone());
    let callback_controller = HtmlVideoCallbackController::new(video_player.clone(), html_controller);
    callback_controller.register_events();

    let open_closure: Box<Closure<dyn FnMut(JsValue)>> = Box::new(Closure::new(move |event: JsValue| {
        let payload = Reflect::get(&event, &JsValue::from_str("payload")).expect("Failed to get payload");
        let video_path = payload.as_string().expect("Failed to get video path");
        debug_console_log!("Video path: {:?}", video_path);

        video_element.set_src(&video_path);
    }));

    tauri_listen("select-video-event", open_closure.as_ref().as_ref().unchecked_ref());

    open_closure.forget();

    Ok(())
}