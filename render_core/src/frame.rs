use std::ops::Range;
use wgpu::{Queue};
use crate::assets::instance::{InstanceBuffer, Instance};
use crate::assets_manager::handle::Handle;
use crate::assets::material::Material;
use crate::assets::mesh::Mesh;

pub struct RenderItem {
    pub mesh: Handle<Mesh>,
    pub material: Handle<Material>,
    pub instance_range: Range<u32>,
}

impl RenderItem {
    fn new(mesh: Handle<Mesh>, material: Handle<Material>,instance_range: Range<u32>) -> Self {
        Self{
            mesh,
            material,
            instance_range,
        }
    }
}

pub struct Frame {
    pub(crate) items: Vec<RenderItem>,
    pub(crate) instance_buffer: InstanceBuffer,
    instances: Vec<Instance>
}

impl Frame {
    pub(crate) fn new(instance_buffer: InstanceBuffer) -> Self {
        Self {
            items: Vec::new(),
            instance_buffer,
            instances: vec![],
        }
    }

    pub(crate) fn clear(&mut self) {
        self.items.clear();
        self.instances.clear();
    }

    pub fn draw(&mut self,instances: &mut Vec<Instance>,material: Handle<Material>,mesh: Handle<Mesh>) {
        let start = self.instances.len() as u32;
        self.instances.append(instances);
        let end = self.instances.len() as u32;
        let instance_range = start..end;
        let item = RenderItem::new(mesh, material, instance_range);
        self.items.push(item);
    }

    pub fn sort(&mut self) {
        self.items.sort_by_key(|item| {
            (
               // item.pipeline.index,
                item.material.index,
                item.mesh.index,
            )
        });
    }

    pub fn upload_instances(&mut self, queue: &Queue){
        self.instance_buffer.write(queue,self.instances.as_slice());
    }
}