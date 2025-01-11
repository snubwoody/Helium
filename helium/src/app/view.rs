use super::AppState;
use crate::{surface::SurfaceManager, widgets::Widget};
use crystal::LayoutSolver;
use std::time::Instant;

pub struct View {
    layout: Box<dyn crystal::Layout>,
    widget: Box<dyn Widget>,
    surfaces: SurfaceManager,
}

impl View {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self {
            layout: widget.layout(),
            surfaces: SurfaceManager::new(&widget),
            widget: Box::new(widget),
        }
    }

    // TODO spawn a task for each function?

    pub fn resize(&mut self, state: &AppState) {
        LayoutSolver::solve(&mut *self.layout, state.size);
        self.surfaces.resize(&*self.layout, state);
    }

    pub fn update(&mut self, state: &AppState) {
        self.layout = self.widget.layout();
        //self.surfaces.rebuild(&*self.widget, state);
    }

    pub fn build(&mut self, state: &AppState) {
        LayoutSolver::solve(&mut *self.layout, state.size);
        self.surfaces.build(state);
        self.surfaces.resize(&*self.layout, state);
    }

    pub fn render(&mut self, state: &AppState) {
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

        let _ = LayoutSolver::solve(&mut *self.layout, state.size);

        let render_now = Instant::now();
        self.surfaces.draw(&mut render_pass, state);
        //log::debug!("Spent {:?} rendering", render_now.elapsed());

        // Drop the render pass because it borrows encoder mutably
        std::mem::drop(render_pass);

        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

#[cfg(test)]
mod test {
    // TODO test that all the layouts and surfaces have the same id's
}
