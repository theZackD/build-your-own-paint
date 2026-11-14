use std::fs::File;
use std::io::Write;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        let pixels = vec![0; w * h * 3];
        Image {
            width: w,
            height: h,
            pixels,
        }
    }
}

impl Image {
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height {
            return;
        }

        let i = (x + y * self.width) * 3;
        self.pixels[i] = r;
        self.pixels[i + 1] = g;
        self.pixels[i + 2] = b;
    }
}

pub fn fill_rectangle(
    img: &mut Image,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    r: u8,
    g: u8,
    b: u8,
) {
    for j in 0..h {
        for i in 0..w {
            img.set_pixel(x + i, y + j, r, g, b);
        }
    }
}

pub fn save_ppm(img: &Image, path: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;

    write!(f, "P6\n{} {}\n255\n", img.width, img.height)?;
    f.write_all(&img.pixels)?;

    Ok(())
}
