use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

impl Vec2 {
    pub fn length(&self) -> f32 {
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

pub fn nombre(pts: &[Vec2]) -> usize {
    pts.len()
}

pub fn longueur_totale(pts: &[Vec2]) -> f32 {
    let mut total: f32 = 0.0;
    for p in pts {
        total += p.length();
    }
    total
}

pub fn plus_long(pts: &[Vec2]) -> Option<Vec2> {
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

pub fn normaliser(a: &Vec2) -> Result<Vec2, String> {
    let len = a.length();
    if len == 0.0 {
        Err(String::from("Vecteur nul !"))
    } else {
        Ok(Vec2 {
            x: a.x / len,
            y: a.y / len,
        })
    }
}
