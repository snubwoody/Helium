use std::fmt::Debug;
use wgpu::util::DeviceExt;
use crate::{
	app::AppState, impl_surface, surface::Surface, 
	geometry::vertex::Vertex, Bounds, Color, Position, Size
};

/// Draws images to the screen
#[derive(Clone)]
pub struct IconSurface{
	position:Position,
	size:Size,
	img: image::DynamicImage,
	color:Color
}

impl IconSurface {
	pub fn new(img:image::DynamicImage,color:Color) -> Self{
		Self {
			position:Position::new(0.0, 0.0), 
			size:Size::default(),
			img,
			color
		}
	}

	// FIXME Creating the texture every frame is not a good idea 
	pub fn build(&self,device: &wgpu::Device) -> (wgpu::Texture,wgpu::Extent3d) {
		// TODO maybe move this to the pipeline
		let texture_size = wgpu::Extent3d{
			width:self.size.width as u32,
			height: self.size.height as u32,
			depth_or_array_layers:1
		};

		let texture = device.create_texture(
			&wgpu::TextureDescriptor {
				size: texture_size,
				mip_level_count: 1,
				sample_count: 1,
				dimension: wgpu::TextureDimension::D2,
				format: wgpu::TextureFormat::Rgba8UnormSrgb,
				usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
				label: Some("Icon Texture"),
				view_formats: &[],
			}
		);

		(texture,texture_size)
	}

	fn to_vertices(&self,width:f32,height:f32) -> Vec<Vertex>{
		let color = self.color.normalize();
		let x = self.position.x;
		let y = self.position.y;

		let vertex1 = Vertex::new_with_uv(x,y,color,[0.0,0.0]); //Top left
		let vertex2 = Vertex::new_with_uv(x+width,y,color,[1.0,0.0]); // Top right
		let vertex3 = Vertex::new_with_uv(x, y+height,color,[0.0,1.0]); //Bottom left
		let vertex4 = Vertex::new_with_uv(x+width,y,color,[1.0,0.0]); //Top right
		let vertex5 = Vertex::new_with_uv(x, y+height,color,[0.0,1.0]); // Bottom left
		let vertex6 = Vertex::new_with_uv(x+width, y+height,color,[1.0,1.0]); //Bottom right
	
		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}
}

impl Surface for IconSurface {
	fn draw(
		&mut self,
		render_pass:&mut wgpu::RenderPass,
		context: &crate::geometry::RenderContext,
		state: &AppState
	) {

		// FIXME wgpu panics if size is 0 
		let (texture,texture_size) = self.build(&state.device);

		let vertices = self.to_vertices(texture_size.width as f32,texture_size.height as f32);

		let vertex_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
			label: Some("Vertex buffer"),
			contents: bytemuck::cast_slice(&vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});

		let texture_view = texture.create_view(&Default::default());
		let texture_sampler = state.device.create_sampler(
			&wgpu::SamplerDescriptor { 
				label: Some("Icon Texture sampler"), 
				..Default::default()
			}
		);

		let texture_bind_group = state.device.create_bind_group(
			&wgpu::BindGroupDescriptor { 
				label: Some("Icon Texture bind group"), 
				layout:&context.text_pipeline.texture_bind_group_layout, 
				entries: &[
					wgpu::BindGroupEntry{
						binding:0,
						resource:wgpu::BindingResource::TextureView(&texture_view)
					},
					wgpu::BindGroupEntry{
						binding:1,
						resource:wgpu::BindingResource::Sampler(&texture_sampler)
					}
				]
			}
		);
		
		let img_data = self.img.resize(
			self.size.width as u32, 
			self.size.height as u32, 
			image::imageops::FilterType::CatmullRom
		).to_rgba8();

		state.queue.write_texture(
			wgpu::ImageCopyTextureBase { 
				texture: &texture, 
				mip_level: 0, 
				origin: wgpu::Origin3d::ZERO, 
				aspect: wgpu::TextureAspect::All
			},
			&img_data, 
			wgpu::ImageDataLayout { 
				offset: 0, 
				bytes_per_row: Some(4 * self.size.width as u32), // TODO don't even know what this is
				rows_per_image: None
			}, 
			texture_size
		);

		// Set the render pipeline and vertex buffer
		render_pass.set_pipeline(&context.icon_pipeline.pipeline);
		render_pass.set_bind_group(0, &context.icon_pipeline.window_bind_group, &[]);
		render_pass.set_bind_group(1, &texture_bind_group, &[]);
		render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

		render_pass.draw(0..vertices.len() as u32, 0..1);
	}

	impl_surface!();
}


impl Debug for IconSurface {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("IconSurface")
		.field("size", &self.size)
		.field("position", &self.position)
		.finish()
	}
}