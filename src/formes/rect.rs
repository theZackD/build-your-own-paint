use crate::config::{HAUTEUR, LARGEUR};

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub color: u32,
}

pub fn rect_depuis_points(a: (f32, f32), b: (f32, f32), color: u32) -> Rect {
    let w = (a.0 - b.0).abs() as usize;
    let h = (a.1 - b.1).abs() as usize;
    let x = a.0.min(b.0) as usize;
    let y = a.1.min(b.1) as usize;

    Rect { x, y, w, h, color }
}

pub fn contient(r: &Rect, p: (f32, f32)) -> bool {
    (p.0 as usize) >= r.x
        && (p.0 as usize) < r.x + r.w
        && (p.1 as usize) >= r.y
        && (p.1 as usize) < r.y + r.h
}

pub fn dessiner(buffer: &mut [u32], r: &Rect) {
    for i in 0..r.w {
        for j in 0..r.h {
            if r.x + i < LARGEUR && r.y + j < HAUTEUR {
                buffer[(r.x + i) + LARGEUR * (r.y + j)] = r.color;
            }
        }
    }
}

pub fn dessiner_contour(buffer: &mut [u32], r: &Rect, e: usize, color: u32) {
    let r_prime = Rect {
        x: r.x.saturating_sub(e),
        y: r.y.saturating_sub(e),
        w: r.w + 2 * e,
        h: r.h + 2 * e,
        color,
    };
    dessiner(buffer, &r_prime);
}
