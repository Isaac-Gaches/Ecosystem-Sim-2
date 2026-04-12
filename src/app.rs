use std::sync::Arc;
use glam::{Quat, Vec3};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use render_core::assets::*;
use render_core::assets_manager::*;
use render_core::Renderer;

pub struct App{
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,

    material: Option<Handle<Material>>,
    instance: Option<Instance>,
    mesh: Option<Handle<Mesh>>,
}



impl App{
    pub fn new()->Self{
        Self{
            window: None,
            renderer: None,
            material: None,
            instance: None,
            mesh: None,
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

        let mut renderer = pollster::block_on(Renderer::new(window.clone()));

        let vertices = [
            Vertex::new([0.,0.]),
            Vertex::new([1.,0.]),
            Vertex::new([1.,1.]),
            Vertex::new([0.,1.])
        ];

        let indices = [0, 1, 2, 0, 2, 3];

        let mesh = renderer.create_mesh(&vertices, &indices);

        let shader = renderer.load_shader(include_str!("render/shader.wgsl"));

        let pipeline_builder = PipelineBuilder::new(shader)
            .vertex_layout(Vertex::buffer_layout())
            .vertex_layout(Instance::buffer_layout())
            .material_layout(&[uniform(0)]);

        let pipeline = renderer.create_pipeline(pipeline_builder);

        let camera_buffer = renderer.create_uniform();

        renderer.write_uniform(camera_buffer.clone(),Camera{ num: 0.2 });

        let material_builder = MaterialBuilder::new(pipeline)
            .uniform(0,camera_buffer);

        let material = renderer.create_material(material_builder);

        let instance = Instance::from_transform(Vec3::new(-0.5, -0.5, 0.0), Quat::IDENTITY, Vec3::ONE);

        self.material = Some(material);
        self.mesh = Some(mesh);
        self.instance= Some(instance);

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent)  {
        let renderer = self.renderer.as_mut().unwrap();
        let _window = self.window.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                renderer.resize_surface(size);
            }

            WindowEvent::RedrawRequested => {
                let frame = renderer.begin_frame();

                frame.draw(
                    &mut vec![self.instance.unwrap()],
                    self.material.unwrap(),
                    self.mesh.unwrap(),
                );

                renderer.submit_frame();

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

#[repr(C)]
#[derive(Copy,Clone,bytemuck::Pod, bytemuck::Zeroable)]
struct Camera{
    num:f32,
}

