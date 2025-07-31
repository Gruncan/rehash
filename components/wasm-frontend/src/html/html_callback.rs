use crate::callback_event;
use crate::html::html_callback::control_closure::ControlClosure;
use crate::html::html_callback::keyboard_closure::KeyboardClosure;
use crate::html::html_callback::progress_closure::create_progress_closures;
use crate::html::html_callback::time_update_closure::TimeUpdateClosure;
use crate::html::html_callback::volume_closure::create_volume_closures;
use crate::html::html_events::drag_events::{BarDragEvent, BarDragEventCtx, MouseDown, MouseMove, MouseUp, ProgressBarClickEvent, VolumeBarClickEvent};
use crate::html::html_events::fast_forward_event::FastForwardEvent;
use crate::html::html_events::fullscreen_event::FullScreenEvent;
use crate::html::html_events::mute_unmute_event::MuteUnmuteEvent;
use crate::html::html_events::play_pause_event::PlayPauseEvent;
use crate::html::html_events::progress_bar_change_event::ProgressBarChangeEvent;
use crate::html::html_events::rewind_event::RewindEvent;
use crate::html::html_events::settings_event::SettingsEvent;
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

pub(crate) struct HtmlVideoCallbackController {
    video_player: SharedVideoPlayer,
    ui_controller: HtmlVideoUIController,
    callback_keyboard_events: HashMap<String, crate::html::html_video::Event>,
    callback_control_events: HashMap<String, crate::html::html_video::Event>,
    callback_progress_event: crate::html::html_video::Event,
    callback_volume_drag_event: crate::html::html_video::EventT<BarDragEventCtx<VolumeBarClickEvent>>,
    callback_progress_drag_event: crate::html::html_video::EventT<BarDragEventCtx<ProgressBarClickEvent>>,
}


impl HtmlVideoCallbackController {
    const PLAY_PAUSE_ID: &'static str = "play-pause";
    const MUTE_UNMUTE_ID: &'static str = "volume-btn";
    const SETTINGS_ID: &'static str = "settings";
    const FULLSCREEN_ID: &'static str = "fullscreen";
    const PROGRESS_BAR_ID: &'static str = "progress-container";
    const FAST_FORWARD_ID: &'static str = "fast-forward";
    const REWIND_ID: &'static str = "rewind";
    const VOLUME_ID: &'static str = "volume-slider";

    pub fn new(video_player: SharedVideoPlayer, ui_controller: HtmlVideoUIController) -> Self {
        let play_pause_event: crate::html::html_video::Event = callback_event!(PlayPauseEvent<HtmlVideoPlayerInternal>);
        let mute_unmute_event: crate::html::html_video::Event = callback_event!(MuteUnmuteEvent);
        let progress_event: crate::html::html_video::Event = callback_event!(ProgressBarChangeEvent);
        let settings_event: crate::html::html_video::Event = callback_event!(SettingsEvent);
        let fullscreen_event: crate::html::html_video::Event = callback_event!(FullScreenEvent);

        let volume_drag_event: Rc<RefCell<dyn CallbackEvent<BarDragEventCtx<VolumeBarClickEvent>>>> = Rc::new(RefCell::new(BarDragEvent::new()));
        let progress_drag_event: Rc<RefCell<dyn CallbackEvent<BarDragEventCtx<ProgressBarClickEvent>>>> = Rc::new(RefCell::new(BarDragEvent::new()));;


        let fast_forward_event: crate::html::html_video::Event = callback_event!(FastForwardEvent);
        let rewind_event: crate::html::html_video::Event = callback_event!(RewindEvent);

        let keyboard_events: HashMap<String, crate::html::html_video::Event> = HashMap::from([
            ("p".to_string(), play_pause_event.clone()),
            ("m".to_string(), mute_unmute_event.clone()),
            ("ArrowRight".to_string(), fast_forward_event.clone()),
            ("ArrowLeft".to_string(), rewind_event.clone()),
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
            callback_volume_drag_event: volume_drag_event,
            callback_progress_drag_event: progress_drag_event,
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
        self.ui_controller.register_global_event_listener_specific("timeupdate", timeupdate_closure);


        // TODO This needs to be a shared state...
        let volume_drag_event = self.callback_volume_drag_event.borrow().clone_box();
        let player = self.video_player.clone();
        let mutex = player.lock().unwrap();
        let instance = Rc::new(RefCell::new(mutex.clone_box()));


        let mouse_up_volume_closure = create_volume_closures::<MouseUp>(instance.clone(), volume_drag_event.clone());
        self.ui_controller.register_element_event_listener_specific("mouseup", Self::VOLUME_ID, mouse_up_volume_closure);

        let mouse_down_volume_closure = create_volume_closures::<MouseDown>(instance.clone(), volume_drag_event.clone());
        self.ui_controller.register_element_event_listener_specific("mousedown", Self::VOLUME_ID, mouse_down_volume_closure);

        let mouse_move_volume_closure = create_volume_closures::<MouseMove>(instance.clone(), volume_drag_event.clone());
        self.ui_controller.register_element_event_listener_specific("mousemove", Self::VOLUME_ID, mouse_move_volume_closure);


        let progress_drag_event = self.callback_progress_drag_event.borrow().clone_box();
        let mouse_up_progress_closure = create_progress_closures::<MouseUp>(instance.clone(), progress_drag_event.clone());
        self.ui_controller.register_element_event_listener_specific("mouseup", Self::PROGRESS_BAR_ID, mouse_up_progress_closure);

        let mouse_down_progress_closure = create_progress_closures::<MouseDown>(instance.clone(), progress_drag_event.clone());
        self.ui_controller.register_element_event_listener_specific("mousedown", Self::PROGRESS_BAR_ID, mouse_down_progress_closure);

        let mouse_move_volume_closure = create_progress_closures::<MouseMove>(instance.clone(), progress_drag_event.clone());
        self.ui_controller.register_element_event_listener_specific("mousemove", Self::VOLUME_ID, mouse_move_volume_closure);
    }
}


mod volume_closure {
    use super::*;
    use crate::html::html_events::drag_events::DragAction;
    use crate::log_to_tauri;
    use crate::video::video_callback::{CallbackClosureWrapper, VideoPlayerState};
    use std::fmt::{Debug, Formatter};
    use web_sys::HtmlElement;

    type Ctx = BarDragEventCtx<VolumeBarClickEvent>;
    type Callback = Box<dyn CallbackEvent<BarDragEventCtx<VolumeBarClickEvent>>>;
    type Closure<S> = Box<wasm_bindgen::closure::Closure<dyn FnMut(S)>>;

    pub(crate) struct VolumeBarDragClosure {
        ctx: Ctx,
        callback: Callback,
    }

    impl VolumeBarDragClosure {
        pub(crate) fn new(ctx: Ctx, callback: Callback) -> Self {
            Self { ctx, callback }
        }
    }

    impl Debug for VolumeBarDragClosure {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "VolumeBarDragClosure{:?}", self.ctx)
        }
    }

