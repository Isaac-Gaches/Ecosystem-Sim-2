use crate::assets_manager::asset_manager::AssetManager;
use crate::assets_manager::handle::Handle;
use crate::assets::material::Material;
use crate::assets::mesh::Mesh;

pub struct RenderItem {
    pub mesh: Handle<Mesh>,
    pub material: Handle<Material>,
    pub instance_range: std::ops::Range<u32>,
}

pub struct RenderQueue{
    pub items: Vec<RenderItem>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn add(&mut self, item: RenderItem) {
        self.items.push(item);
    }
}