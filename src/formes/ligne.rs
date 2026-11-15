#[derive(Clone, Copy)]
pub struct Ligne {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub color: u32,
}

pub fn ligne_depuis_points(a: (f32, f32), b: (f32, f32), color: u32) -> Ligne {
    Ligne {
        x0: a.0,
        y0: a.1,
        x1: b.0,
        y1: b.1,
        color,
    }
}
