pub mod disque;
pub mod ligne;
pub mod rect;

pub use disque::{Disque, disque_depuis_points};
pub use ligne::{Ligne, ligne_depuis_points};
pub use rect::{Rect, contient, dessiner, dessiner_contour, rect_depuis_points};

use crate::raster::{dessiner_disque, dessiner_ligne};

#[derive(Clone, Copy)]
pub enum Forme {
    Rectangle(Rect),
    Disque(Disque),
    Ligne(Ligne),
}

#[derive(Clone, Copy)]
pub enum Outil {
    Rectangle,
    Disque,
    Ligne,
}

pub fn forme_depuis_points(outil: Outil, a: (f32, f32), b: (f32, f32), color: u32) -> Forme {
    match outil {
        Outil::Rectangle => Forme::Rectangle(rect_depuis_points(a, b, color)),
        Outil::Disque => Forme::Disque(disque_depuis_points(a, b, color)),
        Outil::Ligne => Forme::Ligne(ligne_depuis_points(a, b, color)),
    }
}

pub fn forme_contient(f: &Forme, p: (f32, f32)) -> bool {
    match f {
        Forme::Rectangle(r) => contient(r, p),
        Forme::Disque(d) => {
            let dx = p.0 - d.cx as f32;
            let dy = p.1 - d.cy as f32;
            dx * dx + dy * dy <= (d.r * d.r) as f32
        }
        Forme::Ligne(l) => {
            let ux = l.x1 - l.x0;
            let uy = l.y1 - l.y0;
            let vx = p.0 - l.x0;
            let vy = p.1 - l.y0;
            let uu = ux * ux + uy * uy;
            let t = if uu == 0.0 {
                0.0
            } else {
                ((vx * ux + vy * uy) / uu).clamp(0.0, 1.0)
            };
            let dx = p.0 - (l.x0 + t * ux);
            let dy = p.1 - (l.y0 + t * uy);
            dx * dx + dy * dy <= 25.0
        }
    }
}

pub fn forme_sous_souris(formes: &[Forme], p: (f32, f32)) -> Option<usize> {
    for (i, f) in formes.iter().enumerate().rev() {
        if forme_contient(f, p) {
            return Some(i);
        }
    }
    None
}

pub fn dessiner_forme(buffer: &mut [u32], f: &Forme) {
    match f {
        Forme::Rectangle(r) => dessiner(buffer, r),
        Forme::Disque(d) => dessiner_disque(buffer, d.cx, d.cy, d.r, d.color),
        Forme::Ligne(l) => dessiner_ligne(
            buffer,
            l.x0 as i32,
            l.y0 as i32,
            l.x1 as i32,
            l.y1 as i32,
            l.color,
        ),
    }
}

pub fn position(f: &Forme) -> (usize, usize) {
    match f {
        Forme::Rectangle(r) => (r.x, r.y),
        Forme::Disque(d) => (d.cx, d.cy),
        Forme::Ligne(l) => (l.x0 as usize, l.y0 as usize),
    }
}

pub fn placer(f: &mut Forme, x: usize, y: usize) {
    match f {
        Forme::Rectangle(r) => {
            r.x = x;
            r.y = y;
        }
        Forme::Disque(d) => {
            d.cx = x;
            d.cy = y;
        }
        Forme::Ligne(l) => {
            l.x1 = l.x1 + x as f32 - l.x0;
            l.x0 = x as f32;
            l.y1 = l.y1 + y as f32 - l.y0;
            l.y0 = y as f32;
        }
    }
}

pub fn dessiner_selection(buffer: &mut [u32], f: &Forme) {
    let bleu = 0x00378ADD;
    match f {
        Forme::Rectangle(r) => dessiner_contour(buffer, r, 3, bleu),
        Forme::Disque(d) => dessiner_disque(buffer, d.cx, d.cy, d.r + 3, bleu),
        Forme::Ligne(l) => {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    dessiner_ligne(
                        buffer,
                        l.x0 as i32 + dx,
                        l.y0 as i32 + dy,
                        l.x1 as i32 + dx,
                        l.y1 as i32 + dy,
                        bleu,
                    );
                }
            }
            dessiner_disque(buffer, l.x0 as usize, l.y0 as usize, 5, bleu);
            dessiner_disque(buffer, l.x1 as usize, l.y1 as usize, 5, bleu);
        }
    }
}
