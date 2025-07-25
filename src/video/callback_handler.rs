use crate::prelude::*;
use crate::video::html_video::HtmlVideoPlayer;
use crate::video::video_player::{get_state_owned, Paused, Playing, SharedVideoPlayer, VideoPlayerState};
use crate::{console_log, debug_console_log, JsResult};
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, Element, Event, KeyboardEvent};

pub(crate) type VideoCallbackEventType = Rc<RefCell<dyn VideoCallbackEvent>>;


pub struct VideoCallbackHandler {
    video_player: SharedVideoPlayer,
    document: Document,

}

impl VideoCallbackHandler {
    pub fn new(video_player: SharedVideoPlayer, document: Document) -> JsResult<Self> {
        Ok(Self { video_player, document })
    }


    pub fn init_keyboard_callback(&mut self, map: HashMap<String, VideoCallbackEventType>) {
        let mut video_player_clone = self.video_player.clone();
        let closure: Closure<dyn FnMut(KeyboardEvent)> = Closure::new(move |event: KeyboardEvent| {
            let key = event.key();
            // TODO use this return
            let _ = callback_handler(&mut video_player_clone, map.get(&key));
        });

        self.document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("Failed to add event listener");
        closure.forget();
    }

    pub fn init_control_callback(&mut self, map: HashMap<String, VideoCallbackEventType>) {
        let mut video_player_clone = self.video_player.clone();

        // TODO fix this
        let mut keys = Vec::with_capacity(map.len());
        for key in map.keys() {
            keys.push(key.clone());
        }

        debug_console_log!("{:?}", keys);

        let closure: Closure<dyn FnMut(Event)> = Closure::new(move |event: Event| {
            let target = event.current_target().expect("Failed to get target for control callback");
            if let Some(element) = target.dyn_ref::<Element>() {
                let id = element.id();
                if let Err(e) = callback_handler(&mut video_player_clone, map.get(&id)) {
                    console_log!("Id: {}", id);
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


fn callback_handler(ctx: &mut SharedVideoPlayer, callback_ref_opt: Option<&VideoCallbackEventType>) -> JsResult<()> {
    if let Some(callback_ref) = callback_ref_opt {
        let mut callback = callback_ref.borrow_mut();
        match callback.trigger(ctx) {
            Ok(_) => { Ok(()) }
            Err(e) => {
                debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                Err(e)
            }
        }
    } else {
        debug_console_log!("Callback not found");
        Err(JsValue::from_str("Callback not found"))
    }
}

pub fn init_video_callback_handler(video_player: SharedVideoPlayer, document: Document) -> JsResult<VideoCallbackHandler> {
    // let ids = {
    //     let mutex = video_player.lock().unwrap();
    //     mutex.get_controller().get_element_ids()
    // };

    let mut callback_handler = VideoCallbackHandler::new(video_player, document)?;

    let play_pause_event: VideoCallbackEventType = Rc::new(RefCell::new(PlayPauseEvent::new()));

    let keyboard_map = HashMap::from([
        ("p".to_string(), play_pause_event.clone()),
    ]);

    callback_handler.init_keyboard_callback(keyboard_map);


    // let mut control_map = HashMap::with_capacity(ids.len());
    // for id in ids {
    //     control_map.insert(id, )
    // }

    let control_map = HashMap::from([
        ("play-pause".to_string(), play_pause_event.clone()),
        // ("pathplay".to_string(), play_pause_event.clone()),
        // ("pathpause".to_string(), play_pause_event.clone()),
    ]);

    callback_handler.init_control_callback(control_map);

    Ok(callback_handler)
}


pub(crate) trait VideoCallbackEvent {
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()>;
}


pub(crate) struct PlayPauseEvent {
    type_id: TypeId,
}


impl VideoCallbackEvent for PlayPauseEvent {
    fn trigger(&mut self, ctx: &mut SharedVideoPlayer) -> JsResult<()> {
        let mutex = ctx.lock().unwrap();
        let mut cell = mutex;


        let standard: Box<dyn VideoPlayerState>;
        if self.is_paused() {
            let video_paused: HtmlVideoPlayer<Paused> = get_state_owned(cell.deref())?;
            let video: HtmlVideoPlayer<Playing> = video_paused.play();
            self.type_id = TypeId::of::<Playing>();
            standard = Box::new(video);
        } else {
            let video_playing: HtmlVideoPlayer<Playing> = get_state_owned(cell.deref())?;
            let video: HtmlVideoPlayer<Paused> = video_playing.pause();
            self.type_id = TypeId::of::<Paused>();
            standard = Box::new(video);
        }
        *cell = standard;

        Ok(())
    }
}

impl PlayPauseEvent {
    const PLAY_PAUSE_ID: &'static str = "play-pause";
    const PLAY_ICON_ID: &'static str = "play-icon";
    const PAUSE_ICON_ID: &'static str = "pause-icon";

    pub fn new() -> Self {
        PlayPauseEvent {
            type_id: TypeId::of::<Paused>(),
        }
    }

    pub fn is_paused(&self) -> bool {
        self.type_id == TypeId::of::<Paused>()
    }
}


