use log::debug;
use crate::camera::Camera;
use crate::drawutils;
use crate::gameloop::FactoryIslandClient;
use crate::res::R;
use api::server::packets::common::ClientDataPacket;
use api::server::packets::player::{PlayerDataPacket, PlayerMovePacket};
use api::server::ServerBoundPacket;
use api::world::{resolve_unit, SingleTileUnit, TileUnit};
use mvengine::graphics::Drawable;
use mvengine::math::vec::Vec2;
use mvengine::rendering::RenderContext;
use mvengine::ui::geometry::{geom, SimpleRect};
use mvengine::ui::rendering::WideRenderContext;
use api::player::profile::PlayerProfile;
use api::world::tiles::Orientation;
use api::world::tiles::pos::{TileDistance, TilePos};
use crate::drawutils::Fill;

pub const PADDING_FACTOR: i32 = 4;

pub struct ClientPlayer {
    pos: TileUnit,
    pub(crate) camera: Camera,
    pub data: ClientDataPacket,
    pub speed: f64,
    pub reach: SingleTileUnit,
}

impl ClientPlayer {
    pub fn new(view_width: i32, view_height: i32, data: ClientDataPacket) -> Self {
        Self {
            pos: (0.0, 0.0),
            camera: Camera::new(view_width, view_height),
            data,
            speed: 20.0,
            reach: 7.0,
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
        let (padding_x, padding_y) = (self.camera.view_area.width / PADDING_FACTOR, self.camera.view_area.height / PADDING_FACTOR);
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

    pub fn data_packet(&mut self, packet: PlayerDataPacket, tile_size: i32) {
        self.reach = packet.reach;
        self.pos = packet.pos;
        self.update_cam(tile_size)
    }

    pub fn draw(&self, ctx: &mut impl WideRenderContext, tile_size: i32) {
        let fill = Fill::Drawable(Drawable::Texture(R.texture.player), Orientation::North);
        let z = ctx.next_z();
        drawutils::draw_in_world(ctx, &self.camera.view_area, self.pos, (1.0, 1.0), fill, tile_size, z);
    }

    pub fn draw_from_other_pov(&self, ctx: &mut impl WideRenderContext, view_area: &SimpleRect, tile_size: i32) {
        let fill = Fill::Drawable(Drawable::Texture(R.texture.player), Orientation::North);
        let z = ctx.next_z();
        drawutils::draw_in_world(ctx, view_area, self.pos, (1.0, 1.0), fill, tile_size, z);
    }

    pub fn pos(&self) -> TileUnit {
        self.pos
    }

    pub fn profile(&self) -> &PlayerProfile {
        &self.data.profile
    }
}

impl TileDistance for ClientPlayer {
    fn distance(&self, from: &TilePos) -> SingleTileUnit {
        let v1 = Vec2::new(self.pos.0 as f32, self.pos.1 as f32);
        let v2 = Vec2::from_i32s(from.raw);
        geom::distance(v1, v2) as SingleTileUnit
    }
}