mod state_init;
use std::iter;
use futures::executor::block_on; 
pub use crate::state_init::State;
use crate::state_init::Vertex;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 1, 2, 3];

impl State {
    pub async fn new(window: &Window) -> Self {
        let instance = &State::Instance();
        let surface = State::Surface(instance, window);
        let (device, queue) = State::DeviceQueue(instance, &surface).await;
        let size = State::Size(window);
        let sc_desc = State::Sc_Desc(&size);
        let swap_chain = State::Swap_Chain(&device, &surface, &sc_desc);
        let render_pipeline = State::Render_Pipeline(&device, &sc_desc);
        let vertex_buffer = State::Vertex_Buffer(&device, VERTICES);
        let index_buffer = State::Index_Buffer(&device, INDICES);
        let num_indices = State::Num_Indices(INDICES);
        Self {
            surface,
            device,
            queue,
            size,
            sc_desc,
            swap_chain,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }
    pub fn update(&mut self) {}
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self
            .swap_chain
            .get_current_frame()?
            .output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
