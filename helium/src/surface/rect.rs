use crate::{
    app::AppState,
    geometry::{vertex::Vertex, RenderContext},
    impl_surface,
    resources::ResourceManager,
    surface::Surface,
    Bounds, Color, Position, Size,
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor,
};

/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the color, coordinates and size.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RectSurface {
    id: String,
    position: Position,
    size: Size,
    color: Color,
    corner_radius: u32,
	size_buffer:usize,
	radius_buffer:usize,
	position_buffer:usize,
	bind_group:usize
}

impl RectSurface {
    pub fn new(id: &str,resources: &mut ResourceManager, state: &AppState) -> Self {
		let size_buffer = resources.add_uniform(
			"Size Buffer", 
			size_of::<[f64;2]>().try_into().unwrap(), 
			&state.device
		);

		let position_buffer = resources.add_uniform(
			"Position Buffer", 
			size_of::<[f64;2]>().try_into().unwrap(), 
			&state.device
		);

		let radius_buffer = resources.add_uniform(
			"Position Buffer", 
			size_of::<[f64;2]>().try_into().unwrap(), 
			&state.device
		);

		let bind_group = resources.add_bind_group(
			"Rect Bind Group",
			&state.context.rect_pipeline.bounds_layout, 
			&state.device, 
			&[radius_buffer,size_buffer,position_buffer], 
			&[],
			&[]
		).unwrap();


        Self {
            id: id.to_string(),
			size_buffer,
			radius_buffer,
			position_buffer,
			bind_group,
            ..Default::default()
        }
    }

    pub fn color(&mut self, color: Color) {
        self.color = color
    }

    /// Set the `corner radius` of the surface.
    pub fn corner_radius(&mut self, radius: u32) {
        self.corner_radius = radius
    }

    pub fn to_vertices(&self) -> Vec<Vertex> {
        let color = self.color.normalize();
        let x = self.position.x;
        let y = self.position.y;

        let vertex1 = Vertex::new(x, y, color); //Top left
        let vertex2 = Vertex::new(x + self.size.width, y, color); // Top right
        let vertex3 = Vertex::new(x, y + self.size.height, color); //Bottom left
        let vertex4 = Vertex::new(x + self.size.width, y, color); //Top right
        let vertex5 = Vertex::new(x, y + self.size.height, color); // Bottom left
        let vertex6 = Vertex::new(x + self.size.width, y + self.size.height, color); //Bottom right

        return vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    }
}

impl Surface for RectSurface {
	fn build(&mut self, state: &AppState, resources: &ResourceManager) {
		resources.write_buffer(
			self.radius_buffer, 
			0, 
			bytemuck::cast_slice(&[self.corner_radius]), 
			&state.queue
		).unwrap();
		resources.write_buffer(
			self.size_buffer, 
			0, 
			bytemuck::cast_slice(&[self.size.width,self.size.height]), 
			&state.queue
		).unwrap();
		resources.write_buffer(
			self.position_buffer, 
			0, 
			bytemuck::cast_slice(&[self.position.x,self.position.y]), 
			&state.queue
		).unwrap();
	}
    fn draw(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &RenderContext,
        state: &AppState,
    ) {
        let vertices = self.to_vertices();

        let vertex_buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        // TODO could maybe use the uniform struct
        let corner_radius = state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Corner radius buffer"),
            contents: bytemuck::cast_slice(&[self.corner_radius as f32]), // Type casting is important maybe save field as f32
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let size_buffer = state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Size buffer"),
            contents: bytemuck::cast_slice(&[self.size.width, self.size.height]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let position_buffer = state.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Position buffer"),
            contents: bytemuck::cast_slice(&[self.position.x, self.position.y]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bound_bind_group = state.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Rect bounds bind group"),
            layout: &context.rect_pipeline.bounds_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: corner_radius.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: size_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: position_buffer.as_entire_binding(),
                },
            ],
        });

        // Set the render pipeline and vertex buffer
        render_pass.set_pipeline(&context.rect_pipeline.pipeline);
        render_pass.set_bind_group(0, &context.rect_pipeline.window_bind_group, &[]);
        render_pass.set_bind_group(1, resources.bind_group(self.bind_group).unwrap(), &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        // TODO could maybe use some kind of batch drawing later?
        render_pass.draw(0..vertices.len() as u32, 0..1);
    }

    impl_surface!();
}
