use std::any::type_name;
use log::{ trace, debug, info, warn, error };


use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::events::EventListener;
use web_sys::{
    console,
    EventTarget, Event, MouseEvent, WheelEvent, KeyboardEvent,
};




#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseButton {
    Left    = 0,
    Right   = 1,
    Middle  = 2,
    Back    = 3,
    Forward = 4,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Inactive,
    Pressed,
    Held,
    Released,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateBuffer {
    real: State,
    virt: State,
}
impl StateBuffer {
    fn update(&mut self) {
        match self.real {
            State::Pressed => match self.virt {
                State::Inactive => self.virt = State::Pressed,
                State::Released => self.virt = State::Pressed,

                State::Pressed => self.virt = State::Held,
                State::Held => self.virt = State::Held,
            },
            State::Released => match self.virt {
                State::Inactive => self.virt = State::Inactive,
                State::Released => self.virt = State::Inactive,

                State::Pressed => self.virt = State::Released,
                State::Held => self.virt = State::Released,
            }
            _ => (),
        }
    }
}





#[derive(Default, Clone)]
pub struct HandlerBuffer {
    keys: HashMap<String, StateBuffer>,
    buttons: HashMap<i16, StateBuffer>,

    pointer_origin: (f64, f64),
    pointer_delta: (f64, f64),

    wheel_origin: (f64, f64, f64),
    wheel_delta: (f64, f64, f64),
}

#[derive(Default)]
pub struct Handler {
    target: Option<EventTarget>,
    buffer: Arc<Mutex<HandlerBuffer>>,
    

    key_down: Option<EventListener>,
    key_up: Option<EventListener>,
    mouse_down: Option<EventListener>,
    mouse_up: Option<EventListener>,
    mouse_move: Option<EventListener>,
    wheel: Option<EventListener>,

    temp: EventHandler,
    /*
    key_down_up: EventHandler,
    mouse_down_up: EventHandler,
    mouse_move: EventHandler,
    wheel: EventHandler,
    */
}
// Have a mainthread callback?? <- by setting a flag in eventhandler and then calling callbacks in update.
// or other thread callback?? <- by calling callbacks in eventhandler.

// The input design is bad. Allow for hookind multiple targets
// and only set target on hook.

// USE HASHMAP FOR TARGETs
// when a target is no longer highlighed "up" events wont be fired
// So key etc, stay down. TODO: FIX.
impl Handler {
    fn hook_callback<T: AsRef<Event> + JsCast>(
        &mut self, 
        target: &EventTarget, 
        event_type: &str, 
        callback: &'static dyn Fn(&Arc<Mutex<HandlerBuffer>>, &T), 
        forget: bool
    ) -> Option<EventListener>
    {
        let pointer = self.buffer.clone();
        let event_type = event_type.to_owned();
        let listener = EventListener::new(target, event_type.clone(), move |event| {
            match event.clone().dyn_into::<T>() {
                Ok(event) => callback(&pointer, &event),
                Err(error) => warn!(
                    "Handler::hook_callback : Failed to cast '{}' event type into '{}'. Wasm error:\n{:?}", 
                    event_type, 
                    type_name::<T>(),
                    error
                ),
            };
        });
        match forget {
            true => {
                listener.forget();
                None
            }
            false => Some(listener),
        }
    }
    pub fn hook(&mut self, target: &EventTarget) {
        match &self.target {
            Some(target) => warn!("Handler::new : Can't hook with active taret: {:?}", target),
            None => {
                self.key_down = self.hook_callback::<KeyboardEvent>(target, "keydown", &Handler::key_down_callback, false);
                self.key_up = self.hook_callback::<KeyboardEvent>(target, "keyup", &Handler::key_up_callback, false);
                self.mouse_down = self.hook_callback::<MouseEvent>(target, "mousedown", &Handler::mouse_down_callback, false);
                self.mouse_up = self.hook_callback::<MouseEvent>(target, "mouseup", &Handler::mouse_up_callback, false);
                /*
                let pointer = self.buffer.clone();
                let listener = EventListener::new(target, "mousemove", move |event| {
                    //console::log_1(&JsValue::from("moved"));
                    Handler::mouse_move_callback(&pointer, &event);
                });
                listener.forget();*/
                //self.temp = EventHandler::new(target);
                //self.temp.hook("mousemove", false);

                self.mouse_move = self.hook_callback::<MouseEvent>(target, "mousemove", &Handler::mouse_move_callback, false);
                self.wheel = self.hook_callback::<WheelEvent>(target, "wheel", &Handler::wheel_callback, false);
            }
        }
    }
    pub fn unhook(&mut self) {
        self.target = None;
        
        self.key_down = None;
        self.key_up = None;
        self.mouse_down = None;
        self.mouse_up = None;
        self.mouse_move = None;
        self.wheel = None;
    }

