use crate::config::{HAUTEUR, LARGEUR};
use crate::couleur::hsv_vers_u32;

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

pub fn dessiner_disque(buffer: &mut [u32], cx: usize, cy: usize, r: usize, color: u32) {
    let r = r as i32;
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r * r {
                let x = cx as i32 + dx;
                let y = cy as i32 + dy;
                if x >= 0 && x < LARGEUR as i32 && y >= 0 && y < HAUTEUR as i32 {
                    buffer[x as usize + LARGEUR * y as usize] = color;
                }
            }
        }
    }
}

pub fn dessiner_carre_sv(buffer: &mut [u32], x0: usize, y0: usize, taille: usize, teinte: f32) {
    for i in 0..taille {
        let s = i as f32 / taille as f32;
        for j in 0..taille {
            let v = 1.0 - j as f32 / taille as f32;
            let color = hsv_vers_u32(teinte, s, v);
            if x0 + i < LARGEUR && y0 + j < HAUTEUR {
                buffer[(x0 + i) + LARGEUR * (y0 + j)] = color;
            }
        }
    }
}

pub fn mettre_pixel(buffer: &mut [u32], x: i32, y: i32, color: u32) {
    if x >= 0 && x < LARGEUR as i32 && y >= 0 && y < HAUTEUR as i32 {
        buffer[x as usize + LARGEUR * y as usize] = color;
    }
}

pub fn dessiner_ligne(buffer: &mut [u32], x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let pas = dx.abs().max(dy.abs());

    if pas == 0 {
        mettre_pixel(buffer, x0, y0, color);
        return;
    }

    for k in 0..=pas {
        let x = x0 + dx * k / pas;
        let y = y0 + dy * k / pas;
        mettre_pixel(buffer, x, y, color);
    }
}
