use crate::callback_event;
use crate::html::html_events::*;
use crate::html::html_ui::HtmlVideoUIController;
use crate::html::html_video::{Event, HtmlVideoPlayerInternal};
use crate::prelude::*;
use crate::video::event::{CallbackController, CallbackEvent};
use crate::video::video_callback::CallbackClosureWrapper;
use crate::video::video_callback::SharedVideoPlayer;
use crate::video::video_ui::VideoUIRegister;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, Element};

pub(crate) use control_closure::*;
pub(crate) use drag_closure::*;
pub(crate) use keyboard_closure::*;
pub(crate) use time_update_closure::*;

type KeyControlType = Rc<str>;


pub(crate) struct HtmlVideoCallbackController {
    video_player: SharedVideoPlayer,
    ui_controller: HtmlVideoUIController,
    callback_keyboard_events: HashMap<KeyControlType, Event>,
    callback_control_events: HashMap<KeyControlType, Event>,
    callback_progress_event: Event,
}


impl HtmlVideoCallbackController {
    const PLAY_PAUSE_ID: &'static str = "play-pause";
    const MUTE_UNMUTE_ID: &'static str = "volume-btn";
    const SETTINGS_ID: &'static str = "settings";
    const FULLSCREEN_ID: &'static str = "fullscreen";
    const FAST_FORWARD_ID: &'static str = "fast-forward";
    const REWIND_ID: &'static str = "rewind";

