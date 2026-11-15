use crate::dessin::{dessiner, dessiner_disque};
use crate::rect::{Rect, contient, rect_depuis_points};

#[derive(Clone, Copy)]
pub struct Disque {
    pub cx: usize,
    pub cy: usize,
    pub r: usize,
    pub color: u32,
}

#[derive(Clone, Copy)]
pub enum Forme {
    Rectangle(Rect),
    Disque(Disque),
}

#[derive(Clone, Copy)]
pub enum Outil {
    Rectangle,
    Disque,
}

pub fn disque_depuis_points(centre: (f32, f32), bord: (f32, f32), color: u32) -> Disque {
    let dx = bord.0 - centre.0;
    let dy = bord.1 - centre.1;
    Disque {
        cx: centre.0 as usize,
        cy: centre.1 as usize,
        r: (dx * dx + dy * dy).sqrt() as usize,
        color,
    }
}

pub fn forme_depuis_points(outil: Outil, a: (f32, f32), b: (f32, f32), color: u32) -> Forme {
    match outil {
        Outil::Rectangle => Forme::Rectangle(rect_depuis_points(a, b, color)),
        Outil::Disque => Forme::Disque(disque_depuis_points(a, b, color)),
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
    }
}

pub fn position(f: &Forme) -> (usize, usize) {
    match f {
        Forme::Rectangle(r) => (r.x, r.y),
        Forme::Disque(d) => (d.cx, d.cy),
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
    }
}
