#![feature(path_file_prefix)]
#![feature(iter_next_chunk)]
#![feature(exact_size_is_empty)]

mod camera;
mod debug;
mod drawutils;
mod game;
mod gameloop;
mod gamesettings;
mod ingredients;
mod input;
mod player;
mod rendering;
mod res;
mod ui;
mod uistyles;
mod world;

use crate::gameloop::GameHandler;
use api::server::{startup_internal_server, ServerSync};
use log::{debug, info, warn, LevelFilter};
use mvengine::net::client::ClientHandler;
use mvengine::utils::Expect2;
use mvengine::window::{Window, WindowCreateInfo};
use mvutils::save::Savable;
use mvutils::utils::Time;
use std::fs::{File, OpenOptions};
use std::io::stdout;
use std::path::PathBuf;
use std::{env, fs, thread};

fn get_logs_path() -> Option<PathBuf> {
    if let Ok(appdata) = env::var("APPDATA") {
        let path = PathBuf::from(appdata).join(".factoryisland").join("logs");
        Some(path)
    } else {
        None
    }
}

fn main() {
    //let mut logpath = get_logs_path().unwrap();
    //fs::create_dir_all(&logpath);
    //logpath.push(format!("{}.log", u128::time_millis()));
    //let file = OpenOptions::new().write(true).create(true).truncate(true).open(&logpath).unwrap();

    let mut args = env::args();
    let mut server = false;
    let mut world = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-internalserver" => server = true,
            "-world" => {
                if let Some(w) = args.next() {
                    world = Some(w);
                } else {
                    warn!("Expected value after -world, but none was given");
                }
            }
            other => warn!("Unrecognized argument: {other}"),
        }
    }

    let mut sync = None;
    if server {
        let sync1 = ServerSync::new();
        let cloned = sync1.clone();
        thread::spawn(|| {
            info!("Starting internal server...");
            startup_internal_server(false, cloned, world);
        });
        sync = Some(sync1);
    }

    mvlogger::init(stdout(), LevelFilter::Debug);
    let handler = GameHandler::new(server, sync);
    let mut info = WindowCreateInfo::default();
    info.vsync = false;
    info.fps = 60;
    info.title = "FactoryIsland".to_string();
    let window = Window::new(info);
    window.run(handler).expect2("Cannot start window for game!");
}