    pub fn update(&mut self) {
        let mut buffer = self.buffer.lock().unwrap();
        for (_, state) in &mut buffer.keys.iter_mut() {
            state.update();
        }
        for (_, state) in &mut buffer.buttons.iter_mut() {
            state.update();
        }
    }




    fn key_down_callback(pointer: &Arc<Mutex<HandlerBuffer>>, event: &KeyboardEvent) {
        let mut buffer = pointer.lock().unwrap();
        buffer.keys.entry(event.code())
            .and_modify(|state_buffer| state_buffer.real = State::Pressed)
            .or_insert(StateBuffer {
                real: State::Pressed,
                virt: State::Inactive,
            });
    }
    fn key_up_callback(pointer: &Arc<Mutex<HandlerBuffer>>, event: &KeyboardEvent) {
        let mut buffer = pointer.lock().unwrap();
        buffer.keys.entry(event.code())
            .and_modify(|state_buffer| state_buffer.real = State::Released)
            .or_insert(StateBuffer {
                real: State::Released,
                virt: State::Inactive,
            });
    }
    fn mouse_down_callback(pointer: &Arc<Mutex<HandlerBuffer>>, event: &MouseEvent) {
        let mut buffer = pointer.lock().unwrap();
        buffer.buttons.entry(event.button())
            .and_modify(|state_buffer| state_buffer.real = State::Pressed)
            .or_insert(StateBuffer {
                real: State::Pressed,
                virt: State::Inactive,
            });
    }
    fn mouse_up_callback(pointer: &Arc<Mutex<HandlerBuffer>>, event: &MouseEvent) {
        let mut buffer = pointer.lock().unwrap();
        buffer.buttons.entry(event.button())
            .and_modify(|state_buffer| state_buffer.real = State::Released)
            .or_insert(StateBuffer {
                real: State::Released,
                virt: State::Inactive,
            });
    }
    fn mouse_move_callback(pointer: &Arc<Mutex<HandlerBuffer>>, event: &MouseEvent) {
        let mut buffer = pointer.lock().unwrap();
        buffer.pointer_origin = (event.client_x() as f64, event.client_y() as f64);
        buffer.pointer_delta = (event.movement_x() as f64, event.movement_y() as f64);
    }
    fn wheel_callback(pointer: &Arc<Mutex<HandlerBuffer>>, event: &WheelEvent) {
        let mut buffer = pointer.lock().unwrap();
        buffer.wheel_delta = (event.delta_x(), event.delta_y(), event.delta_z());
        buffer.wheel_origin = (
            buffer.wheel_origin.0 + buffer.wheel_delta.0,
            buffer.wheel_origin.1 + buffer.wheel_delta.1,
            buffer.wheel_origin.2 + buffer.wheel_delta.2
        );
    }


/*
    fn key_update(&mut self) {
        let buffer = &mut self.buffer;
        let lock = self.key_down_up.get::<KeyboardEvent>();
        match lock.0 {
            Some(event) => {
                let real_state = match event.type_() == "keydown" {
                    true => State::Pressed,
                    false => State::Released,
                };
                buffer.keys.entry(event.code())
                    .and_modify(|state| {
                        state.real = real_state;
                        state.update();
                    })
                    .or_insert({
                        let mut state = StateBuffer {
                            real: real_state,
                            virt: State::Inactive,
                        };
                        state.update();
                        state
                    });
                self.key_down_up.handle();
            },
            None => self.key_down_up.handle(),
        };
    }
    fn button_update(&mut self) {
        let buffer = &mut self.buffer;
        let lock = self.mouse_down_up.get::<MouseEvent>();
        match lock.0 {
            Some(event) => {
                let real_state = match event.type_() == "mousedown" {
                    true => State::Pressed,
                    false => State::Released,
                };
                buffer.buttons.entry(event.button())
                    .and_modify(|state| {
                        state.real = real_state;
                        state.update();
                    })
                    .or_insert({
                        let mut state = StateBuffer {
                            real: real_state,
                            virt: State::Inactive,
                        };
                        state.update();
                        state
                    });
                self.mouse_down_up.handle();
            },
            None => self.mouse_down_up.handle(),
        };
    }
    fn mouse_update(&mut self) {
        let buffer = &mut self.buffer;
        let lock = self.mouse_move.get::<MouseEvent>();
        match !lock.1 {
            true => match lock.0 {
                Some(event) => {
                    buffer.pointer_delta = (event.movement_x() as f64, event.movement_y() as f64);
                    buffer.pointer_origin = (event.client_x() as f64, event.client_y() as f64);
                    self.mouse_move.handle();
                },
                None => self.mouse_move.handle(),
            },
            false => buffer.pointer_delta = (0.0, 0.0),
        };
    }
    fn wheel_update(&mut self) {
        let buffer = &mut self.buffer;
        let lock = self.wheel.get::<WheelEvent>();
        match !lock.1 {
            true => match lock.0 {
                Some(event) => {
                    buffer.wheel_delta = (event.delta_x(), event.delta_y(), event.delta_z());
                    buffer.wheel_origin = (
                        buffer.wheel_origin.0 + buffer.wheel_delta.0,
                        buffer.wheel_origin.1 + buffer.wheel_delta.1,
                        buffer.wheel_origin.2 + buffer.wheel_delta.2
                    );
                    self.wheel.handle();
                },
                None => self.wheel.handle(),
            },
            false => buffer.wheel_delta = (0.0, 0.0, 0.0),
        }
    }
    */
}
impl Handler {
    pub fn pointer(&self) -> (f64, f64) {
        self.buffer.lock().unwrap().pointer_origin
    }
    pub fn pointer_relative<T: JsCast + Clone>(&self, element: &T) -> (f64, f64) {
        match element.clone().dyn_into::<web_sys::HtmlElement>() {
            Ok(element) => {
                let (x, y) = self.buffer.lock().unwrap().pointer_origin;
                let rect = element.get_bounding_client_rect();
                (x - rect.left(), y - rect.top())
            }
            Err(_) => (0.0, 0.0),
        }
    }
    pub fn pointer_delta(&self) -> (f64, f64) {
        self.buffer.lock().unwrap().pointer_delta
    }
    pub fn wheel(&self) -> (f64, f64, f64) {
        self.buffer.lock().unwrap().wheel_origin
    }
    pub fn wheel_delta(&self) -> (f64, f64, f64) {
        self.buffer.lock().unwrap().wheel_delta
    }




