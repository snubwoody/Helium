use helium_core::size::Size;
use wgpu::vertex_attr_array;
use crate::geometry::{uniform::UniformBuilder, vertex::Vertex};

// TODO could maybe move the buffers into herer
// TODO pls refactor this long and ugly code, there's a lot of reused code;
/// Holds the render pipeline
#[derive(Debug)]
pub struct RectPipeline{
	pub pipeline: wgpu::RenderPipeline,
	pub window_bind_group: wgpu::BindGroup,
	pub bounds_layout: wgpu::BindGroupLayout,
    pub window_buffer: wgpu::Buffer,
}

impl RectPipeline {
	pub fn new(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		size:&Size
	) -> Self {
		// Compile the shaders
		let shader = device.create_shader_module(
			wgpu::ShaderModuleDescriptor{
				label: Some("Rect Shader Module"),
				source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/rect.wgsl").into())
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
