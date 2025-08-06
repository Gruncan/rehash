use crate::callback_event;
use crate::html::html_events::*;
use crate::html::html_ui::HtmlVideoUIController;
use crate::html::html_video::HtmlVideoPlayerInternal;
use crate::prelude::*;
use crate::video::event::{CallbackController, CallbackEvent};
use crate::video::video_callback::CallbackClosureWrapper;
use crate::video::video_callback::SharedVideoPlayer;
use crate::video::video_ui::VideoUIRegister;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::Element;

pub(crate) use control_closure::*;
pub(crate) use drag_closure::*;
pub(crate) use keyboard_closure::*;
pub(crate) use time_update_closure::*;


pub(crate) struct HtmlVideoCallbackController {
    video_player: SharedVideoPlayer,
    ui_controller: HtmlVideoUIController,
    callback_keyboard_events: HashMap<String, crate::html::html_video::Event>,
    callback_control_events: HashMap<String, crate::html::html_video::Event>,
    callback_progress_event: crate::html::html_video::Event,
}


impl HtmlVideoCallbackController {
    const PLAY_PAUSE_ID: &'static str = "play-pause";
    const MUTE_UNMUTE_ID: &'static str = "volume-btn";
    const SETTINGS_ID: &'static str = "settings";
    const FULLSCREEN_ID: &'static str = "fullscreen";
    const FAST_FORWARD_ID: &'static str = "fast-forward";
    const REWIND_ID: &'static str = "rewind";

