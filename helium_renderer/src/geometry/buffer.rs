use std::collections::HashMap;

/// An abstraction over `wgpu::Buffer` and `wgpu::BindGroup`, this struct holds
/// multiple buffers and their bind group]
#[derive(Debug)]
pub struct BufferGroup{
	bind_group_layout:wgpu::BindGroupLayout,
	bind_group:wgpu::BindGroup,
	buffers:Vec<wgpu::Buffer>,
	/// Maps the buffer labels to their index
	buffer_map:HashMap<String,usize>
}

impl BufferGroup {
	/// Get the bind group layout
	pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout{
		&self.bind_group_layout
	}

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
		let index = *self.buffer_map.get(label).expect(format!("{} buffer not found",label).as_str());
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
	pub fn add_buffer(mut self,label:&'b str,size:usize,usage:wgpu::BufferUsages) -> Self{
		let buffer = wgpu::BufferDescriptor{
			size: size as u64,
			usage,
			label:Some(label),
			mapped_at_creation:false,
		};
		self.buffers.push((self.count,buffer));
		self.count += 1;
		self
	}

	/// Add a sampler with a default config
	pub fn add_default_sampler(mut self,label:&'b str) -> Self{
		let sampler = wgpu::SamplerDescriptor { 
			label: Some(label), 
			..Default::default()
		};
		self.samplers.push((self.count,sampler));
		self.count += 1;
		self
	}

	pub fn add_texure(mut self) -> Self{
		todo!();
	}

	/// Build the `wgpu::BindGroupLayout`
	pub fn build_layout(&self,device:&wgpu::Device) -> wgpu::BindGroupLayout{
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
	pub fn build(self,device:&wgpu::Device) -> BufferGroup{
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
		let bind_group_layout = self.build_layout(device);

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(format!("{} bind group",self.label).as_str()),
            layout: &bind_group_layout,
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
			buffer_map,
			bind_group_layout
		}
	}
}


#[cfg(test)]
mod test{
    use winit::{
		event_loop::EventLoopBuilder, 
		platform::windows::EventLoopBuilderExtWindows, 
		window::WindowBuilder
	};
    use super::BufferGroupBuilder;

	async fn setup(window:&winit::window::Window) -> wgpu::Device{
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, _) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device/Queue"),
                    required_features: wgpu::Features::empty(),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

		device
	}

	#[test]
	fn buffer_label_mapped_to_correct_index(){
		let event_loop = 
			EventLoopBuilder::new()
			.with_any_thread(true)
			.build()
			.unwrap();
		
		let window = WindowBuilder::new().build(&event_loop).unwrap();
		
		// Event loops cannot be created in other threads
		let device = async_std::task::block_on(setup(&window));

		let buffer_group = 
			BufferGroupBuilder::new("Triangle")
			.add_buffer("Size", 24, wgpu::BufferUsages::UNIFORM)
			.add_buffer("Position", 52, wgpu::BufferUsages::UNIFORM)
			.add_buffer("Corner radius", 52, wgpu::BufferUsages::UNIFORM)
			.build(&device);

		assert_eq!(
			*buffer_group.buffer_map.get("Size").unwrap(),
			0
		);
		assert_eq!(
			*buffer_group.buffer_map.get("Corner radius").unwrap(),
			2
		);
	}
}
