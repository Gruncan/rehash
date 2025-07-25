use crate::log;
use crate::video::video_callback_event::{CallbackController, MuteUnmuteEvent, PlayPauseEvent, VideoCallbackEvent};
use crate::video::video_internal::{VideoInternal, VideoResult, VideoResultUnit};
use crate::video::video_player::{SharedVideoPlayer, VideoPlayer, VideoUIController, VideoUIRegister};
use crate::{callback_event, console_log, debug_console_log, JsResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::closure::{Closure, WasmClosure};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, Element, HtmlVideoElement, KeyboardEvent, SvgElement};


const SKIP_INCREMENT: f64 = 5.0;

pub(crate) type HtmlVideoPlayer<S> = VideoPlayer<HtmlVideoPlayerInternal, S>;
type Event = Rc<RefCell<dyn VideoCallbackEvent<HtmlVideoPlayerInternal>>>;

pub(crate) struct HtmlVideoPlayerInternal {
    video_element: HtmlVideoElement,
}

impl VideoInternal for HtmlVideoPlayerInternal {
    fn mute(&self, should_be_muted: bool) -> VideoResultUnit {
        self.video_element.set_muted(should_be_muted);
        Ok(())
    }

    fn fast_forward(&self) -> VideoResultUnit {
        let to_move = (self.video_element.current_time() + SKIP_INCREMENT).max(self.video_element.duration());
        self.video_element.set_current_time(to_move);
        Ok(())
    }

    fn rewind(&self) -> VideoResultUnit {
        let current_time = self.video_element.current_time() - SKIP_INCREMENT.min(0f64);
        self.video_element.set_current_time(current_time);
        Ok(())
    }

    fn pause(&self) -> VideoResultUnit {
        match self.video_element.pause() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.as_string().unwrap().into()),
        }
    }

    fn play(&self) -> VideoResult<::js_sys::Promise> {
        match self.video_element.play() {
            Ok(p) => Ok(p),
            Err(err) => Err(err.as_string().unwrap().into()),
        }
    }

    fn get_volume(&self) {
        todo!()
    }

    fn get_playback_time(&self) {
        todo!()
    }
}

impl Clone for HtmlVideoPlayerInternal {
    fn clone(&self) -> Self {
        Self {
            video_element: self.video_element.clone(),
        }
    }
}

impl HtmlVideoPlayerInternal {
    pub fn new(video_element: HtmlVideoElement) -> Self {
        Self { video_element }
    }
}


pub(crate) struct HtmlVideoUIController {
    document: Document,
    play_icon: SvgElement,
    pause_icon: SvgElement,
    volume_icon: SvgElement,
    muted_icon: SvgElement,
}


impl VideoUIController<HtmlVideoPlayerInternal> for HtmlVideoUIController {

    fn swap_play_button(&self) {
        self.play_icon.style().set_property("display", "none").expect("Failed to set play icon");
        self.pause_icon.style().set_property("display", "block").expect("Failed to set pause icon");
    }

    fn swap_pause_button(&self) {
        self.play_icon.style().set_property("display", "block").expect("Failed to set play icon");
        self.pause_icon.style().set_property("display", "none").expect("Failed to set pause icon");
    }

    fn swap_mute_button(&self) {
        self.muted_icon.style().set_property("display", "block").expect("Failed to set mute icon");
        self.volume_icon.style().set_property("display", "none").expect("Failed to set volume icon");
    }

    fn swap_unmute_button(&self) {
        self.muted_icon.style().set_property("display", "none").expect("Failed to set mute icon");
        self.volume_icon.style().set_property("display", "block").expect("Failed to set volume icon");
    }
}

impl VideoUIRegister for HtmlVideoUIController {
    fn register_global_event_listener<T: ?Sized + WasmClosure>(&self, closure: Box<Closure<T>>) {
        self.document.add_event_listener_with_callback("keydown", closure.as_ref().as_ref().unchecked_ref())
            .expect("Failed to register global event listener");
        closure.forget();
    }

    fn register_element_event_listener<T: ?Sized + WasmClosure>(&self, ids: Vec<String>, closure: Box<Closure<T>>) {
        console_log!("{:?}", ids);
        for key in ids {
            if let Some(element) = self.document.get_element_by_id(key.as_str()) {
                element.add_event_listener_with_callback("click", closure.as_ref().as_ref().unchecked_ref())
                    .expect("Failed to add click event listener");
            }
        }
        closure.forget();
    }
}

