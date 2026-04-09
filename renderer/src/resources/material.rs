use crate::assets::handle::Handle;
use crate::resources::pipeline::Pipeline;

pub struct Material {
    pub pipeline: Handle<Pipeline>,
    pub bind_group: wgpu::BindGroup,
}