#![allow(dead_code)]
#![allow(unused_imports)]

mod vec2;
use vec2::Vec2;

mod image;
use image::Image;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

#[derive(Clone, Copy)]
struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: u32,
}

fn dessiner(buffer: &mut [u32], r: &Rect) {
    for i in 0..r.w {
        for j in 0..r.h {
            buffer[(r.x + i) + 640 * (r.y + j)] = r.color;
        }
    }
}

fn rect_depuis_points(a: (f32, f32), b: (f32, f32), color: u32) -> Rect {
    let w = (a.0 - b.0).abs() as usize;
    let h = (a.1 - b.1).abs() as usize;
    let x = a.0.min(b.0) as usize;
    let y = a.1.min(b.1) as usize;

    Rect { x, y, w, h, color }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; 640 * 480];
    let mut window = Window::new("titre", 640, 480, WindowOptions::default()).unwrap();

    let mut etait_enfonce = false;
    let mut debut: (f32, f32) = (0.0, 0.0);

    let mut rects: Vec<Rect> = Vec::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);
        for r in &rects {
            dessiner(&mut buffer, &r);
        }

        let pos: (f32, f32);
        match window.get_mouse_pos(MouseMode::Discard) {
            Some(p) => pos = p,
            None => pos = (0.0, 0.0),
        }

        let enfonce = window.get_mouse_down(MouseButton::Left);

        if enfonce {
            let apercu = rect_depuis_points(debut, pos, 0x00888888);
            dessiner(&mut buffer, &apercu);
        }

        window.update_with_buffer(&buffer, 640, 480).unwrap();

        if enfonce && !etait_enfonce {
            debut = pos;
            println!("Début: X: {}, Y: {}", pos.0, pos.1);
        }
        if !enfonce && etait_enfonce {
            if debut != pos {
                rects.push(rect_depuis_points(debut, pos, 0x0000FFCC));
            }

            println!(
                "Début: ({}, {}), Fin: ({}, {})",
                debut.0, debut.1, pos.0, pos.1
            );

            println!("Longuer rects: {}", rects.len());
        }

        etait_enfonce = enfonce;
    }
}
