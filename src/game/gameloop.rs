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
use crate::game::events::{ChunkLoadEvent, LmaoEnum, LmaoEnumDispatcher, LmaoEnumHandler};

pub struct GameLoop {
    world_screen: CreateOnce<Arc<Mutex<WorldScreen>>>,
    events: LmaoEnumDispatcher,
}

impl WindowCallbacks for GameLoop {
    fn new(window: UninitializedWindow) -> Self {
        Self {
            world_screen: CreateOnce::new(),
            events: LmaoEnumDispatcher::new(),
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

        registry.create_action("fullscreen");
        registry.bind_action("fullscreen", vec![RawInput::KeyPress(Key::F11)]);

        let world = World::create("helloseed");
        let screen = WorldScreen::new(window, world, &mut self.events);

        let arc = Arc::new(Mutex::new(screen));

        window.input.register_new_event_target(arc.clone());

        self.world_screen.create(|| arc);

        // self.events.add_event_handler(Box::new(|event, mut handle| {
        //     if let LmaoEnum::ChunkLoad(x, z) = event {
        //         println!("Loaded Chunk {x}, {z}!, But stopped it now!");
        //         handle.pause();
        //     }
        // }))
    }

    fn update(&mut self, window: &mut Window, delta_t: f64) {

    }

    fn draw(&mut self, window: &mut Window, delta_t: f64) {
        unsafe {
            for event in self.events.poll() {
                match event {
                    LmaoEnum::ChunkLoad(event) => {
                        if event.cancelled { continue; }
                        let mut lock = self.world_screen.lock();
                        lock.load_chunk(event.x, event.z);
                    }
                }
            }

            let mut lock = self.world_screen.lock();
            lock.draw(window);
            drop(lock);

            TIMING_MANAGER.post_frame(delta_t as f32, 0);
        }

        if window.input.was_action("fullscreen") {
            window.toggle_fullscreen();
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