use minifb::{MouseButton, MouseMode, Window};

use crate::action::Action;
use crate::canvas::Canvas;
use crate::color::{GRAY, WHITE, rgb_to_u32};
use crate::shapes::Rect;
use crate::vec2::Vec2;

/// All the state of the application.
pub struct App {
    canvas: Canvas,
    rects: Vec<Rect>,
    action: Action,
    color: u32,
    mouse: Vec2,
    was_down: bool,
}

impl App {
    pub fn new(width: usize, height: usize) -> App {
        App {
            canvas: Canvas::new(width, height),
            rects: Vec::new(),
            action: Action::Idle,
            color: rgb_to_u32(50, 120, 220),
            mouse: Vec2::ZERO,
            was_down: false,
        }
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    /// Reads the mouse and updates the state.
    pub fn update(&mut self, window: &Window) {
        self.mouse = window
            .get_mouse_pos(MouseMode::Clamp)
            .map(Vec2::from)
            .unwrap_or(Vec2::ZERO);
        let down = window.get_mouse_down(MouseButton::Left);

        // Compare with the previous frame to detect press and release.
        if down && !self.was_down {
            self.on_press(self.mouse);
        }
        if !down && self.was_down {
            self.on_release(self.mouse);
        }
        self.was_down = down;
    }

    /// The mouse button was just pressed: start drawing.
    fn on_press(&mut self, p: Vec2) {
        self.action = Action::Drawing { start: p };
    }

    /// The mouse button was just released.
    fn on_release(&mut self, p: Vec2) {
        if let Action::Drawing { start } = self.action
            && start != p
        {
            self.rects.push(Rect::from_points(start, p, self.color));
        }
        self.action = Action::Idle;
    }

    /// Redraws everything on the canvas.
    pub fn render(&mut self) {
        self.canvas.clear(WHITE);

        for rect in &self.rects {
            rect.draw(&mut self.canvas);
        }

        // Preview of the rectangle being drawn.
        if let Action::Drawing { start } = self.action {
            Rect::from_points(start, self.mouse, GRAY).draw(&mut self.canvas);
        }
    }
}
