use crate::prelude::*;
use crate::video::html_video::HtmlVideoPlayer;
use crate::video::video_player::{get_state_owned, Paused, Playing, SharedVideoPlayer};
use crate::{debug_console_log, JsResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Event, KeyboardEvent};

pub(crate) type VideoCallback = Rc<RefCell<dyn FnMut(&mut SharedVideoPlayer) -> JsResult<()>>>;


pub struct VideoCallbackHandler {
    video_player: SharedVideoPlayer,
    document: Document,

}

impl VideoCallbackHandler {
    pub fn new(video_player: SharedVideoPlayer, document: Document) -> JsResult<Self> {
        Ok(Self { video_player, document })
    }


    pub fn init_keyboard_callback(&mut self, map: HashMap<String, VideoCallback>) {
        let mut video_player_clone = self.video_player.clone();
        let closure: Closure<dyn FnMut(KeyboardEvent)> = Closure::new(move |event: KeyboardEvent| {
            let key = event.key();
            let func_option = map.get(&key);
            if let Some(func) = func_option {
                event.prevent_default();
                let mut callback = func.borrow_mut();
                match callback(&mut video_player_clone) {
                    Ok(_) => {}
                    Err(e) => {
                        debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                    }
                }
            } else {
                debug_console_log!("No callback found on key: {}", key)
            }
        });

        self.document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("Failed to add event listener");
        closure.forget();
    }

    pub fn init_control_callback(&mut self, map: HashMap<String, VideoCallback>) {
        let mut video_player_clone = self.video_player.clone();

        // TODO fix this
        let mut keys = Vec::with_capacity(map.len());
        for key in map.keys() {
            keys.push(key.clone());
        }

        debug_console_log!("{:?}", keys);

        let closure: Closure<dyn FnMut(Event)> = Closure::new(move |event: Event| {
            let target = event.target().expect("Failed to get target for control callback");
            if let Some(element) = target.dyn_ref::<Element>() {
                let id = element.id();
                let func_option = map.get(&id);
                if let Some(func) = func_option {
                    let mut callback = func.borrow_mut();
                    match callback(&mut video_player_clone) {
                        Ok(_) => {}
                        Err(e) => {
                            debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                        }
                    }
                } else {
                    // how is this even possible idk..
                    debug_console_log!("Callback failed on id: {}", id);
                }
            }
        });

        for key in keys {
            if let Some(element) = self.document.get_element_by_id(key.as_str()) {
                element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).expect("Failed to add click event listener");
            }
        }

        closure.forget();
    }
}

pub fn init_video_callback_handler(video_player: SharedVideoPlayer, document: Document) -> JsResult<VideoCallbackHandler> {
    // let ids = {
    //     let mutex = video_player.lock().unwrap();
    //     mutex.get_controller().get_element_ids()
    // };

    let mut callback_handler = VideoCallbackHandler::new(video_player, document)?;

    let play_callback: VideoCallback = Rc::new(RefCell::new(&play));
    let pause_callback: VideoCallback = Rc::new(RefCell::new(&pause));

    let keyboard_map = HashMap::from([
        ("k".to_string(), play_callback.clone()),
        ("l".to_string(), pause_callback.clone()),
    ]);

    callback_handler.init_keyboard_callback(keyboard_map);


    // let mut control_map = HashMap::with_capacity(ids.len());
    // for id in ids {
    //     control_map.insert(id, )
    // }

    let control_map = HashMap::from([
        ("play-icon".to_string(), play_callback.clone()),
        ("pause-icon".to_string(), pause_callback.clone()),
    ]);

    callback_handler.init_control_callback(control_map);

    Ok(callback_handler)
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
    let video_playing: HtmlVideoPlayer<Playing> = get_state_owned(cell.deref())?;

    let video: HtmlVideoPlayer<Paused> = video_playing.pause();

    *cell = Box::new(video);

    Ok(())
}