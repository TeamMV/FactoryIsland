use mvengine::color::RgbColor;
use crate::drawutils;
use crate::drawutils::Fill;
use crate::game::worldview::WorldView;
use crate::world::tiles::impls::CLIENT_TILE_REG;
use api::world::tiles::pos::TilePos;
use mvengine::rendering::RenderContext;
use mvengine::ui::context::UiResources;
use mvengine::ui::geometry::shape::shapes;
use mvengine::window::Window;
use api::world::{resolve_unit, SingleTileUnit};
use crate::gamesettings::GameSettings;
use crate::res::R;

pub fn draw_overlay(view: &mut WorldView, window: &Window, settings: &GameSettings) {
    let pipeline = &mut view.overlay_pipeline;
    let player = &view.player;
    let tile_size = view.tile_size;
    let orientation = view.orientation;

    if let Some(sel) = view.tile_selection.selected_tile() {
        view.player_pipeline.next_pipeline(pipeline);


        if let Some(tile) = CLIENT_TILE_REG.reference_object(sel.id.saturating_sub(1)) {
            let mx = window.input.mouse_x;
            let my = window.input.mouse_y;
            let pos = TilePos::from_screen((mx, my), &player.camera.view_area, tile_size);

            if *settings.indicator_circle.read() {
                let (px, py) = drawutils::get_screen_pos(&player.camera.view_area, player.pos(), view.tile_size);
                let reach = (player.reach * view.tile_size as SingleTileUnit) as i32;
                let circle = shapes::circle0(px, py, reach, 30);
                circle.draw(pipeline, |v| {
                    v.color = RgbColor::green().alpha(100).as_vec4();
                });
            }

            if pos.distance_from(player) <= player.reach {
                let y = pipeline.controller().next_z();
                drawutils::draw_in_world_tile(pipeline, &player.camera.view_area, pos, Fill::Drawable(tile.base.clone(), orientation), tile_size, y);
            }
        }

        view.overlay_pipeline.advance(window, |_| {});
        view.overlay_pipeline.advance(window, |s| {
            s.uniform_1f("FRAME", view.frame as f32);
            let noise_tex = R.resolve_texture(R.texture.noise).expect("It exists bro");
            s.uniform_texture(noise_tex, "NOISE");
        });
    }
}