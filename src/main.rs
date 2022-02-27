extern crate raw_window_handle;
extern crate windows;

mod app;
mod render;
mod utils;
mod window;

use crate::app::Application;
use crate::render::State;
use crate::window::Window;
use app::Event;
use windows::core::Result;

const WINDOW_CLASS: &str = "default_window";

struct App {
    window: Window,
    state: State,
}

impl App {
    async fn new() -> Result<Self> {
        let window = *Window::new("my window", WINDOW_CLASS)?;
        let app = Self {
            state: State::new(&window).await,
            window,
        };

        Ok(app)
    }
}

impl Application for App {
    fn get_window(&self) -> &Window {
        &self.window
    }

    fn event_handler(&mut self, event: Event) {
        match event {
            Event::KeyDown => {
                println!("Keydown event");
            }
            Event::Paint => {
                self.state.render().unwrap();
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    // let window = Window::new("my window", WINDOW_CLASS)?;
    // let state = State::new(&window);
    // window.start();
    let mut app = pollster::block_on(App::new())?;
    app.start();
    Ok(())
}
