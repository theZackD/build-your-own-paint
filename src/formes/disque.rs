#[derive(Clone, Copy)]
pub struct Disque {
    pub cx: usize,
    pub cy: usize,
    pub r: usize,
    pub color: u32,
}

pub fn disque_depuis_points(centre: (f32, f32), bord: (f32, f32), color: u32) -> Disque {
    let dx = bord.0 - centre.0;
    let dy = bord.1 - centre.1;
    Disque {
        cx: centre.0 as usize,
        cy: centre.1 as usize,
        r: (dx * dx + dy * dy).sqrt() as usize,
        color,
    }
}
