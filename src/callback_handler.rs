use crate::prelude::*;
use crate::video::html_video::HtmlVideoPlayer;
use crate::video::video_player::{get_state_owned, Paused, Playing, SharedVideoPlayer};
use crate::{debug_console_log, JsResult};
use std::ops::Deref;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, KeyboardEvent};

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
    let video_paused: HtmlVideoPlayer<Paused> = get_state_owned(cell.deref())?;

    let video: HtmlVideoPlayer<Playing> = video_paused.play();

    *cell = Box::new(video);

    Ok(())
}


pub fn pause(ctx: &mut SharedVideoPlayer) -> JsResult<()> {
    let mutex = ctx.lock().unwrap();
    let mut cell = mutex;
    let video_paused: HtmlVideoPlayer<Playing> = get_state_owned(cell.deref())?;

    let video: HtmlVideoPlayer<Paused> = video_paused.pause();

    *cell = Box::new(video);

    Ok(())
}