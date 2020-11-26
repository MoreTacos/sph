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
        let size = State::Size(window);
        let sc_desc = State::Sc_Desc(&size);
        let swap_chain = State::Swap_Chain(&device, &surface, &sc_desc);
        let render_pipeline = State::Render_Pipeline(&device, &sc_desc);
        Self {
            surface,
            device,
            queue,
            size,
            sc_desc,
            swap_chain,
            render_pipeline,
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
