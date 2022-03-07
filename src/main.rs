extern crate raw_window_handle;
extern crate windows;

mod window;
mod utils;
// mod render;

// use crate::render::State;
use crate::window::Window;
use windows::core::Result;


fn main() -> Result<()> {
    let mut window = Window::new("win title", "window class 01012")?;
    // let state = State::new(&window);

    window.start();
    Ok(())
}
