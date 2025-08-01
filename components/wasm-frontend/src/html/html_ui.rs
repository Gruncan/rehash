use crate::get_element_as;
use crate::html::html_video::HtmlVideoPlayerInternal;
use crate::video::video_ui::{VideoUIController, VideoUIRegister};
use wasm_bindgen::closure::{Closure, WasmClosure};
use wasm_bindgen::JsCast;
use wasm_bindings_lib::debug_console_log;
use web_sys::{Document, HtmlDivElement, HtmlSpanElement, HtmlVideoElement, SvgElement};

pub use crate::prelude::*;

#[derive(Debug)]
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
    volume_fill: HtmlDivElement,

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
        // debug_console_log!("update_progress: {}:", progress);
        self.current_time.set_text_content(Some(format_time(progress).as_str()));
        self.total_time.set_text_content(Some(format_time(duration).as_str()));

        let percent = (progress / duration) * 100.0;

        self.progress_fill.style().set_property("width", format!("{}%", percent).as_str())
            .expect("Failed to set style for progress fill");
        self.progress_left.style().set_property("left", format!("{}%", percent).as_str())
            .expect("Failed to set style for progress left");
    }

    fn update_volume(&self, volume: f64) {
        debug_console_log!("Volume width {}", volume);
        self.volume_fill.style().set_property("width", format!("{}%", volume * 100f64).as_str())
            .expect("Failed to set volume");
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

    fn register_video_global_event_listener_specific<T: ?Sized + WasmClosure>(&self, string: &str, closure: Box<Closure<T>>) {
        self.video.add_event_listener_with_callback(string, closure.as_ref().as_ref().unchecked_ref())
            .expect("Failed to register global event listener");
        closure.forget();
    }

    fn register_doc_global_event_listener_specific<T: ?Sized + WasmClosure>(&self, string: &str, closure: Box<Closure<T>>) {
        self.document.add_event_listener_with_callback(string, closure.as_ref().as_ref().unchecked_ref())
            .expect("Failed to register global event listener");
        closure.forget();
    }

    fn register_element_event_listener_specific<T: ?Sized + WasmClosure>(&self, string: &str, id: &str, closure: Box<Closure<T>>) {
        self.document.get_element_by_id(id).expect(format!("Failed to find element with id {}", id).as_str())
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
    const PROGRESS_START_DOT: &'static str = "start-dot";
    const PROGRESS_END_DOT: &'static str = "end-dot";

    const SETTINGS_ICON_ID: &'static str = "settings-icon";
    const FULLSCREEN_ICON_ID: &'static str = "fullscreen-icon";

    const FAST_FORWARD_ICON_ID: &'static str = "fast-forward-icon";
    const REWIND_ICON_ID: &'static str = "rewind-icon";

    const VOLUME_FILL_ID: &'static str = "volume-fill";

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

        let volume_fill = get_element_as!(&document, Self::VOLUME_FILL_ID, HtmlDivElement);

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
            volume_fill,
        }
    }
}