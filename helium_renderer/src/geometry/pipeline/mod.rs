pub mod image;
pub mod text;
pub mod rect;
pub mod circle;
pub mod icon;
use circle::CirclePipeline;
use icon::IconPipeline;
use image::ImagePipeline;
use rect::RectPipeline;
use text::TextPipeline;
use wgpu::{
	ColorTargetState, FragmentState, MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPipelineDescriptor, VertexBufferLayout, VertexState
};
use helium_core::size::Size;
use crate::geometry::{uniform::UniformBuilder, vertex::Vertex};
use super::uniform::Uniform;

// TODO could maybe move the buffers into herer
// TODO pls refactor this long and ugly code, there's a lot of reused code;
/// An abstraction over [`wgpu::RenderPipeline`](https://docs.rs/wgpu/22.1.0/wgpu/struct.RenderPipeline.html)
/// that containts **all** the resources required for the pipeline, namely the buffers, bind groups, 
/// and texture samplers.
#[derive(Debug)]
pub struct Pipeline{
	pub pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
	pub bounds_layout: wgpu::BindGroupLayout,
    pub window_buffer: wgpu::Buffer,
}

impl Pipeline{
	pub fn new(
		shader:wgpu::ShaderSource,
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		// Compile the shaders
		let shader = device.create_shader_module(
			wgpu::ShaderModuleDescriptor{
				label: Some("Rect Shader Module"),
				source:shader
			}
		);

		let window_uniform = 
			UniformBuilder::new()
			.label("Window")
			.contents(&[size.width,size.height])
			.build(device);

		let bounds_layout = device.create_bind_group_layout(
			&wgpu::BindGroupLayoutDescriptor{
				label:Some("Rect bounds layout"),
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
					wgpu::BindGroupLayoutEntry{
						binding:2,
						visibility:wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer { 
							ty: wgpu::BufferBindingType::Uniform, 
							has_dynamic_offset: false, 
							min_binding_size: None 
						},
						count:None
					}
				],
			}
		);

		
		// TODO replace with builder
		let buffer_layout = wgpu::VertexBufferLayout { 
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
			step_mode: wgpu::VertexStepMode::Vertex, 
			attributes: &[
				wgpu::VertexAttribute{
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x2
				},
				wgpu::VertexAttribute{
					offset: size_of::<[f32;2]>() as wgpu::BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x4
				},
				wgpu::VertexAttribute{
					offset: size_of::<[f32;6]>() as wgpu::BufferAddress,
					shader_location: 2,
					format: wgpu::VertexFormat::Float32x2 
				},
			]
		};

		let pipeline_layout = 
			device.create_pipeline_layout(
				&wgpu::PipelineLayoutDescriptor{
					label: Some("Rect Pipeline Layout"),
					bind_group_layouts: &[window_uniform.layout(),&bounds_layout],
					push_constant_ranges: &[]
				}
			);

		let pipeline = device.create_render_pipeline(
			&wgpu::RenderPipelineDescriptor { 
				label: Some("Rect Render Pipeline"), 
				layout: Some(&pipeline_layout), 
				vertex: wgpu::VertexState{
					module: &shader,
					entry_point: "vs_main",
					compilation_options: Default::default(),
					buffers: &[buffer_layout]
				}, 
				fragment: Some(wgpu::FragmentState{
					module: &shader,
					entry_point: "fs_main",
					compilation_options: Default::default(),
					targets: &[Some(wgpu::ColorTargetState {
						format: config.format,
						blend: Some(wgpu::BlendState::ALPHA_BLENDING),
						write_mask: wgpu::ColorWrites::ALL,
					})]
				}), 
				primitive: wgpu::PrimitiveState{
					topology: wgpu::PrimitiveTopology::TriangleList,
                	strip_index_format: None,
                	front_face: wgpu::FrontFace::Ccw,
                	cull_mode: None,
                	unclipped_depth: false,
                	polygon_mode: wgpu::PolygonMode::Fill,
                	conservative: false,
				}, 
				multisample: wgpu::MultisampleState {
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				}, 
				depth_stencil: None, 
				multiview: None, 
				cache: None 
			}
		);

		
		Self { 
			pipeline, 
			window_bind_group:window_uniform.bind_group, 
			bounds_layout,
			window_buffer:window_uniform.buffer
		}
	}
}


