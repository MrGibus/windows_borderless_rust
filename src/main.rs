#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate raw_window_handle;
extern crate windows;

mod application;
mod input;
mod render;
mod utils;
mod winapi_utils;
mod window;

use crate::render::Engine;
use crate::window::Window;
use windows::core::Result;

fn main() -> Result<()> {
    let mut window = Window::new("win title", "window class 01012")?;
    let mut state = pollster::block_on(Engine::new(&window));

    window.set_engine(&mut state);

    window.start();
    Ok(())
}
