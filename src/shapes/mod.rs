mod circle;
mod line;
mod rect;

pub use circle::Circle;
pub use line::Line;
pub use rect::Rect;

use crate::canvas::Canvas;
use crate::color::SELECTION_BLUE;
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

    pub fn contains(&self, p: Vec2) -> bool {
        match self {
            Shape::Rect(r) => r.contains(p),
            Shape::Circle(c) => c.contains(p),
            Shape::Line(l) => l.contains(p),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        match self {
            Shape::Rect(r) => r.draw(canvas),
            Shape::Circle(c) => c.draw(canvas),
            Shape::Line(l) => l.draw(canvas),
        }
    }

    /// Draws a blue highlight around the shape. Call it before `draw`.
    pub fn draw_selection(&self, canvas: &mut Canvas) {
        match self {
            Shape::Rect(r) => r.draw_outline(canvas, 3.0, SELECTION_BLUE),
            Shape::Circle(c) => Circle {
                radius: c.radius + 3.0,
                color: SELECTION_BLUE,
                ..*c
            }
            .draw(canvas),
            Shape::Line(l) => {
                // A thick line: the same line shifted by one pixel around.
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let offset = Vec2::new(dx as f32, dy as f32);
                        Line::from_points(l.start + offset, l.end + offset, SELECTION_BLUE)
                            .draw(canvas);
                    }
                }
                for end in [l.start, l.end] {
                    canvas.fill_circle(end.x as usize, end.y as usize, 5, SELECTION_BLUE);
                }
            }
        }
    }

    /// Moves the shape by `delta`.
    pub fn translate(&mut self, delta: Vec2) {
        match self {
            Shape::Rect(r) => r.pos = r.pos + delta,
            Shape::Circle(c) => c.center = c.center + delta,
            Shape::Line(l) => {
                l.start = l.start + delta;
                l.end = l.end + delta;
            }
        }
    }
}

/// Index of the topmost shape under `p`. Shapes drawn last are on top,
/// so we search from the end of the list.
pub fn shape_under(shapes: &[Shape], p: Vec2) -> Option<usize> {
    for (i, shape) in shapes.iter().enumerate().rev() {
        if shape.contains(p) {
            return Some(i);
        }
    }
    None
}
