use crate::res::R;
use mvengine::graphics::Drawable;
use mvutils::lazy;

lazy! {
    pub static BASE: Drawable = Drawable::Texture(R.texture.tile_wood);
}
