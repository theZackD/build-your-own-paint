use crate::canvas::Canvas;
use crate::vec2::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub color: u32,
}

impl Line {
    pub fn from_points(start: Vec2, end: Vec2, color: u32) -> Line {
        Line { start, end, color }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw_line(
            self.start.x as i32,
            self.start.y as i32,
            self.end.x as i32,
            self.end.y as i32,
            self.color,
        );
    }
}
