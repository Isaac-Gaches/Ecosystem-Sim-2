#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
}

pub struct InstanceBuffer {
    pub buffer: wgpu::Buffer,
    pub capacity: usize,
}

impl InstanceBuffer {
    pub fn new(device: &wgpu::Device, capacity: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("instance buffer"),
            size: (capacity * std::mem::size_of::<InstanceRaw>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self { buffer, capacity }
    }

    pub fn write(&self, queue: &wgpu::Queue, data: &[InstanceRaw]) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
    }
}