#![allow(dead_code)]
#![allow(unused_imports)]

mod vec2;
use vec2::Vec2;

mod image;
use image::Image;

mod config;
use config::{HAUTEUR, LARGEUR};

mod rect;
use rect::{Rect, contient, rect_depuis_points, rect_sous_souris};

mod action;
use action::Action;

mod couleur;
use couleur::hsv_vers_u32;

mod dessin;
use dessin::{
    dessiner, dessiner_barre_teinte, dessiner_carre_sv, dessiner_contour, dessiner_disque,
    dessiner_ligne,
};

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

fn main() {
    let mut buffer: Vec<u32> = vec![0; LARGEUR * HAUTEUR];
    let mut window = Window::new("titre", LARGEUR, HAUTEUR, WindowOptions::default()).unwrap();

    let mut etait_enfonce = false;
    let mut debut: (f32, f32) = (0.0, 0.0);

    let mut rects: Vec<Rect> = Vec::new();

    let mut action = Action::Rien;
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
        color: 0xFFFFFFFF,
    };

    let mut picker_ouvert: bool = false;

    let color_but = Rect {
        x: LARGEUR - 60,
        y: 20,
        w: 40,
        h: 40,
        color: couleur,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pos: (f32, f32);
        match window.get_mouse_pos(MouseMode::Clamp) {
            Some(p) => pos = p,
            None => pos = (0.0, 0.0),
        }

        let enfonce = window.get_mouse_down(MouseButton::Left);

        if enfonce && !etait_enfonce {
            debut = pos;
            println!("Début: X: {}, Y: {}", pos.0, pos.1);
            if contient(&color_wheel, pos) {
                action = Action::Teinte
            } else if contient(&satval_wheel, pos) {
                action = Action::Sv
            } else {
                match rect_sous_souris(&rects, pos) {
                    Some(i) => {
                        decalage = (pos.0 - (rects[i].x as f32), pos.1 - (rects[i].y as f32));
                        action = Action::Deplacement(i);
                    }
                    None => action = Action::Dessin,
                };
            }
            if contient(&color_but, pos) {
                picker_ouvert = !picker_ouvert;
                println!("Picker: {}", picker_ouvert)
            }
        }
        if !enfonce && etait_enfonce {
            if debut != pos && matches!(action, Action::Dessin) {
                rects.push(rect_depuis_points(debut, pos, couleur));
            }

            action = Action::Rien;

            println!(
                "Début: ({}, {}), Fin: ({}, {})",
                debut.0, debut.1, pos.0, pos.1
            );

            println!("Longuer rects: {}", rects.len());
        }

        if enfonce && let Action::Deplacement(i) = action {
            rects[i].x = (pos.0 - decalage.0) as usize;
            rects[i].y = (pos.1 - decalage.1) as usize;
        }

        if enfonce
            && (matches!(action, Action::Teinte) || matches!(action, Action::Sv))
            && picker_ouvert
        {
            if enfonce && let Action::Teinte = action {
                teinte = (pos.1 - ybar as f32).clamp(0.0, 359.0);
            }

            if enfonce && let Action::Sv = action {
                sat = ((pos.0 - xbar as f32 + 360.0 + 20.0) / 360.0).clamp(0.0, 1.0);
                val = 1.0 - ((pos.1 - ybar as f32) / 360.0).clamp(0.0, 1.0)
            }
            couleur = hsv_vers_u32(teinte, sat, val);
        }

        buffer.fill(0x00FFFFFF);

        for r in &rects {
            dessiner(&mut buffer, &r);
        }

        dessiner_contour(&mut buffer, &color_but, 6, 0);
        dessiner_contour(&mut buffer, &color_but, 3, 0x00AAAAAA);
        dessiner(
            &mut buffer,
            &Rect {
                x: LARGEUR - 60,
                y: 20,
                w: 40,
                h: 40,
                color: couleur,
            },
        );

        if picker_ouvert{dessiner_contour(
            &mut buffer,
            &Rect {
                x: xbar,
                y: ybar,
                w: 20,
                h: 360,
                color: 0,
            },
            2,
            0,
        );}

        if enfonce && matches!(action, Action::Dessin) {
            let apercu = rect_depuis_points(debut, pos, 0x00888888);
            dessiner(&mut buffer, &apercu);
        }
        if picker_ouvert {
            dessiner_barre_teinte(&mut buffer, xbar, ybar, 20, 360);

            dessiner_carre_sv(&mut buffer, xbar - 20 - 360, ybar, 360, teinte);

            dessiner_contour(&mut buffer, &satval_wheel, 2, 0);
            dessiner_carre_sv(&mut buffer, satval_wheel.x, satval_wheel.y, 360, teinte);

            let cx = satval_wheel.x + (sat * 360.0) as usize;
            let cy = satval_wheel.y + ((1.0 - val) * 360.0) as usize;
            dessiner_disque(&mut buffer, cx, cy, 10, 0x00000000);
            dessiner_disque(&mut buffer, cx, cy, 9, 0x00FFFFFF);
            dessiner_disque(&mut buffer, cx, cy, 7, couleur);

            dessiner_disque(&mut buffer, xbar, ybar + teinte as usize, 10, 0x00000000);
            dessiner_disque(
                &mut buffer,
                xbar,
                ybar + teinte as usize,
                8,
                hsv_vers_u32(teinte, 1.0, 1.0),
            );
        }
        window
            .update_with_buffer(&buffer, LARGEUR, HAUTEUR)
            .unwrap();

        etait_enfonce = enfonce;
    }
}
