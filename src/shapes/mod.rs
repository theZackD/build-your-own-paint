mod circle;
mod line;
mod rect;

pub use circle::Circle;
pub use line::Line;
pub use rect::Rect;

use crate::canvas::Canvas;
use crate::vec2::Vec2;

/// The drawing tool currently selected.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    Rect,
    Circle,
    Line,
}

/// A shape of the drawing.
#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
    Line(Line),
}

impl Shape {
    /// Builds the shape drawn with `tool` by dragging the mouse from `a` to `b`.
    pub fn from_points(tool: Tool, a: Vec2, b: Vec2, color: u32) -> Shape {
        match tool {
            Tool::Rect => Shape::Rect(Rect::from_points(a, b, color)),
            Tool::Circle => Shape::Circle(Circle::from_points(a, b, color)),
            Tool::Line => Shape::Line(Line::from_points(a, b, color)),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        match self {
            Shape::Rect(r) => r.draw(canvas),
            Shape::Circle(c) => c.draw(canvas),
            Shape::Line(l) => l.draw(canvas),
        }
    }
}