    pub fn key(&self, code: &str) -> State {
        match self.buffer.lock().unwrap().keys.get(code) {
            Some(state) => state.real,
            None => State::Released,
        }
    }
    pub fn vkey(&self,  code: &str) -> State {
        match self.buffer.lock().unwrap().keys.get(code) {
            Some(state) => state.virt,
            None => State::Inactive,
        }
    }
    pub fn vkey_active(&self, code: &str) -> bool {
        match self.vkey(code) {
            State::Held => true,
            State::Pressed => true,
            State::Released => true,
            _ => false,
        }
    }
    pub fn vkey_down(&self, code: &str) -> bool {
        match self.vkey(code) {
            State::Held => true,
            State::Pressed => true,
            _ => false,
        }
    }
    pub fn vkey_inactive(&self, code: &str) -> bool {
        self.vkey(code) == State::Inactive
    }
    pub fn vkey_pressed(&self, code: &str) -> bool {
        self.vkey(code) == State::Pressed
    }
    pub fn vkey_held(&self, code: &str) -> bool {
        self.vkey(code) == State::Held
    }
    pub fn vkey_released(&self, code: &str) -> bool {
        self.vkey(code) == State::Released
    }


    pub fn button(&self, button: MouseButton) -> State {
        match self.buffer.lock().unwrap().buttons.get(&(button as i16)) {
            Some(state) => state.real,
            None => State::Released,
        }
    }
    pub fn vbutton(&self,  button: MouseButton) -> State {
        match self.buffer.lock().unwrap().buttons.get(&(button as i16)) {
            Some(state) => state.virt,
            None => State::Inactive,
        }
    }
    pub fn vbutton_active(&self, button: MouseButton) -> bool {
        match self.vbutton(button) {
            State::Held => true,
            State::Pressed => true,
            State::Released => true,
            _ => false,
        }
    }
    pub fn vbutton_down(&self, button: MouseButton) -> bool {
        match self.vbutton(button) {
            State::Held => true,
            State::Pressed => true,
            _ => false,
        }
    }
    pub fn vbutton_inactive(&self, button: MouseButton) -> bool {
        self.vbutton(button) == State::Inactive
    }
    pub fn vbutton_pressed(&self, button: MouseButton) -> bool {
        self.vbutton(button) == State::Pressed
    }
    pub fn vbutton_held(&self, button: MouseButton) -> bool {
        self.vbutton(button) == State::Held
    }
    pub fn vbutton_released(&self, button: MouseButton) -> bool {
        self.vbutton(button) == State::Released
    }
}




