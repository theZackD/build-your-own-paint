use crate::canvas::Canvas;
use crate::vec2::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
    pub color: u32,
}

impl Circle {
    /// Builds the circle centered on `center` that goes through `edge`.
    pub fn from_points(center: Vec2, edge: Vec2, color: u32) -> Circle {
        Circle {
            center,
            radius: (edge - center).length(),
            color,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.fill_circle(
            self.center.x as usize,
            self.center.y as usize,
            self.radius as usize,
            self.color,
        );
    }
}
