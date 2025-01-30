use helium::BLACK;
use helium_renderer::{Circle, Icon,Rect, Renderer, Text};
use winit::{event::WindowEvent, event_loop::EventLoop, window::WindowBuilder};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warn,helium_renderer=trace");
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = Renderer::new(&window).await;

    event_loop
        .run(|event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                WindowEvent::Resized(size) => {
                    renderer.resize(size.into());
                }
                WindowEvent::RedrawRequested => {
                    renderer.draw([Text::new("Hello world")]);
                    renderer.render();
                }
                event => {
                    window.request_redraw();
                }
            },
            _ => {}
        })
        .expect("Event loop error occured");
}
