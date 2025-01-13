use crate::app::AppState;
use crate::events::EventManager;
use crate::{view::ViewManager, widgets::Widget};
use crystal::LayoutSolver;

pub struct Page {
    layout: Box<dyn crystal::Layout>,
    widget: Box<dyn Widget>,
    views: ViewManager,
	events: EventManager
}

impl Page {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self {
            layout: widget.layout(),
            views: ViewManager::new(&widget),
            widget: Box::new(widget),
			events:EventManager::new(),
        }
    }

	pub fn handle(&mut self,event:&winit::event::WindowEvent){
		let notifications = self.events.handle(event,&*self.layout);
	}

	pub fn resize(&mut self,state: &AppState) -> Result<(),crate::Error>{
		LayoutSolver::solve(&mut *self.layout, state.size);
		self.views.resize(&*self.layout, state)?;
		Ok(())
	}

    pub fn build(&mut self, state: &AppState) -> Result<(), crate::Error> {
        LayoutSolver::solve(&mut *self.layout, state.size);
        self.views.build(&*self.layout, state)
    }

    pub fn render(&mut self, state: &AppState) {
        let instant = std::time::Instant::now();

        let output = state.surface.get_current_texture().unwrap(); // TODO maybe handle this error
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        self.views.draw(&mut render_pass, state);

        // Drop the render pass because it borrows encoder mutably
        std::mem::drop(render_pass);

        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        log::debug!("{:?}", instant.elapsed())
    }
}
