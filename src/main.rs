mod image;
#[allow(dead_code)] // Vec2 comes back in chapter 5.
mod vec2;

use image::Image;

fn main() -> std::io::Result<()> {
    let mut img = Image::new(300, 200);

    // A horizontal gradient, from black to white.
    for y in 0..img.height {
        for x in 0..img.width {
            let shade = (x * 255 / img.width) as u8;
            img.set_pixel(x, y, shade, shade, shade);
        }
    }

    // A red square on top.
    img.fill_rect(100, 50, 100, 100, (255, 0, 0));

    img.save_ppm("out.ppm")?;
    println!("saved to out.ppm");
    Ok(())
}
