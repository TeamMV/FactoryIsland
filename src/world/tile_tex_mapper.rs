use mvengine::graphics::comp::Drawable;
use crate::res::R;

pub fn get_tile_drawable(id: usize) -> Drawable {
    match id {
        1 => Drawable::Texture(R.texture.tile_wood),
        2 => Drawable::Texture(R.texture.player),
        _ => Drawable::Texture(R.mv.texture.missing)
    }
}