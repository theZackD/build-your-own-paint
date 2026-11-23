use crate::color::WHITE;

/// A grid of pixels we can draw on, displayed in the window.
///
/// Drawing functions take `i32` coordinates so that shapes can stick out of
/// the canvas (negative coordinates): pixels outside are simply ignored.
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![WHITE; width * height],
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= self.width || y >= self.height {
            return;
        }
        self.pixels[x + y * self.width] = color;
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        for j in y..y + h {
            for i in x..x + w {
                self.put_pixel(i, j, color);
            }
        }
    }

    pub fn fill_circle(&mut self, cx: i32, cy: i32, r: i32, color: u32) {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    self.put_pixel(cx + dx, cy + dy, color);
                }
            }
        }
    }

    /// Draws a line by stepping along its longest axis, one pixel per step.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let steps = dx.abs().max(dy.abs());

        if steps == 0 {
            self.put_pixel(x0, y0, color);
            return;
        }

        for k in 0..=steps {
            let x = x0 + dx * k / steps;
            let y = y0 + dy * k / steps;
            self.put_pixel(x, y, color);
        }
    }
}
