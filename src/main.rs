mod action;
mod app;
mod canvas;
mod color;
mod config;
mod image;
mod shapes;
mod ui;
mod vec2;

use minifb::{Key, Window, WindowOptions};

use app::App;
use config::{HEIGHT, WIDTH};

fn main() {
    let mut window = Window::new("Paint", WIDTH, HEIGHT, WindowOptions::default())
        .expect("could not open the window");

    let mut app = App::new(WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        app.update(&window);
        app.render();
        window
            .update_with_buffer(&app.canvas().pixels, WIDTH, HEIGHT)
            .expect("could not update the window");
    }
}
