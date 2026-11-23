use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window};

use crate::action::Action;
use crate::canvas::Canvas;
use crate::color::{GRAY, WHITE};
use crate::shapes::{Shape, Tool, shape_under};
use crate::ui::{ColorPicker, Toolbar};
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
    toolbar: Toolbar,
    picker: ColorPicker,
    color: u32,
    mouse: Vec2,
    was_down: bool,
}

impl App {
    pub fn new(width: usize, height: usize) -> App {
        let picker = ColorPicker::new(width as f32, height as f32);
        App {
            canvas: Canvas::new(width, height),
            shapes: Vec::new(),
            tool: Tool::Rect,
            action: Action::Idle,
            selected: None,
            toolbar: Toolbar::new(width as f32),
            color: picker.color(),
            picker,
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
        self.action = Action::Idle;

        if self.toolbar.color_button.contains(p) {
            self.picker.open = !self.picker.open;
        } else if let Some(tool) = self.toolbar.tool_at(p) {
            self.tool = tool;
        } else if self.picker.is_on_hue_bar(p) {
            self.action = Action::PickingHue;
        } else if self.picker.is_on_sv_square(p) {
            self.action = Action::PickingSatVal;
        } else if let Some(i) = shape_under(&self.shapes, p) {
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
            Action::PickingHue => self.picker.pick_hue(p),
            Action::PickingSatVal => self.picker.pick_sat_val(p),
            Action::Idle | Action::Drawing { .. } => {}
        }
        if self.picker.open {
            self.color = self.picker.color();
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

        self.toolbar.draw(&mut self.canvas, self.tool, self.color);
        self.picker.draw(&mut self.canvas);
    }
}
