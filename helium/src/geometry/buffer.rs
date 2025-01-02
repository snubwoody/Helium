use std::collections::HashMap;

/// An abstraction over `wgpu::Buffer` and `wgpu::BindGroup`, this struct holds
/// multiple buffers and their bind group
pub struct BufferGroup{
	bind_group:wgpu::BindGroup,
	buffers:Vec<wgpu::Buffer>,
	/// Maps the buffer labels to their index
	buffer_map:HashMap<String,usize>
}

impl BufferGroup {
	/// Get the bind group
	pub fn bind_group(&self) -> &wgpu::BindGroup{
		&self.bind_group
	}

	/// Get a buffer from the `buffers` in this [`BufferGroup`]
	/// 
	/// # Panics
	/// This function panics if the `buffer` is not found, it is to be used with 
	/// buffers that are not optional i.e. without the buffer the program would not 
	/// run correctly. 
	pub fn buffer(&self,label:&str) -> &wgpu::Buffer{
		let index = *self.buffer_map.get(label).unwrap();
		&self.buffers[index]
	}
}

pub struct BufferGroupBuilder<'b>{
	count:usize,
	buffers:Vec<(usize,wgpu::BufferDescriptor<'b>)>,
	samplers:Vec<(usize,wgpu::SamplerDescriptor<'b>)>,
	label:String
}

impl<'b> BufferGroupBuilder<'b> {
	pub fn new(label:&str) -> Self{
		Self { 
			count:0, 
			buffers:vec![], 
			samplers:vec![], 
			label:label.to_owned() 
		}
	}

	/// Add a buffer
	fn add_buffer(mut self,label:&'b str,size:u64,usage:wgpu::BufferUsages) -> Self{
		let buffer = wgpu::BufferDescriptor{
			size,
			usage,
			label:Some(label),
			mapped_at_creation:false,
		};
		self.buffers.push((self.count,buffer));
		self.count += 1;
		self
	}

	/// Add a sampler with a default config
	fn add_default_sampler(mut self,label:&'b str) -> Self{
		let sampler = wgpu::SamplerDescriptor { 
			label: Some(label), 
			..Default::default()
		};
		self.samplers.push((self.count,sampler));
		self.count += 1;
		self
	}

	fn add_texure(mut self) -> Self{
		todo!();
	}

	/// Build the `wgpu::BindGroupLayout`
	fn build_layout(&self,device:&wgpu::Device) -> wgpu::BindGroupLayout{
		let mut layout_entries = vec![];

		// TODO wait
		let buffer_layout_entries = self.buffers.iter().map(|buffer|{
			wgpu::BindGroupLayoutEntry{
				binding:buffer.0 as u32,
				visibility:wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Buffer { 
					ty: wgpu::BufferBindingType::Uniform, 
					has_dynamic_offset: false, 
					min_binding_size: None 
				},
				count:None
			}
		}).collect::<Vec<_>>();
		
		let sampler_layout_entries = self.buffers.iter().map(|desc|{
			wgpu::BindGroupLayoutEntry{
				binding:desc.0 as u32,
				visibility:wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
				count:None
			}
		}).collect::<Vec<_>>();

		layout_entries.extend(buffer_layout_entries);
		//layout_entries.extend(sampler_layout_entries);

		device.create_bind_group_layout(
			&wgpu::BindGroupLayoutDescriptor{
				label:Some(format!("{} bind group layout",self.label).as_str()),
				entries:&layout_entries,
			}
		)
	}

	/// Build a [`BufferGroup`]
	fn build(self,device:&wgpu::Device) -> BufferGroup{
		let mut entries = vec![];

		let buffers = self.buffers.iter().map(|desc|{
			device.create_buffer(&desc.1)
		}).collect::<Vec<_>>();

		for (i,desc) in self.buffers.iter().enumerate(){
			let entry = wgpu::BindGroupEntry {
				binding: desc.0 as u32,
				resource: buffers[i].as_entire_binding(),
			};
			entries.push(entry);
		}

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(format!("{} bind group",self.label).as_str()),
            layout: &self.build_layout(device),
            entries:&entries,
        });

		let mut buffer_map = HashMap::new();

		for (i,_) in buffers.iter().enumerate(){
			let label = self.buffers[i].1.label.expect("Buffer label missing").to_string();
			buffer_map.insert(label, i);
		}
		

		BufferGroup{
			bind_group,
			buffers,
			buffer_map
		}
	}
}


#[cfg(test)]
mod test{
    use super::BufferGroupBuilder;

	fn setup(){

	}

	#[test]
	fn buffer_builder(){
		setup();
		let buffer_group = 
			BufferGroupBuilder::new("Triangle")
			.add_buffer("Size", 24, wgpu::BufferUsages::UNIFORM);
	}

	#[test]
	fn buffer_label_mapped_to_correct_index(){

	}
}
