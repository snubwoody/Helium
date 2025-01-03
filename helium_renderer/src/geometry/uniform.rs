use wgpu::util::DeviceExt;

pub struct UniformBuilder<T>{
	label:Option<String>,
	visibility:wgpu::ShaderStages,
	contents:Vec<T>
}

impl<T:bytemuck::Pod> UniformBuilder<T> {
	pub fn new() -> Self {
		Self { 
			label: None, 
			visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, 
			contents: vec![] 
		}
	}

	pub fn label(mut self,label:&str) -> Self{
		self.label = Some(label.into());
		self
	}

	pub fn visibility(mut self,visibility:wgpu::ShaderStages) -> Self{
		self.visibility = visibility;
		self
	}

	pub fn contents(mut self,contents:&[T]) -> Self{
		self.contents.extend(contents); // TODO is extend really the best method since we can chain it?
		self
	}

	/// Build a uniform buffer
	pub fn build(self,device:&wgpu::Device) -> Uniform{
		// TODO use create_buffer instead
		let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: self.label.clone().map(|label|format!("{} buffer",label)).as_deref(),
			contents: bytemuck::cast_slice(&self.contents),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});
		
		let layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				label: self.label.clone().map(|label|format!("{} bind group layout",label)).as_deref(),
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: self.visibility,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
			});
		
		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: self.label.map(|label|format!("{} bind group",label)).as_deref(),
			layout: &layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
		});

		Uniform { buffer,layout,bind_group }
	}
}

/// A uniform buffer
pub struct Uniform{
	pub buffer:wgpu::Buffer,
	pub layout:wgpu::BindGroupLayout,
	pub bind_group:wgpu::BindGroup
}

impl Uniform {
	pub fn buffer(&self) -> &wgpu::Buffer{
		&self.buffer
	}

	pub fn layout(&self) -> &wgpu::BindGroupLayout{
		&self.layout
	}
	pub fn bind_group(&self) -> &wgpu::BindGroup{
		&self.bind_group
	}
}
