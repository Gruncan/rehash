use crate::event::CallbackController;
use crate::get_element_as;
use crate::prelude::*;
use crate::video::video_callback_event::{CallbackEvent, CallbackEventInit, FastForwardEvent, FullScreenEvent, MuteUnmuteEvent, PlayPauseEvent, ProgressBarChangeEvent, ProgressBarClickEvent, ProgressBarClickEventCtx, ProgressBarClickEventCtxType, RewindEvent, SettingsEvent};
use crate::video::video_internal::{VideoInternal, VideoResult, VideoResultUnit};
use crate::video::video_player::{SharedVideoPlayer, VideoPlayer, VideoUIController, VideoUIRegister};
use crate::{callback_event, console_log, debug_console_log, JsResult};
use std::cell::RefCell;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::closure::{Closure, WasmClosure};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, Element, HtmlDivElement, HtmlElement, HtmlSpanElement, HtmlVideoElement, KeyboardEvent, SvgElement};

const SKIP_INCREMENT: f64 = 5.0;

pub(crate) type HtmlVideoPlayer<S> = VideoPlayer<HtmlVideoPlayerInternal, S>;
type Event = Rc<RefCell<dyn CallbackEvent<SharedVideoPlayer>>>;
type EventT<T> = Rc<RefCell<dyn CallbackEvent<T>>>;

pub(crate) struct HtmlVideoPlayerInternal {
    video_element: HtmlVideoElement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub(crate) enum InternalVideoReadiness {
    Nothing,
    MedaData,
    CurrentData,
    FutureData,
    AllData,
    Unknown(u16),
}

impl From<u16> for InternalVideoReadiness {
    fn from(value: u16) -> Self {
        match value {
            0 => InternalVideoReadiness::Nothing,
            1 => InternalVideoReadiness::MedaData,
            2 => InternalVideoReadiness::CurrentData,
            3 => InternalVideoReadiness::FutureData,
            4 => InternalVideoReadiness::AllData,
            other => InternalVideoReadiness::Unknown(other),
        }
    }
}



#[macro_export]
macro_rules! get_element_as {
    ($document:expr, $id:expr, $t:ty) => {
        $document
            .get_element_by_id($id)
            .expect(&format!("Failed to get element with id '{}'", $id))
            .dyn_into::<$t>()
            .expect(&format!("Failed to cast element '{}' to {}", $id, stringify!($t)))
    };
}


impl VideoInternal for HtmlVideoPlayerInternal {
    fn mute(&self, should_be_muted: bool) -> VideoResultUnit {
        self.video_element.set_muted(should_be_muted);
        Ok(())
    }

    fn fast_forward(&self) -> VideoResultUnit {
        let to_move = (self.video_element.current_time() + SKIP_INCREMENT).min(self.video_element.duration());
        console_log!("Fast forwarding to: {}", to_move);
        self.video_element.set_current_time(to_move);
        Ok(())
    }

