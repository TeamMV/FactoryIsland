use crate::camera::Camera;
use crate::drawutils;
use crate::gameloop::FactoryIslandClient;
use crate::res::R;
use api::server::packets::player::PlayerMovePacket;
use api::server::ServerBoundPacket;
use api::world::{resolve_unit, TileUnit};
use mvengine::graphics::comp::Drawable;
use mvengine::rendering::RenderContext;
use mvengine::ui::geometry::SimpleRect;
use mvengine::ui::rendering::adaptive::AdaptiveFill;
use api::server::packets::common::ClientDataPacket;

pub const CAMERA_PADDING: TileUnit = (3.0, 3.0);

pub struct ClientPlayer {
    pos: TileUnit,
    pub(crate) camera: Camera,
    pub data: ClientDataPacket
}

impl ClientPlayer {
    pub fn new(view_width: i32, view_height: i32, data: ClientDataPacket) -> Self {
        Self {
            pos: (0.0, 0.0),
            camera: Camera::new(view_width, view_height),
            data,
        }
    }

    pub fn resize_view(&mut self, width: u32, height: u32) {
        self.camera.width = width as i32;
        self.camera.height = height as i32;
        self.camera.update();
    }

    pub fn move_by(&mut self, by: TileUnit, tile_size: i32) {
        self.pos.0 += by.0;
        self.pos.1 += by.1;
        self.update_cam(tile_size);
    }

    pub fn move_to(&mut self, to: TileUnit, tile_size: i32) {
        self.pos = to;
        self.update_cam(tile_size);
    }

    pub fn broadcast_position(&self, client: &mut FactoryIslandClient) {
        client.send(ServerBoundPacket::PlayerMove(PlayerMovePacket {
            pos: self.pos,
        }));
    }

    fn update_cam(&mut self, tile_size: i32) {
        let (padding_x, padding_y) = resolve_unit(CAMERA_PADDING, tile_size);
        let (mut player_x, mut player_y) = resolve_unit(self.pos, tile_size);
        player_x -= self.camera.pos.0;
        player_y -= self.camera.pos.1;
        let mut has_changed = false;
        if player_x < padding_x {
            let diff = padding_x - player_x;
            self.camera.pos.0 -= diff;
            has_changed = true;
        } else if player_x > self.camera.width - padding_x - tile_size {
            let diff = player_x - self.camera.width + padding_x + tile_size;
            self.camera.pos.0 += diff;
            has_changed = true;
        }
        if player_y < padding_y {
            let diff = padding_y - player_y;
            self.camera.pos.1 -= diff;
            has_changed = true;
        } else if player_y > self.camera.height - padding_y - tile_size {
            let diff = player_y - self.camera.height + padding_y + tile_size;
            self.camera.pos.1 += diff;
            has_changed = true;
        }
        if has_changed {
            self.camera.update();
        }
    }


    pub fn draw(&self, ctx: &mut impl RenderContext, tile_size: i32) {
        let fill = AdaptiveFill::Drawable(Drawable::Texture(R.texture.player));
        drawutils::draw_in_world(ctx, &self.camera.view_area, self.pos, (1.0, 1.0), fill, tile_size, 1.0);
    }

    pub fn draw_from_other_pov(&self, ctx: &mut impl RenderContext, view_area: &SimpleRect, tile_size: i32) {
        let fill = AdaptiveFill::Drawable(Drawable::Texture(R.texture.player));
        drawutils::draw_in_world(ctx, view_area, self.pos, (1.0, 1.0), fill, tile_size, 1.0);
    }
}