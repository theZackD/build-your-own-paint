//! Colors are stored as `u32` in the `0x00RRGGBB` format expected by minifb.

pub const WHITE: u32 = 0x00FFFFFF;
pub const BLACK: u32 = 0x00000000;
pub const LIGHT_GRAY: u32 = 0x00AAAAAA;
pub const GRAY: u32 = 0x00888888;
pub const DARK_GRAY: u32 = 0x00444444;
pub const SELECTION_BLUE: u32 = 0x00378ADD;

pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn u32_to_rgb(color: u32) -> (u8, u8, u8) {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    (r, g, b)
}

/// Converts a color from HSV (hue in degrees, saturation and value in [0, 1]).
pub fn hsv_to_u32(h: f32, s: f32, v: f32) -> u32 {
    let h = h.rem_euclid(360.0);
    let c = s * v;
    let h1 = h / 60.0;
    let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h1 as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    rgb_to_u32(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
