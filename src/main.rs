mod vec2;
use vec2::Vec2;

mod image;
use image::Image;

fn main() {
    let mut pts: Vec<Vec2> = Vec::new();
    pts.push(Vec2 { x: 3.0, y: 4.0 });
    pts.push(Vec2 { x: 1.0, y: 2.0 });
    pts.push(Vec2 { x: 6.0, y: 8.0 });

    let vide: Vec<Vec2> = Vec::new();

    let mut total: f32 = 0.0;

    for p in &pts {
        total += p.length();
    }

    let total2 = vec2::longueur_totale(&pts);

    let num = vec2::nombre(&pts);

    println!("Total is {}", total);
    println!("Longueur totale est: {}", total2);
    println!("Nombre: {}", num);

    match vec2::plus_long(&pts) {
        Some(p) => println!("Plus long: {}", p),
        None => println!("Rien !"),
    };

    match vec2::plus_long(&vide) {
        Some(p) => println!("Plus long: {}", p),
        None => println!("Rien !"),
    };

    match vec2::normaliser(&Vec2 { x: 3.0, y: 4.0 }) {
        Ok(r) => println!("Vecteur: {}", r),
        Err(e) => println!("Erreur : {}", e),
    }

    match vec2::normaliser(&Vec2 { x: 0.0, y: 0.0 }) {
        Ok(r) => println!("Vecteur: {}", r),
        Err(e) => println!("Erreur : {}", e),
    }

    let mut img = Image::new(4, 3);
    println!(
        "Width: {}\nHeight: {}\nPixels length: {}",
        img.width,
        img.height,
        img.pixels.len()
    );

    img.set_pixel(1, 2, 255, 0, 0);
    img.set_pixel(10, 10, 1, 2, 3);
    println!("{}", img.pixels[(2 * 4 + 1) * 3]);

    match image::save_ppm(&img, "out.ppm") {
        Ok(()) => println!("fichier écrit"),
        Err(e) => println!("erreur : {}", e),
    }

    let mut img2 = Image::new(6, 4);
    image::fill_rectangle(&mut img2, 3, 2, 3, 2, 255, 0, 0);

    match image::save_ppm(&img2, "out2.ppm") {
        Ok(()) => println!("fichier écrit"),
        Err(e) => println!("erreur : {}", e),
    }
}
