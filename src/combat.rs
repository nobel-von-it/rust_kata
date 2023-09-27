pub fn combat(h: f32, d: f32) -> f32 {
    let r = h - d;
    if r > 0.0 {
        r
    } else {
        0.0
    }
}