mod state;

pub use crate::state::State;
use sph::Sph;
use std::iter;
use utils::Vertex;
use winit::{event::*, window::Window};

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
    pub async fn new(window: &Window, model: Sph) -> Self {
        let instance = &State::instance();
        let surface = State::surface(instance, window);
        let (device, queue) = State::device_queue(instance, &surface).await;
        let size = State::size(window);
        let sc_desc = State::sc_desc(&size);
        let swap_chain = State::swap_chain(&device, &surface, &sc_desc);
        let render_pipeline = State::render_pipeline(&device, &sc_desc);
        let vertex_buffer = State::vertex_buffer(&device, VERTICES);
        let index_buffer = State::index_buffer(&device, INDICES);
        let num_indices = State::num_indices(INDICES);
        let instances = model.instances();
        let instance_buffer = State::instance_buffer(&device, &instances);
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
            instances,
            instance_buffer,
            model,
        }
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }
    pub fn update(&mut self) {}
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.model.timestep();
        self.instances = self.model.instances();
        let _scale_ratio = self.size.width as f32 / self.size.height as f32;
        /*self.instances = self.instances.iter_mut().map(|p| Instance {
            position: p.position,
            rotation: p.rotation,
            scale: Vector2::new(p.scale.x / scale_ratio, p.scale.y),
        }).collect::<Vec<_>>(); */
        self.instance_buffer = State::instance_buffer(&self.device, &self.instances);

        let frame = self.swap_chain.get_current_frame()?.output;
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
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
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
