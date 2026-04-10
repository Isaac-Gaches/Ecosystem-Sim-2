use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use render_core::Renderer;

pub struct App{
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl App{
    pub fn new()->Self{
        Self{
            window: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for App{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let renderer = pollster::block_on(Renderer::new(window.clone()));

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent)  {
        let renderer = self.renderer.as_mut().unwrap();
        let window = self.window.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                renderer.resize_surface(size);
            }

            WindowEvent::RedrawRequested => {
                renderer.render();
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

