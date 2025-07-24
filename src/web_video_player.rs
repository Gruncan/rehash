// use crate::prelude::*;
// use crate::video_player::{get_state_owned, Paused, Playing, SharedVideoPlayer, Uninitialized, VideoPlayer, VideoPlayerState};
// use crate::{console_log, JsResult};
// use std::ops::Deref;
// use std::sync::{Arc, Mutex};
// use wasm_bindgen::closure::Closure;
// use wasm_bindgen::prelude::wasm_bindgen;
// use wasm_bindgen::JsValue;
// use web_sys::{HtmlVideoElement, KeyboardEvent};
//
//
//
// pub type HtmlVideoPlayer<S> = VideoPlayer<S, HtmlVideoElement>;
//
//
// #[wasm_bindgen]
// pub struct WebVideoPlayer {
//     video_player: SharedVideoPlayer,    // TODO fix this
//     closure: Option<Closure<dyn FnMut(KeyboardEvent)>>,
// }
//
// #[wasm_bindgen]
// impl WebVideoPlayer {
//
//     #[wasm_bindgen(constructor)]
//     pub fn new(video_element: HtmlVideoElement) -> Result<Self, JsValue> {
//         video_element.set_src("./pkg/66WithFacesV6Audio.mp4");
//         video_element.set_controls(true);
//         let raw = VideoPlayer::<Uninitialized>::new(video_element);
//         console_log!("VideoPlayer initializing");
//
//
//         let video_player: SharedVideoPlayer = Arc::new(Mutex::new(Box::new(raw)));
//
//         let mut current = WebVideoPlayer {
//             video_player: video_player.clone(),
//             closure: None,
//         };
//
//         let closure: Closure<dyn FnMut(KeyboardEvent)> = Closure::new(move |event: web_sys::KeyboardEvent| {
//             let key = event.key();
//             event.prevent_default();
//             if key == "k" {
//                 current.play().expect("Failed to play");
//             } else if key == "l" {
//                 current.pause().expect("Failed to pause");
//             }
//         });
//
//
//         Ok(Self {
//             video_player,
//             closure: Some(closure),
//         })
//     }
//
//
//     pub fn play(&mut self) -> JsResult<()> {
//         let mutex = self.video_player.lock().unwrap();
//         let mut cell = mutex;
//         let video_paused: VideoPlayer<Paused> = get_state_owned(cell.deref()).unwrap();
//
//         let video: VideoPlayer<Playing> = video_paused.play();
//
//         *cell = Box::new(video);
//
//         Ok(())
//     }
//
//
//     pub fn pause(&mut self) -> JsResult<()> {
//         let mutex = self.video_player.lock().unwrap();
//         let mut cell = mutex;
//         let video_paused: VideoPlayer<Playing> = get_state_owned(cell.deref()).unwrap();
//
//         let video: VideoPlayer<Paused> = video_paused.pause();
//
//         *cell = Box::new(video);
//
//         Ok(())
//     }
//
// }