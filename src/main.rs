mod vec2;

use vec2::{Vec2, longest};

fn main() {
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(1.0, 2.0);

    println!("a = {a}, b = {b}");
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * 2 = {}", a * 2.0);
    println!("a . b = {}", a.dot(b));
    println!("|a| = {}", a.length());

    match a.normalize() {
        Ok(n) => println!("a normalized = {n}"),
        Err(e) => println!("error: {e}"),
    }
    match Vec2::ZERO.normalize() {
        Ok(n) => println!("zero normalized = {n}"),
        Err(e) => println!("error: {e}"),
    }

    match longest(&[a, b]) {
        Some(p) => println!("longest = {p}"),
        None => println!("no points"),
    }
    println!("longest of nothing = {:?}", longest(&[]));
}
