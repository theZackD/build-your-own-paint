use crate::config::{HAUTEUR, LARGEUR};
use crate::couleur::hsv_vers_u32;
use crate::rect::Rect;

pub fn dessiner(buffer: &mut [u32], r: &Rect) {
    for i in 0..r.w {
        for j in 0..r.h {
            if r.x + i < LARGEUR && r.y + j < HAUTEUR {
                buffer[(r.x + i) + LARGEUR * (r.y + j)] = r.color;
            }
        }
    }
}

pub fn dessiner_barre_teinte(buffer: &mut [u32], x0: usize, y0: usize, w: usize, h: usize) {
    for i in 0..h {
        let teinte = hsv_vers_u32((i as f32 / h as f32) * 360.0, 1.0, 1.0);
        for j in 0..w {
            if x0 + j < LARGEUR && y0 + i < HAUTEUR {
                buffer[(x0 + j) + LARGEUR * (y0 + i)] = teinte;
            }
        }
    }
}
