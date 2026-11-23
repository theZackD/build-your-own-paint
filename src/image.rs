use std::fs::File;
use std::io::{self, Write};

use crate::canvas::Canvas;
use crate::color::u32_to_rgb;

/// An RGB image, 3 bytes per pixel, that can be saved as a PPM file.
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![0; width * height * 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let i = (x + y * self.width) * 3;
        self.pixels[i] = r;
        self.pixels[i + 1] = g;
        self.pixels[i + 2] = b;
    }

    pub fn from_canvas(canvas: &Canvas) -> Image {
        let mut img = Image::new(canvas.width, canvas.height);
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                let (r, g, b) = u32_to_rgb(canvas.pixels[x + y * canvas.width]);
                img.set_pixel(x, y, r, g, b);
            }
        }
        img
    }

    /// Writes the image in the binary PPM format (P6): a small text header
    /// followed by the raw RGB bytes.
    pub fn save_ppm(&self, path: &str) -> io::Result<()> {
        let mut f = File::create(path)?;
        write!(f, "P6\n{} {}\n255\n", self.width, self.height)?;
        f.write_all(&self.pixels)?;
        Ok(())
    }
}
