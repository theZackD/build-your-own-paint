use crate::canvas::Canvas;
use crate::color::{BLACK, DARK_GRAY, LIGHT_GRAY, WHITE};
use crate::shapes::{Rect, Shape, Tool};
use crate::vec2::Vec2;

const BUTTON_SIZE: f32 = 40.0;
const TOP: f32 = 20.0;

/// The buttons in the top-right corner: one per tool, plus the color button.
pub struct Toolbar {
    pub tool_buttons: [(Tool, Rect); 3],
    pub color_button: Rect,
}

impl Toolbar {
    pub fn new(canvas_width: f32) -> Toolbar {
        let button = |x: f32| Rect::new(x, TOP, BUTTON_SIZE, BUTTON_SIZE, WHITE);
        Toolbar {
            tool_buttons: [
                (Tool::Rect, button(canvas_width - 230.0)),
                (Tool::Circle, button(canvas_width - 180.0)),
                (Tool::Line, button(canvas_width - 130.0)),
            ],
            color_button: button(canvas_width - 60.0),
        }
    }

    /// The tool whose button is under `p`, if any.
    pub fn tool_at(&self, p: Vec2) -> Option<Tool> {
        for (tool, button) in &self.tool_buttons {
            if button.contains(p) {
                return Some(*tool);
            }
        }
        None
    }

    pub fn draw(&self, canvas: &mut Canvas, active_tool: Tool, current_color: u32) {
        for (tool, button) in &self.tool_buttons {
            if *tool == active_tool {
                button.draw_outline(canvas, 6.0, BLACK);
            } else {
                button.draw_outline(canvas, 3.0, LIGHT_GRAY);
            }
            button.draw(canvas);
            tool_icon(*tool, button).draw(canvas);
        }

        self.color_button.draw_outline(canvas, 6.0, BLACK);
        self.color_button.draw_outline(canvas, 3.0, LIGHT_GRAY);
        Rect {
            color: current_color,
            ..self.color_button
        }
        .draw(canvas);
    }
}

/// The icon of a tool is simply a small shape drawn with that tool.
fn tool_icon(tool: Tool, button: &Rect) -> Shape {
    let (a, b) = match tool {
        Tool::Rect => (Vec2::new(8.0, 12.0), Vec2::new(32.0, 28.0)),
        Tool::Circle => (Vec2::new(20.0, 20.0), Vec2::new(32.0, 20.0)),
        Tool::Line => (Vec2::new(8.0, 32.0), Vec2::new(32.0, 8.0)),
    };
    Shape::from_points(tool, button.pos + a, button.pos + b, DARK_GRAY)
}
