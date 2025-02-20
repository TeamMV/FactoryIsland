use mvengine::rendering::control::RenderController;
use mvengine::window::Window;
use crate::game::camera::Camera;
use crate::game::entity::player::WorldPlayer;
use crate::game::world::tiles::TILE_SIZE;
use crate::res::R;

pub struct EntityScreen {

}

impl EntityScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, controller: &mut RenderController, window: &Window, camera: &Camera) {

    }

    pub fn draw_ui(&mut self, controller: &mut RenderController, window: &Window, camera: &Camera) {

    }
}