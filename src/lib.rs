
mod utils;
mod prelude;
mod video;

use crate::prelude::*;
use crate::video::html_video::{HtmlVideoCallbackController, HtmlVideoPlayerInternal, HtmlVideoUIController};
use crate::video::video_player::{SharedVideoPlayer, VideoPlayer, VideoUIController};
use js_sys::{Array, Reflect};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use video::video_callback_event::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Blob, BlobPropertyBag, HtmlVideoElement, Url};

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

async fn load_video_blob(video_element: &web_sys::HtmlVideoElement, file_path: &str) {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"path".into(), &file_path.into()).unwrap();

    match JsFuture::from(tauri_invoke("get_video_bytes", args.into())).await {
        Ok(result) => {
            // Convert to Uint8Array
            let uint8_array = js_sys::Uint8Array::new(&result);
            debug_console_log!("Received {} bytes", uint8_array.length());

            // Create blob
            let array = Array::new();
            array.push(&uint8_array);

            let mut blob_options = BlobPropertyBag::new();
            blob_options.type_("video/mp4");

            match Blob::new_with_u8_array_sequence_and_options(&array, &blob_options) {
                Ok(blob) => {
                    match Url::create_object_url_with_blob(&blob) {
                        Ok(blob_url) => {
                            debug_console_log!("Created blob URL: {}", blob_url);
                            video_element.set_src(&blob_url);
                            video_element.load();
                        }
                        Err(e) => { debug_console_log!("Failed to create blob URL: {:?}", e); }
                    }
                }
                Err(e) => { debug_console_log!("Failed to create blob: {:?}", e); }
            }
        }
        Err(e) => {
            debug_console_log!("Backend error: {:?}", e);
        }
    }
}

async fn init() -> JsResult<()> {
    let window = web_sys::window().ok_or("Failed to get window")?;
    let document = window.document().ok_or("Failed to get document")?;
    let video_element = document.get_element_by_id("video-player")
        .ok_or("Failed to get video player")?
        .dyn_into::<HtmlVideoElement>()?;

    // video_element.set_src("/static/pkg/66WithFacesV6Audio.mp4");


    let html_controller = HtmlVideoUIController::new(document.clone());
    let video_player = create_shared_video_player(Rc::new(html_controller), video_element);

    let html_controller = HtmlVideoUIController::new(document.clone());
    let callback_controller = HtmlVideoCallbackController::new(video_player.clone(), html_controller);
    callback_controller.register_events();

    let open_closure: Box<Closure<dyn FnMut(JsValue)>> = Box::new(Closure::new(move |event: JsValue| {
        let payload = Reflect::get(&event, &JsValue::from_str("payload")).expect("Failed to get payload");
        let video_path = payload.as_string().expect("Failed to get video path");
        debug_console_log!("Video path: {:?}", video_path);

        let video_element = document.get_element_by_id("video-player")
            .ok_or("Failed to get video player").expect("Failed to get video player")
            .dyn_into::<HtmlVideoElement>().expect("Failed to get video element");


        video_element.set_onloadstart(Some(&js_sys::Function::new_no_args("console.log('Video load started')")));
        video_element.set_oncanplay(Some(&js_sys::Function::new_no_args("console.log('Video can play')")));
        video_element.set_onloadeddata(Some(&js_sys::Function::new_no_args("console.log('Video data loaded')")));
        video_element.set_onerror(Some(&js_sys::Function::new_no_args("console.log('Video error:', this.error)")));


        spawn_local(async move {
            load_video_blob(&video_element, "/home/duncan/Development/Rust/rehash/static/pkg/66WithFacesV6Audio.mp4").await;
        });
        // let value = tauri_convert_file_src("/home/duncan/Development/Rust/rehash/static/pkg/66WithFacesV6Audio.mp4", None);
        // let source = value.as_string().unwrap();
        // let source = percent_encoding::percent_decode_str(&source).decode_utf8().unwrap();
        // debug_console_log!("{}", &source);
    }));

    tauri_listen("select-video-event", open_closure.as_ref().as_ref().unchecked_ref());

    open_closure.forget();

    Ok(())
}