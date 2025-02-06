mod game;
mod res;

use crate::game::gameloop::GameLoop;
use log::LevelFilter;
use mvengine::game::ecs::{EcsStorage, ECS};
use mvengine::game::ecs::entity::{Entity, EntityBehavior, EntityType, LocalComponent};
use mvengine::game::ecs::system::System;
use mvengine::window::{Window, WindowCreateInfo};

pub static mut WINDOW_SIZE: (i32, i32) = (1200, 800);

struct Transform {
    pos: (i32, i32)
}

struct BlockHealth {
    health: f32,
}

struct PlayerBehavior {
    health: LocalComponent<BlockHealth>
}

impl EntityBehavior for PlayerBehavior {
    fn new(storage: EcsStorage) -> Self
    where
        Self: Sized
    {
        Self {
            health: LocalComponent::new(storage),
        }
    }

    fn start(&mut self, entity: EntityType) {
        self.health.aquire(entity);
    }

    fn update(&mut self, entity: EntityType) {
        let health = self.health.health;
    }
}

fn main() {
    unsafe {
        mvlogger::init(std::io::stdout(), LevelFilter::Trace);
        let mut info = WindowCreateInfo::default();
        info.width = WINDOW_SIZE.0 as u32;
        info.height = WINDOW_SIZE.1 as u32;
        info.title = "FactoryIsland".to_string();
        info.fps = 60;
        info.ups = 20;
        info.vsync = true;
        //info.fullscreen = true;

        let mut ecs = ECS::new();
        ecs.world().create_entity::<PlayerBehavior, (BlockHealth)>(Entity::new);

        ecs.world_mut().update();

        let mut damage_fog = System::<(BlockHealth,)>::new(ecs.storage());
        for (entity, health) in damage_fog.iter_mut() {
            health.health -= 1.0;
        }

        let window = Window::new(info);
        window.run::<GameLoop>().expect("Idk mve failed ig");
    }
}