#[derive(Default)]
struct EventHandler {
    target: Option<EventTarget>,
    inner: Arc<Mutex<(Option<Event>, bool)>>,
    listeners: Vec<Option<EventListener>>,
}
impl EventHandler {
    fn new(target: &EventTarget) -> Self {
        Self {
            target: Some(target.clone()),
            inner: Arc::new(Mutex::new((None, false))),
            listeners: vec!(),
        }
    }

    fn hook(&mut self, event_type: &str, forget: bool) -> Option<usize> {
        let target = match &self.target {
            Some(target) => target,
            None => return None,
        };

        let clone = self.inner.clone();
        let listener = EventListener::new(&target, event_type.to_owned(), move |event| {
            info!("movededed");
            *clone.lock().unwrap() = (Some(event.clone()), false);
        });

        match !forget {
            true => {
                self.listeners.push(Some(listener));
                Some(self.listeners.len() - 1)
            },
            false => {
                listener.forget();
                None
            }
        }
    }
    fn unhook(&mut self, index: usize) {
        match self.listeners.get_mut(index) {
            Some(listener) => *listener = None,
            None => (),
        };
    }
    fn unhook_all(&mut self) {
        self.listeners.clear();
    }

    fn handle(&mut self) {
        self.inner.lock().unwrap().1 = true;
    }
    fn get<EventType: AsRef<Event> + JsCast>(&self) -> (Option<EventType>, bool) {
        let inner = self.inner.lock().unwrap();
        match &self.target {
            Some(_) => match &inner.0 {
                Some(event) => (event.clone().dyn_into::<EventType>().ok(), inner.1),
                None => (None, true),
            }
            None => (None, true),
        }
    }    
    fn get_target(&self) -> Option<EventTarget> {
        self.target.clone()
    } 
}



/*
#[derive(Default)]
pub struct EventHandler {
    targets: HashMap<String, Vec<(EventTarget, EventListener, (Option<Event>, bool))>>,
}
impl EventHandler {
    fn hook(&mut self, event_type: &str, target: &EventTarget) -> Option<usize> {
        self.targets.entry(event_type.to_owned()).or_insert(vec!());

        let mut index;
        self.targets.entry(event_type.to_owned())
            .and_modify(|targets| {
                index = targets.len();
                let listener = EventListener::new(&target, event_type.to_owned(), move |event| {
                    *clone.lock().unwrap() = (Some(event.clone()), false);
                });

                targets.push((target.clone(), listener, (None, false)));
            })

        let listener = EventListener::new(&target, event_type.to_owned(), move |event| {
            *clone.lock().unwrap() = (Some(event.clone()), false);
        });

        match !forget {
            true => {
                self.listeners.push(Some(listener));
                Some(self.listeners.len() - 1)
            },
            false => {
                listener.forget();
                None
            }
        }
    }
    fn hook_forget() {

    }

    fn unhook(&mut self, index: usize) {
        match self.listeners.get_mut(index) {
            Some(listener) => *listener = None,
            None => (),
        };
    }
    fn unhook_all(&mut self) {
        self.listeners.clear();
    }

    fn handle(&mut self) {
        self.inner.lock().unwrap().1 = true;
    }
    fn get<EventType: AsRef<Event> + JsCast>(&self) -> (Option<EventType>, bool) {
        let inner = self.inner.lock().unwrap();
        match &self.target {
            Some(_) => match &inner.0 {
                Some(event) => (event.clone().dyn_into::<EventType>().ok(), inner.1),
                None => (None, true),
            }
            None => (None, true),
        }
    }    
    fn get_target(&self) -> Option<EventTarget> {
        self.target.clone()
    } 
}
*/