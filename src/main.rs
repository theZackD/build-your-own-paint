use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

impl Vec2 {
    fn length(&self) -> f32 {
        dot(*self, *self).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn nombre(pts: &[Vec2]) -> usize {
    pts.len()
}

fn longueur_totale(pts: &[Vec2]) -> f32 {
    let mut total: f32 = 0.0;
    for p in pts {
        total += p.length();
    }
    total
}

fn plus_long(pts: &[Vec2]) -> Option<Vec2> {
    let mut max: f32 = 0.0;
    let mut pl: Option<Vec2> = None;
    for p in pts {
        if p.length() >= max {
            pl = Some(*p);
            max = p.length();
        }
    }

    pl
}

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

    let total2 = longueur_totale(&pts);

    let num = nombre(&pts);

    println!("Total is {}", total);
    println!("Longueur totale est: {}", total2);
    println!("Nombre: {}", num);

    match plus_long(&pts) {
        Some(p) => println!("Plus long: {}", p),
        None => println!("Rien !"),
    };

    match plus_long(&vide) {
        Some(p) => println!("Plus long: {}", p),
        None => println!("Rien !"),
    };
}
