use std::sync::Arc;
use wgpu::{Device, Queue, ShaderModule,Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::{frame::Frame};
use crate::assets::material::{Material, MaterialBuilder};
use crate::assets::mesh::Mesh;
use crate::assets::pipeline::{Pipeline, PipelineBuilder};
use crate::assets::uniform::Uniform;
use crate::assets_manager::asset_manager::AssetManager;
use crate::assets_manager::asset_registry::AssetRegistry;
use crate::assets_manager::handle::Handle;

pub struct Renderer {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,

    pub asset_registry: AssetRegistry,
    pub asset_manager: AssetManager,

    frame: Frame,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor::default(),
        ).await.unwrap();

        let caps = surface.get_capabilities(&adapter);

        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let asset_manager = AssetManager::new();
        let asset_registry = AssetRegistry::new();

        let frame = Frame::new(&device);

        Self {
            device,
            queue,
            surface,
            surface_config,
            asset_registry,
            asset_manager,
            frame,
        }
    }

    pub fn render(&self) {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => {
                // still usable, but should reconfigure soon
                frame
            }

            wgpu::CurrentSurfaceTexture::Timeout => {
                return; // skip frame
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                return; // window hidden
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                // reconfigure surface
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // recreate surface ideally, but reconfigure for now
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),

                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],

                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            for item in &self.frame.items {
                let material = self.asset_manager.materials.get(item.material.clone()).unwrap();
                let pipeline = self.asset_manager.pipelines.get(material.pipeline.clone()).unwrap();
                let mesh = self.asset_manager.meshes.get(item.mesh).unwrap();

                render_pass.set_pipeline(&pipeline.pipeline);
                render_pass.set_bind_group(0, &material.bind_group, &[]);
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, self.frame.instance_buffer.buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                render_pass.draw_indexed(0..mesh.index_count, 0, item.instance_range.clone());
            }
        }

        self.queue.submit(Some(encoder.finish()));

        output.present();
    }

    pub fn resize_surface(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn begin_frame(&mut self) -> &mut Frame {
        self.frame.clear();
        &mut self.frame
    }

    pub fn submit_frame(&mut self) {
        self.frame.sort();
        self.frame.upload_instances(&self.queue);
    }

    pub fn create_mesh<T: bytemuck::Pod>(&mut self, vertices: &[T],indices: &[u32]) -> Handle<Mesh>{
        let mesh = Mesh::new(&self.device,vertices,indices);
        self.asset_manager.meshes.insert(mesh)
    }

    pub fn create_pipeline(&mut self,builder: PipelineBuilder) -> Handle<Pipeline> {
        let pipeline = builder.build(&self.device,&self.asset_manager,&self.surface_config);
        self.asset_manager.pipelines.insert(pipeline)
    }

    pub fn create_material(&mut self,builder: MaterialBuilder) -> Handle<Material> {
        let material = builder.build(&self.device,&self.asset_manager);
        self.asset_manager.materials.insert(material)
    }

    pub fn create_uniform(&mut self) -> Handle<Uniform> {
        let uniform = Uniform::new(&self.device);
        self.asset_manager.uniforms.insert(uniform)
    }

    pub fn load_shader(&mut self,src: &'static str) -> Handle<ShaderModule>{
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(src.into()),
        });
        self.asset_manager.shaders.insert(shader)
    }

    pub fn write_uniform<T: bytemuck::Pod>(&self,handle: Handle<Uniform>,data: T){
        let uniform = self.asset_manager.uniforms.get(handle).unwrap();
        self.queue.write_buffer(&uniform.buffer, 0, bytemuck::cast_slice(&[data]));
    }
}

