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
use dessin::{dessiner, dessiner_barre_teinte};

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

fn main() {
    let mut buffer: Vec<u32> = vec![0; LARGEUR * HAUTEUR];
    let mut window = Window::new("titre", LARGEUR, HAUTEUR, WindowOptions::default()).unwrap();

    let mut etait_enfonce = false;
    let mut debut: (f32, f32) = (0.0, 0.0);

    let mut rects: Vec<Rect> = Vec::new();

    let mut action = Action::Rien;
    let mut decalage: (f32, f32) = (0.0, 0.0);

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
            match rect_sous_souris(&rects, pos) {
                Some(i) => {
                    decalage = (pos.0 - (rects[i].x as f32), pos.1 - (rects[i].y as f32));
                    action = Action::Deplacement(i);
                }
                None => action = Action::Dessin,
            };
        }
        if !enfonce && etait_enfonce {
            if debut != pos && matches!(action, Action::Dessin) {
                rects.push(rect_depuis_points(debut, pos, 0x0000FFCC));
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

        buffer.fill(0);
        for r in &rects {
            dessiner(&mut buffer, &r);
        }
        let xbar = LARGEUR - 20 - 10;
        let ybar = (HAUTEUR - 360) / 2;

        dessiner_barre_teinte(&mut buffer, xbar, ybar, 20, 360);
        if enfonce && matches!(action, Action::Dessin) {
            let apercu = rect_depuis_points(debut, pos, 0x00888888);
            dessiner(&mut buffer, &apercu);
        }

        window
            .update_with_buffer(&buffer, LARGEUR, HAUTEUR)
            .unwrap();

        etait_enfonce = enfonce;
    }
}
