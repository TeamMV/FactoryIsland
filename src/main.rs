#![feature(path_file_prefix)]
#![feature(iter_next_chunk)]
#![feature(exact_size_is_empty)]

mod world;
mod game;
mod debug;
mod gameloop;
mod camera;
mod res;
mod input;
mod player;
mod drawutils;
mod mods;
mod rendering;
mod ui;
mod uistyles;

use crate::gameloop::GameHandler;
use log::LevelFilter;
use mvengine::net::client::ClientHandler;
use mvengine::utils::Expect2;
use mvengine::window::{Window, WindowCreateInfo};
use mvutils::save::Savable;
use std::io::stdout;

fn main() {    
    mvlogger::init(stdout(), LevelFilter::Trace);
    let handler = GameHandler::new();
    let mut info = WindowCreateInfo::default();
    info.vsync = true;
    info.title = "FactoryIsland".to_string();
    let window = Window::new(info);
    window.run(handler).expect2("Cannot start window for game!");
    
    let a = [0; 5];
}