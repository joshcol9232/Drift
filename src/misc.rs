use raylib::{Vector2, consts};
use crate::TWO_PI;

#[inline]
pub fn get_components(mag: f32, angle: f32) -> Vector2 {
	Vector2 { x: mag * angle.sin(), y: mag * angle.cos() }
}

#[inline]
pub fn rotate_vec(v: Vector2, angle: f32) -> Vector2 {
	let a_c = angle.cos();
	let a_s = angle.sin();

	Vector2 { x: (v.x * a_c) - (v.y * a_s), y: (v.x * a_s) + (v.y * a_c) }
}

#[inline]
pub fn get_angle_to(from: &Vector2, to: &Vector2) -> f32 {
    (to.y - from.y).atan2(to.x - from.x)
}

// For 0 -> 2pi range. Returns reflex angles too
pub fn get_angle_diff(target: f32, current: f32) -> f32 {
	let a = (target - current) % TWO_PI;
	let b = (current - target) % TWO_PI;

	if a < b { println!("returning a"); -a } else { println!("returning b"); b }
}