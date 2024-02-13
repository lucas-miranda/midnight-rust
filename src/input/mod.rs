pub mod keyboard;

pub use winit::event::DeviceEvent;
pub use winit::event::ElementState;

use std::collections::HashSet;

use winit::{
    event::{ KeyEvent, WindowEvent },
    keyboard::PhysicalKey,
};

#[derive(Default)]
pub struct Input {
    event: Option<Event>,

    key_down: HashSet<PhysicalKey>,
}

impl Input {
    pub fn key_event(&self) -> Option<&KeyEvent> {
        if let Some(ref e) = self.event {
            match e.base() {
                BaseEvent::Device(_dev_ev) => {
                    unimplemented!();
                },
                BaseEvent::Window(win_ev) => {
                    if let WindowEvent::KeyboardInput { event: key_event, .. } = win_ev {
                        return Some(key_event);
                    }
                },
            }
        }

        None
    }

    pub fn is_key(&self, physical_key: PhysicalKey, state: ElementState) -> bool {
        if let Some(key_event) = self.key_event() {
            return key_event.physical_key == physical_key && key_event.state == state
        }

        false
    }

    pub fn is_key_pressed(&self, physical_key: PhysicalKey) -> bool {
        self.is_key(physical_key, ElementState::Pressed)
    }

    pub fn is_key_released(&self, physical_key: PhysicalKey) -> bool {
        self.is_key(physical_key, ElementState::Released)
    }

    pub fn is_key_down(&self, physical_key: PhysicalKey) -> bool {
        self.key_down.contains(&physical_key)
    }

    pub fn get_key_if(&self, state: ElementState) -> Option<PhysicalKey> {
        if let Some(key_event) = self.key_event() {
            if key_event.state == state {
                return Some(key_event.physical_key)
            }
        }

        None
    }

    pub(crate) fn handle(&mut self, e: Event) {
        self.event.replace(e);

        match &self.event {
            Some(event) => match event.base() {
                BaseEvent::Device(_dev_event) => {
                    unimplemented!();
                }
                BaseEvent::Window(win_event) => {
                    match win_event {
                        WindowEvent::KeyboardInput { event: key_event, .. } => {
                            match key_event.state {
                                ElementState::Pressed => {
                                    if !self.key_down.contains(&key_event.physical_key) {
                                        self.key_down.insert(key_event.physical_key);
                                    }
                                },
                                ElementState::Released => {
                                    self.key_down.remove(&key_event.physical_key);
                                },
                            }
                        }
                        _ => (),
                    }
                }
            },
            None => (),
        }
    }
}

pub struct Event {
    base: BaseEvent,
}

impl Event {
    fn base(&self) -> &BaseEvent {
        &self.base
    }
}

impl From<winit::event::DeviceEvent> for Event {
    fn from(value: winit::event::DeviceEvent) -> Self {
        Event {
            base: BaseEvent::Device(value),
        }
    }
}

impl From<winit::event::WindowEvent> for Event {
    fn from(value: winit::event::WindowEvent) -> Self {
        Event {
            base: BaseEvent::Window(value),
        }
    }
}

pub enum BaseEvent {
    Device(winit::event::DeviceEvent),
    Window(winit::event::WindowEvent),
}
