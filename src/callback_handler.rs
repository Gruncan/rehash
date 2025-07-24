use crate::prelude::*;
use crate::video_player::{get_state_owned, Paused, Playing, VideoPlayer, VideoPlayerState};
use crate::{debug_console_log, JsResult};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, KeyboardEvent};

pub(crate) type SharedVideoPlayer = Arc<Mutex<Box<dyn VideoPlayerState>>>;

pub struct CallbackHandler {
    video_player: SharedVideoPlayer,

}

impl CallbackHandler {
    pub fn new(video_player: SharedVideoPlayer, document: Document) -> JsResult<Self> {
        debug_console_log!("Callbackhandler initializing");
        let mut video_player_clone = video_player.clone();
        let closure: Closure<dyn FnMut(KeyboardEvent)> = Closure::new(move |event: KeyboardEvent| {
            let key = event.key();
            event.prevent_default();
            if key == "k" {
                play(&mut video_player_clone);
            } else if key == "l" {
                pause(&mut video_player_clone);
            }
        });

        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(CallbackHandler { video_player })
    }
}


pub fn play(ctx: &mut SharedVideoPlayer) -> JsResult<()> {
    let mutex = ctx.lock().unwrap();
    let mut cell = mutex;
    let video_paused: VideoPlayer<Paused> = get_state_owned(cell.deref())?;

    let video: VideoPlayer<Playing> = video_paused.play();

    *cell = Box::new(video);

    Ok(())
}


pub fn pause(ctx: &mut SharedVideoPlayer) -> JsResult<()> {
    let mutex = ctx.lock().unwrap();
    let mut cell = mutex;
    let video_paused: VideoPlayer<Playing> = get_state_owned(cell.deref())?;

    let video: VideoPlayer<Paused> = video_paused.pause();

    *cell = Box::new(video);

    Ok(())
}