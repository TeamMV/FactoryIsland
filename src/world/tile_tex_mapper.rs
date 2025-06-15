use mvengine::graphics::Drawable;
use crate::res::R;
use crate::world::tiles::resources::LAMP_RES;

pub fn get_tile_drawable(id: usize, state: usize) -> Drawable {
    match id {
        1 => Drawable::Texture(R.texture.tile_wood),
        2 => Drawable::Texture(R.texture.tile_generator),
        3 => LAMP_RES.map(state),
        _ => Drawable::Texture(R.mv.texture.missing)
    }
}