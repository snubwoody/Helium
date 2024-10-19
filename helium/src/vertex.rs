/// Represents a singles vertex.
#[repr(C)]
#[derive(Debug,Clone,Copy,PartialEq,bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex{
	pub position: [f32;2],
	pub colour:[f32;4],
	pub uv:[f32;2],
}

impl Vertex {
	pub fn new(x:f32,y:f32,colour:[f32;4]) -> Self{
		let r = colour[0];
		let g = colour[1];
		let b = colour[2];
		let a = colour[3];

		Self { 
			position: [x,y],
			colour:[r,g,b,a],
			uv:[1.0,1.0],
		}
	}
	
	/// Create a new vertex with texture uv's.
	pub fn new_with_texture(x:f32,y:f32,colour:[f32;4],texture_coords:[f32;2]) -> Self {
		let r = colour[0];
		let g = colour[1];
		let b = colour[2];
		let a = colour[3];

		Self { 
			position: [x,y],
			colour:[r,g,b,a],
			uv:texture_coords,
		}
	}
}