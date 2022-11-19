use rand::prelude::*;

pub fn rand_range(from: f32, to: f32) -> f32 {
    let mut r = random::<f32>();

    if r == 0.0 {
        r = 0.01;
    }

    let min = f32::min(from, to);
    let max = f32::max(from, to);

    return min + (max - min) * r;
}
