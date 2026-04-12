use wgpu::Device;
use crate::assets_manager::handle::Handle;
use crate::assets::pipeline::Pipeline;
use crate::assets::texture::Texture;
use crate::assets_manager::asset_manager::AssetManager;

pub struct Material {
    pub bind_group: wgpu::BindGroup,
    pub pipeline: Handle<Pipeline>,
}

pub struct MaterialBuilder<'a> {
  //  device: &'a wgpu::Device,
  //  layout: &'a wgpu::BindGroupLayout,
    entries: Vec<wgpu::BindGroupEntry<'a>>,
    pipeline: Handle<Pipeline>,
}

impl<'a> MaterialBuilder<'a> {
    pub fn new(
      //  device: &'a wgpu::Device,
      //  layout: &'a wgpu::BindGroupLayout,
      pipeline: Handle<Pipeline>,
    ) -> Self {
        Self {
          //  device,
           // layout,
           // layout: &(),
            entries: Vec::new(),
            pipeline,
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

    pub fn build(self,device:&Device, asset_manager: &AssetManager) -> Material {
        let pipeline = asset_manager.pipelines.get(self.pipeline.clone()).unwrap();

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("material bind group"),
                layout: &pipeline.material_layout,
                entries: &self.entries,
            }
        );

        Material {
            bind_group,
            pipeline: self.pipeline,
        }
    }
}