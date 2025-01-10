use super::AppState;
use crate::{surface::SurfaceManager, widgets::Widget};
use crystal::LayoutSolver;
use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

pub struct View {
    layout: Box<dyn crystal::Layout>,
    widget: Arc<RwLock<dyn Widget>>,
    surfaces: SurfaceManager,
}

impl View {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self {
            layout: widget.layout(),
            surfaces: SurfaceManager::new(&widget),
            widget: Arc::new(RwLock::new(widget)),
        }
    }

    // TODO spawn a task for each function?
    pub fn setup_loop(&mut self) {
        let widget = self.widget.clone();
        std::thread::spawn(move || {
            widget.write().unwrap().update();
        });
    }

    pub fn resize(&mut self, state: &AppState) {
        LayoutSolver::solve(&mut *self.layout, state.size);
        self.surfaces.resize(&*self.layout, state);
    }

    pub fn update(&mut self, state: &AppState) {
        match self.widget.try_read() {
            Ok(widget) => {
                let layout = widget.layout();
                self.surfaces.rebuild(&widget, state);
                self.layout = layout;
            }
            Err(_) => {}
        }
    }

    pub fn build(&mut self, state: &AppState) {
        LayoutSolver::solve(&mut *self.layout, state.size);
        self.surfaces.build(&state);
        self.surfaces.resize(&*self.layout, state);
    }

    pub fn render(&mut self, state: &AppState) {
        let now = Instant::now();
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

        // Has to be in this order otherwise it crashes particularly because of 0 size textures
        // FIXME above
        //self.update();
        let _ = LayoutSolver::solve(&mut *self.layout, state.size);

        //self.root_body.update_sizes(&*self.layout);

        let render_now = Instant::now();
        //self.root_body.render(&mut render_pass, state);

        self.surfaces.draw(&mut render_pass, state);
        log::debug!("Spent {:?} rendering", render_now.elapsed());

        // Drop the render pass because it borrows encoder mutably
        std::mem::drop(render_pass);

        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        log::debug!("{}ms", now.elapsed().as_millis())
    }
}

#[cfg(test)]
mod test {
    // TODO test that all the layouts and surfaces have the same id's
}
