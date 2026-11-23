mod canvas;
mod color;
mod image;
#[allow(dead_code)] // Vec2 comes back in chapter 5.
mod vec2;

use canvas::Canvas;
use color::rgb_to_u32;
use image::Image;

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::new(400, 300);
    canvas.clear(rgb_to_u32(240, 240, 240));

    canvas.fill_rect(30, 30, 120, 80, rgb_to_u32(220, 50, 50));
    canvas.fill_circle(260, 90, 50, rgb_to_u32(50, 120, 220));
    canvas.draw_line(20, 280, 380, 160, rgb_to_u32(0, 0, 0));

    // Shapes can stick out of the canvas: hidden pixels are simply ignored.
    canvas.fill_circle(380, 280, 60, rgb_to_u32(50, 180, 80));

    Image::from_canvas(&canvas).save_ppm("out.ppm")?;
    println!("saved to out.ppm");
    Ok(())
}
