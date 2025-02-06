use mvengine::rendering::control::RenderController;
use mvengine::window::Window;
use crate::game::camera::Camera;
use crate::game::entity::player::Player;
use crate::res::R;

pub struct EntityScreen {
    players: Vec<Player>
}

impl EntityScreen {
    pub fn new() -> Self {
        Self {
            players: vec![Player::new("v22", R.texture.tile_sand)],
        }
    }

    pub fn draw(&mut self, controller: &mut RenderController, window: &Window, camera: &Camera) {
        for player in &self.players {
            player.draw(controller, window, camera);
        }
    }

    pub fn draw_ui(&mut self, controller: &mut RenderController, window: &Window, camera: &Camera) {
        for player in &self.players {
            player.draw_ui(controller, window, camera);
        }
    }
}