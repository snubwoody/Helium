/// Represents a single vertex
#[repr(C)]
#[derive(Debug,Clone,Copy,PartialEq,bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex{
	pub position: [f32;2],
	pub color:[f32;4],
	pub uv:[f32;2],
}

impl Vertex {
	/// Creates a new [`Vertex`]
	pub fn new(x:f32,y:f32,color:[f32;4]) -> Self{
		Self { 
			position: [x,y],
			color,
			uv:[1.0,1.0],
		}
	}
	
	/// Creates a new [`Vertex`] with texture uv's.
	pub fn new_with_uv(x:f32,y:f32,color:[f32;4],uv:[f32;2]) -> Self {
		let r = color[0];
		let g = color[1];
		let b = color[2];
		let a = color[3];

		Self { 
			position: [x,y],
			color:[r,g,b,a],
			uv
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

	/// Adds a vertex attribute to the `VertexBufferLayout`
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
			attributes: Box::leak(Box::new(self.attributes))
		}
	}
}