    impl CallbackClosureWrapper<web_sys::MouseEvent> for VolumeBarDragClosure {
        fn closure(&mut self, event: web_sys::MouseEvent) {
            let target = event.target().unwrap();
            if let Some(element) = target.dyn_ref::<HtmlElement>() {
                let rec = element.get_bounding_client_rect();
                let click_x = event.client_x() as f64;
                let percent = ((click_x - rec.left()) / rec.width()).max(0f64);


                self.ctx.percent = percent;
                // todo fix second player clone
                match self.callback.trigger(&mut self.ctx) {
                    Ok(_) => {}
                    Err(e) => {
                        debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                    }
                }
            }
        }
    }

    #[inline]
    pub fn create_volume_closures<T>(video_player: Rc<RefCell<Box<dyn VideoPlayerState>>>, callback: Callback) -> Closure<web_sys::MouseEvent>
    where
        T: DragAction + 'static,
    {
        let ctx = BarDragEventCtx::new::<T>(video_player);
        let ref_closure_wrapper = Box::new(VolumeBarDragClosure::new(ctx, callback));
        CallbackClosureWrapper::create_callback(ref_closure_wrapper)
    }
}

mod progress_closure {
    use super::*;
    use crate::html::html_events::drag_events::DragAction;
    use crate::log_to_tauri;
    use crate::video::video_callback::{CallbackClosureWrapper, VideoPlayerState};
    use web_sys::HtmlElement;

    type Ctx = BarDragEventCtx<ProgressBarClickEvent>;
    type Callback = Box<dyn CallbackEvent<BarDragEventCtx<ProgressBarClickEvent>>>;
    type Closure<S> = Box<wasm_bindgen::closure::Closure<dyn FnMut(S)>>;

    #[derive(Debug)]
    pub(crate) struct ProgressBarDragClosure {
        ctx: Ctx,
        callback: Callback,
    }

    impl ProgressBarDragClosure {
        pub(crate) fn new(ctx: Ctx, callback: Callback) -> Self {
            Self { ctx, callback }
        }
    }

    impl CallbackClosureWrapper<web_sys::MouseEvent> for ProgressBarDragClosure {
        fn closure(&mut self, event: web_sys::MouseEvent) {
            let target = event.target().unwrap();
            if let Some(element) = target.dyn_ref::<HtmlElement>() {
                let rec = element.get_bounding_client_rect();
                let click_x = event.client_x() as f64;
                let percent = ((click_x - rec.left()) / rec.width()).max(0f64);

                self.ctx.percent = percent;

                // todo fix second player clone
                match self.callback.trigger(&mut self.ctx) {
                    Ok(_) => {}
                    Err(e) => {
                        debug_console_log!("Tried to go into an unusable state: {}", e.as_string().unwrap());
                    }
                }
            }
        }
    }

    #[inline]
    pub fn create_progress_closures<T>(video_player: Rc<RefCell<Box<dyn VideoPlayerState>>>, callback: Callback) -> Closure<web_sys::MouseEvent>
    where
        T: DragAction + 'static,
    {
        let ctx = BarDragEventCtx::new::<T>(video_player);
        let ref_closure_wrapper = Box::new(ProgressBarDragClosure::new(ctx, callback));
        CallbackClosureWrapper::create_callback(ref_closure_wrapper)
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
    type Callback = Rc<RefCell<dyn CallbackEvent<SharedVideoPlayer>>>;

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