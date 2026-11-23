//! Colors are stored as `u32` in the `0x00RRGGBB` format expected by minifb.

pub const WHITE: u32 = 0x00FFFFFF;
pub const GRAY: u32 = 0x00888888;
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
