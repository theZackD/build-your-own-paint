#![allow(dead_code)]
#![allow(unused_imports)]

mod vec2;
use vec2::Vec2;

mod image;
use image::Image;

mod config;
use config::{HAUTEUR, LARGEUR};

mod raster;
use raster::{dessiner_barre_teinte, dessiner_carre_sv, dessiner_disque, dessiner_ligne};

mod formes;
use formes::{
    Forme, Outil, Rect, contient, dessiner, dessiner_contour, dessiner_forme, dessiner_selection,
    forme_depuis_points, forme_sous_souris, placer, position,
};

mod action;
use action::Action;

mod couleur;
use couleur::hsv_vers_u32;

use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};

fn main() {
    let mut buffer: Vec<u32> = vec![0; LARGEUR * HAUTEUR];
    let mut window = Window::new("titre", LARGEUR, HAUTEUR, WindowOptions::default()).unwrap();

    let mut etait_enfonce = false;
    let mut debut: (f32, f32) = (0.0, 0.0);

    let mut formes: Vec<Forme> = Vec::new();
    let mut outil = Outil::Rectangle;

    let mut action = Action::Rien;
    let mut selected: Option<usize> = None;
    let mut decalage: (f32, f32) = (0.0, 0.0);

    let mut teinte: f32 = 0.0;
    let mut sat: f32 = 0.0;
    let mut val: f32 = 0.0;
    let mut couleur = hsv_vers_u32(teinte, sat, val);

    let xbar = LARGEUR - 30 - 10;
    let ybar = (HAUTEUR - 360) / 2;

    let color_wheel = Rect {
        x: xbar,
        y: ybar,
        w: 20,
        h: 360,
        color: 0xFFFFFF,
    };

    let satval_wheel = Rect {
        x: xbar - 20 - 360,
        y: ybar,
        w: 360,
        h: 360,
        color: 0xFFFFFF,
    };

    let mut picker_ouvert: bool = false;

    let color_but = Rect {
        x: LARGEUR - 60,
        y: 20,
        w: 40,
        h: 40,
        color: couleur,
    };

    let rect_but = Rect {
        x: LARGEUR - 230,
        y: 20,
        w: 40,
        h: 40,
        color: 0x00FFFFFF,
    };
    let disque_but = Rect {
        x: LARGEUR - 180,
        ..rect_but
    };
    let ligne_but = Rect {
        x: LARGEUR - 130,
        ..rect_but
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pos: (f32, f32);
        match window.get_mouse_pos(MouseMode::Clamp) {
            Some(p) => pos = p,
            None => pos = (0.0, 0.0),
        }

        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            outil = Outil::Rectangle;
        }
        if window.is_key_pressed(Key::D, KeyRepeat::No) {
            outil = Outil::Disque;
        }

        let enfonce = window.get_mouse_down(MouseButton::Left);

        if (window.is_key_pressed(Key::Delete, KeyRepeat::No)
            || window.is_key_pressed(Key::Backspace, KeyRepeat::No))
            && !enfonce
        {
            if let Some(i) = selected {
                formes.remove(i);
                selected = None;
            }
        }

        if enfonce && !etait_enfonce {
            debut = pos;
            if contient(&color_but, pos) {
                picker_ouvert = !picker_ouvert;
                action = Action::Rien;
            } else if contient(&rect_but, pos) {
                outil = Outil::Rectangle;
                action = Action::Rien;
            } else if contient(&disque_but, pos) {
                outil = Outil::Disque;
                action = Action::Rien;
            } else if contient(&ligne_but, pos) {
                outil = Outil::Ligne;
                action = Action::Rien;
            } else if picker_ouvert && contient(&color_wheel, pos) {
                action = Action::Teinte;
            } else if picker_ouvert && contient(&satval_wheel, pos) {
                action = Action::Sv;
            } else {
                match forme_sous_souris(&formes, pos) {
                    Some(i) => {
                        let (fx, fy) = position(&formes[i]);
                        decalage = (pos.0 - fx as f32, pos.1 - fy as f32);
                        action = Action::Deplacement(i);
                        selected = Some(i);
                    }
                    None => {
                        action = Action::Dessin;
                        selected = None;
                    }
                }
            }
        }

        if !enfonce && etait_enfonce {
            if debut != pos && matches!(action, Action::Dessin) {
                formes.push(forme_depuis_points(outil, debut, pos, couleur));
            }
            action = Action::Rien;
        }

        if enfonce && let Action::Deplacement(i) = action {
            placer(
                &mut formes[i],
                (pos.0 - decalage.0) as usize,
                (pos.1 - decalage.1) as usize,
            );
        }

        if enfonce && picker_ouvert {
            if let Action::Teinte = action {
                teinte = (pos.1 - ybar as f32).clamp(0.0, 359.0);
            }
            if let Action::Sv = action {
                sat = ((pos.0 - satval_wheel.x as f32) / 360.0).clamp(0.0, 1.0);
                val = 1.0 - ((pos.1 - satval_wheel.y as f32) / 360.0).clamp(0.0, 1.0);
            }
            couleur = hsv_vers_u32(teinte, sat, val);
        }

        buffer.fill(0x00FFFFFF);

        for (i, f) in formes.iter().enumerate() {
            if selected == Some(i) {
                dessiner_selection(&mut buffer, f);
            }
            dessiner_forme(&mut buffer, f);
        }

        if enfonce && matches!(action, Action::Dessin) {
            dessiner_forme(
                &mut buffer,
                &forme_depuis_points(outil, debut, pos, 0x00888888),
            );
        }

        dessiner_contour(&mut buffer, &color_but, 6, 0x00000000);
        dessiner_contour(&mut buffer, &color_but, 3, 0x00AAAAAA);
        dessiner(
            &mut buffer,
            &Rect {
                color: couleur,
                ..color_but
            },
        );

        let rect_actif = matches!(outil, Outil::Rectangle);
        dessiner_contour(
            &mut buffer,
            &rect_but,
            if rect_actif { 6 } else { 3 },
            if rect_actif { 0x00000000 } else { 0x00AAAAAA },
        );
        dessiner(&mut buffer, &rect_but);
        dessiner(
            &mut buffer,
            &Rect {
                x: rect_but.x + 8,
                y: rect_but.y + 12,
                w: 24,
                h: 16,
                color: 0x00444444,
            },
        );

        let disque_actif = matches!(outil, Outil::Disque);
        dessiner_contour(
            &mut buffer,
            &disque_but,
            if disque_actif { 6 } else { 3 },
            if disque_actif { 0x00000000 } else { 0x00AAAAAA },
        );
        dessiner(&mut buffer, &disque_but);
        dessiner_disque(
            &mut buffer,
            disque_but.x + 20,
            disque_but.y + 20,
            12,
            0x00444444,
        );

        let ligne_actif = matches!(outil, Outil::Ligne);
        dessiner_contour(
            &mut buffer,
            &ligne_but,
            if ligne_actif { 6 } else { 3 },
            if ligne_actif { 0x00000000 } else { 0x00AAAAAA },
        );
        dessiner(&mut buffer, &ligne_but);
        dessiner_ligne(
            &mut buffer,
            (ligne_but.x + 8) as i32,
            (ligne_but.y + 32) as i32,
            (ligne_but.x + 32) as i32,
            (ligne_but.y + 8) as i32,
            0x00444444,
        );

        if picker_ouvert {
            dessiner_contour(&mut buffer, &color_wheel, 2, 0x00000000);
            dessiner_barre_teinte(
                &mut buffer,
                color_wheel.x,
                color_wheel.y,
                color_wheel.w,
                color_wheel.h,
            );

            dessiner_contour(&mut buffer, &satval_wheel, 2, 0x00000000);
            dessiner_carre_sv(
                &mut buffer,
                satval_wheel.x,
                satval_wheel.y,
                satval_wheel.w,
                teinte,
            );

            let cx = satval_wheel.x + (sat * 360.0) as usize;
            let cy = satval_wheel.y + ((1.0 - val) * 360.0) as usize;
            dessiner_disque(&mut buffer, cx, cy, 10, 0x00000000);
            dessiner_disque(&mut buffer, cx, cy, 9, 0x00FFFFFF);
            dessiner_disque(&mut buffer, cx, cy, 7, couleur);

            let hy = ybar + teinte as usize;
            dessiner_disque(&mut buffer, xbar, hy, 10, 0x00000000);
            dessiner_disque(&mut buffer, xbar, hy, 8, hsv_vers_u32(teinte, 1.0, 1.0));
        }

        window
            .update_with_buffer(&buffer, LARGEUR, HAUTEUR)
            .unwrap();

        etait_enfonce = enfonce;
    }
}
