pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
}

pub struct PipelineBuilder<'a> {
    device: &'a wgpu::Device,
    shader: &'a wgpu::ShaderModule,
    layout: Option<&'a wgpu::PipelineLayout>,
    vertex_layouts: Vec<wgpu::VertexBufferLayout<'a>>,
    color_format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(
        device: &'a wgpu::Device,
        shader: &'a wgpu::ShaderModule,
        color_format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            device,
            shader,
            layout: None,
            vertex_layouts: Vec::new(),
            color_format,
            depth_format: None,
        }
    }

    pub fn layout(mut self, layout: &'a wgpu::PipelineLayout) -> Self {
        self.layout = Some(layout);
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

    pub fn build(self) -> Pipeline {
        let pipeline = self.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("pipeline"),
                layout: self.layout,
                vertex: wgpu::VertexState {
                    module: self.shader,
                    entry_point: Option::from("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &self.vertex_layouts,
                },
                fragment: Some(wgpu::FragmentState {
                    module: self.shader,
                    entry_point: Option::from("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.color_format,
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

        Pipeline { pipeline }
    }
}