use std::iter;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use core::future::Future;
use futures::executor::block_on;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    /*pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer, */
}

impl State {
    pub fn Instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::BackendBit::PRIMARY)
    }
    pub fn Surface(instance: &wgpu::Instance, window: &Window) -> wgpu::Surface {
        unsafe { instance.create_surface(window) }
    }
    pub fn DeviceQueue(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface,
    ) -> (wgpu::Device, wgpu::Queue) {
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(surface),
        }))
        .unwrap();
        let (device, queue) = block_on(adapter.request_device(
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
    pub fn Size(window: &Window) -> winit::dpi::PhysicalSize<u32> {
        window.inner_size()
    }
    pub fn Sc_Desc(size: &winit::dpi::PhysicalSize<u32>) -> wgpu::SwapChainDescriptor {
        wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
    pub fn Swap_Chain(
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        sc_desc: &wgpu::SwapChainDescriptor,
    ) -> wgpu::SwapChain {
        device.create_swap_chain(surface, sc_desc)
    }
    pub fn Render_Pipeline(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
    ) -> wgpu::RenderPipeline {
        let vs_module =
            device.create_shader_module(wgpu::include_spirv!("./shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(wgpu::include_spirv!("./shaders/shader.frag.spv"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Reder Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
    }
    pub fn Vertex_Buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsage::VERTEX,
        })
    }
    pub fn Index_Buffer(device: &wgpu::Device, indices: &[u16]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        })
    }
    pub fn Num_Indices(indices: &[u16]) -> u32 {
        indices.len() as u32
    }
}
