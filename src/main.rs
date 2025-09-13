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
mod rendering;
mod ui;
mod uistyles;
mod gamesettings;
mod ingredients;

use std::{env, fs, thread};
use std::fs::{File, OpenOptions};
use crate::gameloop::GameHandler;
use log::{debug, info, warn, LevelFilter};
use mvengine::net::client::ClientHandler;
use mvengine::utils::Expect2;
use mvengine::window::{Window, WindowCreateInfo};
use mvutils::save::Savable;
use std::io::stdout;
use std::path::PathBuf;
use mvutils::utils::Time;
use api::server::{startup_internal_server, ServerSync};

fn get_logs_path() -> Option<PathBuf> {
    if let Ok(appdata) = env::var("APPDATA") {
        let path = PathBuf::from(appdata)
            .join(".factoryisland")
            .join("logs");
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

    let args = env::args();
    let mut server = false;
    for arg in args {
        debug!("arg: {arg}");
        match arg.as_str() {
            "-internalserver" => { server = true }
            other => warn!("Unrecognized argument: {other}")
        }
    }

    let mut sync = None;
    if server {
        let sync1 = ServerSync::new();
        let cloned = sync1.clone();
        thread::spawn(|| {
            info!("Starting internal server...");
            startup_internal_server(false, cloned);
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