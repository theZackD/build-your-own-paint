use crate::canvas::Canvas;
use crate::color::{BLACK, WHITE, hsv_to_u32};
use crate::shapes::Rect;
use crate::vec2::Vec2;

const SIZE: f32 = 360.0;
const BAR_WIDTH: f32 = 20.0;
const GAP: f32 = 20.0;
const MARGIN: f32 = 20.0;

/// An HSV color picker: a vertical hue bar and a saturation/value square.
pub struct ColorPicker {
    pub open: bool,
    /// In degrees, [0, 360).
    hue: f32,
    /// In [0, 1].
    sat: f32,
    /// In [0, 1].
    val: f32,
    hue_bar: Rect,
    sv_square: Rect,
}

impl ColorPicker {
    pub fn new(canvas_width: f32, canvas_height: f32) -> ColorPicker {
        let bar_x = canvas_width - MARGIN - BAR_WIDTH;
        let top = (canvas_height - SIZE) / 2.0;
        ColorPicker {
            open: false,
            hue: 0.0,
            sat: 0.0,
            val: 0.0,
            hue_bar: Rect::new(bar_x, top, BAR_WIDTH, SIZE, WHITE),
            sv_square: Rect::new(bar_x - GAP - SIZE, top, SIZE, SIZE, WHITE),
        }
    }

    pub fn color(&self) -> u32 {
        hsv_to_u32(self.hue, self.sat, self.val)
    }

    pub fn is_on_hue_bar(&self, p: Vec2) -> bool {
        self.open && self.hue_bar.contains(p)
    }

    pub fn is_on_sv_square(&self, p: Vec2) -> bool {
        self.open && self.sv_square.contains(p)
    }

    /// Sets the hue from the mouse height on the bar.
    pub fn pick_hue(&mut self, p: Vec2) {
        let t = (p.y - self.hue_bar.pos.y) / self.hue_bar.size.y;
        self.hue = t.clamp(0.0, 0.999) * 360.0;
    }

    /// Sets saturation (horizontal) and value (vertical) from the mouse.
    pub fn pick_sat_val(&mut self, p: Vec2) {
        let local = p - self.sv_square.pos;
        self.sat = (local.x / self.sv_square.size.x).clamp(0.0, 1.0);
        self.val = 1.0 - (local.y / self.sv_square.size.y).clamp(0.0, 1.0);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        if !self.open {
            return;
        }

        // Hue bar: one hue per row.
        self.hue_bar.draw_outline(canvas, 2.0, BLACK);
        let Rect { pos, size, .. } = self.hue_bar;
        let rows = size.y as i32;
        for row in 0..rows {
            let hue = row as f32 / rows as f32 * 360.0;
            let color = hsv_to_u32(hue, 1.0, 1.0);
            canvas.fill_rect(pos.x as i32, pos.y as i32 + row, size.x as i32, 1, color);
        }

        // Saturation/value square for the current hue.
        self.sv_square.draw_outline(canvas, 2.0, BLACK);
        let Rect { pos, size, .. } = self.sv_square;
        let (w, h) = (size.x as i32, size.y as i32);
        for i in 0..w {
            let s = i as f32 / w as f32;
            for j in 0..h {
                let v = 1.0 - j as f32 / h as f32;
                canvas.put_pixel(
                    pos.x as i32 + i,
                    pos.y as i32 + j,
                    hsv_to_u32(self.hue, s, v),
                );
            }
        }

        // Cursor on the square.
        let cursor = pos + Vec2::new(self.sat * size.x, (1.0 - self.val) * size.y);
        let (cx, cy) = (cursor.x as i32, cursor.y as i32);
        canvas.fill_circle(cx, cy, 10, BLACK);
        canvas.fill_circle(cx, cy, 9, WHITE);
        canvas.fill_circle(cx, cy, 7, self.color());

        // Cursor on the hue bar.
        let bar = self.hue_bar;
        let hx = (bar.pos.x + bar.size.x / 2.0) as i32;
        let hy = (bar.pos.y + self.hue / 360.0 * bar.size.y) as i32;
        canvas.fill_circle(hx, hy, 10, BLACK);
        canvas.fill_circle(hx, hy, 8, hsv_to_u32(self.hue, 1.0, 1.0));
    }
}
