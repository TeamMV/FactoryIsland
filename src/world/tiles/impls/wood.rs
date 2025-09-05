use mvengine::graphics::Drawable;
use mvutils::lazy;
use crate::res::R;

lazy! {
    pub static BASE: Drawable = Drawable::Texture(R.texture.tile_wood);
}