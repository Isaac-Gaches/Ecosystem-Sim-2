use crate::assets_manager::handle::Handle;
use crate::assets::pipeline::Pipeline;
use crate::assets::texture::Texture;

pub struct Material {
    pub pipeline: Handle<Pipeline>,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        texture: &Texture,
        pipeline: Handle<Pipeline>,
    ) -> Material {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("material bind group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Material {
            pipeline,
            bind_group,
        }
    }
}

pub struct MaterialBuilder<'a> {
    device: &'a wgpu::Device,
    layout: &'a wgpu::BindGroupLayout,
    entries: Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> MaterialBuilder<'a> {
    pub fn new(
        device: &'a wgpu::Device,
        layout: &'a wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            device,
            layout,
            entries: Vec::new(),
        }
    }

    pub fn texture(
        mut self,
        texture_binding: u32,
        sampler_binding: u32,
        texture: &'a Texture,
    ) -> Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding: texture_binding,
            resource: wgpu::BindingResource::TextureView(&texture.view),
        });
        self.entries.push(wgpu::BindGroupEntry {
            binding: sampler_binding,
            resource: wgpu::BindingResource::Sampler(&texture.sampler),
        });
        self
    }


    pub fn uniform_buffer(
        mut self,
        binding: u32,
        buffer: &'a wgpu::Buffer,
    ) -> Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        });
        self
    }

    pub fn bind_resource(
        mut self,
        binding: u32,
        resource: wgpu::BindingResource<'a>,
    ) -> Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding,
            resource,
        });
        self
    }

    pub fn build(self, pipeline: Handle<Pipeline>) -> Material {
        let bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("material bind group"),
                layout: self.layout,
                entries: &self.entries,
            }
        );

        Material {
            pipeline,
            bind_group,
        }
    }
}