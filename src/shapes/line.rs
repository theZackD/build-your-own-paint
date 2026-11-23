use crate::canvas::Canvas;
use crate::vec2::Vec2;

/// How close (in pixels) the mouse must be to a line to select it.
const HIT_DISTANCE: f32 = 5.0;

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

    /// True if `p` is close to the segment: we project `p` on the segment,
    /// keep the projection between both ends, and measure the distance.
    pub fn contains(&self, p: Vec2) -> bool {
        let u = self.end - self.start;
        let v = p - self.start;
        let uu = u.dot(u);
        let t = if uu == 0.0 {
            0.0
        } else {
            (v.dot(u) / uu).clamp(0.0, 1.0)
        };
        let closest = self.start + u * t;
        (p - closest).length() <= HIT_DISTANCE
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