    pub fn new(video_player: SharedVideoPlayer, ui_controller: HtmlVideoUIController) -> Self {
        let play_pause_event: crate::html::html_video::Event = callback_event!(PlayPauseEvent<HtmlVideoPlayerInternal>);
        let mute_unmute_event: crate::html::html_video::Event = callback_event!(MuteUnmuteEvent);
        let progress_event: crate::html::html_video::Event = callback_event!(ProgressBarChangeEvent);
        let settings_event: crate::html::html_video::Event = callback_event!(SettingsEvent);
        let fullscreen_event: crate::html::html_video::Event = callback_event!(FullScreenEvent);



        let fast_forward_event: crate::html::html_video::Event = callback_event!(FastForwardEvent);
        let rewind_event: crate::html::html_video::Event = callback_event!(RewindEvent);

        let playback_increase = callback_event!(PlaybackSpeedEvent<PlaybackIncreaseAction>);
        let playback_decrease = callback_event!(PlaybackSpeedEvent<PlaybackDecreaseAction>);

        let keyboard_events: HashMap<String, crate::html::html_video::Event> = HashMap::from([
            ("p".to_string(), play_pause_event.clone()),
            ("m".to_string(), mute_unmute_event.clone()),
            ("ArrowRight".to_string(), fast_forward_event.clone()),
            ("ArrowLeft".to_string(), rewind_event.clone()),
            ("ArrowUp".to_string(), playback_increase.clone()),
            ("ArrowDown".to_string(), playback_decrease.clone()),
        ]);

        let control_events: HashMap<String, crate::html::html_video::Event> = HashMap::from([
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
        }
    }
}


impl CallbackController for HtmlVideoCallbackController {
    fn register_events(&self) {
        let keyboard = Box::new(KeyboardClosure::new(self.video_player.clone(), self.callback_keyboard_events.clone()));
        let keyboard_closure = CallbackClosureWrapper::create_callback(keyboard);
        self.ui_controller.register_global_event_listener(keyboard_closure);


        let control = Box::new(ControlClosure::new(self.video_player.clone(), self.callback_control_events.clone()));
        let control_closure = CallbackClosureWrapper::create_callback(control);
        let keys: Vec<String> = self.callback_control_events.iter().map(|(k, _)| k.clone()).collect();
        self.ui_controller.register_element_event_listener(keys, control_closure);


        let time_update = Box::new(TimeUpdateClosure::new(self.video_player.clone(), self.callback_progress_event.clone()));
        let timeupdate_closure = CallbackClosureWrapper::create_callback(time_update);
        self.ui_controller.register_video_global_event_listener_specific("timeupdate", timeupdate_closure);


        let progress_bar_id: &str = MoveState::ProgressBar.try_into().unwrap();
        let volume_bar_id: &str = MoveState::VolumeBar.try_into().unwrap();
        let start_dot_id: &str = MoveState::StartClipDot.try_into().unwrap();
        let end_dot_id: &str = MoveState::EndClipDot.try_into().unwrap();


        let drag_ctx = Rc::new(RefCell::new(DragEventCtx::new(self.video_player.clone())));


        let doc = self.ui_controller.get_document();

        let volume_bar_element = doc.get_element_by_id(volume_bar_id).unwrap(); //.dyn_into::<Element>().unwrap();

        let mouse_click_volume_wrapper = Box::new(DragClickClosure::new(volume_bar_element, drag_ctx.clone()));
        let mouse_click_volume_closure = CallbackClosureWrapper::create_callback(mouse_click_volume_wrapper);

        self.ui_controller.register_element_event_listener_specific("mousedown", volume_bar_id, mouse_click_volume_closure);


        let progress_bar_element = doc.get_element_by_id(progress_bar_id).unwrap(); //.dyn_into::<HtmlElement>().unwrap();

        let mouse_click_progress_wrapper = Box::new(DragClickClosure::new(progress_bar_element, drag_ctx.clone()));
        let mouse_click_progress_closure = CallbackClosureWrapper::create_callback(mouse_click_progress_wrapper);

        self.ui_controller.register_element_event_listener_specific("mousedown", progress_bar_id, mouse_click_progress_closure);


        let start_dot_element = doc.get_element_by_id(start_dot_id).unwrap(); //.dyn_into::<HtmlElement>().unwrap();

        let mouse_click_start_dot_wrapper = Box::new(DragClickClosure::new(start_dot_element, drag_ctx.clone()));
        let mouse_click_start_dot_closure = CallbackClosureWrapper::create_callback(mouse_click_start_dot_wrapper);

        self.ui_controller.register_element_event_listener_specific("mousedown", start_dot_id, mouse_click_start_dot_closure);


        let end_dot_element = doc.get_element_by_id(end_dot_id).unwrap(); //.dyn_into::<HtmlElement>().unwrap();

        let mouse_click_end_dot_wrapper = Box::new(DragClickClosure::new(end_dot_element, drag_ctx.clone()));
        let mouse_click_end_dot_closure = CallbackClosureWrapper::create_callback(mouse_click_end_dot_wrapper);

        self.ui_controller.register_element_event_listener_specific("mousedown", end_dot_id, mouse_click_end_dot_closure);


        // percentage should be relative to specific element but global mouse down need current click to get width
        // DragMoveClosure::new(container_element)
        // self.ui_controller.register_doc_global_event_listener_specific("mousemove", mouse_move_start_dot_closure);
        //
        //
        // self.ui_controller.register_doc_global_event_listener_specific("mouseup", drag_exit_closure);


        debug_console_log!("Registered callback handlers");
    }
}


mod drag_closure {
    use super::*;
    use crate::log_to_tauri;
    use crate::video::video_callback::CallbackClosureWrapper;
    use std::fmt::Debug;

    #[derive(Debug, Clone)]
    pub(crate) struct DragClickClosure {
        slider_width: f64,
        slider_left: f64,
        slider_type: MoveState,
        callback: DragClickEvent,
        ctx: DragEventCtxType,
    }


    impl CallbackClosureWrapper<web_sys::MouseEvent> for DragClickClosure {
        fn closure(&mut self, event: web_sys::MouseEvent) {
            let click_x = event.client_x() as f64;
            let percent = ((click_x - self.slider_left) / self.slider_width).max(0f64).min(1f64);
            {
                let mut ctx = self.ctx.borrow_mut();
                ctx.set_percent(percent);
                ctx.set_clicked(self.slider_type);
            }
            match self.callback.trigger(&mut self.ctx) {
                Ok(_) => {}
                Err(e) => {
                    debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                }
            }
        }
    }

