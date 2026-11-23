use crate::canvas::Canvas;
use crate::vec2::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// Top-left corner.
    pub pos: Vec2,
    pub size: Vec2,
    pub color: u32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: u32) -> Rect {
        Rect {
            pos: Vec2::new(x, y),
            size: Vec2::new(w, h),
            color,
        }
    }

    /// Builds the rectangle between two opposite corners, in any order.
    pub fn from_points(a: Vec2, b: Vec2, color: u32) -> Rect {
        Rect::new(
            a.x.min(b.x),
            a.y.min(b.y),
            (a.x - b.x).abs(),
            (a.y - b.y).abs(),
            color,
        )
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.pos.x
            && p.x < self.pos.x + self.size.x
            && p.y >= self.pos.y
            && p.y < self.pos.y + self.size.y
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.fill_rect(
            self.pos.x as usize,
            self.pos.y as usize,
            self.size.x as usize,
            self.size.y as usize,
            self.color,
        );
    }

    /// Draws a border around the rectangle by filling a slightly bigger
    /// rectangle. Call it before `draw` so the rectangle covers the middle.
    pub fn draw_outline(&self, canvas: &mut Canvas, thickness: f32, color: u32) {
        let t = Vec2::new(thickness, thickness);
        Rect {
            pos: self.pos - t,
            size: self.size + t * 2.0,
            color,
        }
        .draw(canvas);
    }
}
