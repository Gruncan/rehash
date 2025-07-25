use crate::video::video_internal::{VideoInternal, VideoResult, VideoResultUnit};
use crate::video::video_player::{VideoController, VideoPlayer};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlVideoElement, SvgElement};

pub(crate) type HtmlVideoPlayer<S> = VideoPlayer<HtmlVideoPlayerInternal, S>;

pub(crate) struct HtmlVideoPlayerInternal {
    video_element: HtmlVideoElement,
}

impl VideoInternal for HtmlVideoPlayerInternal {
    fn mute(&self, should_be_muted: bool) -> VideoResultUnit {
        self.video_element.set_muted(should_be_muted);
        Ok(())
    }

    fn fast_forward(&self) -> VideoResultUnit {
        todo!()
    }

    fn rewind(&self) -> VideoResultUnit {
        todo!()
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


pub(crate) struct HtmlVideoController {
    play_icon: SvgElement,
    pause_icon: SvgElement,
}


impl VideoController for HtmlVideoController {
    fn get_element_ids(&self) -> Vec<String> {
        todo!()
    }

    fn swap_play_button(&self) {
        self.play_icon.style().set_property("display", "none").expect("Failed to set play icon");
        self.pause_icon.style().set_property("display", "block").expect("Failed to set pause icon");
    }

    fn swap_pause_button(&self) {
        self.play_icon.style().set_property("display", "block").expect("Failed to set play icon");
        self.pause_icon.style().set_property("display", "none").expect("Failed to set pause icon");
    }
}

impl HtmlVideoController {
    pub fn new(document: &Document) -> Self {
        let play_icon = document.get_element_by_id("play-icon")
            .expect("Failed to get play-icon")
            .dyn_into::<SvgElement>()
            .expect("Failed to cast SvgElement");

        let pause_icon = document.get_element_by_id("pause-icon")
            .expect("Failed to get pause-icon")
            .dyn_into::<SvgElement>()
            .expect("Failed to cast SvgElement");

        Self {
            play_icon,
            pause_icon,
        }
    }
}

