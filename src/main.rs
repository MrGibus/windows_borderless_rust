extern crate raw_window_handle;
extern crate windows;

mod render;
mod utils;
mod window;

use crate::render::State;
use crate::window::Window;
use windows::core::Result;
use window::Event;
use window::Application;
use std::thread::sleep;
use std::time::Duration;

const WINDOW_CLASS: &str = "default_window";

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl Application for App {
}

fn main() -> Result<()> {
    let mut app = App::new();
    let mut window = Window::new("my App", WINDOW_CLASS, app)?;
    let mut state = pollster::block_on(State::new(&window));
    window.start();
    Ok(())
}
