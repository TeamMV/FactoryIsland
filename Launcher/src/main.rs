use std::sync::Arc;
use mvengine::window::{Window, WindowCreateInfo};
use parking_lot::RwLock;
use crate::app::LauncherApp;

mod app;
mod res;
mod uistyles;
mod err;

fn main() {
    let mut info = WindowCreateInfo::default();
    info.title = "FactoryIsland Launcher".to_string();
    info.vsync = true;

    let handler = LauncherApp::new();
    let handler = Arc::new(RwLock::new(handler));

    let window = Window::new(info);
    if let Err(e) = window.run(handler) {
        println!("Error: {e:?}");
    }
}
