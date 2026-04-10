use crate::assets_manager::arena::Arena;
use crate::assets::material::Material;
use crate::assets::mesh::Mesh;
use crate::assets::pipeline::Pipeline;
use crate::assets::texture::Texture;

pub struct AssetManager{
    pub pipelines: Arena<Pipeline>,
    pub meshes: Arena<Mesh>,
    pub materials: Arena<Material>,
    pub textures: Arena<Texture>,
}

impl AssetManager{
    pub fn new()->Self{
        Self{
            pipelines: Arena::new(),
            meshes: Arena::new(),
            materials: Arena::new(),
            textures: Arena::new(),
        }
    }
}