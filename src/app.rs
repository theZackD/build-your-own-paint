use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window};

use crate::action::Action;
use crate::canvas::Canvas;
use crate::color::{GRAY, WHITE, rgb_to_u32};
use crate::shapes::{Shape, Tool, shape_under};
use crate::vec2::Vec2;

/// All the state of the application.
pub struct App {
    canvas: Canvas,
    shapes: Vec<Shape>,
    tool: Tool,
    action: Action,
    /// Index of the selected shape in `shapes`. We store an index and not a
    /// reference: a reference would borrow `shapes` for as long as the
    /// selection exists, and we could no longer modify the list.
    selected: Option<usize>,
    color: u32,
    mouse: Vec2,
    was_down: bool,
}

impl App {
    pub fn new(width: usize, height: usize) -> App {
        App {
            canvas: Canvas::new(width, height),
            shapes: Vec::new(),
            tool: Tool::Rect,
            action: Action::Idle,
            selected: None,
            color: rgb_to_u32(50, 120, 220),
            mouse: Vec2::ZERO,
            was_down: false,
        }
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    /// Reads the keyboard and mouse, and updates the state.
    pub fn update(&mut self, window: &Window) {
        self.mouse = window
            .get_mouse_pos(MouseMode::Clamp)
            .map(Vec2::from)
            .unwrap_or(Vec2::ZERO);
        let down = window.get_mouse_down(MouseButton::Left);

        self.handle_keys(window, down);

        // Compare with the previous frame to detect press and release.
        if down && !self.was_down {
            self.on_press(self.mouse);
        }
        if down {
            self.on_drag(self.mouse);
        }
        if !down && self.was_down {
            self.on_release(self.mouse);
        }
        self.was_down = down;
    }

    fn handle_keys(&mut self, window: &Window, mouse_down: bool) {
        let pressed = |key| window.is_key_pressed(key, KeyRepeat::No);

        if pressed(Key::R) {
            self.tool = Tool::Rect;
        }
        if pressed(Key::C) {
            self.tool = Tool::Circle;
        }
        if pressed(Key::L) {
            self.tool = Tool::Line;
        }

        // Not while dragging: the index stored in `Action::Moving` must stay valid.
        if mouse_down {
            return;
        }

        if (pressed(Key::Delete) || pressed(Key::Backspace))
            && let Some(i) = self.selected.take()
        {
            self.shapes.remove(i);
        }
    }

    /// The mouse button was just pressed: decide what this drag will do.
    fn on_press(&mut self, p: Vec2) {
        if let Some(i) = shape_under(&self.shapes, p) {
            self.selected = Some(i);
            self.action = Action::Moving { index: i, last: p };
        } else {
            self.selected = None;
            self.action = Action::Drawing { start: p };
        }
    }

    /// The mouse button is held down.
    fn on_drag(&mut self, p: Vec2) {
        match &mut self.action {
            Action::Moving { index, last } => {
                self.shapes[*index].translate(p - *last);
                *last = p;
            }
            Action::Idle | Action::Drawing { .. } => {}
        }
    }

    /// The mouse button was just released.
    fn on_release(&mut self, p: Vec2) {
        if let Action::Drawing { start } = self.action
            && start != p
        {
            let shape = Shape::from_points(self.tool, start, p, self.color);
            self.shapes.push(shape);
        }
        self.action = Action::Idle;
    }

    /// Redraws everything on the canvas.
    pub fn render(&mut self) {
        self.canvas.clear(WHITE);

        for (i, shape) in self.shapes.iter().enumerate() {
            if self.selected == Some(i) {
                shape.draw_selection(&mut self.canvas);
            }
            shape.draw(&mut self.canvas);
        }

        // Preview of the shape being drawn.
        if let Action::Drawing { start } = self.action {
            Shape::from_points(self.tool, start, self.mouse, GRAY).draw(&mut self.canvas);
        }
    }
}
