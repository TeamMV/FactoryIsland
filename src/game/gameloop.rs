use crate::game::screens::world::WorldScreen;
use crate::game::world::chunk::{TilePos, CHUNK_SIZE};
use crate::game::world::tiles::Tile;
use crate::game::world::World;
use crate::res::R;
use crate::WINDOW_SIZE;
use mvengine::input::consts::{Key, MouseButton};
use mvengine::input::registry::{Direction, RawInput};
use mvengine::input::{Input, MouseAction, RawInputEvent};
use mvengine::ui::timing::TIMING_MANAGER;
use mvengine::window::app::WindowCallbacks;
use mvengine::window::{UninitializedWindow, Window};
use mvutils::once::CreateOnce;
use std::ops::Deref;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::game::event::EventDispatcher;

pub struct GameLoop {
    world_screen: CreateOnce<Arc<Mutex<WorldScreen>>>,
    event_dispatcher: EventDispatcher
}

impl WindowCallbacks for GameLoop {
    fn new(window: UninitializedWindow) -> Self {
        Self {
            world_screen: CreateOnce::new(),
            event_dispatcher: EventDispatcher::new(Box::new(|_| {})),
        }
    }

    fn post_init(&mut self, window: &mut Window) {
        R::initialize();
        let ui = &mut window.ui;;
        ui.init(R.deref().deref());

        let registry = window.input.action_registry_mut();
        registry.create_action("move_up");
        registry.create_action("move_down");
        registry.create_action("move_left");
        registry.create_action("move_right");

        registry.bind_action("move_up", vec![RawInput::KeyPress(Key::W)]);
        registry.bind_action("move_down", vec![RawInput::KeyPress(Key::S)]);
        registry.bind_action("move_left", vec![RawInput::KeyPress(Key::A)]);
        registry.bind_action("move_right", vec![RawInput::KeyPress(Key::D)]);

        let world = World::create("helloseed");
        let screen = WorldScreen::new(window, world);

        let arc = Arc::new(Mutex::new(screen));

        window.input.register_new_event_target(arc.clone());

        self.world_screen.create(|| arc);
    }

    fn update(&mut self, window: &mut Window, delta_t: f64) {

    }

    fn draw(&mut self, window: &mut Window, delta_t: f64) {
        unsafe {
            let mut lock = self.world_screen.lock();
            lock.draw(window, &mut self.event_dispatcher);
            drop(lock);

            TIMING_MANAGER.post_frame(delta_t as f32, 0);
        }
    }

    fn exiting(&mut self, window: &mut Window) {

    }

    fn resize(&mut self, window: &mut Window, width: u32, height: u32) {
        unsafe { WINDOW_SIZE = (width as i32, height as i32); }
        let mut lock = self.world_screen.lock();
        lock.resize(window);
    }
}