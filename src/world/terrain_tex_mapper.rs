use crate::res::R;
use mvengine::graphics::Drawable;

pub fn get_terrain_drawable(id: usize) -> Drawable {
    match id {
        1 => Drawable::Texture(R.texture.terrain_water),
        2 => Drawable::Texture(R.texture.terrain_sand),
        3 => Drawable::Texture(R.texture.terrain_grass),
        4 => Drawable::Texture(R.texture.terrain_stone),
        _ => Drawable::Texture(R.mv.texture.missing),
    }
}
