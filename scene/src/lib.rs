mod state_init;
use crate::state_init::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

impl State {
    pub async fn new(window: &Window) -> Self {
        let instance = &State::Instance();
        let surface = State::Surface(instance, window);
        let (device, queue) = State::DeviceQueue(instance, &surface);
        Self {
            surface,
            device,
            queue,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
