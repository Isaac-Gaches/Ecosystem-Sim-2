use wgpu::{BufferDescriptor, BufferUsages, Device};

pub struct Uniform {
    pub buffer: wgpu::Buffer,
}
impl Uniform {
    pub fn new(device: &Device)->Self{
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Uniform"),
            size: 256,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self { buffer }
    }
}