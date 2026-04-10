use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, SurfaceTargetUnsafe};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::{render_queue::RenderQueue};
use crate::assets::instance::InstanceBuffer;
use crate::assets_manager::asset_manager::AssetManager;

pub struct Renderer {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,

    asset_manager: AssetManager,
    instance_buffer: InstanceBuffer,
    render_items: RenderQueue,
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

        let surface_config = wgpu::SurfaceConfiguration {
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
        let render_items = RenderQueue::new();
        let instance_buffer = InstanceBuffer::new(&device,16384);

        Self {
            device,
            queue,
            surface,
            surface_config,
            asset_manager,
            instance_buffer,
            render_items,
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
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
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

            for item in &self.render_items.items {
                let material = self.asset_manager.materials.get(item.material.clone()).unwrap();
                let pipeline = &self.asset_manager.pipelines.get(material.pipeline.clone()).unwrap().pipeline;
                let mesh = self.asset_manager.meshes.get(item.mesh).unwrap();

                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(0, &material.bind_group, &[]);
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, self.instance_buffer.buffer.slice(..));
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


}