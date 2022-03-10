extern crate raw_window_handle;
extern crate windows;

mod application;
mod render;
mod utils;
mod window;
mod input;

use crate::render::State;
use crate::window::Window;
use windows::core::Result;

fn main() -> Result<()> {
    let mut window = Window::new("win title", "window class 01012")?;
    let mut state = pollster::block_on(State::new(&window));

    window.set_state(&mut state);

    window.start();
    Ok(())
}
