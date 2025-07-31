
mod prelude;
mod video;
mod html;
mod tauri;

use crate::html::html_callback::HtmlVideoCallbackController;
use crate::prelude::*;
use crate::tauri::tauri_callback::FileOpenCallbackController;
use crate::video::video_player::{SharedVideoPlayer, VideoPlayer, VideoUIController};
use html::html_video::{HtmlVideoPlayerInternal, HtmlVideoUIController};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use video::event::CallbackController;
use video::video_callback::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, HtmlVideoElement};

use crate::prelude::*;

pub const WASM_VERSION: &str = env!("CARGO_PKG_VERSION");

type JsResult<T> = RehashResult<T>;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = init().await {
            error_log!("{}", e.to_string());
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


async fn init() -> RehashResultUnit {
    console_log!("WASM version: {}", WASM_VERSION);
    let window = web_sys::window().ok_or("Failed to get window")?;
    let document = window.document().ok_or("Failed to get document")?;
    let video_element = document.get_element_by_id("video-player")
        .ok_or("Failed to get video player")?
        .dyn_into::<HtmlVideoElement>()?;

    #[cfg(debug_assertions)]
    {
        let version_header = document.get_element_by_id("build-wasm")
            .ok_or("Failed to get video player")?
            .dyn_into::<HtmlElement>()?;

        version_header.set_text_content(Some(&format!("Build WASM: {}", WASM_VERSION)));

        let promise = tauri_invoke("get_desktop_build", JsValue::NULL);
        let result = JsFuture::from(promise).await?;
        let desktop_version = result.as_string().unwrap_or_default();

        let version_header = document.get_element_by_id("build-desktop")
            .ok_or("Failed to get video player")?
            .dyn_into::<HtmlElement>()?;

        version_header.set_text_content(Some(&format!("Build Desktop: {}", desktop_version)));
    }

    let html_controller = HtmlVideoUIController::new(document.clone());
    let video_player = create_shared_video_player(Rc::new(html_controller), video_element.clone());

    let html_controller = HtmlVideoUIController::new(document.clone());
    let mut callback_controller = HtmlVideoCallbackController::new(video_player.clone(), html_controller);
    callback_controller.register_events();


    let mut file_open_controller = FileOpenCallbackController::new(video_element);
    file_open_controller.register_events();

    Ok(())
}