struct RenderPipelineBuilder<'a>{
	label:String,
	shader:&'a wgpu::ShaderModule,
	vertex_entry_point:String,
	fragment_entry_point:String,
	layout:Option<&'a wgpu::PipelineLayout>,
	bind_group_layouts:Vec<&'a wgpu::BindGroupLayout>,
	buffer_layouts:Vec<VertexBufferLayout<'a>>
}

impl<'a> RenderPipelineBuilder<'a> {
	fn new(label:&str,shader:&'a wgpu::ShaderModule) -> Self{
		let vertex_entry_point = String::from("vs_main");
		let fragment_entry_point = String::from("fs_main");
		
		Self{
			label:label.to_owned(),
			shader,
			vertex_entry_point,
			fragment_entry_point,
			buffer_layouts:vec![],
			bind_group_layouts:vec![],
			layout:None
		}
	}


	fn vertex_entry_point(mut self,entry_point:&str) -> Self {
		self.vertex_entry_point = entry_point.to_owned();
		self
	}

	fn fragment_entry_point(mut self,entry_point:&str) -> Self {
		self.fragment_entry_point = entry_point.to_owned();
		self
	}

	fn add_bind_group_layout(mut self,layout:&'a wgpu::BindGroupLayout) -> Self{
		self.bind_group_layouts.push(layout);
		self
	}

	fn add_buffer(mut self,buffer:wgpu::VertexBufferLayout<'a>) -> Self{
		self.buffer_layouts.push(buffer);
		self
	}

	fn build(self,device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline{
		let render_pipeline_layout = 
			device.create_pipeline_layout(
				&PipelineLayoutDescriptor{
					label: Some(format!("{} Pipeline Layout",self.label).as_str()),
					bind_group_layouts: self.bind_group_layouts.as_slice(),
					push_constant_ranges: &[]
				}
			);

		device.create_render_pipeline(
			&RenderPipelineDescriptor { 
				label: Some(format!("{} Pipeline Layout",self.label).as_str()), 
				layout: Some(&render_pipeline_layout), 
				vertex: VertexState{
					module: &self.shader,
					entry_point: &self.vertex_entry_point,
					compilation_options: Default::default(),
					buffers: &self.buffer_layouts
				}, 
				fragment: Some(FragmentState{
					module: &self.shader,
					entry_point: &self.fragment_entry_point,
					compilation_options: Default::default(),
					targets: &[Some(ColorTargetState {
						format: config.format,
						blend: Some(wgpu::BlendState::ALPHA_BLENDING), // TODO check pre-multiplied alpha blending
						write_mask: wgpu::ColorWrites::ALL,
					})]
				}), 
				primitive: PrimitiveState{
					topology: wgpu::PrimitiveTopology::TriangleList,
                	strip_index_format: None,
                	front_face: wgpu::FrontFace::Ccw,
                	cull_mode: None,
                	unclipped_depth: false,
                	polygon_mode: wgpu::PolygonMode::Fill,
                	conservative: false,
				}, 
				multisample: MultisampleState {
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				}, 
				depth_stencil: None, 
				multiview: None, 
				cache: None 
			}
		)
	}
}

/// Contains the renderers
pub struct RenderContext {
	pub rect_pipeline: RectPipeline,
	pub text_pipeline: TextPipeline,
	pub circle_pipeline: CirclePipeline,
	pub image_pipeline: ImagePipeline,
	pub icon_pipeline: IconPipeline,
	pub window_uniform:Uniform
}

impl RenderContext {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, size: &Size) -> Self {
		let rect_pipeline = RectPipeline::new(device, config, size);
		let text_pipeline = TextPipeline::new(device, config, size);
		let circle_pipeline = CirclePipeline::new(device, config, size);
		let image_pipeline = ImagePipeline::new(device, config, size);
		let icon_pipeline = IconPipeline::new(device, config, size);

		let window_buffer = UniformBuilder::new()
			.label("Window uniform")
			.contents(&[size.width,size.height])
			.build(device);
		
        Self {
			rect_pipeline,
			text_pipeline,
			circle_pipeline,
			image_pipeline,
			icon_pipeline,
			window_uniform:window_buffer
        }
    }
}