use wgpu::{Device, SurfaceConfiguration};
use wgpu::ShaderModule;
use crate::assets_manager::asset_manager::AssetManager;
use crate::assets_manager::handle::Handle;

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub material_layout: wgpu::BindGroupLayout,
}

pub struct PipelineBuilder<'a> {
    shader: Handle<ShaderModule>,
    vertex_layouts: Vec<wgpu::VertexBufferLayout<'a>>,
    depth_format: Option<wgpu::TextureFormat>,
    material_entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(
        shader: Handle<ShaderModule>,
    ) -> Self {
        Self {
            shader,
            vertex_layouts: Vec::new(),
            depth_format: None,
            material_entries: vec![],
        }
    }
    pub fn material_layout(
        mut self,
        entries: &[wgpu::BindGroupLayoutEntry],
    ) -> Self {
        self.material_entries = entries.to_vec();
        self
    }

    pub fn vertex_layout(mut self, layout: wgpu::VertexBufferLayout<'a>) -> Self {
        self.vertex_layouts.push(layout);
        self
    }

    pub fn depth(mut self, format: wgpu::TextureFormat) -> Self {
        self.depth_format = Some(format);
        self
    }

    pub fn build(self,device: &Device,asset_manager: &AssetManager,surface_config: &SurfaceConfiguration) -> Pipeline {
        let shader = asset_manager.shaders.get(self.shader).unwrap();

        let material_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("material layout"),
                entries: &self.material_entries,
            }
        );

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("pipeline layout"),
                bind_group_layouts: &[
                   // camera_layout,
                    Option::from(&material_layout),
                ],
                immediate_size: 0,
            }
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: shader,
                    entry_point: Option::from("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &self.vertex_layouts,
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader,
                    entry_point: Option::from("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: self.depth_format.map(|format| {
                    wgpu::DepthStencilState {
                        format,
                        depth_write_enabled: Option::from(true),
                        depth_compare: Option::from(wgpu::CompareFunction::Less),
                        stencil: Default::default(),
                        bias: Default::default(),
                    }
                }),
                multisample: wgpu::MultisampleState::default(),
                cache: None,
            }
        );

        Pipeline { pipeline, material_layout }
    }
}

pub fn texture_2d(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
        },
        count: None,
    }
}

pub fn sampler(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(
            wgpu::SamplerBindingType::Filtering
        ),
        count: None,
    }
}