    fn rewind(&self) -> VideoResultUnit {
        let current_time = (self.video_element.current_time() - SKIP_INCREMENT).max(0f64);
        console_log!("Rewinding to: {}", current_time);
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

    fn get_progress(&self) -> VideoResult<f64> {
        Ok(self.video_element.current_time())
    }

    fn get_video_length(&self) -> VideoResult<f64> {
        Ok(self.video_element.duration())
    }

    fn set_video_progress(&self, progress: f64) {
        self.video_element.set_current_time(progress);
    }

    fn ready(&self) -> bool {
        let state: InternalVideoReadiness = self.video_element.ready_state().into();
        state >= InternalVideoReadiness::CurrentData
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
    video: HtmlVideoElement,
    play_icon: SvgElement,
    pause_icon: SvgElement,
    volume_icon: SvgElement,
    muted_icon: SvgElement,
    current_time: HtmlSpanElement,
    total_time: HtmlSpanElement,
    progress_fill: HtmlDivElement,
    progress_left: HtmlDivElement,
    settings_icon: SvgElement,
    fullscreen_icon: SvgElement,
    fast_forward_icon: SvgElement,
    rewind_icon: SvgElement,
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

    fn update_progress(&self, progress: f64, duration: f64) {
        self.current_time.set_text_content(Some(format_time(progress).as_str()));
        self.total_time.set_text_content(Some(format_time(duration).as_str()));

        let percent = (progress / duration) * 100.0;

        self.progress_fill.style().set_property("width", format!("{}%", percent).as_str())
            .expect("Failed to set style for progress fill");
        self.progress_left.style().set_property("left", format!("{}%", percent).as_str())
            .expect("Failed to set style for progress left");

    }
}

#[inline]
fn format_time(time: f64) -> String {
    let mins = (time / 60.0).floor();
    let secs = (time % 60.0).floor();
    format!("{:02}:{:02}", mins, secs)
}

impl VideoUIRegister for HtmlVideoUIController {
    fn register_global_event_listener<T: ?Sized + WasmClosure>(&self, closure: Box<Closure<T>>) {
        self.document.add_event_listener_with_callback("keydown", closure.as_ref().as_ref().unchecked_ref())
            .expect("Failed to register global event listener");
        closure.forget();
    }

    fn register_element_event_listener<T: ?Sized + WasmClosure>(&self, ids: Vec<String>, closure: Box<Closure<T>>) {
        debug_console_log!("Registers control events: {:?}", ids);
        for key in ids {
            if let Some(element) = self.document.get_element_by_id(key.as_str()) {
                element.add_event_listener_with_callback("click", closure.as_ref().as_ref().unchecked_ref())
                    .expect("Failed to add click event listener");
            }
        }
        closure.forget();
    }

    fn register_global_event_listener_specific<T: ?Sized + WasmClosure>(&self, string: &str, closure: Box<Closure<T>>) {
        self.video.add_event_listener_with_callback(string, closure.as_ref().as_ref().unchecked_ref())
            .expect("Failed to register global event listener");
        closure.forget();
    }

    fn register_element_event_listener_specific<T: ?Sized + WasmClosure>(&self, string: &str, id: &str, closure: Box<Closure<T>>) {
        self.document.get_element_by_id(id).expect("Failed to find element with id")
            .add_event_listener_with_callback(string, closure.as_ref().as_ref().unchecked_ref())
            .expect("Failed to register element event listener");
        closure.forget();
    }
}

impl HtmlVideoUIController {
    const PLAY_ICON_ID: &'static str = "play-icon";
    const PAUSE_ICON_ID: &'static str = "pause-icon";

    const VOLUME_ICON_ID: &'static str = "volume-icon";
    const MUTE_ICON_ID: &'static str = "mute-icon";

    const CURRENT_TIME_ID: &'static str = "current-time";
    const TOTAL_TIME_ID: &'static str = "total-time";

    const PROGRESS_FILL: &'static str = "progress-fill";
    const PROGRESS_LEFT: &'static str = "progress-handle";

    const SETTINGS_ICON_ID: &'static str = "settings-icon";
    const FULLSCREEN_ICON_ID: &'static str = "fullscreen-icon";

    const FAST_FORWARD_ICON_ID: &'static str = "fast-forward-icon";
    const REWIND_ICON_ID: &'static str = "rewind-icon";

    const VIDEO_ID: &'static str = "video-player";


    pub fn new(document: Document) -> Self {
        let play_icon = get_element_as!(&document, Self::PLAY_ICON_ID, SvgElement);
        let pause_icon = get_element_as!(&document, Self::PAUSE_ICON_ID, SvgElement);

        let volume_icon = get_element_as!(&document, Self::VOLUME_ICON_ID, SvgElement);
        let muted_icon = get_element_as!(&document, Self::MUTE_ICON_ID, SvgElement);

        let current_time = get_element_as!(&document, Self::CURRENT_TIME_ID, HtmlSpanElement);
        let total_time = get_element_as!(&document, Self::TOTAL_TIME_ID, HtmlSpanElement);

        let progress_fill = get_element_as!(&document, Self::PROGRESS_FILL, HtmlDivElement);
        let progress_left = get_element_as!(&document, Self::PROGRESS_LEFT, HtmlDivElement);

        let settings_icon = get_element_as!(&document, Self::SETTINGS_ICON_ID, SvgElement);
        let fullscreen_icon = get_element_as!(&document, Self::FULLSCREEN_ICON_ID, SvgElement);

        let fast_forward_icon = get_element_as!(&document, Self::FAST_FORWARD_ICON_ID, SvgElement);
        let rewind_icon = get_element_as!(&document, Self::REWIND_ICON_ID, SvgElement);

        let video_element = get_element_as!(&document, Self::VIDEO_ID, HtmlVideoElement);


        Self {
            document,
            video: video_element,
            play_icon,
            pause_icon,
            volume_icon,
            muted_icon,
            current_time,
            total_time,
            progress_fill,
            progress_left,
            settings_icon,
            fullscreen_icon,
            fast_forward_icon,
            rewind_icon,
        }
    }


}


pub(crate) struct HtmlVideoCallbackController {
    video_player: SharedVideoPlayer,
    ui_controller: HtmlVideoUIController,
    callback_keyboard_events: HashMap<String, Event>,
    callback_control_events: HashMap<String, Event>,
    callback_progress_event: Event,
    callback_progress_click_event: EventT<ProgressBarClickEventCtxType>,
}


impl HtmlVideoCallbackController {
    const PLAY_PAUSE_ID: &'static str = "play-pause";
    const MUTE_UNMUTE_ID: &'static str = "volume-btn";
    const SETTINGS_ID: &'static str = "settings";
    const FULLSCREEN_ID: &'static str = "fullscreen";
    const PROGRESS_BAR_ID: &'static str = "progress-bar";
    const FAST_FORWARD_ID: &'static str = "fast-forward";
    const REWIND_ID: &'static str = "rewind";

    pub fn new(video_player: SharedVideoPlayer, ui_controller: HtmlVideoUIController) -> Self {
        let play_pause_event: Event = callback_event!(PlayPauseEvent<HtmlVideoPlayerInternal>);
        let mute_unmute_event: Event = callback_event!(MuteUnmuteEvent);
        let progress_event: Event = callback_event!(ProgressBarChangeEvent);
        let settings_event: Event = callback_event!(SettingsEvent);
        let fullscreen_event: Event = callback_event!(FullScreenEvent);
        let progress_click_event: EventT<ProgressBarClickEventCtxType> = callback_event!(ProgressBarClickEvent);

        let fast_forward_event: Event = callback_event!(FastForwardEvent);
        let rewind_event: Event = callback_event!(RewindEvent);

        let keyboard_events: HashMap<String, Event> = HashMap::from([
            ("p".to_string(), play_pause_event.clone()),
            ("m".to_string(), mute_unmute_event.clone()),
            ("ArrowRight".to_string(), fast_forward_event.clone()),
            ("ArrowLeft".to_string(), rewind_event.clone()),
        ]);

        let control_events: HashMap<String, Event> = HashMap::from([
            (Self::PLAY_PAUSE_ID.to_string(), play_pause_event.clone()),
            (Self::MUTE_UNMUTE_ID.to_string(), mute_unmute_event.clone()),
            (Self::SETTINGS_ID.to_string(), settings_event.clone()),
            (Self::FULLSCREEN_ID.to_string(), fullscreen_event.clone()),
            (Self::FAST_FORWARD_ID.to_string(), fast_forward_event.clone()),
            (Self::REWIND_ID.to_string(), rewind_event.clone()),
        ]);


        Self {
            video_player,
            ui_controller,
            callback_keyboard_events: keyboard_events,
            callback_control_events: control_events,
            callback_progress_event: progress_event,
            callback_progress_click_event: progress_click_event
        }
    }
}

impl CallbackController for HtmlVideoCallbackController {
    fn register_events(&mut self) {
        let mut video_player_k = self.video_player.clone();
        let d = self.callback_keyboard_events.clone();

        let keyboard_closure: Box<Closure<dyn FnMut(KeyboardEvent)>> = Box::new(Closure::new(move |event: KeyboardEvent| {
            let key = event.key();
            #[cfg(not(debug_assertions))]
            {
                if !d.contains_key(&key) {
                    event.prevent_default();
                }
            }
            // TODO use this return
            let _ = callback_handler(&mut video_player_k, d.get(&key));
        }));

        self.ui_controller.register_global_event_listener(keyboard_closure);

        let mut video_player_c = self.video_player.clone();
        let c = self.callback_control_events.clone();
        let control_closure: Box<Closure<dyn FnMut(web_sys::Event)>> = Box::new(Closure::new(move |event: web_sys::Event| {
            let target = event.current_target().expect("Failed to get target for control callback");

            if let Some(element) = target.dyn_ref::<Element>() {
                let id = element.id();
                if let Err(e) = callback_handler(&mut video_player_c, c.get(&id)) {
                    console_log!("Failed callback on ID: {}", id);
                }
            }
        }));

        let keys: Vec<String> = self.callback_control_events.iter().map(|(k, _)| k.clone()).collect();
        self.ui_controller.register_element_event_listener(keys, control_closure);

        let mut video_player_t = self.video_player.clone();
        let t = self.callback_progress_event.clone();

        let timeupdate_closure: Box<Closure<dyn FnMut()>> = Box::new(Closure::new(move || {
            // TODO use this return
            let _ = callback_handler(&mut video_player_t, Some(&t));
        }));

        self.ui_controller.register_global_event_listener_specific("timeupdate", timeupdate_closure);

        let video_player_p = self.video_player.clone();
        let p = self.callback_progress_click_event.clone();
        let progress_bar_click_closure: Box<Closure<dyn FnMut(web_sys::MouseEvent)>> = Box::new(Closure::new(move |event: web_sys::MouseEvent| {
            let target = event.target().unwrap();
            if let Some(element) = target.dyn_ref::<HtmlElement>() {
                let rec = element.get_bounding_client_rect();
                let click_x = event.client_x() as f64;
                let percent = (click_x - rec.left()) / rec.width();

                // think unneeded as trigger will take 'single ownership'
                // todo fix second player clone
                let mut ctx = Arc::new(Mutex::new(ProgressBarClickEventCtx { video_player: video_player_p.clone(), time_to_seek: percent }));
                let mut callback = p.borrow_mut();
                match callback.trigger(&mut ctx) {
                    Ok(_) => {}
                    Err(e) => {
                        debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                    }
                }
            }
        }));

        self.ui_controller.register_element_event_listener_specific("click", Self::PROGRESS_BAR_ID, progress_bar_click_closure);
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