    pub fn new(video_player: SharedVideoPlayer, ui_controller: HtmlVideoUIController) -> Self {
        let play_pause_event: Event = callback_event!(PlayPauseEvent<HtmlVideoPlayerInternal>);
        let mute_unmute_event: Event = callback_event!(MuteUnmuteEvent);
        let progress_event: Event = callback_event!(ProgressBarChangeEvent<HtmlVideoPlayerInternal>);
        let settings_event: Event = callback_event!(SettingsEvent);
        let fullscreen_event: Event = callback_event!(FullScreenEvent);


        let fast_forward_event: Event = callback_event!(FastForwardEvent);
        let rewind_event: Event = callback_event!(RewindEvent);

        let playback_increase = callback_event!(PlaybackSpeedEvent<PlaybackIncreaseAction>);
        let playback_decrease = callback_event!(PlaybackSpeedEvent<PlaybackDecreaseAction>);

        let keyboard_events: HashMap<KeyControlType, Event> = HashMap::from([
            (Rc::from("p"), play_pause_event.clone()),
            (Rc::from("m"), mute_unmute_event.clone()),
            (Rc::from("ArrowRight"), fast_forward_event.clone()),
            (Rc::from("ArrowLeft"), rewind_event.clone()),
            (Rc::from("ArrowUp"), playback_increase.clone()),
            (Rc::from("ArrowDown"), playback_decrease.clone()),
        ]);

        let control_events: HashMap<KeyControlType, Event> = HashMap::from([
            (Rc::from(Self::PLAY_PAUSE_ID), play_pause_event.clone()),
            (Rc::from(Self::MUTE_UNMUTE_ID), mute_unmute_event.clone()),
            (Rc::from(Self::SETTINGS_ID), settings_event.clone()),
            (Rc::from(Self::FULLSCREEN_ID), fullscreen_event.clone()),
            (Rc::from(Self::FAST_FORWARD_ID), fast_forward_event.clone()),
            (Rc::from(Self::REWIND_ID), rewind_event.clone()),
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

        let keys: Vec<&str> = self.callback_control_events.iter().map(|(k, _)| k.as_ref()).collect();
        self.ui_controller.register_element_event_listener(&keys, control_closure);


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

        let mouse_click_volume_wrapper = Box::new(DragClickClosure::new(&volume_bar_element, drag_ctx.clone()));
        let mouse_click_volume_closure = CallbackClosureWrapper::create_callback(mouse_click_volume_wrapper);

        self.ui_controller.register_element_event_listener_specific("mousedown", volume_bar_id, mouse_click_volume_closure);


        let progress_bar_element = doc.get_element_by_id(progress_bar_id).unwrap(); //.dyn_into::<HtmlElement>().unwrap();

        let mouse_click_progress_wrapper = Box::new(DragClickClosure::new(&progress_bar_element, drag_ctx.clone()));
        let mouse_click_progress_closure = CallbackClosureWrapper::create_callback(mouse_click_progress_wrapper);

        self.ui_controller.register_element_event_listener_specific("mousedown", progress_bar_id, mouse_click_progress_closure);


        let start_dot_element = doc.get_element_by_id(start_dot_id).unwrap(); //.dyn_into::<HtmlElement>().unwrap();

        let progress_bar_dom_rec = progress_bar_element.get_bounding_client_rect();
        let mouse_click_start_dot_wrapper = Box::new(DragClickClosure::new_dom_rec(&start_dot_element, &progress_bar_dom_rec, drag_ctx.clone()));
        let mouse_click_start_dot_closure = CallbackClosureWrapper::create_callback(mouse_click_start_dot_wrapper);

        self.ui_controller.register_element_event_listener_specific("mousedown", start_dot_id, mouse_click_start_dot_closure);


        let end_dot_element = doc.get_element_by_id(end_dot_id).unwrap(); //.dyn_into::<HtmlElement>().unwrap();

        let mouse_click_end_dot_wrapper = Box::new(DragClickClosure::new_dom_rec(&end_dot_element, &progress_bar_dom_rec, drag_ctx.clone()));
        let mouse_click_end_dot_closure = CallbackClosureWrapper::create_callback(mouse_click_end_dot_wrapper);

        self.ui_controller.register_element_event_listener_specific("mousedown", end_dot_id, mouse_click_end_dot_closure);


        let volume_dom_rec = volume_bar_element.get_bounding_client_rect();
        let progress_dom_rec = progress_bar_element.get_bounding_client_rect();
        let element_dom_recs: Vec<(Element, &DomRect)> = vec![
            (volume_bar_element, &volume_dom_rec),
            (progress_bar_element, &progress_dom_rec),
            (start_dot_element, &progress_dom_rec),
            (end_dot_element, &progress_dom_rec),
        ];

        let drag_move_wrapper = Box::new(DragMoveClosure::new(element_dom_recs, drag_ctx.clone()));
        let drag_move_closure = CallbackClosureWrapper::create_callback(drag_move_wrapper);
        self.ui_controller.register_doc_global_event_listener_specific("mousemove", drag_move_closure);

        let drag_exit_wrapper = Box::new(DragExitClosure::new(drag_ctx));
        let drag_exit_closure = CallbackClosureWrapper::create_callback(drag_exit_wrapper);
        self.ui_controller.register_doc_global_event_listener_specific("mouseup", drag_exit_closure);

        debug_console_log!("Registered callback handlers");
    }
}


mod drag_closure {
    use super::*;
    use crate::log_to_tauri;
    use crate::video::video_callback::CallbackClosureWrapper;
    use std::fmt::Debug;
    use web_sys::DomRect;

    #[derive(Debug, Clone)]
    pub(crate) struct DragClickClosure {
        slider_width: f64,
        slider_left: f64,
        slider_type: MoveState,
        callback: DragClickEvent<HtmlVideoPlayerInternal>,
        ctx: DragEventCtxType,
    }


    impl CallbackClosureWrapper<web_sys::MouseEvent> for DragClickClosure {
        fn closure(&mut self, event: web_sys::MouseEvent) {
            let click_x = event.client_x() as f64;

            let percent = ((click_x - self.slider_left) / self.slider_width)
                .max(0f64).min(1f64);
            debug_console_log!("Click x: {} | left: {} | width {}", click_x,
                self.slider_left, self.slider_width);

            {
                let mut ctx = self.ctx.borrow_mut();
                ctx.set_percent(percent);
                ctx.set_clicked(self.slider_type);
            }
            match self.callback.trigger(&mut self.ctx) {
                Ok(_) => {}
                Err(e) => {
                    debug_console_log!("Tried to go into an unusable state: {}", e);
                }
            }
        }
    }

    impl DragClickClosure {
        pub fn new(element: &Element, ctx: DragEventCtxType) -> Self {
            let moving_state: MoveState = element.id().as_str().into();
            let rec = element.get_bounding_client_rect();
            let callback = DragClickEvent::new();

            Self {
                slider_left: rec.left(),
                slider_width: rec.width(),
                slider_type: moving_state,
                callback,
                ctx,
            }
        }

        pub fn new_dom_rec(element: &Element, rec: &DomRect, ctx: DragEventCtxType) -> Self {
            let moving_state: MoveState = element.id().as_str().into();
            let callback = DragClickEvent::new();
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
    pub(crate) struct DragSliderElement {
        slider_width: f64,
        slider_left: f64,
    }

    #[derive(Debug, Clone)]
    pub(crate) struct DragMoveClosure {
        map: HashMap<MoveState, DragSliderElement>,
        callback: DragMoveEvent,
        ctx: DragEventCtxType,
    }

    impl CallbackClosureWrapper<web_sys::MouseEvent> for DragMoveClosure {
        fn closure(&mut self, event: web_sys::MouseEvent) {
            match self.handle_closure(event) {
                _ => {
                    return;
                }
            }
        }
    }

    impl DragMoveClosure {
        pub fn new(elements: Vec<(Element, &DomRect)>, ctx: DragEventCtxType) -> Self {
            let map: HashMap<MoveState, DragSliderElement> = HashMap::from_iter(
                elements.into_iter().map(|(el, dom_rect)| {
                    let key: MoveState = el.id().as_str().into();
                    let value = DragSliderElement {
                        slider_width: dom_rect.width(),
                        slider_left: dom_rect.left(),
                    };
                    (key, value)
                })
            );
            let callback = DragMoveEvent {};

            Self {
                map,
                callback,
                ctx,
            }
        }

        fn handle_closure(&mut self, event: web_sys::MouseEvent) -> Option<()> {
            let slider_element = {
                let ctx = self.ctx.borrow();
                if !ctx.is_clicked() {
                    return None;
                }
                self.map.get(&ctx.get_clicked())?
            };
            let click_x = event.client_x() as f64;
            debug_console_log!("Click x: {} | left: {} | width {}", click_x,
                slider_element.slider_left, slider_element.slider_width);
            let percent = ((click_x - slider_element.slider_left) / slider_element.slider_width)
                .max(0f64).min(1f64);

            {
                let mut ctx = self.ctx.borrow_mut();
                ctx.set_percent(percent);
            }

            self.callback.trigger(&mut self.ctx).ok()?;

            Some(())
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
                    debug_console_log!("Tried to go into an unusable state: {}", e);
                }
            }
        }
    }

    impl DragExitClosure {
        pub fn new(ctx: DragEventCtxType) -> Self {
            Self {
                callback: DragExitEvent {},
                ctx,
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
        control_callbacks: HashMap<KeyControlType, Callback>,
    }

    impl ControlClosure {
        pub(crate) fn new(ctx: Ctx, control_callbacks: HashMap<KeyControlType, Callback>) -> Self {
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
                if let Some(callback_ref) = self.control_callbacks.get(id.as_str()) {
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
        keyboard_callbacks: HashMap<KeyControlType, Callback>,
    }


    impl KeyboardClosure {
        pub(crate) fn new(ctx: Ctx, keyboard_callbacks: HashMap<KeyControlType, Callback>) -> Self {
            Self {
                ctx,
                keyboard_callbacks,
            }
        }
    }

    impl CallbackClosureWrapper<web_sys::KeyboardEvent> for KeyboardClosure {
        fn closure(&mut self, event: web_sys::KeyboardEvent) {
            let key = event.key();
            if let Some(callback_ref) = self.keyboard_callbacks.get(key.as_str()) {
                let mut callback = callback_ref.borrow_mut();
                let _ = callback.trigger(&mut self.ctx);
            }
        }
    }
}
