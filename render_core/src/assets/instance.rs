use wgpu::VertexBufferLayout;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub model: [[f32; 4]; 4],
}

impl Instance {
    pub fn from_transform(
        position: glam::Vec3,
        rotation: glam::Quat,
        scale: glam::Vec3,
    ) -> Self {
        let transform =
            glam::Mat4::from_scale_rotation_translation(
                scale,
                rotation,
                position,
            );

        Self {
            model: transform.to_cols_array_2d(),
        }
    }

    pub fn buffer_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 48,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct InstanceBuffer {
    pub buffer: wgpu::Buffer,
    pub capacity: usize,
}

impl InstanceBuffer {
    pub fn new(device: &wgpu::Device, capacity: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("instance buffer"),
            size: (capacity * std::mem::size_of::<Instance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self { buffer, capacity }
    }

    pub fn write(&self, queue: &wgpu::Queue, data: &[Instance]) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
    }
}