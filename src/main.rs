mod game;
mod res;

use crate::game::gameloop::GameLoop;
use log::LevelFilter;
use mvengine::window::{Window, WindowCreateInfo};

pub static mut WINDOW_SIZE: (i32, i32) = (1200, 800);

struct Transform {
    pos: (i32, i32)
}

struct BlockHealth {
    health: f32,
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
        info.fullscreen = true;

        let window = Window::new(info);
        window.run::<GameLoop>().expect("Idk mve failed ig");
    }
}
