use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};

use core::future::Future;
use futures::executor::block_on;

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
/*    sc_desc: wgpu::SwapChainDescriptor,
    render_pipeline: wgpu::RenderPipeline,
    size: winit::dpi::PhysicalSize<u32>,
    vertex_buffer: wgpu::Buffer,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer, */
}



impl State {
    pub fn Size(window: &Window) -> winit::dpi::PhysicalSize<u32> {
        window.inner_size()
    }
    pub fn Instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::BackendBit::PRIMARY)
    }
    pub fn Surface(instance: &wgpu::Instance, window: &Window) -> wgpu::Surface {
        unsafe { instance.create_surface(window) }
    }
    pub fn DeviceQueue(instance: &wgpu::Instance, surface: &wgpu::Surface) -> (wgpu::Device, wgpu::Queue) {
        let adapter = block_on(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(surface),
            })
        )
            .unwrap();
        let (device, queue) = block_on(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            ))
            .unwrap();
        (device, queue)
    }
}