    impl DragClickClosure {
        pub fn new(element: Element, ctx: DragEventCtxType) -> Self {
            let moving_state: MoveState = element.id().as_str().into();
            let rec = element.get_bounding_client_rect();
            let callback = DragClickEvent {};

            Self {
                slider_left: rec.left(),
                slider_width: rec.width(),
                slider_type: moving_state,
                callback,
                ctx,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct DragMoveClosure {
        slider_width: f64,
        slider_left: f64,
        callback: DragMoveEvent,
        ctx: DragEventCtxType,
    }

    impl CallbackClosureWrapper<web_sys::MouseEvent> for DragMoveClosure {
        fn closure(&mut self, event: web_sys::MouseEvent) {
            //todo fix this to not repeat
            let click_x = event.client_x() as f64;
            let percent = ((click_x - self.slider_left) / self.slider_width).max(0f64).min(1f64);
            {
                let mut ctx = self.ctx.borrow_mut();
                ctx.set_percent(percent);
            }
            match self.callback.trigger(&mut self.ctx) {
                Ok(_) => {}
                Err(e) => {
                    debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                }
            }
        }
    }

    impl DragMoveClosure {
        pub fn new(element: Element, ctx: DragEventCtxType) -> Self {
            let rec = element.get_bounding_client_rect();
            let callback = DragMoveEvent {};

            Self {
                slider_left: rec.left(),
                slider_width: rec.width(),
                callback,
                ctx,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct DragExitClosure {
        callback: DragExitEvent,
        ctx: DragEventCtxType,
    }

    impl CallbackClosureWrapper<web_sys::MouseEvent> for DragExitClosure {
        fn closure(&mut self, _: web_sys::MouseEvent) {
            match self.callback.trigger(&mut self.ctx) {
                Ok(_) => {}
                Err(e) => {
                    debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                }
            }
        }
    }

}

mod time_update_closure {
    use super::*;

    type Ctx = SharedVideoPlayer;
    type Callback = Rc<RefCell<dyn CallbackEvent<SharedVideoPlayer>>>;

    #[derive(Debug)]
    pub(crate) struct TimeUpdateClosure {
        ctx: Ctx,
        callback: Callback,
    }

    impl TimeUpdateClosure {
        pub(crate) fn new(ctx: Ctx, callback: Callback) -> Self {
            Self { ctx, callback }
        }
    }

    impl CallbackClosureWrapper<web_sys::Event> for TimeUpdateClosure {
        fn closure(&mut self, _: web_sys::Event) {
            let mut callback = self.callback.borrow_mut();
            let _ = callback.trigger(&mut self.ctx);
        }
    }
}

mod control_closure {
    use super::*;

    type Ctx = SharedVideoPlayer;
    type Callback = Rc<RefCell<dyn CallbackEvent<SharedVideoPlayer>>>;

    #[derive(Debug)]
    pub(crate) struct ControlClosure {
        ctx: Ctx,
        control_callbacks: HashMap<String, Callback>,
    }

    impl ControlClosure {
        pub(crate) fn new(ctx: Ctx, control_callbacks: HashMap<String, Callback>) -> Self {
            Self {
                ctx,
                control_callbacks,
            }
        }
    }

    impl CallbackClosureWrapper<web_sys::Event> for ControlClosure {
        fn closure(&mut self, event: web_sys::Event) {
            let target = event.current_target()
                .expect("Failed to get target for control callback");

            if let Some(element) = target.dyn_ref::<Element>() {
                let id = element.id();
                if let Some(callback_ref) = self.control_callbacks.get(&id) {
                    let mut callback = callback_ref.borrow_mut();
                    let _ = callback.trigger(&mut self.ctx);
                }
            }
        }
    }
}

mod keyboard_closure {
    use super::*;

    type Ctx = SharedVideoPlayer;
    type Callback = Rc<RefCell<dyn CallbackEvent<Ctx>>>;

    #[derive(Debug)]
    pub(crate) struct KeyboardClosure {
        ctx: Ctx,
        keyboard_callbacks: HashMap<String, Callback>,
    }


    impl KeyboardClosure {
        pub(crate) fn new(ctx: Ctx, keyboard_callbacks: HashMap<String, Callback>) -> Self {
            Self {
                ctx,
                keyboard_callbacks,
            }
        }
    }

    impl CallbackClosureWrapper<web_sys::KeyboardEvent> for KeyboardClosure {
        fn closure(&mut self, event: web_sys::KeyboardEvent) {
            let key = event.key();
            #[cfg(not(debug_assertions))]
            {
                if !d.contains_key(&key) {
                    event.prevent_default();
                }
            }
            if let Some(callback_ref) = self.keyboard_callbacks.get(&key) {
                let mut callback = callback_ref.borrow_mut();
                let _ = callback.trigger(&mut self.ctx);
            }
        }
    }
}
