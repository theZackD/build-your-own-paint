pub fn hsv_vers_u32(h: f32, s: f32, v: f32) -> u32 {
    let c = s * v;
    let h1 = h / 60.0;
    let x = c * (1.0 - (h1 % 2.0 - 1.0).abs());
    let m = v - c;

    let rgb: (f32, f32, f32);
    match h1 as u32 {
        0 => rgb = (c, x, 0.0),
        1 => rgb = (x, c, 0.0),
        2 => rgb = (0.0, c, x),
        3 => rgb = (0.0, x, c),
        4 => rgb = (x, 0.0, c),
        5 => rgb = (c, 0.0, x),
        _ => rgb = (c, 0.0, x),
    }

    let r = ((rgb.0 + m) * 255.0) as u32;
    let g = ((rgb.1 + m) * 255.0) as u32;
    let b = ((rgb.2 + m) * 255.0) as u32;

    (r << 16) | (g << 8) | b
}
