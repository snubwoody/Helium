use std::str;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt}, BindGroupDescriptor, PipelineLayoutDescriptor,
	ShaderModuleDescriptor, ShaderSource, ShaderStages, VertexAttribute, VertexBufferLayout, VertexState
};
use helium_core::size::Size;
use crate::{
	geometry::uniform::{Uniform, UniformBuilder}, 
	geometry::vertex::VertexBufferLayoutBuilder
};

use super::RenderPipelineBuilder;


/// Holds the render pipeline
pub struct CirclePipeline{
	pub pipeline: wgpu::RenderPipeline,
	pub bounds_layout: wgpu::BindGroupLayout,
	pub window_uniform:Uniform,
	pub bounds_bind_group:wgpu::BindGroup,
	pub position_buffer:wgpu::Buffer,
	pub diameter_buffer:wgpu::Buffer
}

impl CirclePipeline {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		// Compile the shader
		let shader = device.create_shader_module(
			wgpu::ShaderModuleDescriptor{
				label: Some("Circle Shader Module"),
				source: ShaderSource::Wgsl(include_str!("../../../shaders/circle.wgsl").into())
			}
		);

		let window_uniform = 
			UniformBuilder::new()
			.label("Window")
			.contents(&[size.width,size.height])
			.build(device);

		let diameter_buffer = device.create_buffer(
			&wgpu::BufferDescriptor { 
				label: Some("Diameter buffer"), 
				size: size_of::<[f32;2]>() as wgpu::BufferAddress, 
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
				mapped_at_creation: false
			}
		);

		let position_buffer = device.create_buffer(
			&wgpu::BufferDescriptor { 
				label: Some("Position buffer"), 
				size: size_of::<[f32;2]>() as wgpu::BufferAddress, 
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
				mapped_at_creation: false
			}
		);

		let bounds_layout = device.create_bind_group_layout(
			&wgpu::BindGroupLayoutDescriptor{
				label:Some("Circle bounds layout"),
				entries:&[
					wgpu::BindGroupLayoutEntry{
						binding:0,
						visibility:wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer { 
							ty: wgpu::BufferBindingType::Uniform, 
							has_dynamic_offset: false, 
							min_binding_size: None 
						},
						count:None
					},
					wgpu::BindGroupLayoutEntry{
						binding:1,
						visibility:wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer { 
							ty: wgpu::BufferBindingType::Uniform, 
							has_dynamic_offset: false, 
							min_binding_size: None 
						},
						count:None
					},
				],
			}
		);

		
		let bounds_bind_group = device.create_bind_group(
			&BindGroupDescriptor{
				label:Some("Cirlce bounds bind group"),
				layout:&bounds_layout,
				entries:&[
					wgpu::BindGroupEntry{
						binding:0,
						resource:diameter_buffer.as_entire_binding()
					},
					wgpu::BindGroupEntry{
						binding:1,
						resource:position_buffer.as_entire_binding()
					}
				]
			}
		);

		let buffer_layout = 
			VertexBufferLayoutBuilder::new()
			.add_attribute(0, wgpu::VertexFormat::Float32x2)
			.add_attribute(size_of::<[f32;2]>(), wgpu::VertexFormat::Float32x4)
			.add_attribute(size_of::<[f32;6]>(), wgpu::VertexFormat::Float32x2)
			.build();

		let pipeline = 
			RenderPipelineBuilder::new("Circle",&shader)
			.add_bind_group_layout(&window_uniform.layout)
			.add_bind_group_layout(&bounds_layout)
			.add_buffer(buffer_layout)
			.build(device, config);

		Self { 
			pipeline, 
			window_uniform,
			bounds_layout,
			bounds_bind_group,
			position_buffer,
			diameter_buffer
		}
	}
}