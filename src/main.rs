mod canvas;
mod color;
mod config;
mod image;
#[allow(dead_code)] // Vec2 comes back in chapter 5.
mod vec2;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

use canvas::Canvas;
use color::{WHITE, rgb_to_u32};
use config::{HEIGHT, WIDTH};
use image::Image;

fn main() {
    let mut window = Window::new("Paint", WIDTH, HEIGHT, WindowOptions::default())
        .expect("could not open the window");

    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Each frame, we clear the canvas and draw everything again.
        canvas.clear(WHITE);
        canvas.fill_rect(100, 100, 240, 160, rgb_to_u32(220, 50, 50));
        canvas.fill_circle(600, 250, 120, rgb_to_u32(50, 120, 220));
        canvas.draw_line(50, 650, 1030, 400, rgb_to_u32(0, 0, 0));

        if window.is_key_pressed(Key::S, KeyRepeat::No) {
            match Image::from_canvas(&canvas).save_ppm("screenshot.ppm") {
                Ok(()) => println!("saved to screenshot.ppm"),
                Err(e) => eprintln!("could not save the image: {e}"),
            }
        }

        window
            .update_with_buffer(&canvas.pixels, WIDTH, HEIGHT)
            .expect("could not update the window");
    }
}
