use crate::{
    geometry::uniform::UniformBuilder,
    geometry::vertex::VertexBufferLayoutBuilder,
};
use helium_core::size::Size;
use wgpu::{
    ShaderModuleDescriptor, ShaderSource,
};

use super::RenderPipelineBuilder;

/// Holds the buffers and pipeline for rendering text to the screen
#[derive(Debug)]
pub struct IconPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub window_bind_group: wgpu::BindGroup,
    pub window_buffer: wgpu::Buffer,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl IconPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, size: &Size) -> Self {
		 // Compile the shader
		let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Icon Shader Module"),
            source: ShaderSource::Wgsl(include_str!("../../../shaders/icon.wgsl").into()),
        });

        let window_uniform = UniformBuilder::new()
            .label("Window")
            .contents(&[size.width, size.height])
            .build(device);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Icon texture bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let vertex_buffer_layout = VertexBufferLayoutBuilder::new()
            .add_attribute(0, wgpu::VertexFormat::Float32x2)
            .add_attribute(size_of::<[f32; 2]>(), wgpu::VertexFormat::Float32x4)
            .add_attribute(size_of::<[f32; 6]>(), wgpu::VertexFormat::Float32x2)
            .build();

        let pipeline = RenderPipelineBuilder::new("Icon", &shader)
            .add_bind_group_layout(&window_uniform.layout)
            .add_bind_group_layout(&texture_bind_group_layout)
            .add_buffer(vertex_buffer_layout)
            .build(device, config);
	

        Self {
            pipeline,
            window_bind_group:window_uniform.bind_group,
            window_buffer:window_uniform.buffer,
            texture_bind_group_layout,
        }
    }
}
