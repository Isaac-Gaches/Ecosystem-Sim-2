use crate::assets::arena::Arena;
use crate::resources::material::Material;
use crate::resources::mesh::Mesh;
use crate::resources::pipeline::Pipeline;
use crate::resources::texture::Texture;

pub struct AssetManager{
    pub pipelines: Arena<Pipeline>,
    pub meshes: Arena<Mesh>,
    pub materials: Arena<Material>,
    pub textures: Arena<Texture>,
}