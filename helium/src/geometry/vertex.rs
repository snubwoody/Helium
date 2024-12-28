/// Represents a singles vertex.
#[repr(C)]
#[derive(Debug,Clone,Copy,PartialEq,bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex{
	pub position: [f32;2],
	pub color:[f32;4],
	pub uv:[f32;2],
}

impl Vertex {
	pub fn new(x:f32,y:f32,color:[f32;4]) -> Self{
		let r = color[0];
		let g = color[1];
		let b = color[2];
		let a = color[3];

		Self { 
			position: [x,y],
			color:[r,g,b,a],
			uv:[1.0,1.0],
		}
	}
	
	/// Create a new vertex with texture uv's.
	pub fn new_with_texture(x:f32,y:f32,color:[f32;4],texture_coords:[f32;2]) -> Self {
		let r = color[0];
		let g = color[1];
		let b = color[2];
		let a = color[3];

		Self { 
			position: [x,y],
			color:[r,g,b,a],
			uv:texture_coords,
		}
	}
}


pub struct VertexBufferLayoutBuilder{
	attributes:Vec<wgpu::VertexAttribute>
}

impl VertexBufferLayoutBuilder {
	pub fn new() -> Self{
		Self{
			attributes:vec![]
		}
	}

	pub fn add_attribute(mut self,offset:usize,format:wgpu::VertexFormat) -> Self{
		let shader_location = self.attributes.len() as u32;
		let attribute = wgpu::VertexAttribute{
			offset: offset as wgpu::BufferAddress,
			shader_location,
			format
		};
		self.attributes.push(attribute);
		self
	}

	pub fn build(self) -> wgpu::VertexBufferLayout<'static>{
		wgpu::VertexBufferLayout { 
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
		}
	}
}
