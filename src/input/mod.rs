pub use winit::event::DeviceEvent;
pub use winit::event::VirtualKeyCode;
pub use winit::event::ElementState;

use winit::event::KeyboardInput;

#[derive(Default)]
pub struct Input {
    pub event: Option<Event>,
}

impl Input {
    pub fn key_event(&self) -> Option<&KeyboardInput> {
        if let Some(ref e) = self.event {
            match e.base() {
                BaseEvent::Device(dev_ev) => {
                    if let DeviceEvent::Key(keyboard_input) = dev_ev {
                        return Some(keyboard_input);
                    }
                },
                //BaseEvent::Window(win_ev) => {
                //},
            }
        }

        None
    }

    pub fn is_key(&self, key_code: VirtualKeyCode, state: ElementState) -> bool {
        if let Some(keyboard_input) = self.key_event() {
            return keyboard_input.virtual_keycode.is_some_and(|k| k == key_code)
                && keyboard_input.state == state
        }

        false
    }

    pub fn is_key_pressed(&self, key_code: VirtualKeyCode) -> bool {
        self.is_key(key_code, ElementState::Pressed)
    }

    pub fn is_key_released(&self, key_code: VirtualKeyCode) -> bool {
        self.is_key(key_code, ElementState::Released)
    }

    pub fn get_key_if(&self, state: ElementState) -> Option<VirtualKeyCode> {
        if let Some(keyboard_input) = self.key_event() {
            if keyboard_input.state == state {
                return keyboard_input.virtual_keycode
            }
        }

        None
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

pub enum BaseEvent {
    Device(winit::event::DeviceEvent),
    //Window(winit::event::WindowEvent<'a>)
}
