use std::collections::HashMap;
use crate::assets::texture::Texture;
use crate::assets_manager::handle::Handle;

pub struct AssetRegistry {
    textures: HashMap<String,Handle<Texture>>
}