impl HtmlVideoUIController {
    const PLAY_ICON_ID: &'static str = "play-icon";
    const PAUSE_ICON_ID: &'static str = "pause-icon";
    const PLAY_PAUSE_ID: &'static str = "play-pause";

    const VOLUME_ICON_ID: &'static str = "volume-icon";
    const MUTE_ICON_ID: &'static str = "mute-icon";
    const MUTE_UNMUTE_ID: &'static str = "volume-btn";


    pub fn new(document: Document) -> Self {
        let play_icon = Self::get_icon(&document, Self::PLAY_ICON_ID);

        let pause_icon = Self::get_icon(&document, Self::PAUSE_ICON_ID);

        let volume_icon = Self::get_icon(&document, Self::VOLUME_ICON_ID);

        let muted_icon = Self::get_icon(&document, Self::MUTE_ICON_ID);

        Self {
            document,
            play_icon,
            pause_icon,
            volume_icon,
            muted_icon,
        }
    }

    #[inline]
    fn get_icon(document: &Document, id: &str) -> SvgElement {
        document.get_element_by_id(id)
            .expect("Failed to get play-icon")
            .dyn_into::<SvgElement>()
            .expect("Failed to cast SvgElement")
    }
}


pub(crate) struct HtmlVideoCallbackController {
    video_player: SharedVideoPlayer,
    ui_controller: HtmlVideoUIController,
    callback_keyboard_events: HashMap<String, Event>,
    callback_control_events: HashMap<String, Event>,
}


impl HtmlVideoCallbackController {
    const PLAY_PAUSE_ID: &'static str = "play-pause";
    const MUTE_UNMUTE_ID: &'static str = "volume-btn";

    pub fn new(video_player: SharedVideoPlayer, ui_controller: HtmlVideoUIController) -> Self {
        let play_pause_event: Event = callback_event!(PlayPauseEvent);
        let mute_unmute_event: Event = callback_event!(MuteUnmuteEvent);

        let keyboard_events: HashMap<String, Event> = HashMap::from([
            ("p".to_string(), play_pause_event.clone()),
            ("m".to_string(), mute_unmute_event.clone()),
        ]);

        let control_events: HashMap<String, Event> = HashMap::from([
            (Self::PLAY_PAUSE_ID.to_string(), play_pause_event.clone()),
            (Self::MUTE_UNMUTE_ID.to_string(), mute_unmute_event.clone()),
        ]);

        Self {
            video_player,
            ui_controller,
            callback_keyboard_events: keyboard_events,
            callback_control_events: control_events,
        }
    }
}

impl CallbackController for HtmlVideoCallbackController {
    fn register_events(&self) {
        let mut video_player_k = self.video_player.clone();
        let d = self.callback_keyboard_events.clone();

        let keyboard_closure: Box<Closure<dyn FnMut(KeyboardEvent)>> = Box::new(Closure::new(move |event: KeyboardEvent| {
            let key = event.key();
            // TODO use this return
            let _ = callback_handler(&mut video_player_k, d.get(&key));
        }));

        self.ui_controller.register_global_event_listener(keyboard_closure);
        let mut video_player_c = self.video_player.clone();
        let c = self.callback_control_events.clone();
        console_log!("{:?}", c);
        let control_closure: Box<Closure<dyn FnMut(web_sys::Event)>> = Box::new(Closure::new(move |event: web_sys::Event| {
            let target = event.current_target().expect("Failed to get target for control callback");
            console_log!("{:?}", target);
            if let Some(element) = target.dyn_ref::<Element>() {
                let id = element.id();
                console_log!("Clicked Id: {}", id);
                if let Err(e) = callback_handler(&mut video_player_c, c.get(&id)) {
                    console_log!("Id: {}", id);
                }
            }
        }));

        let keys: Vec<String> = self.callback_control_events.iter().map(|(k, _)| k.clone()).collect();
        self.ui_controller.register_element_event_listener(keys, control_closure);
    }
}


fn callback_handler(ctx: &mut SharedVideoPlayer, callback_ref_opt: Option<&Event>) -> JsResult<()> {
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